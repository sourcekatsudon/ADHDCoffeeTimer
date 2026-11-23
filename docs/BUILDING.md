# Building from source

ADHD Coffee Time uses a React/TypeScript frontend and a Rust backend through Tauri 2. Released installers currently cover Windows and macOS; Linux (targeting Ubuntu 24.04 LTS x86_64 GNOME on X11) has an unqualified candidate backend in source that still requires native Linux packaging and desktop QA before public release.

These are build instructions, not a claim that every platform has been tested. See [platform status](PLATFORMS.md) for recorded verification and limitations.

## Requirements

- Git, Node.js 24, and npm (use the checked-in `package-lock.json`).
- Stable Rust through rustup, with Cargo, rustfmt, and Clippy.
- Windows: Visual Studio Build Tools with **Desktop development with C++**, a Windows SDK, and the Microsoft Edge WebView2 runtime.
- macOS: Xcode Command Line Tools. Build on a Mac; use the appropriate Rust target for the intended architecture.
- Linux (Ubuntu 24.04 LTS x86_64 candidate): build on an actual Linux x86_64 environment with Tauri 2 GTK/WebKit/AppIndicator development packages (`libwebkit2gtk-4.1-dev`, `libxdo-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `patchelf`, `build-essential`, `pkg-config`, `libssl-dev`) and runtime desktop helpers (`libxss1`, `libglib2.0-bin`, and one of `pipewire-bin`, `pulseaudio-utils`, or `alsa-utils` for optional break sound).

Install the native prerequisites for your platform before running Tauri. The frontend alone needs only Node.js and npm.

## Get the source

```sh
git clone https://github.com/sourcekatsudon/ADHDCoffeeTimer.git
cd ADHDCoffeeTimer
npm ci
```

## Run locally

```sh
npm run tauri dev
```

This starts Vite and the native desktop app. For a browser-only layout preview:

```sh
npm run dev
```

Open the local URL Vite prints. The browser preview uses sample state; it cannot validate OS activity detection, login startup, native windows, tray behavior, or native backdrop effects.

## Check changes

```sh
npm test
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --locked
cargo clippy --manifest-path src-tauri/Cargo.toml --locked -- -D warnings
```

Run Rust checks on a supported native host with its system prerequisites installed. `npm run check` is a shorter build-and-Rust-test command, but does not include all checks above. `npm run format` rewrites frontend formatting and runs `cargo fmt`.

## Package Windows

On a Windows host with the MSVC toolchain:

```sh
rustup target add x86_64-pc-windows-msvc
npm run tauri build -- --target x86_64-pc-windows-msvc --bundles nsis
```

The installer is written under `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/`.

## Package macOS

On a Mac, select the architecture you need:

```sh
rustup target add aarch64-apple-darwin
npm run tauri build -- --target aarch64-apple-darwin --bundles app,dmg
```

For Intel, replace `aarch64-apple-darwin` with `x86_64-apple-darwin` in both commands. Bundles are written under `src-tauri/target/<target>/release/bundle/`.

Local unsigned/ad-hoc builds are not Developer ID signed or notarized. They can trigger Gatekeeper restrictions. Creating a bundle is separate from validating native UI and activity behavior; native macOS UI has not been manually validated for this release.

## Package Linux (Ubuntu 24.04 LTS x86_64 candidate)

Build on an actual Ubuntu 24.04 LTS x86_64 Linux host (do not rename macOS or Windows outputs). Install prerequisites first:

```sh
sudo apt-get update
sudo apt-get install -y \
  build-essential pkg-config libssl-dev \
  libwebkit2gtk-4.1-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev patchelf \
  libxss1 libglib2.0-bin pipewire-bin
```

Install the Rust target, run the full source checks, and build the `.deb` package (and optionally `appimage`):

```sh
rustup target add x86_64-unknown-linux-gnu
npm ci
npm test
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --locked --target x86_64-unknown-linux-gnu
cargo clippy --manifest-path src-tauri/Cargo.toml --locked --target x86_64-unknown-linux-gnu -- -D warnings
npm run tauri build -- --target x86_64-unknown-linux-gnu --bundles deb
```

To build both `.deb` and AppImage where AppImage FUSE/linuxdeploy tooling is available, pass `--bundles deb,appimage`. Bundles are written under `src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/` (and `appimage/`).

Install and test the generated `.deb` on a real **Ubuntu 24.04 LTS x86_64 GNOME on X11** desktop session:

```sh
sudo apt install ./src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/*.deb
sha256sum src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/*.deb
```

Keep version metadata consistent across `package.json`, `package-lock.json`, `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, and `src-tauri/tauri.conf.json` (using a new patch version/tag such as `v0.2.1` after source changes; never attach changed-source Linux artifacts to the already published `v0.2.0` release). A passing build or headless CI run is **not** desktop qualification; see [platform status](PLATFORMS.md#linux) for the required desktop QA checklist before any public release.

## Where things live

| Path | Purpose |
| --- | --- |
| `src/main.tsx`, `src/style.css` | Timer, settings, and reminder interface |
| `src/locales.json`, `src/i18n.ts` | Ten interface languages |
| `src/themes.ts` | Seven themes |
| `src-tauri/src/engine.rs` | Timer state machine and defaults |
| `src-tauri/src/platform.rs` | Platform activity sampling and native helpers |
| `src-tauri/src/storage.rs` | Local settings and timer checkpoint persistence |
| `src-tauri/tauri.conf.json` | Product name, version, and bundle configuration |

To report a platform problem, include the target architecture, OS version, command, and relevant log excerpt. Avoid sharing personal data or signing credentials.
