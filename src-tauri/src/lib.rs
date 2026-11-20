mod engine;
mod history;
mod i18n;
mod platform;
mod snapping;
mod storage;

use engine::{Engine, Settings, Snapshot, Status, PRESETS};
use history::WorkHistory;
use i18n::{text, APP_NAME};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Mutex,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tauri::menu::{CheckMenuItem, Menu, MenuItem, Submenu};
use tauri::{
    Emitter, LogicalSize, Manager, PhysicalPosition, State, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};
use tauri_plugin_autostart::ManagerExt;

struct Runtime {
    engine: Mutex<Engine>,
    path: PathBuf,
    snap_targets: Mutex<Vec<snapping::Target>>,
    notification_started: Mutex<Option<Instant>>,
    history: Mutex<WorkHistory>,
    recent_reward: Mutex<Option<(i64, Instant)>>,
    history_hover: AtomicU64,
    history_hovered: AtomicBool,
    completion_history_until: Mutex<Option<Instant>>,
    reward_generation: AtomicU64,
}

const TIMER_WIDTH: f64 = 172.;
const TIMER_HEIGHT: f64 = 542.;
const LEGACY_TIMER_HEIGHT: f64 = 474.;
fn timer_height(s: &Settings) -> f64 {
    if s.history_enabled {
        TIMER_HEIGHT
    } else {
        LEGACY_TIMER_HEIGHT
    }
}
#[cfg(any(target_os = "windows", target_os = "macos"))]
const TIMER_RADIUS: f64 = 82.;
// Roughly 70% of the former expanded popup in each direction.
const HISTORY_WIDTH: f64 = 490.;
const HISTORY_HEIGHT: f64 = 192.;
const NOTIFICATION_WIDTH: f64 = 740.;
const NOTIFICATION_HEIGHT: f64 = 350.;
#[cfg(any(target_os = "windows", target_os = "macos"))]
const NOTIFICATION_RADIUS: f64 = 48.;

#[derive(serde::Serialize)]
struct DragOrigin {
    x: i32,
    y: i32,
    scale: f64,
}

#[tauri::command]
fn get_drag_origin(window: WebviewWindow, rt: State<Runtime>) -> Result<DragOrigin, String> {
    if window.label() != "timer" {
        return Err("errorMove".into());
    }
    let p = window.outer_position().map_err(|e| e.to_string())?;
    let scale = window.scale_factor().map_err(|e| e.to_string())?;
    let mut targets = Vec::new();
    for m in window.available_monitors().map_err(|e| e.to_string())? {
        let r = m.work_area();
        targets.push(snapping::Target {
            rect: snapping::Rect {
                left: r.position.x,
                top: r.position.y,
                right: r.position.x.saturating_add(r.size.width as i32),
                bottom: r.position.y.saturating_add(r.size.height as i32),
            },
            screen: true,
        });
    }
    targets.extend(
        platform::window_frames(scale)
            .into_iter()
            .map(|rect| snapping::Target {
                rect,
                screen: false,
            }),
    );
    *rt.snap_targets.lock().unwrap() = targets;
    Ok(DragOrigin {
        x: p.x,
        y: p.y,
        scale,
    })
}

#[tauri::command]
fn move_timer(
    window: WebviewWindow,
    rt: State<Runtime>,
    x: i32,
    y: i32,
    finished: bool,
) -> Result<(), String> {
    if window.label() != "timer" {
        return Err("errorMove".into());
    }
    let size = window.outer_size().map_err(|e| e.to_string())?;
    let threshold = (12. * window.scale_factor().map_err(|e| e.to_string())?).round() as i32;
    let (x, y) = snapping::snap(
        x,
        y,
        size.width,
        size.height,
        &rt.snap_targets.lock().unwrap(),
        threshold,
    );
    window
        .set_position(PhysicalPosition::new(x, y))
        .map_err(|e| e.to_string())?;
    if finished {
        rt.snap_targets.lock().unwrap().clear();
        glass(&window);
    }
    Ok(())
}

fn persist(rt: &Runtime) -> Result<(), String> {
    let e = rt.engine.lock().map_err(|_| "errorConnect")?;
    storage::save(
        &rt.path,
        &storage::Saved {
            settings: e.settings.clone(),
            checkpoint: e.checkpoint(),
            history: rt.history.lock().map_err(|_| "errorConnect")?.clone(),
        },
    )
}
fn snapshot(rt: &Runtime) -> Snapshot {
    let mut state = rt.engine.lock().unwrap().snapshot();
    if state.settings.history_enabled {
        state.history = rt.history.lock().unwrap().visible(wall_seconds());
        state.reward_slot = rt
            .recent_reward
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|(slot, at)| (at.elapsed() < Duration::from_secs(8)).then_some(*slot));
    }
    state
}

fn wall_seconds() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn broadcast(app: &tauri::AppHandle) {
    let rt = app.state::<Runtime>();
    let snapshot = snapshot(&rt);
    for (label, window) in app.webview_windows() {
        if window.is_visible().unwrap_or(false) {
            let _ = app.emit_to(label, "state", &snapshot);
        }
    }
}

#[tauri::command]
fn get_state(rt: State<Runtime>) -> Snapshot {
    snapshot(&rt)
}

