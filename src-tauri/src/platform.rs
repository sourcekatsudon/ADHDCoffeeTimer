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
        assert_eq!(sanitize_session_id(Some("--property=foo")), "auto");
        assert_eq!(sanitize_session_id(Some("1; id")), "auto");
        assert_eq!(sanitize_session_id(Some("   ")), "auto");
        assert_eq!(sanitize_session_id(Some("")), "auto");
        assert_eq!(sanitize_session_id(None), "auto");
    }

    #[test]
    fn linux_audio_player_selection_prefers_pipewire_then_pulse_then_alsa() {
        const EMPTY: &[&str] = &[];
        const QUIET: &[&str] = &["-q"];
        assert_eq!(
            linux_audio_candidates(|_| true, None),
            vec![
                ("/usr/bin/pw-play".to_string(), EMPTY),
                ("/usr/bin/paplay".to_string(), EMPTY),
                ("/usr/bin/aplay".to_string(), QUIET),
            ]
        );
        assert_eq!(
            linux_audio_candidates(|p| p == "/usr/bin/paplay" || p == "/usr/bin/aplay", None),
            vec![
                ("/usr/bin/paplay".to_string(), EMPTY),
                ("/usr/bin/aplay".to_string(), QUIET),
            ]
        );
        assert_eq!(
            linux_audio_candidates(|p| p == "/custom/bin/aplay", Some("/custom/bin:/other")),
            vec![("/custom/bin/aplay".to_string(), QUIET)]
        );
        assert!(linux_audio_candidates(|_| false, Some("/nonexistent")).is_empty());
    }

    #[test]
    fn linux_audio_fallback_chain_recovers_from_nonzero_exit_and_timeout() {
        const EMPTY: &[&str] = &[];
        const QUIET: &[&str] = &["-q"];
        let candidates = vec![
            ("/usr/bin/pw-play".to_string(), EMPTY),
            ("/usr/bin/paplay".to_string(), EMPTY),
            ("/usr/bin/aplay".to_string(), QUIET),
        ];

        // First exits nonzero -> falls back to second -> stops without calling third.
        let mut called = Vec::new();
        let ok = run_audio_fallback_chain(&candidates, |bin, _| {
            called.push(bin.to_string());
            if bin == "/usr/bin/pw-play" {
                CommandOutcome::Exited {
                    success: false,
                    stdout: String::new(),
                    stderr: "pipewire down".into(),
                }
            } else {
                CommandOutcome::Exited {
                    success: true,
                    stdout: String::new(),
                    stderr: String::new(),
                }
            }
        });
        assert!(ok);
        assert_eq!(called, vec!["/usr/bin/pw-play", "/usr/bin/paplay"]);

        // First times out, second fails to spawn, third succeeds.
        let mut called = Vec::new();
        let ok = run_audio_fallback_chain(&candidates, |bin, _| {
            called.push(bin.to_string());
            match bin {
                "/usr/bin/pw-play" => CommandOutcome::TimedOut,
                "/usr/bin/paplay" => CommandOutcome::SpawnFailed("enoent".into()),
                _ => CommandOutcome::Exited {
                    success: true,
                    stdout: String::new(),
                    stderr: String::new(),
                },
            }
        });
        assert!(ok);
        assert_eq!(
            called,
            vec!["/usr/bin/pw-play", "/usr/bin/paplay", "/usr/bin/aplay"]
        );

        // All candidates exit nonzero -> returns false after trying all.
        let mut called = Vec::new();
        let ok = run_audio_fallback_chain(&candidates, |bin, _| {
            called.push(bin.to_string());
            CommandOutcome::Exited {
                success: false,
                stdout: String::new(),
                stderr: "failed".into(),
            }
        });
        assert!(!ok);
        assert_eq!(called.len(), 3);
    }

    #[cfg(unix)]
    #[test]
    fn bounded_subprocess_times_out_drains_large_output_and_chains_real_fallback() {
        use std::time::{Duration, Instant};

        // Hung process is killed and reaped promptly within timeout bound.
        let start = Instant::now();
        let outcome = run_command_with_timeout("/bin/sleep", &["10"], Duration::from_millis(60));
        assert_eq!(outcome, CommandOutcome::TimedOut);
        assert!(
            start.elapsed() < Duration::from_secs(2),
            "timed-out child blocked for {:?}",
            start.elapsed()
        );

        // >64 KiB stdout does not pipe-deadlock and capture is bounded.
        let outcome = run_command_with_timeout(
            "/bin/sh",
            &[
                "-c",
                "dd if=/dev/zero bs=1024 count=128 2>/dev/null | tr '\\0' 'x'",
            ],
            Duration::from_secs(3),
        );
        match outcome {
            CommandOutcome::Exited {
                success, stdout, ..
            } => {
                assert!(success);
                assert_eq!(stdout.len(), MAX_CAPTURE_BYTES);
            }
            other => panic!("expected successful bounded output, got {other:?}"),
        }

        // Real child fallback: nonzero exit (/usr/bin/false) falls back to success (/usr/bin/true).
        const EMPTY: &[&str] = &[];
        let chain = vec![
            ("/usr/bin/false".to_string(), EMPTY),
            ("/usr/bin/true".to_string(), EMPTY),
        ];
        let mut attempts = 0usize;
        let ok = run_audio_fallback_chain(&chain, |bin, args| {
            attempts += 1;
            run_command_with_timeout(bin, args, Duration::from_secs(1))
        });
        assert!(ok);
        assert_eq!(attempts, 2);
    }

    #[cfg(unix)]
    #[test]
    fn exited_probe_descendant_cannot_keep_capture_pipes_open_past_deadline() {
        use std::time::{Duration, Instant};
        let start = Instant::now();
        let result = run_command_with_timeout(
            "/bin/sh",
            &["-c", "sleep 2 & exit 0"],
            Duration::from_millis(60),
        );
        // A loaded host can time out the shell before it exits; both outcomes
        // honor the deadline. Spawn failures and nonzero exits remain failures.
        assert!(matches!(
            result,
            CommandOutcome::Exited { success: true, .. } | CommandOutcome::TimedOut
        ));
        assert!(
            start.elapsed() < Duration::from_secs(1),
            "descendant held capture open: {:?}",
            start.elapsed()
        );
    }

    #[test]
    fn background_sampler_never_stalls_caller_bounds_refreshes_and_expires_stale_cache() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Condvar, Mutex,
        };
        use std::time::{Duration, Instant};

        // Gate lets the test control exactly when each background probe completes.
        let gate = Arc::new((Mutex::new((0usize, 0usize)), Condvar::new()));
        let probe_calls = Arc::new(AtomicUsize::new(0));
        let gate_worker = Arc::clone(&gate);
        let calls_worker = Arc::clone(&probe_calls);

        let sampler = BackgroundSampler::new(
            move || {
                let n = calls_worker.fetch_add(1, Ordering::SeqCst) + 1;
                let (lock, cvar) = &*gate_worker;
                let mut state = lock.lock().unwrap();
                state.0 = n;
                cvar.notify_all();
                while state.1 < n {
                    state = cvar.wait(state).unwrap();
                }
                ActivitySample {
                    idle_secs: Some(n as f64),
                    locked: true,
                    issue: None,
                }
            },
            Duration::from_millis(40),
            Duration::from_millis(120),
        );

        // Wait until first background probe is in flight, then hammer sample() while probe is blocked.
        {
            let (lock, cvar) = &*gate;
            let mut state = lock.lock().unwrap();
            while state.0 < 1 {
                state = cvar.wait(state).unwrap();
            }
        }
        let started = Instant::now();
        for _ in 0..10 {
            let s = sampler.sample();
            assert_eq!(s.idle_secs, None);
            assert_eq!(s.issue, Some("errorActivityUnavailable"));
        }
        assert!(
            started.elapsed() < Duration::from_millis(100),
            "sample() stalled caller for {:?}",
            started.elapsed()
        );
        // Only ONE background probe job may be in flight despite 10 rapid calls.
        assert_eq!(probe_calls.load(Ordering::SeqCst), 1);

        // Release probe #1 and wait for cached fresh sample to become visible.
        {
            let (lock, cvar) = &*gate;
            lock.lock().unwrap().1 = 1;
            cvar.notify_all();
        }
        let deadline = Instant::now() + Duration::from_secs(1);
        let updated_at = loop {
            if let Some((_, at)) = sampler.shared.0.lock().unwrap().latest {
                break at;
            }
            assert!(Instant::now() < deadline, "first probe did not complete");
            std::thread::sleep(Duration::from_millis(5));
        };
        // Advance the sampling clock explicitly: host scheduling delays must not
        // change whether 50ms and 130ms observations straddle the 120ms expiry.
        let fresh = sampler.sample_at(updated_at);
        assert_eq!(fresh.idle_secs, Some(1.0));
        assert_eq!(fresh.issue, None);
        let still_fresh = sampler.sample_at(updated_at + Duration::from_millis(50));
        assert_eq!(still_fresh.idle_secs, Some(1.0));
        // The second probe is blocked or queued; expiry must never wait for it.
        let expired_start = Instant::now();
        let expired = sampler.sample_at(updated_at + Duration::from_millis(130));
        assert!(expired_start.elapsed() < Duration::from_millis(50));
        assert_eq!(expired.idle_secs, None);
        assert!(
            expired.locked,
            "stale locked observation must not become an unlock"
        );
        assert_eq!(expired.issue, Some("errorActivityUnavailable"));

        // Unblock worker before dropping sampler so worker exits cleanly.
        {
            let (lock, cvar) = &*gate;
            lock.lock().unwrap().1 = usize::MAX;
            cvar.notify_all();
        }
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
        _ => "auto",
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
const MAX_CAPTURE_BYTES: usize = 16 * 1024;

