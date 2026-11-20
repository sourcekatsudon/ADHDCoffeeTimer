# ☕ ADHD Coffee Time

**A timer for people who forget timers.**

Automatically starts when you work. Pauses when you’re away. Reminds you to rest.

| Windows | macOS | Linux |
| :---: | :---: | :---: |
| [![Windows x64 download](https://img.shields.io/badge/Windows-x64_download-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.1.0_windows-x64-setup.exe) | [![macOS Apple Silicon download](https://img.shields.io/badge/macOS-Apple_Silicon-333333?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.1.0_macos-arm64.dmg)<br>[![macOS Intel download](https://img.shields.io/badge/macOS-Intel-555555?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest/download/ADHD-Coffee-Time_0.1.0_macos-x64.dmg) | [![Linux unavailable](https://img.shields.io/badge/Linux-unavailable-999999?style=for-the-badge&logo=linux&logoColor=white)](docs/PLATFORMS.md#linux) |

[All releases](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases) · [Platform status and limitations](docs/PLATFORMS.md)

English · [日本語](docs/readme/ja.md) · [简体中文](docs/readme/zh-CN.md) · [한국어](docs/readme/ko.md) · [Español](docs/readme/es.md) · [Français](docs/readme/fr.md) · [Deutsch](docs/readme/de.md) · [Português (Brasil)](docs/readme/pt-BR.md) · [Русский](docs/readme/ru.md) · [العربية](docs/readme/ar.md)

![Seven colors of the same quiet desktop timer](docs/screenshots/themes-en.png)

*Real UI browser previews. Native glass effects vary by OS; these are not Mac hardware tests.*

## Why this exists

I'm impatient. I bought a physical Pomodoro timer, but starting it every time felt like one step too many. So I built a timer that starts itself.

## What it does

- Starts with PC activity. Pauses after **90 seconds** away; resets after **10 minutes** (adjustable).
- **5, 10, 15, 25, 30, 60, 90 minute** presets and manual pause/reset.
- Break reminders with a soft optional chime, off by default.
- **Seven pastel themes**, opacity, **S/M/L (40%/70%/100%)**, and **10 languages**.
- Tray/menu-bar residency, optional login startup, and saved preferences.
- **Focus history:** a discreet three-row activity heatmap on the timer; hover to see the last **seven days** in a compact 15-minute (0–24h) grid, and click a lit cell to clear its record.
- **Gentle completion reward:** the work-history window briefly opens with a soft glow on the completed cell whenever the timer reminds you to rest. The whole focus-history feature is **on by default** and can be turned off from Settings.

Focus history is stored locally in the existing app state file, and counts **only timer running seconds**—not idle or paused time. Deleted cells remain cleared, even after restart. Disabling the feature stops new recording and hides the indicator, popup and completion animation while restoring the original widget height.

| Timer | Settings | Break reminder |
| :---: | :---: | :---: |
| ![Timer](docs/screenshots/timer-en.png) | ![Settings](docs/screenshots/settings-en.png) | ![Break reminder](docs/screenshots/notification-en.png) |

*Browser previews; native glass and window appearance may differ.*

## Install

**Windows:** run the x64 `.exe`. It is unsigned; SmartScreen may warn.

**Mac:** choose Apple Silicon (M-series) or Intel, open the DMG, and drag the app to Applications. macOS 11+. Ad-hoc signed, **not notarized**; first launch may be blocked. [Installation help and verification limits](docs/PLATFORMS.md).

**Linux:** unavailable; activity detection is incomplete.

Use Settings to personalize the app; quit from the tray/menu bar.

## Your work stays yours

Local only: no accounts, telemetry or cloud. Reads input age, never keystroke content. Settings stay on your computer. A reminder tool, not a medical treatment.

## Build or contribute

[Build from source](docs/BUILDING.md) · [Report a bug](https://github.com/sourcekatsudon/ADHDCoffeeTimer/issues) · [Platform status](docs/PLATFORMS.md) · [MIT License](LICENSE) · [Third-party notices](THIRD-PARTY-NOTICES.md)

## Multiple-PC focus history (opt in)

The native app now includes **Settings → Cloud sync**. Existing local activity
stays local until you connect the PC. To join your Google account:

1. Open [ADHD Coffee Time Focus](https://adhd-coffee-focus.vercel.app) and sign in with Google.
2. Select **Get pairing code** and copy the 16-character code (one use; 10-minute validity).
3. On **each PC**, open **Settings → Cloud sync**, paste the code and select **Connect**.
4. The app uploads only its local activity cells, at most once per hour automatically;
   use **Sync now** for a manual refresh. In the website, the account's devices are
   merged into a seven-day 15-minute activity heatmap.

A separate pairing code is required for each computer. The web dashboard merges
per-device seconds in each 15-minute cell, capped at 900 seconds (this is an
approximation if two PCs were running simultaneously). The local 3-day / expanded
7-day display remains independent of the account-level website. Disabling focus
history on the device also pauses uploads, while disconnecting stops uploading
from that PC. Do not share pairing codes or local device tokens.

The backend requires a Google OAuth web application and a private Vercel Blob
store. See [web/README.md](web/README.md) for setup and access controls.

## Low-power timer rendering (macOS)

The timer's native engine still samples OS input age and advances remaining time
once per second, but **only a three-field `timer-tick` event is emitted at 1 Hz**.
The WebView no longer receives the full 7-day activity snapshot per second;
activity data refreshes every 60 seconds and when status/settings change.

The timer ring no longer has an infinite animation or continuously interpolated
SVG blur. The Mac timer capsule uses its existing static translucent gradients
rather than a live 28px backdrop blur, avoiding repeated WKWebView compositing.

Calendar/era formatting is cached for the current date, the native clock shares
the timer's 1Hz update, and the 3-day heatmap uses React.memo to avoid remounting
144 decorative cells every second. The engine prunes activity data about once a
minute and checkpoints a running timer every 2 minutes (also on transitions and
completion). Cloud sync starts no background timer on an unpaired PC, and paired
PCs upload on a 30-minute cadence.

**Performance target:** reduce M2's observed ~6% CPU to <=0.6% during a stable
visible countdown. This is a target; verify with Activity Monitor on the actual
M2 Mac (same desktop background and monitor). Automated CI tests validate
functionality and platform builds but cannot assert a CPU percentage.