#[tauri::command]
fn clear_history_slot(app: tauri::AppHandle, start: i64) -> Result<(), String> {
    let rt = app.state::<Runtime>();
    if !rt.engine.lock().unwrap().settings.history_enabled {
        return Err("errorSetting".into());
    }
    rt.history.lock().unwrap().clear(start, wall_seconds())?;
    // A cleared reward cannot reappear on the next state update.
    if rt
        .recent_reward
        .lock()
        .unwrap()
        .as_ref()
        .is_some_and(|(slot, _)| *slot == start)
    {
        *rt.recent_reward.lock().unwrap() = None;
    }
    persist(&rt)?;
    broadcast(&app);
    Ok(())
}

#[tauri::command]
fn set_theme(app: tauri::AppHandle, theme: String) -> Result<(), String> {
    let rt = app.state::<Runtime>();
    let old = rt.engine.lock().unwrap().settings.theme.clone();
    rt.engine.lock().unwrap().set_theme(theme)?;
    if let Err(error) = persist(&rt) {
        rt.engine.lock().unwrap().settings.theme = old;
        return Err(error);
    }
    broadcast(&app);
    Ok(())
}

#[tauri::command]
fn start_drag(window: WebviewWindow) -> Result<(), String> {
    if window.label() != "timer" {
        return Err("errorMove".into());
    }
    platform::start_drag(&window)
}

#[tauri::command]
async fn action(app: tauri::AppHandle, kind: String) -> Result<(), String> {
    perform_action(app, kind)
}

fn perform_action(app: tauri::AppHandle, kind: String) -> Result<(), String> {
    let rt = app.state::<Runtime>();
    match kind.as_str() {
        "toggle" => rt.engine.lock().unwrap().toggle(),
        "reset" => rt.engine.lock().unwrap().reset(),
        "history-show" => {
            rt.history_hovered.store(true, Ordering::Relaxed);
            rt.history_hover.fetch_add(1, Ordering::Relaxed);
            return show_history(&app, false);
        }
        "history-leave" => {
            rt.history_hovered.store(false, Ordering::Relaxed);
            let generation = rt.history_hover.fetch_add(1, Ordering::Relaxed) + 1;
            // Mouse-leave must never close a freshly completed session before
            // the delayed achievement bloom has been presented.
            if rt
                .completion_history_until
                .lock()
                .unwrap()
                .is_some_and(|until| Instant::now() < until)
            {
                return Ok(());
            }
            let handle = app.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(650));
                let rt = handle.state::<Runtime>();
                let pinned = rt
                    .completion_history_until
                    .lock()
                    .unwrap()
                    .is_some_and(|until| Instant::now() < until);
                if !pinned
                    && !rt.history_hovered.load(Ordering::Relaxed)
                    && rt.history_hover.load(Ordering::Relaxed) == generation
                {
                    if let Some(w) = handle.get_webview_window("history") {
                        let _ = w.hide();
                    }
                }
            });
            return Ok(());
        }
        "history-close" => {
            // Explicit close is always respected, including during a reward.
            rt.reward_generation.fetch_add(1, Ordering::Relaxed);
            rt.history_hover.fetch_add(1, Ordering::Relaxed);
            rt.history_hovered.store(false, Ordering::Relaxed);
            *rt.completion_history_until.lock().unwrap() = None;
            *rt.recent_reward.lock().unwrap() = None;
            if let Some(w) = app.get_webview_window("history") {
                w.hide().map_err(|e| e.to_string())?;
            }
            broadcast(&app);
            return Ok(());
        }
        "settings" => {
            show_settings(&app);
            return Ok(());
        }
        "dismiss" => {
            dismiss_notification(&app);
            return Ok(());
        }
        "hide-selector" => {
            if let Some(w) = app.get_webview_window("selector") {
                let _ = w.destroy();
            }
            return Ok(());
        }
        "selector" => {
            show_selector(&app);
            return Ok(());
        }
        "preview-sound" => {
            play_chime(&app);
            return Ok(());
        }
        "preview-notification" => {
            notify(&app, false);
            return Ok(());
        }
        "hide-settings" => {
            if let Some(w) = app.get_webview_window("settings") {
                let _ = w.destroy();
            }
            return Ok(());
        }
        "reposition" => {
            place_timer(&app, None);
            return Ok(());
        }
        "quit" => {
            persist(&rt)?;
            app.exit(0);
            return Ok(());
        }
        _ => return Err("errorUnknownAction".into()),
    }
    persist(&rt)?;
    update_menu(&app);
    broadcast(&app);
    Ok(())
}

#[tauri::command]
async fn set_preset(app: tauri::AppHandle, minutes: u32) -> Result<(), String> {
    perform_preset(app, minutes)
}

fn perform_preset(app: tauri::AppHandle, minutes: u32) -> Result<(), String> {
    if !PRESETS.contains(&minutes) {
        return Err("errorDuration".into());
    }
    let rt = app.state::<Runtime>();
    {
        let mut e = rt.engine.lock().unwrap();
        e.select_manual(minutes);
    }
    persist(&rt)?;
    broadcast(&app);
    update_menu(&app);
    show_selector(&app);
    Ok(())
}

