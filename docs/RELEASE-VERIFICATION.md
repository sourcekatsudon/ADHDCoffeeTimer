# v0.1.0 verification

Verified on November 14, 2026 (Japan time). This records observed results and distinguishes build checks from desktop functionality.

## Source and history

- Official display name: **ADHD Coffee Time**, configured version **0.1.0**.
- [Installer source/tag](https://github.com/sourcekatsudon/ADHDCoffeeTimer/tree/v0.1.0): `a4b375b1c991279c492e057eaf6dfe72b48af868`.
- Public history was initialized independently from a copy of the current source, including the completed renaming, ten-language UI, sizing and soft chime. No previous private Git history or old installers were imported.
- Later source-check cleanup restricts an unused native radius constant to Windows/macOS; this does not change the released Windows/macOS behavior. Documentation improvements may follow the tagged application source.

## Automated checks

- Frontend: **12 tests** and TypeScript/Vite build passed locally and in CI.
- Rust: **24 tests**, rustfmt and Clippy passed. [Source checks](https://github.com/sourcekatsudon/ADHDCoffeeTimer/actions/runs/43471591984) succeeded for Windows x64, macOS Intel, macOS Apple Silicon and Ubuntu 24.04.
- [Installer workflow](https://github.com/sourcekatsudon/ADHDCoffeeTimer/actions/runs/43470717187) successfully built Windows NSIS and both macOS applications/DMGs on their native hosted runners, without private repository secrets.
- All ten README language destinations and 705 local documentation/license/image references were checked. All eight screenshot PNGs were decoded/inspected; GitHub-rendered README images loaded successfully. The seven-theme gallery uses the real UI, the same clock and the same remaining time.

## Installer and resource checks

- Windows NSIS archive integrity passed; its extracted application has an x64 PE header, product name **ADHD Coffee Time** and product version **0.1.0**.
- Both Mac app ZIPs passed CRC checks; Info.plist name/version, ARM64/x86_64 Mach-O types and executable permission bits were checked. Both DMGs passed archive-integrity checks and have valid DMG trailers.
- In each extracted application, the MIT license, third-party notices, all **930 retained attribution/license files** and original synthesized notification WAV matched the exact build commit. This comparison uses committed text bytes to account for platform line endings.
- Asset source-commit/version records were checked. `BUILD-INFO.json` records the build provenance. `SHA256SUMS.txt` covers both DMGs, both alternative app ZIPs, the Windows installer and build information.

## Windows desktop checks

The exact application extracted from the public NSIS installer was launched on the Windows desktop. Activity-triggered start, a real completed-cycle break notification, idle pause, activity-triggered resume and long-idle reset were observed. Ten languages, S/M/L native window sizes, show/hide, native movement and saved settings were checked. Restart restored the selected language, medium size, manual pause and remaining time.

The check used temporarily backed-up settings with login startup disabled; the previous settings and running development application were restored afterwards. Physical login/autostart, multiple monitors/DPI values and every suspend/lock scenario were not retested for this public build.

## Public downloads

The [public Latest release](https://github.com/sourcekatsudon/ADHDCoffeeTimer/releases/latest) is neither Draft nor Prerelease. All seven `releases/latest/download/<filename>` URLs were fetched anonymously after publication: each returned **HTTP 200 after two redirects**, its exact advertised byte size, and a SHA256 matching both the uploaded file and GitHub's asset digest. All three primary README download buttons point directly to their actual installer assets.

## Limits

- **Linux has no installer in this release.** Compilation and pure state-machine tests pass, but the idle/lock backend is unimplemented and even manual countdown cannot advance correctly. No Linux desktop-functionality claim is made. See [Linux blockers and required work](PLATFORMS.md#linux).
- Native Mac UI, activity/lock sensing, Spaces, suspend/resume and login startup have not been manually validated on Mac hardware. Mac runners establish build/test/package results only.
- Windows is unsigned. macOS uses ad-hoc signing and is not notarized. OS warnings or first-launch blocks may occur; [installation notes](PLATFORMS.md) explain this limitation.
- Screenshots are labeled browser previews; native glass effects can differ. The project makes no medical or treatment claims.
