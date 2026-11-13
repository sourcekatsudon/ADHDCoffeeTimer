# Platform status

This describes the checked-in implementation, not a promise inferred from Tauri's supported targets. A successful package build does not verify desktop behavior.

| Platform | Implementation | Release qualification |
| --- | --- | --- |
| Windows | Idle age, lock detection, sound playback, native window handling and snapping are implemented. | Validate the current release artifact on Windows. |
| macOS, Intel and Apple Silicon | Idle age, lock detection, sound playback, native vibrancy and window geometry are implemented. | Native interaction, suspend, Spaces and actual hardware behavior require a Mac; cross-platform code inspection cannot certify them. |
| Linux | Incomplete platform backend. | **Unsupported; Linux release artifacts are on hold.** |

## Linux blockers

In `src-tauri/src/platform.rs`, the non-Windows/non-macOS `sample()` returns `(None, false)`. It supplies neither idle age nor lock state. In `src-tauri/src/engine.rs`, `Engine::tick()` handles `None` before inspecting `auto_detect` or advancing time: it returns immediately with `Unavailable` (or `Paused` when manually paused). Consequently, input-triggered start, idle pause/reset and input-triggered resume do not work on Linux. Manual Start/Resume can briefly set `Running`, but subsequent ticks still cannot advance the countdown, even with automatic detection disabled.

The same platform fallback makes `play_sound()` a no-op and `window_frames()` an empty list, so optional sound and snapping to other application windows are absent. Native backdrop handling in `native_glass()` is Windows/macOS-specific. Generic Tauri window dragging, tray, window visibility, always-on-top, autostart registration and WebKit rendering have not been tested on a Linux desktop. Their presence in generic code is not evidence of Linux functionality. The Linux clock uses `std::time::Instant`; its suspend behavior also needs explicit verification before describing it as suspend-inclusive.

The audit host is Windows. An Ubuntu 24.04 WSL2 distribution is available with display environment variables, but the inspected distribution has no `cargo`, `pkg-config` or `Xvfb` available. No Linux build or desktop behavior test was run. A WSLg window alone would not establish behavior across native X11 and Wayland desktop sessions.

## Work required before a Linux release

1. Define an explicit X11/Wayland and desktop-environment support scope. Implement supported idle/lock/session backends and make unavailable or denied permissions visible. Do not report a constant fake idle age as operating-system detection.
2. Decide how manual mode should behave when operating-system samples are unavailable; separate that policy from automatic detection and cover it with state-machine tests.
3. Implement or explicitly disable unsupported sound, backdrop and other-window snapping options. Verify monotonic suspend accounting.
4. Build on Linux with the native GTK/WebKit/AppIndicator dependencies, then test the packaged app in real supported desktop sessions. Exercise first input start; idle pause; long-idle reset; lock/unlock; suspend/resume; manual start/pause/resume with auto detection on and off; cycle completion and optional sound; persistence; autostart across login; tray actions and quit; transparent windows, scaling, dragging and snapping.
5. Record the distribution, desktop, display server, artifact hash and observed results. Publish Linux artifacts only after the supported behavior passes; compilation and headless tests alone are insufficient.