#[tauri::command]
fn update_setting(
    app: tauri::AppHandle,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    if ![
        "defaultMinutes",
        "idlePauseSecs",
        "idleResetSecs",
        "sound",
        "opacity",
        "autostart",
        "autoDetect",
        "timerVisible",
        "historyEnabled",
        "language",
        "timerSize",
    ]
    .contains(&key.as_str())
    {
        return Err("errorSetting".into());
    }
    // Merge only the changed field with the current native state. A slider
    // update must not overwrite a newer duration, position or switch change.
    let settings = app
        .state::<Runtime>()
        .engine
        .lock()
        .unwrap()
        .settings
        .clone();
    let mut settings = serde_json::to_value(settings).map_err(|e| e.to_string())?;
    settings[&key] = value;
    let settings = serde_json::from_value(settings).map_err(|e| e.to_string())?;
    save_settings(app, settings)
}

#[tauri::command]
fn save_settings(app: tauri::AppHandle, mut settings: Settings) -> Result<(), String> {
    settings.validate()?;
    let rt = app.state::<Runtime>();
    // Position can only be changed by actual native window movement.
    settings.position = rt.engine.lock().unwrap().settings.position;
    let old = rt.engine.lock().unwrap().settings.clone();
    if old.autostart != settings.autostart {
        let result = if settings.autostart {
            app.autolaunch().enable()
        } else {
            app.autolaunch().disable()
        };
        result.map_err(|e| format!("errorAutostart: {e}"))?;
    }
    rt.engine
        .lock()
        .unwrap()
        .apply_preferences(settings.clone());
    if let Err(error) = persist(&rt) {
        rt.engine.lock().unwrap().update(old.clone());
        if old.autostart {
            let _ = app.autolaunch().enable();
        } else {
            let _ = app.autolaunch().disable();
        }
        return Err(error);
    }
    if !settings.history_enabled {
        rt.reward_generation.fetch_add(1, Ordering::Relaxed);
        rt.history_hover.fetch_add(1, Ordering::Relaxed);
        rt.history_hovered.store(false, Ordering::Relaxed);
        *rt.completion_history_until.lock().unwrap() = None;
        if let Some(w) = app.get_webview_window("history") {
            let _ = w.destroy();
        }
        *rt.recent_reward.lock().unwrap() = None;
    }
    if let Some(w) = app.get_webview_window("timer") {
        if old.timer_size != settings.timer_size || old.history_enabled != settings.history_enabled
        {
            let scale = settings.timer_scale();
            w.set_size(LogicalSize::new(
                TIMER_WIDTH * scale,
                timer_height(&settings) * scale,
            ))
            .map_err(|e| e.to_string())?;
            // Keep the existing position, clamped to the timer's current monitor.
            if let (Ok(position), Ok(size), Ok(Some(m))) =
                (w.outer_position(), w.outer_size(), w.current_monitor())
            {
                let r = m.work_area();
                let x = position.x.clamp(
                    r.position.x,
                    (r.position.x + r.size.width as i32 - size.width as i32).max(r.position.x),
                );
                let y = position.y.clamp(
                    r.position.y,
                    (r.position.y + r.size.height as i32 - size.height as i32).max(r.position.y),
                );
                let _ = w.set_position(PhysicalPosition::new(x, y));
            }
            glass(&w);
            if let Some(selector) = app.get_webview_window("selector") {
                let _ = selector.destroy();
            }
        }
        if settings.timer_visible {
            let _ = w.show();
        } else {
            let _ = w.hide();
        }
    }
    update_menu(&app);
    broadcast(&app);
    Ok(())
}

fn glass(w: &WebviewWindow) {
    let window = w.clone();
    let _ = w
        .app_handle()
        .run_on_main_thread(move || native_glass(&window));
}

fn native_glass(w: &WebviewWindow) {
    #[cfg(target_os = "windows")]
    {
        if w.label() == "settings" {
            if window_vibrancy::apply_acrylic(w, Some((143, 166, 201, 38))).is_err() {
                let _ = window_vibrancy::apply_blur(w, Some((143, 166, 201, 38)));
            }
        } else {
            // System backdrops paint a rectangular surface beyond a custom
            // capsule region. Keep frameless HWNDs transparent at rest too;
            // their material, reflections and lighting are drawn inside CSS.
            let _ = window_vibrancy::clear_acrylic(w);
            let _ = window_vibrancy::clear_blur(w);
        }
    }
    #[cfg(target_os = "macos")]
    let _ = window_vibrancy::apply_vibrancy(
        w,
        window_vibrancy::NSVisualEffectMaterial::HudWindow,
        None,
        Some(match w.label() {
            "timer" => {
                TIMER_RADIUS
                    * w.state::<Runtime>()
                        .engine
                        .lock()
                        .unwrap()
                        .settings
                        .timer_scale()
            }
            "notification" => NOTIFICATION_RADIUS,
            _ => 36.,
        }),
    );
    shape_overlay(w);
}