#[cfg(any(target_os = "linux", test))]
#[derive(Debug, Clone, PartialEq, Eq)]
enum CommandOutcome {
    Exited {
        success: bool,
        stdout: String,
        stderr: String,
    },
    TimedOut,
    SpawnFailed(String),
}

#[cfg(any(target_os = "linux", test))]
fn drain_bounded<R: std::io::Read>(mut reader: R, limit: usize) -> Vec<u8> {
    let mut out = Vec::new();
    let mut buf = [0u8; 1024];
    loop {
        match reader.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                if out.len() < limit {
                    let take = (limit - out.len()).min(n);
                    out.extend_from_slice(&buf[..take]);
                }
            }
        }
    }
    out
}

#[cfg(any(target_os = "linux", test))]
fn run_command_with_timeout(
    program: &str,
    args: &[&str],
    timeout: std::time::Duration,
) -> CommandOutcome {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // Isolate this probe and its descendants from the application process group.
        command.process_group(0);
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(err) => return CommandOutcome::SpawnFailed(err.to_string()),
    };

    // Drain stdout and stderr concurrently to avoid pipe deadlocks.
    let stdout_handle = child
        .stdout
        .take()
        .map(|r| std::thread::spawn(move || drain_bounded(r, MAX_CAPTURE_BYTES)));
    let stderr_handle = child
        .stderr
        .take()
        .map(|r| std::thread::spawn(move || drain_bounded(r, MAX_CAPTURE_BYTES)));

    let deadline = Instant::now() + timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {
                let now = Instant::now();
                if now >= deadline {
                    let _ = child.kill();
                    let _ = child.wait();
                    break None;
                }
                let remaining = deadline.saturating_duration_since(now);
                std::thread::sleep(remaining.min(Duration::from_millis(10)));
            }
            Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
        }
    };

    // A successful parent may leave descendants holding its pipes open. Clear
    // only this dedicated group before joining drainers, including after timeout.
    #[cfg(unix)]
    unsafe {
        extern "C" {
            fn kill(pid: i32, sig: i32) -> i32;
        }
        let _ = kill(-(child.id() as i32), 9); // SIGKILL, dedicated probe process group.
    }

    match status {
        Some(exit) => {
            let stdout = stdout_handle
                .and_then(|h| h.join().ok())
                .map(|b| String::from_utf8_lossy(&b).into_owned())
                .unwrap_or_default();
            let stderr = stderr_handle
                .and_then(|h| h.join().ok())
                .map(|b| String::from_utf8_lossy(&b).into_owned())
                .unwrap_or_default();
            CommandOutcome::Exited {
                success: exit.success(),
                stdout,
                stderr,
            }
        }
        None => CommandOutcome::TimedOut,
    }
}

