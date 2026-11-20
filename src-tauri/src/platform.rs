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
    #[test]
    fn drag_coordinates_support_negative_monitors() {
        for (x, y) in [(-1920, -1080), (3840, 2160), (0, 0), (-32768, 32767)] {
            let packed = super::packed_point(x, y) as u32;
            assert_eq!(packed as i16 as i32, x);
            assert_eq!((packed >> 16) as i16 as i32, y);
        }
    }
}

pub fn play_sound(resource_path: &std::path::Path) {
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
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("/usr/bin/afplay")
            .arg(resource_path)
            .spawn();
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let _ = resource_path;
}

#[cfg(target_os = "windows")]
/// Only the age of the last input is read. There are no input hooks.
pub fn sample() -> (Option<f64>, bool) {
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
        (idle, locked)
    }
}

#[cfg(target_os = "macos")]
pub fn sample() -> (Option<f64>, bool) {
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
        let idle = if idle.is_finite() && idle >= 0. { Some(idle) } else { None };
        let cache = LOCK_CACHE.get_or_init(|| Mutex::new((Instant::now() - Duration::from_secs(6), false)));
        let mut cached = cache.lock().unwrap();
        if cached.0.elapsed() < Duration::from_secs(5) {
            return (idle, cached.1);
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
        (idle, locked)
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn sample() -> (Option<f64>, bool) {
    (None, false)
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
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
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