// CSS border-radius cannot clip Windows DWM blur or native hit-testing.
fn shape_overlay(w: &WebviewWindow) {
    #[cfg(target_os = "windows")]
    {
        #[link(name = "gdi32")]
        extern "system" {
            fn CreateRoundRectRgn(
                left: i32,
                top: i32,
                right: i32,
                bottom: i32,
                ew: i32,
                eh: i32,
            ) -> isize;
            fn DeleteObject(object: isize) -> i32;
        }
        #[link(name = "user32")]
        extern "system" {
            fn SetWindowRgn(window: isize, region: isize, redraw: i32) -> i32;
            fn GetWindowLongPtrW(window: isize, index: i32) -> isize;
            fn SetWindowLongPtrW(window: isize, index: i32, value: isize) -> isize;
            fn SetWindowPos(
                window: isize,
                after: isize,
                x: i32,
                y: i32,
                width: i32,
                height: i32,
                flags: u32,
            ) -> i32;
        }
        #[link(name = "comctl32")]
        extern "system" {
            fn SetWindowSubclass(
                window: isize,
                callback: Option<
                    unsafe extern "system" fn(isize, u32, usize, isize, usize, usize) -> isize,
                >,
                id: usize,
                data: usize,
            ) -> i32;
            fn DefSubclassProc(window: isize, message: u32, wparam: usize, lparam: isize) -> isize;
        }
        unsafe extern "system" fn borderless_proc(
            hwnd: isize,
            message: u32,
            wparam: usize,
            lparam: isize,
            _: usize,
            _: usize,
        ) -> isize {
            if message == 0x007c && wparam as isize == -16 && lparam != 0 {
                #[repr(C)]
                struct StyleChange {
                    old: u32,
                    new: u32,
                }
                let styles = &mut *(lparam as *mut StyleChange);
                styles.new = (styles.new & !0x00cf0000) | 0x80000000;
            }
            if message == 0x0085 {
                return 0;
            } // WM_NCPAINT: no native frame to paint.
              // Preserve tao's activation tracking while suppressing non-client repaint.
            DefSubclassProc(
                hwnd,
                message,
                wparam,
                if message == 0x0086 { -1 } else { lparam },
            )
        }
        let (width, height, inset, radius) = match w.label() {
            "timer" => {
                let scale = w
                    .state::<Runtime>()
                    .engine
                    .lock()
                    .unwrap()
                    .settings
                    .timer_scale();
                (
                    TIMER_WIDTH * scale,
                    timer_height(&w.state::<Runtime>().engine.lock().unwrap().settings) * scale,
                    4. * scale,
                    TIMER_RADIUS * scale,
                )
            }
            "selector" => (112., 302., 4., 24.),
            "notification" => (
                NOTIFICATION_WIDTH,
                NOTIFICATION_HEIGHT,
                6.,
                NOTIFICATION_RADIUS,
            ),
            _ => return,
        };
        if let (Ok(hwnd), Ok(scale)) = (w.hwnd(), w.scale_factor()) {
            unsafe {
                let hwnd = hwnd.0 as isize;
                SetWindowSubclass(hwnd, Some(borderless_proc), 1, 0);
                // tao keeps WS_CAPTION for its non-client calculations. A custom
                // DWM region can expose that caption when focused: remove it.
                let style = GetWindowLongPtrW(hwnd, -16);
                SetWindowLongPtrW(hwnd, -16, (style & !0x00CF0000) | 0x80000000);
                SetWindowPos(hwnd, 0, 0, 0, 0, 0, 0x37);
                // Removing tao's hidden caption can enlarge the client area on
                // first show. Restore the requested logical size after that
                // style change, so startup and interactive resizing agree.
                if w.label() == "timer" {
                    if let Ok(size) = w.inner_size() {
                        let expected_width = (width * scale).round() as u32;
                        let expected_height = (height * scale).round() as u32;
                        if size.width != expected_width || size.height != expected_height {
                            let _ = w.set_size(LogicalSize::new(width, height));
                        }
                    }
                }
                let region = CreateRoundRectRgn(
                    (inset * scale) as i32,
                    (inset * scale) as i32,
                    ((width - inset) * scale) as i32,
                    ((height - inset) * scale) as i32,
                    (2. * radius * scale) as i32,
                    (2. * radius * scale) as i32,
                );
                if region != 0 && SetWindowRgn(hwnd, region, 1) == 0 {
                    DeleteObject(region);
                }
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    let _ = w;
}

fn overlay(
    app: &tauri::AppHandle,
    label: &str,
    route: &str,
    width: f64,
    height: f64,
) -> tauri::Result<WebviewWindow> {
    let window = WebviewWindowBuilder::new(
        app,
        label,
        WebviewUrl::App(format!("index.html?view={route}").into()),
    )
    .title(APP_NAME)
    .inner_size(width, height)
    .decorations(false)
    .transparent(true)
    .shadow(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .focused(false)
    .focusable(label == "timer")
    .visible(false)
    .visible_on_all_workspaces(true)
    .build()?;
    glass(&window);
    Ok(window)
}

fn place_timer(app: &tauri::AppHandle, saved: Option<(i32, i32)>) {
    let Some(w) = app.get_webview_window("timer") else {
        return;
    };
    let scale = app
        .state::<Runtime>()
        .engine
        .lock()
        .unwrap()
        .settings
        .timer_scale();
    let height = timer_height(&app.state::<Runtime>().engine.lock().unwrap().settings);
    let (width, height) = (TIMER_WIDTH * scale, height * scale);
    if let Ok(monitors) = w.available_monitors() {
        // Accept only coordinates with the entire capsule inside a currently connected work area.
        if let Some((x, y)) = saved {
            if monitors.iter().any(|m| {
                let r = m.work_area();
                let s = m.scale_factor();
                x >= r.position.x
                    && y >= r.position.y
                    && x as f64 + width * s <= r.position.x as f64 + r.size.width as f64
                    && y as f64 + height * s <= r.position.y as f64 + r.size.height as f64
            }) {
                let _ = w.set_position(PhysicalPosition::new(x, y));
                return;
            }
        }
    }
    if let Ok(Some(m)) = w.primary_monitor() {
        let r = m.work_area();
        let s = m.scale_factor();
        let _ = w.set_position(PhysicalPosition::new(
            r.position.x + r.size.width as i32 - ((width + 28.) * s) as i32,
            r.position.y + ((r.size.height as f64 - height * s) * 0.28).max(0.) as i32,
        ));
    }
}

fn show_settings(app: &tauri::AppHandle) {
    let window = app.get_webview_window("settings").or_else(|| {
        let (width, height) = app
            .get_webview_window("timer")
            .and_then(|w| w.primary_monitor().ok().flatten())
            .map(|m| {
                let area = m.work_area();
                let scale = m.scale_factor();
                (
                    (area.size.width as f64 / scale - 48.).clamp(360., 580.),
                    (area.size.height as f64 / scale - 80.).clamp(320., 850.),
                )
            })
            .unwrap_or((580., 640.));
        let w = WebviewWindowBuilder::new(
            app,
            "settings",
            WebviewUrl::App("index.html?view=settings".into()),
        )
        .title(format!(
            "{APP_NAME} — {}",
            text(
                &app.state::<Runtime>()
                    .engine
                    .lock()
                    .unwrap()
                    .settings
                    .language,
                "settings"
            )
        ))
        .inner_size(width, height)
        .min_inner_size(360., 320.)
        .transparent(true)
        .visible(false)
        .center()
        .build()
        .ok()?;
        glass(&w);
        Some(w)
    });
    if let Some(w) = window {
        let _ = w.show();
        let _ = w.set_focus();
    }
}
fn show_history(app: &tauri::AppHandle, on_completion: bool) -> Result<(), String> {
    let rt = app.state::<Runtime>();
    if !rt.engine.lock().unwrap().settings.history_enabled {
        return Ok(());
    }
    let timer = app
        .get_webview_window("timer")
        .ok_or_else(|| "errorHistoryWindow: timer missing".to_string())?;
    // A completion is allowed to show history even when the user has hidden
    // the tiny timer; otherwise a visible break notification can lack history.
    if !on_completion && !timer.is_visible().unwrap_or(false) {
        return Ok(());
    }
    let popup = match app.get_webview_window("history") {
        Some(w) => w,
        None => overlay(app, "history", "history", HISTORY_WIDTH, HISTORY_HEIGHT)
            .map_err(|e| format!("errorHistoryWindow: {e}"))?,
    };
    if let (Ok(p), Ok(size), Ok(scale)) = (
        timer.outer_position(),
        timer.outer_size(),
        timer.scale_factor(),
    ) {
        let width = (HISTORY_WIDTH * scale).round() as i32;
        let height = (HISTORY_HEIGHT * scale).round() as i32;
        let mut x = p.x + size.width as i32 + (8. * scale) as i32;
        let mut y = p.y + (size.height as i32 - height) / 2;
        if let Ok(Some(m)) = timer.current_monitor() {
            let area = m.work_area();
            if x + width > area.position.x + area.size.width as i32 {
                x = p.x - width - (8. * scale) as i32;
            }
            x = x.clamp(
                area.position.x,
                (area.position.x + area.size.width as i32 - width).max(area.position.x),
            );
            y = y.clamp(
                area.position.y,
                (area.position.y + area.size.height as i32 - height).max(area.position.y),
            );
        }
        let _ = popup.set_position(PhysicalPosition::new(x, y));
    }
    popup
        .show()
        .map_err(|e| format!("errorHistoryWindow: {e}"))?;
    glass(&popup);
    // The break notice and history are separate topmost native windows.
    // Raise history above the notice, but keep the time selector above both.
    #[cfg(target_os = "windows")]
    {
        let raised = popup.clone();
        let _ = app.run_on_main_thread(move || {
            #[link(name = "user32")]
            extern "system" {
                fn SetWindowPos(
                    hwnd: isize,
                    after: isize,
                    x: i32,
                    y: i32,
                    width: i32,
                    height: i32,
                    flags: u32,
                ) -> i32;
            }
            if let Ok(hwnd) = raised.hwnd() {
                unsafe {
                    let _ = SetWindowPos(hwnd.0 as isize, -1, 0, 0, 0, 0, 0x0013);
                }
            }
        });
    }
    keep_selector_above_history(app);
    Ok(())
}

// Windows keeps frameless overlays in the topmost band. Explicitly reorder the
// selector after each history/selector show, rather than relying on show order.
fn keep_selector_above_history(app: &tauri::AppHandle) {
    #[cfg(target_os = "windows")]
    {
        let Some(selector) = app.get_webview_window("selector") else {
            return;
        };
        if !selector.is_visible().unwrap_or(false) {
            return;
        }
        let _ = app.run_on_main_thread(move || {
            #[link(name = "user32")]
            extern "system" {
                fn SetWindowPos(
                    hwnd: isize,
                    after: isize,
                    x: i32,
                    y: i32,
                    width: i32,
                    height: i32,
                    flags: u32,
                ) -> i32;
            }
            if let Ok(hwnd) = selector.hwnd() {
                // HWND_TOPMOST, SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE.
                unsafe {
                    let _ = SetWindowPos(hwnd.0 as isize, -1, 0, 0, 0, 0, 0x0013);
                }
            }
        });
    }
    #[cfg(not(target_os = "windows"))]
    let _ = app;
}

fn show_selector(app: &tauri::AppHandle) {
    let selector = app
        .get_webview_window("selector")
        .or_else(|| overlay(app, "selector", "selector", 112., 302.).ok());
    let (Some(timer), Some(selector)) = (app.get_webview_window("timer"), selector) else {
        return;
    };
    if let (Ok(p), Ok(size), Ok(scale)) = (
        timer.outer_position(),
        timer.outer_size(),
        timer.scale_factor(),
    ) {
        let width = (112. * scale) as i32;
        let mut x = p.x - width - (10. * scale) as i32;
        let mut y = p.y + (size.height as i32 - (302. * scale) as i32) / 2;
        if let Ok(Some(m)) = timer.current_monitor() {
            let r = m.work_area();
            if x < r.position.x {
                x = p.x + size.width as i32 + (10. * scale) as i32;
            }
            x = x.clamp(
                r.position.x,
                (r.position.x + r.size.width as i32 - width).max(r.position.x),
            );
            y = y.clamp(
                r.position.y,
                (r.position.y + r.size.height as i32 - (302. * scale) as i32).max(r.position.y),
            );
        }
        let _ = selector.set_position(PhysicalPosition::new(x, y));
    }
    let _ = selector.show();
    keep_selector_above_history(app);
    let _ = app.emit("selector-show", ());
}

fn notify(app: &tauri::AppHandle, sound: bool) {
    if sound && app.state::<Runtime>().engine.lock().unwrap().settings.sound {
        play_chime(app);
    }
    if let Some(w) = app.get_webview_window("notification").or_else(|| {
        overlay(
            app,
            "notification",
            "notification",
            NOTIFICATION_WIDTH,
            NOTIFICATION_HEIGHT,
        )
        .ok()
    }) {
        // Center on the timer's monitor, preserving the user's active application.
        if let Some(timer) = app.get_webview_window("timer") {
            if let Ok(Some(m)) = timer.current_monitor() {
                let r = m.work_area();
                let scale = m.scale_factor();
                let _ = w.set_position(PhysicalPosition::new(
                    r.position.x + (r.size.width as i32 - (NOTIFICATION_WIDTH * scale) as i32) / 2,
                    r.position.y
                        + (r.size.height as i32 - (NOTIFICATION_HEIGHT * scale) as i32) / 2,
                ));
            }
        }
        let _ = app.emit("break", ());
        let _ = w.show();
        // Reapply after positioning/showing, including on monitors with different DPI.
        glass(&w);
        *app.state::<Runtime>().notification_started.lock().unwrap() = Some(Instant::now());
    }
}

fn play_chime(app: &tauri::AppHandle) {
    if let Ok(dir) = app.path().resource_dir() {
        platform::play_sound(&dir.join("sounds/seatbelt.wav"));
    }
}

fn dismiss_notification(app: &tauri::AppHandle) {
    *app.state::<Runtime>().notification_started.lock().unwrap() = None;
    let _ = app.emit("break-hide", ());
    if let Some(w) = app.get_webview_window("notification") {
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(180));
            // A new preview or completion may have arrived during the fade.
            if w.app_handle()
                .state::<Runtime>()
                .notification_started
                .lock()
                .unwrap()
                .is_none()
            {
                let _ = w.destroy();
            }
        });
    }
}

fn update_menu(app: &tauri::AppHandle) {
    let rt = app.state::<Runtime>();
    let (status, settings) = {
        let e = rt.engine.lock().unwrap();
        (e.status, e.settings.clone())
    };
    let language = settings.language.clone();
    let tr = |key| text(&language, key);
    let build = || -> tauri::Result<Menu<tauri::Wry>> {
        let toggle = MenuItem::with_id(
            app,
            "toggle",
            if status == Status::Running {
                tr("pause")
            } else {
                tr("startResume")
            },
            true,
            None::<&str>,
        )?;
        let reset = MenuItem::with_id(app, "reset", tr("reset"), true, None::<&str>)?;
        let auto = CheckMenuItem::with_id(
            app,
            "auto",
            tr("autoDetectMenu"),
            true,
            settings.auto_detect,
            None::<&str>,
        )?;
        let visible = CheckMenuItem::with_id(
            app,
            "visible",
            tr("timerVisible"),
            true,
            settings.timer_visible,
            None::<&str>,
        )?;
        let presets = Submenu::new(app, tr("duration"), true)?;
        for m in PRESETS {
            presets.append(&CheckMenuItem::with_id(
                app,
                format!("preset-{m}"),
                format!("{m} {}", tr("minutesUnit")),
                true,
                settings.minutes == m,
                None::<&str>,
            )?)?;
        }
        let settings = MenuItem::with_id(app, "settings", tr("settings"), true, None::<&str>)?;
        let quit = MenuItem::with_id(app, "quit", tr("quit"), true, None::<&str>)?;
        Menu::with_items(
            app,
            &[&toggle, &reset, &auto, &visible, &presets, &settings, &quit],
        )
    };
    if let (Ok(menu), Some(tray)) = (build(), app.tray_by_id("ambient")) {
        let _ = tray.set_menu(Some(menu));
        let _ = tray.set_tooltip(Some(format!("{APP_NAME} — {}", tr("tagline"))));
    }
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.set_title(&format!("{APP_NAME} — {}", tr("settings")));
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            show_settings(app);
        }))
        .plugin(
            tauri_plugin_autostart::Builder::new()
                // Keep the existing startup registration identity across the
                // display-name change, so disabling removes the old entry too.
                .app_name("Ambient Pomodoro")
                .arg("--autostart")
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_state,
            action,
            set_preset,
            save_settings,
            update_setting,
            start_drag,
            get_drag_origin,
            move_timer,
            set_theme,
            clear_history_slot
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let path = app
                .path()
                .app_config_dir()?
                .join(if cfg!(debug_assertions) {
                    "dev-state.json"
                } else {
                    "state.json"
                });
            let saved = storage::load(&path);
            let (mut settings, checkpoint, history) = saved
                .map(|s| (s.settings, Some(s.checkpoint), s.history))
                .unwrap_or_else(|| {
                    let mut defaults = Settings::default();
                    if cfg!(debug_assertions) {
                        defaults.autostart = false;
                    }
                    (defaults, None, WorkHistory::default())
                });
            // Debug intervals cannot be enabled in a distribution build.
            let debug = if cfg!(debug_assertions) {
                std::env::var("AMBIENT_DEBUG_SECONDS")
                    .ok()
                    .and_then(|s| s.parse::<f64>().ok())
                    .filter(|s| [5., 10.].contains(s))
            } else {
                None
            };
            let startup_error = if settings.autostart {
                if let Err(e) = app.autolaunch().enable() {
                    settings.autostart = false;
                    Some(format!("errorAutostartEnable: {e}"))
                } else {
                    None
                }
            } else {
                None
            };
            let mut engine = Engine::new(settings.clone(), checkpoint, debug);
            engine.error = startup_error;
            app.manage(Runtime {
                engine: Mutex::new(engine),
                path,
                snap_targets: Mutex::new(Vec::new()),
                notification_started: Mutex::new(None),
                history: Mutex::new(history),
                recent_reward: Mutex::new(None),
                history_hover: AtomicU64::new(0),
                history_hovered: AtomicBool::new(false),
                completion_history_until: Mutex::new(None),
                reward_generation: AtomicU64::new(0),
            });
            // Only the small timer owns a WebView while resident. Temporary
            // windows are created on demand and destroyed after use.
            let scale = settings.timer_scale();
            let timer = overlay(
                app.handle(),
                "timer",
                "timer",
                TIMER_WIDTH * scale,
                timer_height(&settings) * scale,
            )?;
            place_timer(app.handle(), settings.position);
            if settings.timer_visible {
                timer.show()?;
            }
            tauri::tray::TrayIconBuilder::with_id("ambient")
                .icon(tauri::image::Image::from_bytes(include_bytes!(
                    "../icons/tray.png"
                ))?)
                .icon_as_template(true)
                .tooltip(format!(
                    "{APP_NAME} — {}",
                    text(&settings.language, "tagline")
                ))
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| {
                    let id = event.id.as_ref();
                    if let Some(m) = id.strip_prefix("preset-").and_then(|s| s.parse().ok()) {
                        let _ = perform_preset(app.clone(), m);
                        return;
                    }
                    if id == "auto" || id == "visible" {
                        let mut s = app
                            .state::<Runtime>()
                            .engine
                            .lock()
                            .unwrap()
                            .settings
                            .clone();
                        if id == "auto" {
                            s.auto_detect = !s.auto_detect;
                        } else {
                            s.timer_visible = !s.timer_visible;
                        }
                        if let Err(error) = save_settings(app.clone(), s) {
                            app.state::<Runtime>().engine.lock().unwrap().error = Some(error);
                            broadcast(app);
                        }
                    } else if let Err(error) = perform_action(app.clone(), id.into()) {
                        app.state::<Runtime>().engine.lock().unwrap().error = Some(error);
                        broadcast(app);
                    }
                })
                .build(app)?;
            update_menu(app.handle());
            let handle = app.handle().clone();
            std::thread::spawn(move || {
                let mut last = platform::monotonic_seconds();
                let mut checkpoint = Instant::now();
                let mut old_status = Status::Waiting;
                loop {
                    std::thread::sleep(Duration::from_secs(1));
                    let now = Instant::now();
                    let clock = platform::monotonic_seconds();
                    let delta = (clock - last).max(0.);
                    last = clock;
                    let (idle, locked) = platform::sample();
                    let rt = handle.state::<Runtime>();
                    let notified = *rt.notification_started.lock().unwrap();
                    if notified.is_some_and(|t| t.elapsed() >= Duration::from_secs(5))
                        || ((locked || delta > 5.) && notified.is_some())
                    {
                        dismiss_notification(&handle);
                    }
                    let (due, status, charged, track) = {
                        let mut e = rt.engine.lock().unwrap();
                        let due = e.tick(delta, idle, locked);
                        (due, e.status, e.charged_secs, e.settings.history_enabled)
                    };
                    let mut completed_slot = None;
                    if track && charged > 0.0 {
                        let wall = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs_f64();
                        completed_slot = rt.history.lock().unwrap().record(wall, charged);
                    }
                    if due {
                        notify(&handle, true);
                        if track {
                            // Showing the history is independent of whether a slot
                            // happened to receive seconds in this exact tick.
                            // Previously this guard sporadically skipped the popup.
                            let reward_slot = {
                                let h = rt.history.lock().unwrap();
                                match completed_slot {
                                    Some(slot) => h.contains_slot(slot).then_some(slot),
                                    None => h.recent_slot(wall_seconds()),
                                }
                            };
                            *rt.recent_reward.lock().unwrap() = None;
                            let reward_id =
                                rt.reward_generation.fetch_add(1, Ordering::Relaxed) + 1;
                            rt.history_hover.fetch_add(1, Ordering::Relaxed);
                            *rt.completion_history_until.lock().unwrap() =
                                Some(Instant::now() + Duration::from_secs(10));
                            let first_shown = show_history(&handle, true).is_ok();
                            let later = handle.clone();
                            std::thread::spawn(move || {
                                let started = Instant::now();
                                // Overlay creation can fail while another native
                                // window is showing. Retry rather than silently
                                // discarding the user's completion feedback.
                                if !first_shown {
                                    for _ in 0..4 {
                                        std::thread::sleep(Duration::from_millis(180));
                                        let rt = later.state::<Runtime>();
                                        if rt.reward_generation.load(Ordering::Relaxed) != reward_id
                                        {
                                            return;
                                        }
                                        if show_history(&later, true).is_ok() {
                                            break;
                                        }
                                    }
                                }
                                std::thread::sleep(
                                    Duration::from_secs(2).saturating_sub(started.elapsed()),
                                );
                                let rt = later.state::<Runtime>();
                                if rt.reward_generation.load(Ordering::Relaxed) != reward_id {
                                    return;
                                }
                                // A second show/raise ensures that the popup is
                                // visible above "take a break" before the bloom.
                                let mut shown = show_history(&later, true).is_ok();
                                for _ in 0..3 {
                                    if shown {
                                        break;
                                    }
                                    std::thread::sleep(Duration::from_millis(180));
                                    shown = show_history(&later, true).is_ok();
                                }
                                if shown {
                                    if let Some(slot) = reward_slot {
                                        // Respect cells cleared during the 2s delay.
                                        if rt.history.lock().unwrap().contains_slot(slot) {
                                            *rt.recent_reward.lock().unwrap() =
                                                Some((slot, Instant::now()));
                                        }
                                    }
                                    broadcast(&later);
                                }
                                // Keep the popup through the full reward animation;
                                // only auto-hide if the pointer has left it.
                                std::thread::sleep(Duration::from_secs(8));
                                if rt.reward_generation.load(Ordering::Relaxed) == reward_id {
                                    *rt.completion_history_until.lock().unwrap() = None;
                                    if !rt.history_hovered.load(Ordering::Relaxed) {
                                        if let Some(w) = later.get_webview_window("history") {
                                            let _ = w.hide();
                                        }
                                    }
                                }
                            });
                        }
                        let _ = persist(&rt);
                    }
                    if status != old_status {
                        update_menu(&handle);
                        old_status = status;
                    }
                    broadcast(&handle);
                    if checkpoint.elapsed() >= Duration::from_secs(15) {
                        if let Err(error) = persist(&rt) {
                            rt.engine.lock().unwrap().error = Some(error);
                        }
                        checkpoint = now;
                    }
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::Focused(true) if window.label() != "settings" => {
                if let Some(w) = window.app_handle().get_webview_window(window.label()) {
                    shape_overlay(&w);
                }
            }
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                if window.label() == "timer" {
                    if let Some(history) = window.app_handle().get_webview_window("history") {
                        let _ = history.hide();
                    }
                    let _ = window.hide();
                    let rt = window.state::<Runtime>();
                    rt.engine.lock().unwrap().settings.timer_visible = false;
                    let _ = persist(&rt);
                    update_menu(window.app_handle());
                    broadcast(window.app_handle());
                } else {
                    let _ = window.destroy();
                }
            }
            tauri::WindowEvent::Moved(p) if window.label() == "timer" => {
                let rt = window.state::<Runtime>();
                rt.engine.lock().unwrap().settings.position = Some((p.x, p.y));
            }
            tauri::WindowEvent::ScaleFactorChanged { .. } => {
                let handle = window.app_handle().clone();
                if let Some(w) = handle.get_webview_window(window.label()) {
                    shape_overlay(&w);
                }
                if window.label() == "timer" {
                    let pos = window
                        .state::<Runtime>()
                        .engine
                        .lock()
                        .unwrap()
                        .settings
                        .position;
                    place_timer(&handle, pos);
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("ADHD Coffee Time could not initialize")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event {
                let _ = persist(&app.state::<Runtime>());
            }
        });
}
