# Platform status

This describes the checked-in implementation, not a promise inferred from Tauri's supported targets. A successful package build does not verify desktop behavior.

| Platform | Implementation | Release qualification |
| --- | --- | --- |
| Windows | Idle age, lock detection, sound playback, native window handling and snapping are implemented. | The current UI has been exercised on Windows, including language/size changes and saved state. The packaged release is checked separately below. |
| macOS, Intel and Apple Silicon | Idle age, lock detection, sound playback, native vibrancy and window geometry are implemented. | Native interaction, suspend, Spaces and actual hardware behavior require a Mac; cross-platform code inspection cannot certify them. |
| Linux (Ubuntu 24.04 LTS x86_64 GNOME on X11 target) | X11 MIT-SCREEN-SAVER idle-age sampling (`libXss.so.1` via runtime `dlopen`, with GNOME Mutter `IdleMonitor` fallback on X11), GNOME/freedesktop/`loginctl` lock polling, `CLOCK_BOOTTIME` suspend-inclusive monotonic clock, manual countdown decoupled from auto-detect availability, and PipeWire/PulseAudio/ALSA chime playback (`pw-play`/`paplay`/`aplay`) are implemented in source. Wayland auto-detection is explicitly unsupported and surfaces an unavailable UI state. | **Unqualified candidate source; no public Linux release yet.** Native Ubuntu 24.04 x86_64 `.deb` build and real GNOME X11 desktop QA remain pending. |

## Installing on Windows and macOS

On Windows x64, run the `.exe` installer from the README. This release is not code-signed, so Microsoft Defender SmartScreen may show an unknown-publisher warning. Only proceed if you trust the project and obtained the file from this repository's Release assets. WebView2 is required; the installer can download Microsoft's runtime when it is missing.

On a Mac, use **Apple Silicon** for an M-series chip or **Intel** for an Intel processor (Apple menu → About This Mac). Open the matching `.dmg` and drag **ADHD Coffee Time** into Applications. macOS 11 or later is required.

The Mac builds are ad-hoc signed and are **not notarized** by Apple. If Gatekeeper blocks the first launch and you trust this release, attempt to open it, then follow Apple's **Privacy & Security → Open Anyway** instructions. Older macOS versions use System Preferences rather than System Settings. See [Apple's guidance for apps from an unidentified developer](https://support.apple.com/en-us/102445). Do not disable Gatekeeper system-wide.

Building, archive validation, and code signing checks do not establish that native Mac activity detection, tray/menu-bar interaction, Spaces, suspend/resume, or login startup work correctly on every Mac. Those hardware/desktop checks remain unverified for this release.

## Linux

### Implemented scope (unqualified candidate)

Target baseline is **Ubuntu 24.04 LTS x86_64 GNOME on X11**. Wayland automatic activity detection is **not** claimed or implemented.

- **Idle-age detection (`src-tauri/src/platform.rs`):** On X11 sessions (`XDG_SESSION_TYPE=x11` with `DISPLAY`, or `DISPLAY` without `WAYLAND_DISPLAY`), the backend dynamically loads `libX11.so.6` and `libXss.so.1` (`XScreenSaverQueryInfo`) to sample only the scalar millisecond idle age from the X server, with no keylogging or input hooks. If `libXss.so.1` or the MIT-SCREEN-SAVER extension is unavailable on an X11 GNOME session, it falls back to GNOME's `org.gnome.Mutter.IdleMonitor.GetIdletime` over `gdbus`. On Wayland sessions (including when XWayland sets `DISPLAY`) and headless sessions, `sample()` reports `None` with an explicit localized reason (`errorActivityWayland`, `errorActivityX11`, or `errorActivityPermission`) rather than faking constant activity.
- **Lock detection:** Polls `org.gnome.ScreenSaver.GetActive`, `org.freedesktop.ScreenSaver.GetActive`, or `loginctl show-session <id> -p LockedHint --value` at most once every 5 seconds.
- **Manual mode separation (`src-tauri/src/engine.rs`):** When **Detect activity automatically** (`autoDetect`) is disabled, explicit Start / Pause / Resume advances and completes the countdown even when OS idle sampling is unavailable (such as on Wayland), while still refusing to charge locked-screen time or suspend/scheduler gaps.
- **Suspend-inclusive clock:** `monotonic_seconds()` on Linux reads `clock_gettime(CLOCK_BOOTTIME)` so suspend/resume intervals are included in gap detection and long-suspend resets.
- **Sound, snapping, and backdrop:**
  - **Optional break chime:** Spawns `pw-play`, `paplay`, or `aplay -q` asynchronously (reaped without zombies) against the bundled `sounds/seatbelt.wav`. If no player is present, previewing sound in Settings reports a localized error without blocking visual break notifications.
  - **Window snapping:** Monitor work-area edge snapping uses Tauri's monitor geometry. Snapping to other application windows (`window_frames()`) is intentionally disabled on Linux (returns an empty list) to avoid unsafe X11 `BadWindow` error-handler conflicts with GTK/GDK.
  - **Window backdrop:** `native_glass()` has no OS blur/vibrancy backend on Linux; the UI uses CSS translucency over a transparent GTK/WebKit2GTK window.

### Qualification status and pending work before a public Linux release

No Ubuntu 24.04 x86_64 GNOME X11 desktop host or Linux container runtime was available during source preparation on the macOS Apple Silicon host. **No `.deb` or AppImage installer has been built or published as a qualified release.** Compilation and headless CI checks (`build.yml` on `ubuntu-24.04`) verify compilation and unit tests only; they do not establish native Linux desktop behavior.

Before advertising or publishing a public Linux release (which must use a new version/tag such as `v0.2.1` and never overwrite `v0.2.0`):

1. Build on an actual Ubuntu 24.04 LTS x86_64 host with GTK/WebKit/AppIndicator prerequisites using `npm ci` and `npm run tauri build -- --target x86_64-unknown-linux-gnu --bundles deb` (and optionally `appimage`). Verify product name, version, architecture, bundled `LICENSE`/`THIRD-PARTY-NOTICES.md`/`licenses/`/`sounds/seatbelt.wav`, and SHA-256 hashes.
2. Install the built `.deb` on a real **Ubuntu 24.04 LTS x86_64 GNOME on X11** desktop session and manually verify: OS input auto-start; 90-second idle pause; input resume; 10-minute idle reset; manual Start/Pause/Resume and completion with `autoDetect` disabled (including when idle sampling is unavailable); Wayland session unavailable banner; screen lock/unlock; suspend/resume (`CLOCK_BOOTTIME`); settings and focus-history persistence; tray menu and quit; break popup and optional chime; all 7 color themes; and opt-in cloud sync remaining dormant unless paired.
3. Record Ubuntu version, display server (X11/Wayland), architecture, executed commands, exit codes, unit-test counts, screenshots/evidence, exact source commit SHA, and final artifact SHA-256 before promoting beyond a Draft/unqualified candidate.
