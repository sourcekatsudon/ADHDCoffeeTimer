/// Keep Windows coordinates packed into LPARAM (including negative monitors).
/// This avoids passing a pointer to a POINTS struct to WM_NCLBUTTONDOWN.
pub fn start_drag(window: &tauri::WebviewWindow) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        #[repr(C)]
        struct Point {
            x: i32,
            y: i32,
        }
        #[link(name = "user32")]
        extern "system" {
            fn GetCursorPos(point: *mut Point) -> i32;
            fn ReleaseCapture() -> i32;
            fn PostMessageW(window: isize, message: u32, wparam: usize, lparam: isize) -> i32;
        }
        unsafe {
            let hwnd = window.hwnd().map_err(|e| e.to_string())?;
            let mut p = Point { x: 0, y: 0 };
            if GetCursorPos(&mut p) == 0 {
                return Err("errorPointer".into());
            }
            ReleaseCapture();
            if PostMessageW(hwnd.0 as isize, 0x00a1, 2, packed_point(p.x, p.y)) == 0 {
                return Err("errorDrag".into());
            }
        }
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    window.start_dragging().map_err(|e| e.to_string())
}

#[cfg(any(target_os = "windows", test))]
fn packed_point(x: i32, y: i32) -> isize {
    (((y as u16 as u32) << 16) | x as u16 as u32) as isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drag_coordinates_support_negative_monitors() {
        for (x, y) in [(-1920, -1080), (3840, 2160), (0, 0), (-32768, 32767)] {
            let packed = super::packed_point(x, y) as u32;
            assert_eq!(packed as i16 as i32, x);
            assert_eq!((packed >> 16) as i16 as i32, y);
        }
    }

    #[test]
    fn linux_session_classification_distinguishes_x11_wayland_and_headless() {
        assert_eq!(
            classify_linux_session(Some("x11"), Some(":0"), None),
            LinuxSession::X11
        );
        assert_eq!(
            classify_linux_session(Some("X11"), Some(":1"), None),
            LinuxSession::X11
        );
        // Wayland with XWayland DISPLAY must still be classified as Wayland.
        assert_eq!(
            classify_linux_session(Some("wayland"), Some(":0"), Some("wayland-0")),
            LinuxSession::Wayland
        );
        assert_eq!(
            classify_linux_session(Some("wayland"), None, None),
            LinuxSession::Wayland
        );
        assert_eq!(
            classify_linux_session(None, Some(":0"), Some("wayland-0")),
            LinuxSession::Wayland
        );
        assert_eq!(
            classify_linux_session(None, Some(":0"), None),
            LinuxSession::X11
        );
        assert_eq!(
            classify_linux_session(Some("x11"), Some("   "), None),
            LinuxSession::Headless
        );
        assert_eq!(
            classify_linux_session(Some("tty"), None, None),
            LinuxSession::Headless
        );
        assert_eq!(
            classify_linux_session(None, None, None),
            LinuxSession::Headless
        );
    }

    #[test]
    fn linux_gdbus_and_loginctl_parsers_handle_valid_and_malformed_outputs() {
        assert_eq!(parse_gdbus_u64_ms("(uint64 4250,)\n"), Some(4.25));
        assert_eq!(parse_gdbus_u64_ms("(uint64 0,)"), Some(0.0));
        assert_eq!(parse_gdbus_u64_ms("(1500,)"), Some(1.5));
        assert_eq!(parse_gdbus_u64_ms("(uint64 -1,)"), None);
        assert_eq!(parse_gdbus_u64_ms("Error: AccessDenied"), None);
        assert_eq!(parse_gdbus_u64_ms(""), None);

        assert_eq!(parse_gdbus_bool("(true,)\n"), Some(true));
        assert_eq!(parse_gdbus_bool("(false,)\n"), Some(false));
        assert_eq!(parse_gdbus_bool("( true , )"), Some(true));
        assert_eq!(parse_gdbus_bool("true"), None);
        assert_eq!(parse_gdbus_bool(""), None);

        assert_eq!(parse_loginctl_locked("yes\n"), Some(true));
        assert_eq!(parse_loginctl_locked("no\n"), Some(false));
        assert_eq!(parse_loginctl_locked("TRUE"), Some(true));
        assert_eq!(parse_loginctl_locked("0"), Some(false));
        assert_eq!(parse_loginctl_locked("unknown"), None);
    }

    #[test]
    fn linux_probe_error_and_session_id_sanitization() {
        assert_eq!(
            classify_probe_stderr("Error: org.freedesktop.DBus.Error.AccessDenied"),
            "errorActivityPermission"
        );
        assert_eq!(
            classify_probe_stderr("Permission denied"),
            "errorActivityPermission"
        );
        assert_eq!(
            classify_probe_stderr("Error: org.freedesktop.DBus.Error.ServiceUnknown"),
            "errorActivityX11"
        );

        assert_eq!(sanitize_session_id(Some("2")), "2");
        assert_eq!(sanitize_session_id(Some("c1_user-1.0")), "c1_user-1.0");
        assert_eq!(sanitize_session_id(Some("--property=foo")), "self");
        assert_eq!(sanitize_session_id(Some("1; id")), "self");
        assert_eq!(sanitize_session_id(Some("")), "self");
        assert_eq!(sanitize_session_id(None), "self");
    }

    #[test]
    fn linux_audio_player_selection_prefers_pipewire_then_pulse_then_alsa() {
        const EMPTY: &[&str] = &[];
        const QUIET: &[&str] = &["-q"];
        assert_eq!(
            select_linux_audio_player(|_| true),
            Some(("/usr/bin/pw-play", EMPTY))
        );
        assert_eq!(
            select_linux_audio_player(|p| p != "/usr/bin/pw-play"),
            Some(("/usr/bin/paplay", EMPTY))
        );
        assert_eq!(
            select_linux_audio_player(|p| p == "/usr/bin/aplay"),
            Some(("/usr/bin/aplay", QUIET))
        );
        assert_eq!(select_linux_audio_player(|_| false), None);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ActivitySample {
    pub idle_secs: Option<f64>,
    pub locked: bool,
    pub issue: Option<&'static str>,
}

pub fn play_sound(resource_path: &std::path::Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    unsafe {
        let _ = resource_path;
        #[link(name = "winmm")]
        extern "system" {
            fn PlaySoundW(sound: *const u16, module: isize, flags: u32) -> i32;
        }
        // Static memory outlives asynchronous playback. No system fallback beep.
        static CHIME: &[u8] = include_bytes!("../sounds/seatbelt.wav");
        PlaySoundW(CHIME.as_ptr().cast(), 0, 0x0001 | 0x0002 | 0x0004);
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("/usr/bin/afplay")
            .arg(resource_path)
            .spawn()
            .map(|_| ())
            .map_err(|_| "errorSoundUnavailable".into())
    }
    #[cfg(target_os = "linux")]
    {
        play_linux_sound(resource_path)
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        let _ = resource_path;
        Err("errorSoundUnavailable".into())
    }
}

#[cfg(target_os = "windows")]
/// Only the age of the last input is read. There are no input hooks.
pub fn sample() -> ActivitySample {
    #[repr(C)]
    struct LastInputInfo {
        size: u32,
        time: u32,
    }
    #[link(name = "user32")]
    extern "system" {
        fn GetLastInputInfo(info: *mut LastInputInfo) -> i32;
        fn OpenInputDesktop(flags: u32, inherit: i32, access: u32) -> isize;
        fn CloseDesktop(desktop: isize) -> i32;
        fn GetUserObjectInformationW(
            handle: isize,
            index: i32,
            info: *mut u16,
            len: u32,
            needed: *mut u32,
        ) -> i32;
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn GetTickCount64() -> u64;
    }
    unsafe {
        let mut info = LastInputInfo {
            size: std::mem::size_of::<LastInputInfo>() as u32,
            time: 0,
        };
        let idle = if GetLastInputInfo(&mut info) != 0 {
            Some((GetTickCount64() as u32).wrapping_sub(info.time) as f64 / 1000.)
        } else {
            None
        };
        let desk = OpenInputDesktop(0, 0, 1); // DESKTOP_READOBJECTS
        let locked = if desk == 0 {
            true
        } else {
            let mut name = [0u16; 128];
            let mut needed = 0;
            let ok = GetUserObjectInformationW(desk, 2, name.as_mut_ptr(), 256, &mut needed);
            CloseDesktop(desk);
            ok == 0
                || String::from_utf16_lossy(
                    &name[..name.iter().position(|c| *c == 0).unwrap_or(128)],
                ) != "Default"
        };
        ActivitySample {
            idle_secs: idle,
            locked,
            issue: idle.is_none().then_some("errorActivityUnavailable"),
        }
    }
}

#[cfg(target_os = "macos")]
pub fn sample() -> ActivitySample {
    use std::ffi::{c_char, c_void};
    use std::sync::{Mutex, OnceLock};
    use std::time::{Duration, Instant};
    // Session dictionaries allocate CF objects. Checking once every five
    // seconds is sufficient for idle/lock fallback and avoids 1Hz allocations.
    static LOCK_CACHE: OnceLock<Mutex<(Instant, bool)>> = OnceLock::new();
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventSourceSecondsSinceLastEventType(state: i32, event: u32) -> f64;
        fn CGSessionCopyCurrentDictionary() -> *const c_void;
    }
    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CFStringCreateWithCString(
            allocator: *const c_void,
            string: *const c_char,
            encoding: u32,
        ) -> *const c_void;
        fn CFDictionaryGetValue(dict: *const c_void, key: *const c_void) -> *const c_void;
        fn CFBooleanGetValue(value: *const c_void) -> bool;
        fn CFRelease(value: *const c_void);
    }
    unsafe {
        // Fast CoreGraphics event timestamp; no event tap or key logging.
        let idle = CGEventSourceSecondsSinceLastEventType(0, u32::MAX);
        let idle = if idle.is_finite() && idle >= 0. {
            Some(idle)
        } else {
            None
        };
        let issue = idle.is_none().then_some("errorActivityPermission");
        let cache =
            LOCK_CACHE.get_or_init(|| Mutex::new((Instant::now() - Duration::from_secs(6), false)));
        let mut cached = cache.lock().unwrap();
        if cached.0.elapsed() < Duration::from_secs(5) {
            return ActivitySample {
                idle_secs: idle,
                locked: cached.1,
                issue,
            };
        }
        let dict = CGSessionCopyCurrentDictionary();
        let mut locked = dict.is_null();
        if !dict.is_null() {
            let key = CFStringCreateWithCString(
                std::ptr::null(),
                c"CGSSessionScreenIsLocked".as_ptr(),
                0x08000100,
            );
            let value = CFDictionaryGetValue(dict, key);
            if !value.is_null() {
                locked = CFBooleanGetValue(value);
            }
            CFRelease(key);
            CFRelease(dict);
        }
        cached.0 = Instant::now();
        cached.1 = locked;
        ActivitySample {
            idle_secs: idle,
            locked,
            issue,
        }
    }
}