#[cfg(any(target_os = "linux", test))]
const LINUX_AUDIO_SPECS: [(&str, &str, &[&str]); 3] = [
    ("/usr/bin/pw-play", "pw-play", &[]),
    ("/usr/bin/paplay", "paplay", &[]),
    ("/usr/bin/aplay", "aplay", &["-q"]),
];

#[cfg(any(target_os = "linux", test))]
fn linux_audio_candidates<F>(
    exists: F,
    path_var: Option<&str>,
) -> Vec<(String, &'static [&'static str])>
where
    F: Fn(&str) -> bool,
{
    let path_dirs: Vec<&str> = path_var
        .map(|v| v.split(':').filter(|d| !d.is_empty()).collect())
        .unwrap_or_default();
    let mut out = Vec::new();
    for (abs, bare, args) in LINUX_AUDIO_SPECS {
        if exists(abs) {
            out.push((abs.to_string(), args));
        } else if let Some(found) = path_dirs
            .iter()
            .map(|dir| format!("{dir}/{bare}"))
            .find(|candidate| exists(candidate))
        {
            out.push((found, args));
        }
    }
    out
}

#[cfg(any(target_os = "linux", test))]
fn run_audio_fallback_chain<F>(candidates: &[(String, &[&str])], mut run: F) -> bool
where
    F: FnMut(&str, &[&str]) -> CommandOutcome,
{
    for (bin, args) in candidates {
        match run(bin, args) {
            CommandOutcome::Exited { success: true, .. } => return true,
            CommandOutcome::Exited { success: false, .. }
            | CommandOutcome::TimedOut
            | CommandOutcome::SpawnFailed(_) => continue,
        }
    }
    false
}

#[cfg(any(target_os = "linux", test))]
struct SamplerState {
    latest: Option<(ActivitySample, std::time::Instant)>,
    refresh_requested: bool,
    in_flight: bool,
    shutdown: bool,
}

#[cfg(any(target_os = "linux", test))]
struct BackgroundSampler {
    shared: std::sync::Arc<(std::sync::Mutex<SamplerState>, std::sync::Condvar)>,
    refresh_interval: std::time::Duration,
    max_age: std::time::Duration,
}

#[cfg(any(target_os = "linux", test))]
impl BackgroundSampler {
    fn new<F>(
        mut probe: F,
        refresh_interval: std::time::Duration,
        max_age: std::time::Duration,
    ) -> Self
    where
        F: FnMut() -> ActivitySample + Send + 'static,
    {
        use std::sync::{Arc, Condvar, Mutex};
        use std::time::Instant;

        let shared = Arc::new((
            Mutex::new(SamplerState {
                latest: None,
                refresh_requested: true,
                in_flight: false,
                shutdown: false,
            }),
            Condvar::new(),
        ));
        let worker = Arc::clone(&shared);
        std::thread::spawn(move || {
            let (lock, cvar) = &*worker;
            loop {
                {
                    let Ok(mut state) = lock.lock() else {
                        return;
                    };
                    while !state.refresh_requested && !state.shutdown {
                        state = match cvar.wait(state) {
                            Ok(guard) => guard,
                            Err(_) => return,
                        };
                    }
                    if state.shutdown {
                        return;
                    }
                    state.refresh_requested = false;
                    state.in_flight = true;
                }
                let sample = probe();
                let Ok(mut state) = lock.lock() else {
                    return;
                };
                state.latest = Some((sample, Instant::now()));
                state.in_flight = false;
            }
        });
        Self {
            shared,
            refresh_interval,
            max_age,
        }
    }