#[cfg(any(target_os = "linux", test))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinuxSession {
    X11,
    Wayland,
    Headless,
}

#[cfg(any(target_os = "linux", test))]
fn non_empty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|s| !s.is_empty())
}

#[cfg(any(target_os = "linux", test))]
fn classify_linux_session(
    xdg_session_type: Option<&str>,
    display: Option<&str>,
    wayland_display: Option<&str>,
) -> LinuxSession {
    let session = non_empty(xdg_session_type).map(str::to_ascii_lowercase);
    let has_display = non_empty(display).is_some();
    let has_wayland = non_empty(wayland_display).is_some();

    match session.as_deref() {
        // Never treat XWayland DISPLAY as full Wayland idle support.
        Some("wayland") => LinuxSession::Wayland,
        Some("x11") => {
            if has_display {
                LinuxSession::X11
            } else {
                LinuxSession::Headless
            }
        }
        _ => {
            if has_wayland {
                LinuxSession::Wayland
            } else if has_display {
                LinuxSession::X11
            } else {
                LinuxSession::Headless
            }
        }
    }
}

#[cfg(any(target_os = "linux", test))]
fn parse_gdbus_u64_ms(output: &str) -> Option<f64> {
    let trimmed = output.trim();
    let inner = trimmed
        .strip_prefix('(')?
        .strip_suffix(')')?
        .trim()
        .trim_end_matches(',')
        .trim();
    let number = inner.strip_prefix("uint64 ").unwrap_or(inner).trim();
    let ms: u64 = number.parse().ok()?;
    Some(ms as f64 / 1000.0)
}