    fn sample(&self) -> ActivitySample {
        self.sample_at(std::time::Instant::now())
    }

    fn sample_at(&self, now: std::time::Instant) -> ActivitySample {
        let (lock, cvar) = &*self.shared;
        let Ok(mut state) = lock.lock() else {
            return ActivitySample {
                idle_secs: None,
                locked: false,
                issue: Some("errorActivityUnavailable"),
            };
        };
        let needs_refresh = !state.in_flight
            && state
                .latest
                .is_none_or(|(_, at)| now.saturating_duration_since(at) >= self.refresh_interval);
        if needs_refresh {
            state.refresh_requested = true;
            cvar.notify_one();
        }
        match state.latest {
            Some((sample, updated_at))
                if now.saturating_duration_since(updated_at) <= self.max_age =>
            {
                sample
            }
            Some((stale, _)) => ActivitySample {
                idle_secs: None,
                locked: stale.locked,
                issue: Some(stale.issue.unwrap_or("errorActivityUnavailable")),
            },
            None => ActivitySample {
                idle_secs: None,
                locked: false,
                issue: Some("errorActivityUnavailable"),
            },
        }
    }
}

#[cfg(any(target_os = "linux", test))]
impl Drop for BackgroundSampler {
    fn drop(&mut self) {
        let (lock, cvar) = &*self.shared;
        if let Ok(mut state) = lock.lock() {
            state.shutdown = true;
            cvar.notify_one();
        }
    }
}

#[cfg(target_os = "linux")]
fn play_linux_sound(resource_path: &std::path::Path) -> Result<(), String> {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    const AUDIO_TIMEOUT: Duration = Duration::from_secs(10);
    static AUDIO_IN_FLIGHT: AtomicBool = AtomicBool::new(false);

    struct ClearOnDrop(&'static AtomicBool);
    impl Drop for ClearOnDrop {
        fn drop(&mut self) {
            self.0.store(false, Ordering::Release);
        }
    }

    if !resource_path.is_file() {
        return Err("errorSoundUnavailable".into());
    }
    let path_var = std::env::var("PATH").ok();
    let candidates =
        linux_audio_candidates(|p| std::path::Path::new(p).is_file(), path_var.as_deref());
    if candidates.is_empty() {
        return Err("errorSoundUnavailable".into());
    }
    // Bound concurrent playback threads while still reporting accepted dispatch.
    if AUDIO_IN_FLIGHT
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return Ok(());
    }
    let audio_file = resource_path.to_string_lossy().into_owned();
    std::thread::spawn(move || {
        let _guard = ClearOnDrop(&AUDIO_IN_FLIGHT);
        let _ = run_audio_fallback_chain(&candidates, |bin, args| {
            let mut full_args: Vec<&str> = Vec::with_capacity(args.len() + 1);
            full_args.extend_from_slice(args);
            full_args.push(&audio_file);
            run_command_with_timeout(bin, &full_args, AUDIO_TIMEOUT)
        });
    });
    Ok(())
}

#[cfg(target_os = "linux")]
mod linux_x11 {
    use std::ffi::{c_char, c_int, c_ulong, c_void};
    use std::ptr;
    use std::sync::OnceLock;

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
    // immutable; X11Sampler itself is owned exclusively by the single sampler thread.
    unsafe impl Send for X11Api {}
    unsafe impl Sync for X11Api {}

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

    /// Dedicated X11 connection used exclusively on the single background sampler thread.
    pub struct X11Sampler {
        display: *mut c_void,
        info: *mut XScreenSaverInfo,
    }

    // SAFETY: Moved once at initialization to the single background sampler thread;
    // never shared concurrently across threads and never shares GTK/GDK's Display*.
    unsafe impl Send for X11Sampler {}

    impl X11Sampler {
        pub fn new() -> Self {
            Self {
                display: ptr::null_mut(),
                info: ptr::null_mut(),
            }
        }