#[cfg(any(target_os = "linux", test))]
fn parse_gdbus_bool(output: &str) -> Option<bool> {
    let trimmed = output.trim();
    let inner = trimmed
        .strip_prefix('(')?
        .strip_suffix(')')?
        .trim()
        .trim_end_matches(',')
        .trim();
    match inner {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

#[cfg(any(target_os = "linux", test))]
fn parse_loginctl_locked(output: &str) -> Option<bool> {
    match output.trim().to_ascii_lowercase().as_str() {
        "yes" | "true" | "1" => Some(true),
        "no" | "false" | "0" => Some(false),
        _ => None,
    }
}

#[cfg(any(target_os = "linux", test))]
fn sanitize_session_id(raw: Option<&str>) -> &str {
    match non_empty(raw) {
        Some(id)
            if !id.starts_with('-')
                && id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.')) =>
        {
            id
        }
        _ => "self",
    }
}

#[cfg(any(target_os = "linux", test))]
fn classify_probe_stderr(stderr: &str) -> &'static str {
    let lower = stderr.to_ascii_lowercase();
    if lower.contains("accessdenied")
        || lower.contains("permissiondenied")
        || lower.contains("permission denied")
        || lower.contains("notauthorized")
        || lower.contains("not authorized")
    {
        "errorActivityPermission"
    } else {
        "errorActivityX11"
    }
}

#[cfg(any(target_os = "linux", test))]
const LINUX_AUDIO_PLAYERS: [(&str, &[&str]); 3] = [
    ("/usr/bin/pw-play", &[]),
    ("/usr/bin/paplay", &[]),
    ("/usr/bin/aplay", &["-q"]),
];

#[cfg(any(target_os = "linux", test))]
fn select_linux_audio_player<F>(exists: F) -> Option<(&'static str, &'static [&'static str])>
where
    F: Fn(&str) -> bool,
{
    LINUX_AUDIO_PLAYERS
        .into_iter()
        .find(|(path, _)| exists(path))
}

#[cfg(target_os = "linux")]
fn play_linux_sound(resource_path: &std::path::Path) -> Result<(), String> {
    const BARE_AUDIO_PLAYERS: [(&str, &[&str]); 3] =
        [("pw-play", &[]), ("paplay", &[]), ("aplay", &["-q"])];
    if !resource_path.is_file() {
        return Err("errorSoundUnavailable".into());
    }
    let spawn_and_reap = |bin: &str, args: &[&str]| -> bool {
        if let Ok(mut child) = std::process::Command::new(bin)
            .args(args)
            .arg(resource_path)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
        {
            std::thread::spawn(move || {
                let _ = child.wait();
            });
            true
        } else {
            false
        }
    };
    if select_linux_audio_player(|p| std::path::Path::new(p).is_file())
        .is_some_and(|(bin, args)| spawn_and_reap(bin, args))
    {
        return Ok(());
    }
    for (bin, args) in BARE_AUDIO_PLAYERS {
        if spawn_and_reap(bin, args) {
            return Ok(());
        }
    }
    Err("errorSoundUnavailable".into())
}

#[cfg(target_os = "linux")]
mod linux_x11 {
    use std::ffi::{c_char, c_int, c_ulong, c_void};
    use std::ptr;
    use std::sync::{Mutex, OnceLock};

    const RTLD_LAZY: c_int = 1;

    #[repr(C)]
    struct XScreenSaverInfo {
        _window: c_ulong,
        _state: c_int,
        _kind: c_int,
        _til_or_since: c_ulong,
        idle: c_ulong,
        _event_mask: c_ulong,
    }

    type XOpenDisplayFn = unsafe extern "C" fn(*const c_char) -> *mut c_void;
    type XCloseDisplayFn = unsafe extern "C" fn(*mut c_void) -> c_int;
    type XDefaultRootWindowFn = unsafe extern "C" fn(*mut c_void) -> c_ulong;
    type XFreeFn = unsafe extern "C" fn(*mut c_void) -> c_int;
    type XScreenSaverQueryExtensionFn =
        unsafe extern "C" fn(*mut c_void, *mut c_int, *mut c_int) -> c_int;
    type XScreenSaverAllocInfoFn = unsafe extern "C" fn() -> *mut XScreenSaverInfo;
    type XScreenSaverQueryInfoFn =
        unsafe extern "C" fn(*mut c_void, c_ulong, *mut XScreenSaverInfo) -> c_int;

    #[link(name = "dl")]
    extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        fn dlclose(handle: *mut c_void) -> c_int;
    }

    struct X11Api {
        open_display: XOpenDisplayFn,
        close_display: XCloseDisplayFn,
        default_root_window: XDefaultRootWindowFn,
        x_free: XFreeFn,
        query_extension: XScreenSaverQueryExtensionFn,
        alloc_info: XScreenSaverAllocInfoFn,
        query_info: XScreenSaverQueryInfoFn,
    }

    // SAFETY: Function pointers loaded from process-lifetime shared libraries are
    // immutable and safe to invoke across threads when Display* access is serialized.
    unsafe impl Send for X11Api {}
    unsafe impl Sync for X11Api {}

    struct X11Connection {
        display: *mut c_void,
        info: *mut XScreenSaverInfo,
    }

    // SAFETY: Access to X11Connection is strictly serialized inside a Mutex and
    // uses a dedicated Display* not shared with GTK/GDK.
    unsafe impl Send for X11Connection {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum X11IdleError {
        LibraryUnavailable,
        DisplayOpenFailed,
        ExtensionUnavailable,
        QueryFailed,
    }

    fn load_api() -> Option<&'static X11Api> {
        static API: OnceLock<Option<X11Api>> = OnceLock::new();
        API.get_or_init(|| unsafe {
            let x11 = dlopen(c"libX11.so.6".as_ptr(), RTLD_LAZY);
            if x11.is_null() {
                return None;
            }
            let xss = dlopen(c"libXss.so.1".as_ptr(), RTLD_LAZY);
            if xss.is_null() {
                let _ = dlclose(x11);
                return None;
            }
            let open_display = dlsym(x11, c"XOpenDisplay".as_ptr());
            let close_display = dlsym(x11, c"XCloseDisplay".as_ptr());
            let default_root_window = dlsym(x11, c"XDefaultRootWindow".as_ptr());
            let x_free = dlsym(x11, c"XFree".as_ptr());
            let query_extension = dlsym(xss, c"XScreenSaverQueryExtension".as_ptr());
            let alloc_info = dlsym(xss, c"XScreenSaverAllocInfo".as_ptr());
            let query_info = dlsym(xss, c"XScreenSaverQueryInfo".as_ptr());

            if open_display.is_null()
                || close_display.is_null()
                || default_root_window.is_null()
                || x_free.is_null()
                || query_extension.is_null()
                || alloc_info.is_null()
                || query_info.is_null()
            {
                let _ = dlclose(xss);
                let _ = dlclose(x11);
                return None;
            }

            Some(X11Api {
                open_display: std::mem::transmute::<*mut c_void, XOpenDisplayFn>(open_display),
                close_display: std::mem::transmute::<*mut c_void, XCloseDisplayFn>(close_display),
                default_root_window: std::mem::transmute::<*mut c_void, XDefaultRootWindowFn>(
                    default_root_window,
                ),
                x_free: std::mem::transmute::<*mut c_void, XFreeFn>(x_free),
                query_extension: std::mem::transmute::<*mut c_void, XScreenSaverQueryExtensionFn>(
                    query_extension,
                ),
                alloc_info: std::mem::transmute::<*mut c_void, XScreenSaverAllocInfoFn>(alloc_info),
                query_info: std::mem::transmute::<*mut c_void, XScreenSaverQueryInfoFn>(query_info),
            })
        })
        .as_ref()
    }

    /// Reads only the X11 MIT-SCREEN-SAVER idle duration in seconds.
    /// No keycodes, cursor positions, or input hooks are installed.
    pub fn idle_seconds() -> Result<f64, X11IdleError> {
        let Some(api) = load_api() else {
            return Err(X11IdleError::LibraryUnavailable);
        };
        static CONN: OnceLock<Mutex<X11Connection>> = OnceLock::new();
        let lock = CONN.get_or_init(|| {
            Mutex::new(X11Connection {
                display: ptr::null_mut(),
                info: ptr::null_mut(),
            })
        });
        let Ok(mut conn) = lock.lock() else {
            return Err(X11IdleError::QueryFailed);
        };
        unsafe {
            if conn.display.is_null() {
                let display = (api.open_display)(ptr::null());
                if display.is_null() {
                    return Err(X11IdleError::DisplayOpenFailed);
                }
                let mut event_base: c_int = 0;
                let mut error_base: c_int = 0;
                if (api.query_extension)(display, &mut event_base, &mut error_base) == 0 {
                    let _ = (api.close_display)(display);
                    return Err(X11IdleError::ExtensionUnavailable);
                }
                let info = (api.alloc_info)();
                if info.is_null() {
                    let _ = (api.close_display)(display);
                    return Err(X11IdleError::QueryFailed);
                }
                conn.display = display;
                conn.info = info;
            }
            let root = (api.default_root_window)(conn.display);
            if (api.query_info)(conn.display, root, conn.info) == 0 {
                let _ = (api.x_free)(conn.info.cast());
                let _ = (api.close_display)(conn.display);
                conn.info = ptr::null_mut();
                conn.display = ptr::null_mut();
                return Err(X11IdleError::QueryFailed);
            }
            let idle_ms = (*conn.info).idle;
            Ok(idle_ms as f64 / 1000.0)
        }
    }
}

#[cfg(target_os = "linux")]
fn run_linux_probe(program: &str, args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

#[cfg(target_os = "linux")]
fn linux_screen_locked() -> bool {
    use std::sync::{Mutex, OnceLock};
    use std::time::{Duration, Instant};

    // Query lock status at most every 5 seconds, matching macOS LOCK_CACHE.
    static LOCK_CACHE: OnceLock<Mutex<(Instant, bool)>> = OnceLock::new();
    let cache =
        LOCK_CACHE.get_or_init(|| Mutex::new((Instant::now() - Duration::from_secs(6), false)));
    let Ok(mut cached) = cache.lock() else {
        return false;
    };
    if cached.0.elapsed() < Duration::from_secs(5) {
        return cached.1;
    }

    let locked = run_linux_probe(
        "gdbus",
        &[
            "call",
            "--session",
            "--timeout",
            "1",
            "--dest",
            "org.gnome.ScreenSaver",
            "--object-path",
            "/org/gnome/ScreenSaver",
            "--method",
            "org.gnome.ScreenSaver.GetActive",
        ],
    )
    .ok()
    .and_then(|out| parse_gdbus_bool(&out))
    .or_else(|| {
        run_linux_probe(
            "gdbus",
            &[
                "call",
                "--session",
                "--timeout",
                "1",
                "--dest",
                "org.freedesktop.ScreenSaver",
                "--object-path",
                "/org/freedesktop/ScreenSaver",
                "--method",
                "org.freedesktop.ScreenSaver.GetActive",
            ],
        )
        .ok()
        .and_then(|out| parse_gdbus_bool(&out))
    })
    .or_else(|| {
        let session_env = std::env::var("XDG_SESSION_ID").ok();
        let session_id = sanitize_session_id(session_env.as_deref());
        run_linux_probe(
            "loginctl",
            &["show-session", session_id, "-p", "LockedHint", "--value"],
        )
        .ok()
        .and_then(|out| parse_loginctl_locked(&out))
    })
    .unwrap_or(false);

    cached.0 = Instant::now();
    cached.1 = locked;
    locked
}

#[cfg(target_os = "linux")]
fn gnome_mutter_idle_secs() -> Result<f64, &'static str> {
    match run_linux_probe(
        "gdbus",
        &[
            "call",
            "--session",
            "--timeout",
            "1",
            "--dest",
            "org.gnome.Mutter.IdleMonitor",
            "--object-path",
            "/org/gnome/Mutter/IdleMonitor/Core",
            "--method",
            "org.gnome.Mutter.IdleMonitor.GetIdletime",
        ],
    ) {
        Ok(stdout) => parse_gdbus_u64_ms(&stdout).ok_or("errorActivityX11"),
        Err(stderr) => Err(classify_probe_stderr(&stderr)),
    }
}

#[cfg(target_os = "linux")]
/// Samples OS idle age on supported Linux X11 sessions (MIT-SCREEN-SAVER, with
/// GNOME Mutter IdleMonitor fallback on X11) and screen lock state.
/// Wayland and headless sessions report unavailable rather than faking idle.
pub fn sample() -> ActivitySample {
    let session_type = std::env::var("XDG_SESSION_TYPE").ok();
    let display = std::env::var("DISPLAY").ok();
    let wayland_display = std::env::var("WAYLAND_DISPLAY").ok();

    match classify_linux_session(
        session_type.as_deref(),
        display.as_deref(),
        wayland_display.as_deref(),
    ) {
        LinuxSession::Wayland => ActivitySample {
            idle_secs: None,
            locked: linux_screen_locked(),
            issue: Some("errorActivityWayland"),
        },
        LinuxSession::Headless => ActivitySample {
            idle_secs: None,
            locked: false,
            issue: Some("errorActivityX11"),
        },
        LinuxSession::X11 => {
            let locked = linux_screen_locked();
            match linux_x11::idle_seconds() {
                Ok(idle) if idle.is_finite() && idle >= 0.0 => ActivitySample {
                    idle_secs: Some(idle),
                    locked,
                    issue: None,
                },
                Ok(_) => ActivitySample {
                    idle_secs: None,
                    locked,
                    issue: Some("errorActivityX11"),
                },
                Err(x11_err) => match gnome_mutter_idle_secs() {
                    Ok(idle) if idle.is_finite() && idle >= 0.0 => ActivitySample {
                        idle_secs: Some(idle),
                        locked,
                        issue: None,
                    },
                    Ok(_) => ActivitySample {
                        idle_secs: None,
                        locked,
                        issue: Some("errorActivityX11"),
                    },
                    Err(dbus_issue) => {
                        let issue = match x11_err {
                            linux_x11::X11IdleError::DisplayOpenFailed => "errorActivityPermission",
                            linux_x11::X11IdleError::LibraryUnavailable
                            | linux_x11::X11IdleError::ExtensionUnavailable
                            | linux_x11::X11IdleError::QueryFailed => dbus_issue,
                        };
                        ActivitySample {
                            idle_secs: None,
                            locked,
                            issue: Some(issue),
                        }
                    }
                },
            }
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn sample() -> ActivitySample {
    ActivitySample {
        idle_secs: None,
        locked: false,
        issue: Some("errorActivityUnavailable"),
    }
}
/// Suspend-inclusive monotonic time; wall-clock changes never affect a cycle.
pub fn monotonic_seconds() -> f64 {
    #[cfg(target_os = "windows")]
    {
        #[link(name = "kernel32")]
        extern "system" {
            fn GetTickCount64() -> u64;
        }
        unsafe { GetTickCount64() as f64 / 1000. }
    }
    #[cfg(target_os = "macos")]
    {
        #[repr(C)]
        struct Timebase {
            numer: u32,
            denom: u32,
        }
        #[link(name = "System")]
        extern "C" {
            fn mach_continuous_time() -> u64;
            fn mach_timebase_info(info: *mut Timebase) -> i32;
        }
        static RATIO: std::sync::OnceLock<f64> = std::sync::OnceLock::new();
        let ratio = RATIO.get_or_init(|| unsafe {
            let mut base = Timebase { numer: 1, denom: 1 };
            mach_timebase_info(&mut base);
            base.numer as f64 / base.denom.max(1) as f64 / 1_000_000_000.
        });
        unsafe { mach_continuous_time() as f64 * ratio }
    }
    #[cfg(target_os = "linux")]
    {
        use std::ffi::{c_int, c_long};
        const CLOCK_BOOTTIME: c_int = 7;
        #[repr(C)]
        struct Timespec {
            tv_sec: c_long,
            tv_nsec: c_long,
        }
        extern "C" {
            fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
        }
        let mut ts = Timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // CLOCK_BOOTTIME is monotonic and includes time spent suspended.
        if unsafe { clock_gettime(CLOCK_BOOTTIME, &mut ts) } == 0 {
            ts.tv_sec as f64 + ts.tv_nsec as f64 / 1_000_000_000.0
        } else {
            static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
            START
                .get_or_init(std::time::Instant::now)
                .elapsed()
                .as_secs_f64()
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
        START
            .get_or_init(std::time::Instant::now)
            .elapsed()
            .as_secs_f64()
    }
}
/// Read only on-screen frame geometry while a drag starts; never titles or content.
pub fn window_frames(scale: f64) -> Vec<crate::snapping::Rect> {
    #[cfg(target_os = "windows")]
    {
        use crate::snapping::Rect;
        #[repr(C)]
        struct WinRect {
            left: i32,
            top: i32,
            right: i32,
            bottom: i32,
        }
        #[link(name = "user32")]
        extern "system" {
            fn EnumWindows(
                callback: Option<unsafe extern "system" fn(isize, isize) -> i32>,
                data: isize,
            ) -> i32;
            fn IsWindowVisible(hwnd: isize) -> i32;
            fn IsIconic(hwnd: isize) -> i32;
            fn GetWindowThreadProcessId(hwnd: isize, pid: *mut u32) -> u32;
            fn GetWindowRect(hwnd: isize, rect: *mut WinRect) -> i32;
        }
        #[link(name = "dwmapi")]
        extern "system" {
            fn DwmGetWindowAttribute(
                hwnd: isize,
                attribute: u32,
                value: *mut std::ffi::c_void,
                size: u32,
            ) -> i32;
        }
        unsafe extern "system" fn collect(hwnd: isize, data: isize) -> i32 {
            if IsWindowVisible(hwnd) == 0 || IsIconic(hwnd) != 0 {
                return 1;
            }
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);
            if pid == std::process::id() {
                return 1;
            }
            let mut cloaked = 0u32;
            DwmGetWindowAttribute(hwnd, 14, (&mut cloaked as *mut u32).cast(), 4);
            if cloaked != 0 {
                return 1;
            }
            let mut r = WinRect {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            };
            if DwmGetWindowAttribute(
                hwnd,
                9,
                (&mut r as *mut WinRect).cast(),
                std::mem::size_of::<WinRect>() as u32,
            ) != 0
                && GetWindowRect(hwnd, &mut r) == 0
            {
                return 1;
            }
            if r.right - r.left >= 100 && r.bottom - r.top >= 60 {
                (*(data as *mut Vec<Rect>)).push(Rect {
                    left: r.left,
                    top: r.top,
                    right: r.right,
                    bottom: r.bottom,
                });
            }
            1
        }
        let _ = scale;
        let mut frames = Vec::new();
        unsafe {
            EnumWindows(Some(collect), (&mut frames as *mut Vec<Rect>) as isize);
        };
        frames
    }
    #[cfg(target_os = "macos")]
    {
        use std::ffi::c_void;
        #[repr(C)]
        struct CGRect {
            x: f64,
            y: f64,
            width: f64,
            height: f64,
        }
        #[link(name = "CoreGraphics", kind = "framework")]
        extern "C" {
            fn CGWindowListCopyWindowInfo(options: u32, relative: u32) -> *const c_void;
            fn CGRectMakeWithDictionaryRepresentation(
                dict: *const c_void,
                rect: *mut CGRect,
            ) -> bool;
            static kCGWindowBounds: *const c_void;
            static kCGWindowOwnerPID: *const c_void;
            static kCGWindowLayer: *const c_void;
        }
        #[link(name = "CoreFoundation", kind = "framework")]
        extern "C" {
            fn CFArrayGetCount(array: *const c_void) -> isize;
            fn CFArrayGetValueAtIndex(array: *const c_void, index: isize) -> *const c_void;
            fn CFDictionaryGetValue(dict: *const c_void, key: *const c_void) -> *const c_void;
            fn CFNumberGetValue(number: *const c_void, kind: i32, value: *mut i32) -> bool;
            fn CFRelease(value: *const c_void);
        }
        let mut frames = Vec::new();
        unsafe {
            let list = CGWindowListCopyWindowInfo(1 | 16, 0);
            if list.is_null() {
                return frames;
            }
            for i in 0..CFArrayGetCount(list) {
                let dict = CFArrayGetValueAtIndex(list, i);
                let p = CFDictionaryGetValue(dict, kCGWindowOwnerPID);
                let l = CFDictionaryGetValue(dict, kCGWindowLayer);
                let (mut pid, mut layer) = (0, 0);
                if !p.is_null() {
                    CFNumberGetValue(p, 3, &mut pid);
                }
                if !l.is_null() {
                    CFNumberGetValue(l, 3, &mut layer);
                }
                if pid == std::process::id() as i32 || layer != 0 {
                    continue;
                }
                let b = CFDictionaryGetValue(dict, kCGWindowBounds);
                let mut r = CGRect {
                    x: 0.,
                    y: 0.,
                    width: 0.,
                    height: 0.,
                };
                if !b.is_null()
                    && CGRectMakeWithDictionaryRepresentation(b, &mut r)
                    && r.width >= 100.
                    && r.height >= 60.
                {
                    frames.push(crate::snapping::Rect {
                        left: (r.x * scale).round() as i32,
                        top: (r.y * scale).round() as i32,
                        right: ((r.x + r.width) * scale).round() as i32,
                        bottom: ((r.y + r.height) * scale).round() as i32,
                    });
                }
            }
            CFRelease(list);
        }
        frames
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let _ = scale;
        Vec::new()
    }
}