        fn reset(&mut self, api: &X11Api) {
            unsafe {
                if !self.info.is_null() {
                    let _ = (api.x_free)(self.info.cast());
                    self.info = ptr::null_mut();
                }
                if !self.display.is_null() {
                    let _ = (api.close_display)(self.display);
                    self.display = ptr::null_mut();
                }
            }
        }

        /// Reads only the X11 MIT-SCREEN-SAVER idle duration in seconds.
        /// No keycodes, cursor positions, or input hooks are installed.
        pub fn idle_seconds(&mut self) -> Result<f64, X11IdleError> {
            let Some(api) = load_api() else {
                return Err(X11IdleError::LibraryUnavailable);
            };
            unsafe {
                if self.display.is_null() {
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
                    self.display = display;
                    self.info = info;
                }
                let root = (api.default_root_window)(self.display);
                if (api.query_info)(self.display, root, self.info) == 0 {
                    self.reset(api);
                    return Err(X11IdleError::QueryFailed);
                }
                let idle_ms = (*self.info).idle;
                Ok(idle_ms as f64 / 1000.0)
            }
        }
    }

    impl Drop for X11Sampler {
        fn drop(&mut self) {
            if let Some(api) = load_api() {
                self.reset(api);
            }
        }
    }
}

#[cfg(target_os = "linux")]
const LINUX_PROBE_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(1200);

#[cfg(target_os = "linux")]
fn run_linux_probe(program: &str, args: &[&str]) -> Result<String, String> {
    match run_command_with_timeout(program, args, LINUX_PROBE_TIMEOUT) {
        CommandOutcome::Exited {
            success: true,
            stdout,
            ..
        } => Ok(stdout),
        CommandOutcome::Exited {
            success: false,
            stderr,
            ..
        } => Err(stderr),
        CommandOutcome::TimedOut => Err("probe timed out".into()),
        CommandOutcome::SpawnFailed(err) => Err(err),
    }
}

#[cfg(target_os = "linux")]
struct LinuxLockCache {
    last_checked: Option<std::time::Instant>,
    locked: bool,
}

#[cfg(target_os = "linux")]
impl LinuxLockCache {
    fn new() -> Self {
        Self {
            last_checked: None,
            locked: false,
        }
    }

    fn is_locked(&mut self) -> bool {
        use std::time::{Duration, Instant};
        if self
            .last_checked
            .is_some_and(|at| at.elapsed() < Duration::from_secs(5))
        {
            return self.locked;
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
        .unwrap_or(self.locked);

        self.last_checked = Some(Instant::now());
        self.locked = locked;
        locked
    }
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
fn probe_linux_activity(
    x11: &mut linux_x11::X11Sampler,
    lock_cache: &mut LinuxLockCache,
) -> ActivitySample {
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
            locked: lock_cache.is_locked(),
            issue: Some("errorActivityWayland"),
        },
        LinuxSession::Headless => ActivitySample {
            idle_secs: None,
            locked: false,
            issue: Some("errorActivityX11"),
        },
        LinuxSession::X11 => {
            let locked = lock_cache.is_locked();
            match x11.idle_seconds() {
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

#[cfg(target_os = "linux")]
/// Returns the latest fresh sample from a single bounded background sampler
/// thread so the 1Hz tick loop never blocks on subprocesses or X11/D-Bus I/O.
pub fn sample() -> ActivitySample {
    use std::sync::OnceLock;
    use std::time::Duration;

    const REFRESH_INTERVAL: Duration = Duration::from_millis(800);
    const MAX_SAMPLE_AGE: Duration = Duration::from_secs(3);

    static SAMPLER: OnceLock<BackgroundSampler> = OnceLock::new();
    SAMPLER
        .get_or_init(|| {
            let mut x11 = linux_x11::X11Sampler::new();
            let mut lock_cache = LinuxLockCache::new();
            BackgroundSampler::new(
                move || probe_linux_activity(&mut x11, &mut lock_cache),
                REFRESH_INTERVAL,
                MAX_SAMPLE_AGE,
            )
        })
        .sample()
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
