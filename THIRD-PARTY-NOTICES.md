# Third-party notices

Audit date: 2026-11-14. This project's MIT license applies to its own work. Dependencies keep their own licenses; the tables below reproduce package metadata, not a relicensing of those packages. Complete available upstream license and notice files are retained under `licenses/`. When a crate archive omitted its license text, the exact upstream Git commit recorded in `.cargo_vcs_info.json` was used where available; `UPSTREAM-SOURCE.txt` records those URLs. The generic MPL text for selectors is also retained.

## Scope and redistribution

- The inventory follows the exact `package-lock.json` and `src-tauri/Cargo.lock` versions. Cargo metadata was resolved with `--locked` for Windows x64 and both macOS architectures; this is a dependency audit, not a functional platform certification. The complete Cargo lock graph is also listed, including unsupported targets.
- Preserve the applicable copyright, permission, license and NOTICE files when redistributing dependency code or binaries. MIT, BSD, ISC, Zlib and Apache conditions remain applicable. For dual-license alternatives, follow an offered license; `AND` requires both. Legacy slash expressions below are preserved exactly as published, not rewritten.
- MPL-2.0 is file-level copyleft. Keep covered files under MPL and make the corresponding covered source, including any modifications, available to recipients of executable distributions. The source links in the Cargo table point to the exact published crate source archives; no third-party crate source modifications are included in this repository. The MIT license for this application does not replace MPL obligations.
- Unicode-3.0 notices apply to ICU data/code and unicode-ident alongside any other stated license. Retain those notices.
- `caniuse-lite` is build-time browser data licensed CC-BY-4.0. Its upstream attribution and license text are preserved in `licenses/npm/caniuse-lite/`; see [caniuse-lite](https://github.com/browserslist/caniuse-lite) and [Can I use](https://caniuse.com/). It is not a declared application runtime dependency.
- Native operating-system libraries, WebView runtimes and installer bootstrappers are not fully described by Rust/npm package metadata. Before distributing a new bundle format, check the actual bundled components and their notices. A Linux package would additionally require an audit of its native GTK/WebKit stack; no Linux artifact is released.

## Project assets

- `src-tauri/sounds/seatbelt.wav` is a synthesized single-note tone, byte-identical to the adopted generated candidate (SHA-256 `83305da82a9013b0ef0baf48f4a4252a4723f9bbec3109c3dc248ce8c563fd90`). The inspected generator uses Python standard-library sine synthesis, with no sampled recording or external sound library. Its filename does not identify a licensed recording.
- The coffee-cup/ring icon and tray artwork are project assets attributed to sourcekatsudon. No third-party asset attribution was found in the inspected source. The original icon binaries have no separate embedded provenance record or retained drawing generator; visual inspection alone cannot prove exclusive originality.
- No bundled webfont files or externally hosted font imports were found in the application source. System font availability is platform-specific.

## npm packages

Runtime dependencies are `@tauri-apps/api`, `react`, `react-dom`, and transitive `scheduler`. Other entries below are development/build/test tools or optional platform binaries. Uninstalled optional platform variants have metadata inventory only; the installed upstream package notices cover the inspected build host. Their licenses must also be retained if those optional packages themselves are redistributed.

| Package | Version | Declared license | Scope | License files |
| --- | --- | --- | --- | --- |
| `@babel/code-frame` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/code-frame/) |
| `@babel/compat-data` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/compat-data/) |
| `@babel/core` | 7.29.13 | MIT | dev/optional | [files](licenses/npm/@babel/core/) |
| `@babel/generator` | 7.29.14 | MIT | dev/optional | [files](licenses/npm/@babel/generator/) |
| `@babel/helper-compilation-targets` | 7.29.13 | MIT | dev/optional | [files](licenses/npm/@babel/helper-compilation-targets/) |
| `@babel/helper-globals` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/helper-globals/) |
| `@babel/helper-module-imports` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/helper-module-imports/) |
| `@babel/helper-module-transforms` | 7.29.14 | MIT | dev/optional | [files](licenses/npm/@babel/helper-module-transforms/) |
| `@babel/helper-plugin-utils` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/helper-plugin-utils/) |
| `@babel/helper-string-parser` | 7.29.14 | MIT | dev/optional | [files](licenses/npm/@babel/helper-string-parser/) |
| `@babel/helper-validator-identifier` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/helper-validator-identifier/) |
| `@babel/helper-validator-option` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/helper-validator-option/) |
| `@babel/helpers` | 7.29.14 | MIT | dev/optional | [files](licenses/npm/@babel/helpers/) |
| `@babel/parser` | 7.29.14 | MIT | dev/optional | [files](licenses/npm/@babel/parser/) |
| `@babel/plugin-transform-react-jsx-self` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/plugin-transform-react-jsx-self/) |
| `@babel/plugin-transform-react-jsx-source` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/plugin-transform-react-jsx-source/) |
| `@babel/template` | 7.29.7 | MIT | dev/optional | [files](licenses/npm/@babel/template/) |
| `@babel/traverse` | 7.29.14 | MIT | dev/optional | [files](licenses/npm/@babel/traverse/) |
| `@babel/types` | 7.29.14 | MIT | dev/optional | [files](licenses/npm/@babel/types/) |
| `@esbuild/aix-ppc64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/android-arm` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/android-arm64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/android-x64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/darwin-arm64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/darwin-x64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/freebsd-arm64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/freebsd-x64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-arm` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-arm64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-ia32` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-loong64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-mips64el` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-ppc64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-riscv64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-s390x` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/linux-x64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/netbsd-arm64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/netbsd-x64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/openbsd-arm64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/openbsd-x64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/openharmony-arm64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/sunos-x64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/win32-arm64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/win32-ia32` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@esbuild/win32-x64` | 0.28.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@jridgewell/gen-mapping` | 0.3.13 | MIT | dev/optional | [files](licenses/npm/@jridgewell/gen-mapping/) |
| `@jridgewell/remapping` | 2.3.5 | MIT | dev/optional | [files](licenses/npm/@jridgewell/remapping/) |
| `@jridgewell/resolve-uri` | 3.1.2 | MIT | dev/optional | [files](licenses/npm/@jridgewell/resolve-uri/) |
| `@jridgewell/sourcemap-codec` | 1.6.0 | MIT | dev/optional | [files](licenses/npm/@jridgewell/sourcemap-codec/) |
| `@jridgewell/trace-mapping` | 0.3.31 | MIT | dev/optional | [files](licenses/npm/@jridgewell/trace-mapping/) |
| `@rolldown/pluginutils` | 1.0.0-rc.3 | MIT | dev/optional | [files](licenses/npm/@rolldown/pluginutils/) |
| `@rollup/rollup-android-arm-eabi` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-android-arm64` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-android-x64` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-darwin-arm64` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-darwin-x64` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-freebsd-arm64` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-freebsd-x64` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-arm-gnueabihf` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-arm-musleabihf` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-arm64-gnu` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-arm64-musl` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-loong64-gnu` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-loong64-musl` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-ppc64-gnu` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-ppc64-musl` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-riscv64-gnu` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-riscv64-musl` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-s390x-gnu` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-x64-gnu` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-linux-x64-musl` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-openbsd-x64` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-openharmony-arm64` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-win32-arm64-msvc` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-win32-ia32-msvc` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-win32-x64-gnu` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@rollup/rollup-win32-x64-msvc` | 4.68.2 | MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/api` | 2.12.4 | Apache-2.0 OR MIT | runtime | [files](licenses/npm/@tauri-apps/api/) |
| `@tauri-apps/cli` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | [files](licenses/npm/@tauri-apps/cli/) |
| `@tauri-apps/cli-darwin-arm64` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-darwin-x64` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-linux-arm-gnueabihf` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-linux-arm64-gnu` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-linux-arm64-musl` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-linux-riscv64-gnu` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-linux-x64-gnu` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-linux-x64-musl` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-win32-arm64-msvc` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-win32-ia32-msvc` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@tauri-apps/cli-win32-x64-msvc` | 2.12.4 | Apache-2.0 OR MIT | dev/optional | Not installed / no top-level text found |
| `@types/babel__core` | 7.20.5 | MIT | dev/optional | [files](licenses/npm/@types/babel__core/) |
| `@types/babel__generator` | 7.27.0 | MIT | dev/optional | [files](licenses/npm/@types/babel__generator/) |
| `@types/babel__template` | 7.4.4 | MIT | dev/optional | [files](licenses/npm/@types/babel__template/) |
| `@types/babel__traverse` | 7.28.0 | MIT | dev/optional | [files](licenses/npm/@types/babel__traverse/) |
| `@types/chai` | 5.2.3 | MIT | dev/optional | [files](licenses/npm/@types/chai/) |
| `@types/deep-eql` | 4.0.2 | MIT | dev/optional | [files](licenses/npm/@types/deep-eql/) |
| `@types/estree` | 1.0.10 | MIT | dev/optional | [files](licenses/npm/@types/estree/) |
| `@types/react` | 19.3.0 | MIT | dev/optional | [files](licenses/npm/@types/react/) |
| `@types/react-dom` | 19.3.0 | MIT | dev/optional | [files](licenses/npm/@types/react-dom/) |
| `@vitejs/plugin-react` | 5.2.0 | MIT | dev/optional | [files](licenses/npm/@vitejs/plugin-react/) |
| `@vitest/mocker` | 5.1.0 | MIT | dev/optional | [files](licenses/npm/@vitest/mocker/) |
| `@vitest/spy` | 5.1.0 | MIT | dev/optional | [files](licenses/npm/@vitest/spy/) |
| `assertion-error` | 2.0.1 | MIT | dev/optional | [files](licenses/npm/assertion-error/) |
| `baseline-browser-mapping` | 2.11.36 | Apache-2.0 | dev/optional | [files](licenses/npm/baseline-browser-mapping/) |
| `browserslist` | 4.29.5 | MIT | dev/optional | [files](licenses/npm/browserslist/) |
| `caniuse-lite` | 1.0.30001826 | CC-BY-4.0 | dev/optional | [files](licenses/npm/caniuse-lite/) |
| `chai` | 6.3.0 | MIT | dev/optional | [files](licenses/npm/chai/) |
| `convert-source-map` | 2.0.0 | MIT | dev/optional | [files](licenses/npm/convert-source-map/) |
| `csstype` | 3.2.3 | MIT | dev/optional | [files](licenses/npm/csstype/) |
| `debug` | 4.4.3 | MIT | dev/optional | [files](licenses/npm/debug/) |
| `electron-to-chromium` | 1.5.483 | ISC | dev/optional | [files](licenses/npm/electron-to-chromium/) |
| `es-module-lexer` | 2.3.2 | MIT | dev/optional | [files](licenses/npm/es-module-lexer/) |
| `esbuild` | 0.28.2 | MIT | dev/optional | [files](licenses/npm/esbuild/) |
| `escalade` | 3.2.0 | MIT | dev/optional | [files](licenses/npm/escalade/) |
| `estree-walker` | 3.0.3 | MIT | dev/optional | [files](licenses/npm/estree-walker/) |
| `expect-type` | 1.4.0 | Apache-2.0 | dev/optional | [files](licenses/npm/expect-type/) |
| `fdir` | 6.5.0 | MIT | dev/optional | [files](licenses/npm/fdir/) |
| `fsevents` | 2.3.3 | MIT | dev/optional | Not installed / no top-level text found |
| `gensync` | 1.0.0-beta.2 | MIT | dev/optional | [files](licenses/npm/gensync/) |
| `js-tokens` | 4.0.0 | MIT | dev/optional | [files](licenses/npm/js-tokens/) |
| `jsesc` | 3.1.0 | MIT | dev/optional | [files](licenses/npm/jsesc/) |
| `json5` | 2.2.3 | MIT | dev/optional | [files](licenses/npm/json5/) |
| `lru-cache` | 5.1.1 | ISC | dev/optional | [files](licenses/npm/lru-cache/) |
| `magic-string` | 1.4.5 | MIT | dev/optional | [files](licenses/npm/magic-string/) |
| `ms` | 2.1.3 | MIT | dev/optional | [files](licenses/npm/ms/) |
| `nanoid` | 3.3.20 | MIT | dev/optional | [files](licenses/npm/nanoid/) |
| `node-releases` | 2.0.63 | MIT | dev/optional | [files](licenses/npm/node-releases/) |
| `obug` | 3.0.0 | MIT | dev/optional | [files](licenses/npm/obug/) |
| `picocolors` | 1.1.1 | ISC | dev/optional | [files](licenses/npm/picocolors/) |
| `picomatch` | 4.0.7 | MIT | dev/optional | [files](licenses/npm/picomatch/) |
| `postcss` | 8.5.32 | MIT | dev/optional | [files](licenses/npm/postcss/) |
| `prettier` | 3.10.1 | MIT | dev/optional | [files](licenses/npm/prettier/) |
| `react` | 19.3.1 | MIT | runtime | [files](licenses/npm/react/) |
| `react-dom` | 19.3.1 | MIT | runtime | [files](licenses/npm/react-dom/) |
| `react-refresh` | 0.18.0 | MIT | dev/optional | [files](licenses/npm/react-refresh/) |
| `rollup` | 4.68.2 | MIT | dev/optional | [files](licenses/npm/rollup/) |
| `scheduler` | 0.28.0 | MIT | runtime | [files](licenses/npm/scheduler/) |
| `semver` | 6.3.2 | ISC | dev/optional | [files](licenses/npm/semver/) |
| `source-map-js` | 1.2.2 | BSD-3-Clause | dev/optional | [files](licenses/npm/source-map-js/) |
| `std-env` | 4.3.0 | MIT | dev/optional | [files](licenses/npm/std-env/) |
| `tinybench` | 6.2.3 | MIT | dev/optional | [files](licenses/npm/tinybench/) |
| `tinyexec` | 1.3.1 | MIT | dev/optional | [files](licenses/npm/tinyexec/) |
| `tinyglobby` | 0.2.17 | MIT | dev/optional | [files](licenses/npm/tinyglobby/) |
| `typescript` | 5.9.3 | Apache-2.0 | dev/optional | [files](licenses/npm/typescript/) |
| `update-browserslist-db` | 1.3.4 | MIT | dev/optional | [files](licenses/npm/update-browserslist-db/) |
| `vite` | 7.3.8 | MIT | dev/optional | [files](licenses/npm/vite/) |
| `vitest` | 5.1.0 | MIT | dev/optional | [files](licenses/npm/vitest/) |
| `why-is-node-running` | 3.2.1 | MIT | dev/optional | [files](licenses/npm/why-is-node-running/) |
| `yallist` | 3.1.1 | ISC | dev/optional | [files](licenses/npm/yallist/) |

## Cargo packages

Target labels describe Cargo's resolved dependency graph, which includes build dependencies and procedural macros; a graph entry does not prove that all package code is linked into the final executable. `Other` means outside the three intended binary target graphs.

| Crate | Version | Declared license | Target graph | License files / corresponding source |
| --- | --- | --- | --- | --- |
| `adler2` | 2.0.1 | 0BSD OR MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/adler2-2.0.1/); [source](https://crates.io/api/v1/crates/adler2/2.0.1/download) |
| `aho-corasick` | 1.1.5 | Unlicense OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/aho-corasick-1.1.5/); [source](https://crates.io/api/v1/crates/aho-corasick/1.1.5/download) |
| `alloc-no-stdlib` | 3.0.0 | BSD-3-Clause | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/alloc-no-stdlib-3.0.0/); [source](https://crates.io/api/v1/crates/alloc-no-stdlib/3.0.0/download) |
| `alloc-stdlib` | 0.3.0 | BSD-3-Clause | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/alloc-stdlib-0.3.0/); [source](https://crates.io/api/v1/crates/alloc-stdlib/0.3.0/download) |
| `android_system_properties` | 0.1.6 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/android_system_properties-0.1.6/); [source](https://crates.io/api/v1/crates/android_system_properties/0.1.6/download) |
| `anyhow` | 1.0.104 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/anyhow-1.0.104/); [source](https://crates.io/api/v1/crates/anyhow/1.0.104/download) |
| `async-broadcast` | 0.7.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/async-broadcast-0.7.2/); [source](https://crates.io/api/v1/crates/async-broadcast/0.7.2/download) |
| `async-channel` | 2.5.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/async-channel-2.5.0/); [source](https://crates.io/api/v1/crates/async-channel/2.5.0/download) |
| `async-executor` | 1.14.1 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/async-executor-1.14.1/); [source](https://crates.io/api/v1/crates/async-executor/1.14.1/download) |
| `async-io` | 2.6.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/async-io-2.6.0/); [source](https://crates.io/api/v1/crates/async-io/2.6.0/download) |
| `async-lock` | 3.5.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/async-lock-3.5.0/); [source](https://crates.io/api/v1/crates/async-lock/3.5.0/download) |
| `async-process` | 2.5.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/async-process-2.5.0/); [source](https://crates.io/api/v1/crates/async-process/2.5.0/download) |
| `async-recursion` | 1.2.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/async-recursion-1.2.0/); [source](https://crates.io/api/v1/crates/async-recursion/1.2.0/download) |
| `async-signal` | 0.2.14 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/async-signal-0.2.14/); [source](https://crates.io/api/v1/crates/async-signal/0.2.14/download) |
| `async-task` | 4.7.1 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/async-task-4.7.1/); [source](https://crates.io/api/v1/crates/async-task/4.7.1/download) |
| `async-trait` | 0.1.92 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/async-trait-0.1.92/); [source](https://crates.io/api/v1/crates/async-trait/0.1.92/download) |
| `atk` | 0.18.2 | MIT | Other | [files](licenses/cargo/atk-0.18.2/); [source](https://crates.io/api/v1/crates/atk/0.18.2/download) |
| `atk-sys` | 0.18.2 | MIT | Other | [files](licenses/cargo/atk-sys-0.18.2/); [source](https://crates.io/api/v1/crates/atk-sys/0.18.2/download) |
| `atomic-waker` | 1.1.2 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/atomic-waker-1.1.2/); [source](https://crates.io/api/v1/crates/atomic-waker/1.1.2/download) |
| `auto-launch` | 0.6.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/auto-launch-0.6.0/); [source](https://crates.io/api/v1/crates/auto-launch/0.6.0/download) |
| `autocfg` | 1.5.1 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/autocfg-1.5.1/); [source](https://crates.io/api/v1/crates/autocfg/1.5.1/download) |
| `base64` | 0.21.7 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/base64-0.21.7/); [source](https://crates.io/api/v1/crates/base64/0.21.7/download) |
| `base64` | 0.23.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/base64-0.23.1/); [source](https://crates.io/api/v1/crates/base64/0.23.1/download) |
| `bit-set` | 0.11.1 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/bit-set-0.11.1/); [source](https://crates.io/api/v1/crates/bit-set/0.11.1/download) |
| `bit-set` | 0.8.0 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/bit-set-0.8.0/); [source](https://crates.io/api/v1/crates/bit-set/0.8.0/download) |
| `bit-vec` | 0.10.2 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/bit-vec-0.10.2/); [source](https://crates.io/api/v1/crates/bit-vec/0.10.2/download) |
| `bit-vec` | 0.8.0 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/bit-vec-0.8.0/); [source](https://crates.io/api/v1/crates/bit-vec/0.8.0/download) |
| `bitflags` | 1.3.2 | MIT/Apache-2.0 | Other | [files](licenses/cargo/bitflags-1.3.2/); [source](https://crates.io/api/v1/crates/bitflags/1.3.2/download) |
| `bitflags` | 2.13.2 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/bitflags-2.13.2/); [source](https://crates.io/api/v1/crates/bitflags/2.13.2/download) |
| `block-buffer` | 0.10.4 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/block-buffer-0.10.4/); [source](https://crates.io/api/v1/crates/block-buffer/0.10.4/download) |
| `block2` | 0.6.2 | MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/block2-0.6.2/); [source](https://crates.io/api/v1/crates/block2/0.6.2/download) |
| `blocking` | 1.7.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/blocking-1.7.0/); [source](https://crates.io/api/v1/crates/blocking/1.7.0/download) |
| `brotli` | 9.1.1 | BSD-3-Clause AND MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/brotli-9.1.1/); [source](https://crates.io/api/v1/crates/brotli/9.1.1/download) |
| `brotli-decompressor` | 6.3.1 | BSD-3-Clause/MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/brotli-decompressor-6.3.1/); [source](https://crates.io/api/v1/crates/brotli-decompressor/6.3.1/download) |
| `bs58` | 0.5.1 | MIT/Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/bs58-0.5.1/); [source](https://crates.io/api/v1/crates/bs58/0.5.1/download) |
| `bumpalo` | 3.20.3 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/bumpalo-3.20.3/); [source](https://crates.io/api/v1/crates/bumpalo/3.20.3/download) |
| `bytemuck` | 1.25.2 | Zlib OR Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/bytemuck-1.25.2/); [source](https://crates.io/api/v1/crates/bytemuck/1.25.2/download) |
| `byteorder` | 1.5.0 | Unlicense OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/byteorder-1.5.0/); [source](https://crates.io/api/v1/crates/byteorder/1.5.0/download) |
| `byteorder-lite` | 0.1.0 | Unlicense OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/byteorder-lite-0.1.0/); [source](https://crates.io/api/v1/crates/byteorder-lite/0.1.0/download) |
| `bytes` | 1.12.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/bytes-1.12.1/); [source](https://crates.io/api/v1/crates/bytes/1.12.1/download) |
| `cairo-rs` | 0.18.5 | MIT | Other | [files](licenses/cargo/cairo-rs-0.18.5/); [source](https://crates.io/api/v1/crates/cairo-rs/0.18.5/download) |
| `cairo-sys-rs` | 0.18.2 | MIT | Other | [files](licenses/cargo/cairo-sys-rs-0.18.2/); [source](https://crates.io/api/v1/crates/cairo-sys-rs/0.18.2/download) |
| `camino` | 1.2.6 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/camino-1.2.6/); [source](https://crates.io/api/v1/crates/camino/1.2.6/download) |
| `cargo-platform` | 0.1.9 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cargo-platform-0.1.9/); [source](https://crates.io/api/v1/crates/cargo-platform/0.1.9/download) |
| `cargo_metadata` | 0.19.2 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cargo_metadata-0.19.2/); [source](https://crates.io/api/v1/crates/cargo_metadata/0.19.2/download) |
| `cargo_toml` | 1.0.1 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cargo_toml-1.0.1/); [source](https://crates.io/api/v1/crates/cargo_toml/1.0.1/download) |
| `cc` | 1.11.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cc-1.11.1/); [source](https://crates.io/api/v1/crates/cc/1.11.1/download) |
| `cesu8` | 1.1.0 | Apache-2.0/MIT | Other | [files](licenses/cargo/cesu8-1.1.0/); [source](https://crates.io/api/v1/crates/cesu8/1.1.0/download) |
| `cfb` | 0.14.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cfb-0.14.0/); [source](https://crates.io/api/v1/crates/cfb/0.14.0/download) |
| `cfg-expr` | 0.15.8 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/cfg-expr-0.15.8/); [source](https://crates.io/api/v1/crates/cfg-expr/0.15.8/download) |
| `cfg-if` | 1.0.5 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cfg-if-1.0.5/); [source](https://crates.io/api/v1/crates/cfg-if/1.0.5/download) |
| `cfg_aliases` | 0.2.2 | MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/cfg_aliases-0.2.2/); [source](https://crates.io/api/v1/crates/cfg_aliases/0.2.2/download) |
| `chrono` | 0.4.45 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/chrono-0.4.45/); [source](https://crates.io/api/v1/crates/chrono/0.4.45/download) |
| `combine` | 4.6.9 | MIT | Other | [files](licenses/cargo/combine-4.6.9/); [source](https://crates.io/api/v1/crates/combine/4.6.9/download) |
| `concurrent-queue` | 2.5.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/concurrent-queue-2.5.0/); [source](https://crates.io/api/v1/crates/concurrent-queue/2.5.0/download) |
| `cookie` | 0.18.2 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cookie-0.18.2/); [source](https://crates.io/api/v1/crates/cookie/0.18.2/download) |
| `core-foundation` | 0.10.1 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/core-foundation-0.10.1/); [source](https://crates.io/api/v1/crates/core-foundation/0.10.1/download) |
| `core-foundation-sys` | 0.8.7 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/core-foundation-sys-0.8.7/); [source](https://crates.io/api/v1/crates/core-foundation-sys/0.8.7/download) |
| `core-graphics` | 0.25.0 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/core-graphics-0.25.0/); [source](https://crates.io/api/v1/crates/core-graphics/0.25.0/download) |
| `core-graphics-types` | 0.2.0 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/core-graphics-types-0.2.0/); [source](https://crates.io/api/v1/crates/core-graphics-types/0.2.0/download) |
| `cpufeatures` | 0.2.17 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cpufeatures-0.2.17/); [source](https://crates.io/api/v1/crates/cpufeatures/0.2.17/download) |
| `crc32fast` | 1.5.5 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/crc32fast-1.5.5/); [source](https://crates.io/api/v1/crates/crc32fast/1.5.5/download) |
| `crossbeam-channel` | 0.5.17 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/crossbeam-channel-0.5.17/); [source](https://crates.io/api/v1/crates/crossbeam-channel/0.5.17/download) |
| `crossbeam-utils` | 0.8.23 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/crossbeam-utils-0.8.23/); [source](https://crates.io/api/v1/crates/crossbeam-utils/0.8.23/download) |
| `crypto-common` | 0.1.7 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/crypto-common-0.1.7/); [source](https://crates.io/api/v1/crates/crypto-common/0.1.7/download) |
| `cssparser` | 0.37.0 | MPL-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cssparser-0.37.0/); [source](https://crates.io/api/v1/crates/cssparser/0.37.0/download) |
| `cssparser` | 0.38.0 | MPL-2.0 | Other | [files](licenses/cargo/cssparser-0.38.0/); [source](https://crates.io/api/v1/crates/cssparser/0.38.0/download) |
| `cssparser-macros` | 0.7.1 | MPL-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/cssparser-macros-0.7.1/); [source](https://crates.io/api/v1/crates/cssparser-macros/0.7.1/download) |
| `ctor` | 1.0.13 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/ctor-1.0.13/); [source](https://crates.io/api/v1/crates/ctor/1.0.13/download) |
| `darling` | 0.24.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/darling-0.24.1/); [source](https://crates.io/api/v1/crates/darling/0.24.1/download) |
| `darling_core` | 0.24.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/darling_core-0.24.1/); [source](https://crates.io/api/v1/crates/darling_core/0.24.1/download) |
| `darling_macro` | 0.24.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/darling_macro-0.24.1/); [source](https://crates.io/api/v1/crates/darling_macro/0.24.1/download) |
| `dbus` | 0.9.12 | Apache-2.0/MIT | Other | [files](licenses/cargo/dbus-0.9.12/); [source](https://crates.io/api/v1/crates/dbus/0.9.12/download) |
| `deranged` | 0.5.9 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/deranged-0.5.9/); [source](https://crates.io/api/v1/crates/deranged/0.5.9/download) |
| `derive_more` | 2.1.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/derive_more-2.1.1/); [source](https://crates.io/api/v1/crates/derive_more/2.1.1/download) |
| `derive_more-impl` | 2.1.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/derive_more-impl-2.1.1/); [source](https://crates.io/api/v1/crates/derive_more-impl/2.1.1/download) |
| `digest` | 0.10.7 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/digest-0.10.7/); [source](https://crates.io/api/v1/crates/digest/0.10.7/download) |
| `dirs` | 6.0.0 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/dirs-6.0.0/); [source](https://crates.io/api/v1/crates/dirs/6.0.0/download) |
| `dirs` | 7.0.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/dirs-7.0.0/); [source](https://crates.io/api/v1/crates/dirs/7.0.0/download) |
| `dirs-sys` | 0.5.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/dirs-sys-0.5.0/); [source](https://crates.io/api/v1/crates/dirs-sys/0.5.0/download) |
| `dispatch2` | 0.3.1 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/dispatch2-0.3.1/); [source](https://crates.io/api/v1/crates/dispatch2/0.3.1/download) |
| `displaydoc` | 0.2.7 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/displaydoc-0.2.7/); [source](https://crates.io/api/v1/crates/displaydoc/0.2.7/download) |
| `dlopen2` | 0.8.2 | MIT | Other | [files](licenses/cargo/dlopen2-0.8.2/); [source](https://crates.io/api/v1/crates/dlopen2/0.8.2/download) |
| `dlopen2_derive` | 0.4.3 | MIT | Other | [files](licenses/cargo/dlopen2_derive-0.4.3/); [source](https://crates.io/api/v1/crates/dlopen2_derive/0.4.3/download) |
| `dom_query` | 0.28.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/dom_query-0.28.0/); [source](https://crates.io/api/v1/crates/dom_query/0.28.0/download) |
| `dom_query` | 0.29.0 | MIT | Other | [files](licenses/cargo/dom_query-0.29.0/); [source](https://crates.io/api/v1/crates/dom_query/0.29.0/download) |
| `dpi` | 0.1.2 | Apache-2.0 AND MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/dpi-0.1.2/); [source](https://crates.io/api/v1/crates/dpi/0.1.2/download) |
| `dtoa` | 1.0.11 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/dtoa-1.0.11/); [source](https://crates.io/api/v1/crates/dtoa/1.0.11/download) |
| `dtoa-short` | 0.3.5 | MPL-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/dtoa-short-0.3.5/); [source](https://crates.io/api/v1/crates/dtoa-short/0.3.5/download) |
| `dunce` | 1.0.5 | CC0-1.0 OR MIT-0 OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/dunce-1.0.5/); [source](https://crates.io/api/v1/crates/dunce/1.0.5/download) |
| `dyn-clone` | 1.0.20 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/dyn-clone-1.0.20/); [source](https://crates.io/api/v1/crates/dyn-clone/1.0.20/download) |
| `embed-resource` | 3.0.12 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/embed-resource-3.0.12/); [source](https://crates.io/api/v1/crates/embed-resource/3.0.12/download) |
| `embed_plist` | 1.2.2 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/embed_plist-1.2.2/); [source](https://crates.io/api/v1/crates/embed_plist/1.2.2/download) |
| `endi` | 1.1.1 | MIT | Other | [files](licenses/cargo/endi-1.1.1/); [source](https://crates.io/api/v1/crates/endi/1.1.1/download) |
| `enumflags2` | 0.7.12 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/enumflags2-0.7.12/); [source](https://crates.io/api/v1/crates/enumflags2/0.7.12/download) |
| `enumflags2_derive` | 0.7.12 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/enumflags2_derive-0.7.12/); [source](https://crates.io/api/v1/crates/enumflags2_derive/0.7.12/download) |
| `equivalent` | 1.0.3 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/equivalent-1.0.3/); [source](https://crates.io/api/v1/crates/equivalent/1.0.3/download) |
| `erased-serde` | 0.4.11 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/erased-serde-0.4.11/); [source](https://crates.io/api/v1/crates/erased-serde/0.4.11/download) |
| `errno` | 0.3.15 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/errno-0.3.15/); [source](https://crates.io/api/v1/crates/errno/0.3.15/download) |
| `event-listener` | 5.4.2 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/event-listener-5.4.2/); [source](https://crates.io/api/v1/crates/event-listener/5.4.2/download) |
| `event-listener-strategy` | 0.5.4 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/event-listener-strategy-0.5.4/); [source](https://crates.io/api/v1/crates/event-listener-strategy/0.5.4/download) |
| `fastrand` | 2.5.0 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/fastrand-2.5.0/); [source](https://crates.io/api/v1/crates/fastrand/2.5.0/download) |
| `fdeflate` | 0.3.7 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/fdeflate-0.3.7/); [source](https://crates.io/api/v1/crates/fdeflate/0.3.7/download) |
| `field-offset` | 0.3.6 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/field-offset-0.3.6/); [source](https://crates.io/api/v1/crates/field-offset/0.3.6/download) |
| `find-msvc-tools` | 0.1.15 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/find-msvc-tools-0.1.15/); [source](https://crates.io/api/v1/crates/find-msvc-tools/0.1.15/download) |
| `flate2` | 1.1.11 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/flate2-1.1.11/); [source](https://crates.io/api/v1/crates/flate2/1.1.11/download) |
| `fnv` | 1.0.7 | Apache-2.0 / MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/fnv-1.0.7/); [source](https://crates.io/api/v1/crates/fnv/1.0.7/download) |
| `foldhash` | 0.2.0 | Zlib | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/foldhash-0.2.0/); [source](https://crates.io/api/v1/crates/foldhash/0.2.0/download) |
| `foreign-types` | 0.5.0 | MIT/Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/foreign-types-0.5.0/); [source](https://crates.io/api/v1/crates/foreign-types/0.5.0/download) |
| `foreign-types-macros` | 0.2.4 | MIT/Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/foreign-types-macros-0.2.4/); [source](https://crates.io/api/v1/crates/foreign-types-macros/0.2.4/download) |
| `foreign-types-shared` | 0.3.1 | MIT/Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/foreign-types-shared-0.3.1/); [source](https://crates.io/api/v1/crates/foreign-types-shared/0.3.1/download) |
| `form_urlencoded` | 1.2.2 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/form_urlencoded-1.2.2/); [source](https://crates.io/api/v1/crates/form_urlencoded/1.2.2/download) |
| `futures-channel` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/futures-channel-0.3.34/); [source](https://crates.io/api/v1/crates/futures-channel/0.3.34/download) |
| `futures-core` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/futures-core-0.3.34/); [source](https://crates.io/api/v1/crates/futures-core/0.3.34/download) |
| `futures-executor` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/futures-executor-0.3.34/); [source](https://crates.io/api/v1/crates/futures-executor/0.3.34/download) |
| `futures-io` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/futures-io-0.3.34/); [source](https://crates.io/api/v1/crates/futures-io/0.3.34/download) |
| `futures-lite` | 2.6.1 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/futures-lite-2.6.1/); [source](https://crates.io/api/v1/crates/futures-lite/2.6.1/download) |
| `futures-macro` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/futures-macro-0.3.34/); [source](https://crates.io/api/v1/crates/futures-macro/0.3.34/download) |
| `futures-sink` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/futures-sink-0.3.34/); [source](https://crates.io/api/v1/crates/futures-sink/0.3.34/download) |
| `futures-task` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/futures-task-0.3.34/); [source](https://crates.io/api/v1/crates/futures-task/0.3.34/download) |
| `futures-util` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/futures-util-0.3.34/); [source](https://crates.io/api/v1/crates/futures-util/0.3.34/download) |
| `gdk` | 0.18.2 | MIT | Other | [files](licenses/cargo/gdk-0.18.2/); [source](https://crates.io/api/v1/crates/gdk/0.18.2/download) |
| `gdk-pixbuf` | 0.18.5 | MIT | Other | [files](licenses/cargo/gdk-pixbuf-0.18.5/); [source](https://crates.io/api/v1/crates/gdk-pixbuf/0.18.5/download) |
| `gdk-pixbuf-sys` | 0.18.0 | MIT | Other | [files](licenses/cargo/gdk-pixbuf-sys-0.18.0/); [source](https://crates.io/api/v1/crates/gdk-pixbuf-sys/0.18.0/download) |
| `gdk-sys` | 0.18.2 | MIT | Other | [files](licenses/cargo/gdk-sys-0.18.2/); [source](https://crates.io/api/v1/crates/gdk-sys/0.18.2/download) |
| `gdkwayland-sys` | 0.18.2 | MIT | Other | [files](licenses/cargo/gdkwayland-sys-0.18.2/); [source](https://crates.io/api/v1/crates/gdkwayland-sys/0.18.2/download) |
| `gdkx11` | 0.18.2 | MIT | Other | [files](licenses/cargo/gdkx11-0.18.2/); [source](https://crates.io/api/v1/crates/gdkx11/0.18.2/download) |
| `gdkx11-sys` | 0.18.2 | MIT | Other | [files](licenses/cargo/gdkx11-sys-0.18.2/); [source](https://crates.io/api/v1/crates/gdkx11-sys/0.18.2/download) |
| `generic-array` | 0.14.7 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/generic-array-0.14.7/); [source](https://crates.io/api/v1/crates/generic-array/0.14.7/download) |
| `getrandom` | 0.3.4 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/getrandom-0.3.4/); [source](https://crates.io/api/v1/crates/getrandom/0.3.4/download) |
| `getrandom` | 0.4.3 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/getrandom-0.4.3/); [source](https://crates.io/api/v1/crates/getrandom/0.4.3/download) |
| `gio` | 0.18.4 | MIT | Other | [files](licenses/cargo/gio-0.18.4/); [source](https://crates.io/api/v1/crates/gio/0.18.4/download) |
| `gio-sys` | 0.18.1 | MIT | Other | [files](licenses/cargo/gio-sys-0.18.1/); [source](https://crates.io/api/v1/crates/gio-sys/0.18.1/download) |
| `glib` | 0.18.5 | MIT | Other | [files](licenses/cargo/glib-0.18.5/); [source](https://crates.io/api/v1/crates/glib/0.18.5/download) |
| `glib-macros` | 0.18.5 | MIT | Other | [files](licenses/cargo/glib-macros-0.18.5/); [source](https://crates.io/api/v1/crates/glib-macros/0.18.5/download) |
| `glib-sys` | 0.18.1 | MIT | Other | [files](licenses/cargo/glib-sys-0.18.1/); [source](https://crates.io/api/v1/crates/glib-sys/0.18.1/download) |
| `glob` | 0.3.4 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/glob-0.3.4/); [source](https://crates.io/api/v1/crates/glob/0.3.4/download) |
| `gobject-sys` | 0.18.0 | MIT | Other | [files](licenses/cargo/gobject-sys-0.18.0/); [source](https://crates.io/api/v1/crates/gobject-sys/0.18.0/download) |
| `gtk` | 0.18.2 | MIT | Other | [files](licenses/cargo/gtk-0.18.2/); [source](https://crates.io/api/v1/crates/gtk/0.18.2/download) |
| `gtk-sys` | 0.18.2 | MIT | Other | [files](licenses/cargo/gtk-sys-0.18.2/); [source](https://crates.io/api/v1/crates/gtk-sys/0.18.2/download) |
| `gtk3-macros` | 0.18.2 | MIT | Other | [files](licenses/cargo/gtk3-macros-0.18.2/); [source](https://crates.io/api/v1/crates/gtk3-macros/0.18.2/download) |
| `hashbrown` | 0.12.3 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/hashbrown-0.12.3/); [source](https://crates.io/api/v1/crates/hashbrown/0.12.3/download) |
| `hashbrown` | 0.17.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/hashbrown-0.17.1/); [source](https://crates.io/api/v1/crates/hashbrown/0.17.1/download) |
| `heck` | 0.4.1 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/heck-0.4.1/); [source](https://crates.io/api/v1/crates/heck/0.4.1/download) |
| `heck` | 0.5.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/heck-0.5.0/); [source](https://crates.io/api/v1/crates/heck/0.5.0/download) |
| `hermit-abi` | 0.5.3 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/hermit-abi-0.5.3/); [source](https://crates.io/api/v1/crates/hermit-abi/0.5.3/download) |
| `hex` | 0.4.3 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/hex-0.4.3/); [source](https://crates.io/api/v1/crates/hex/0.4.3/download) |
| `html5ever` | 0.39.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/html5ever-0.39.0/); [source](https://crates.io/api/v1/crates/html5ever/0.39.0/download) |
| `html5ever` | 0.40.1 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/html5ever-0.40.1/); [source](https://crates.io/api/v1/crates/html5ever/0.40.1/download) |
| `http` | 1.5.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/http-1.5.0/); [source](https://crates.io/api/v1/crates/http/1.5.0/download) |
| `http-body` | 1.1.0 | MIT | Other | [files](licenses/cargo/http-body-1.1.0/); [source](https://crates.io/api/v1/crates/http-body/1.1.0/download) |
| `http-body-util` | 0.1.5 | MIT | Other | [files](licenses/cargo/http-body-util-0.1.5/); [source](https://crates.io/api/v1/crates/http-body-util/0.1.5/download) |
| `httparse` | 1.10.1 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/httparse-1.10.1/); [source](https://crates.io/api/v1/crates/httparse/1.10.1/download) |
| `hyper` | 1.12.0 | MIT | Other | [files](licenses/cargo/hyper-1.12.0/); [source](https://crates.io/api/v1/crates/hyper/1.12.0/download) |
| `hyper-util` | 0.1.21 | MIT | Other | [files](licenses/cargo/hyper-util-0.1.21/); [source](https://crates.io/api/v1/crates/hyper-util/0.1.21/download) |
| `iana-time-zone` | 0.1.65 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/iana-time-zone-0.1.65/); [source](https://crates.io/api/v1/crates/iana-time-zone/0.1.65/download) |
| `iana-time-zone-haiku` | 0.1.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/iana-time-zone-haiku-0.1.2/); [source](https://crates.io/api/v1/crates/iana-time-zone-haiku/0.1.2/download) |
| `ico` | 0.6.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/ico-0.6.0/); [source](https://crates.io/api/v1/crates/ico/0.6.0/download) |
| `icu_collections` | 2.4.0 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/icu_collections-2.4.0/); [source](https://crates.io/api/v1/crates/icu_collections/2.4.0/download) |
| `icu_locale_core` | 2.4.0 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/icu_locale_core-2.4.0/); [source](https://crates.io/api/v1/crates/icu_locale_core/2.4.0/download) |
| `icu_normalizer` | 2.4.0 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/icu_normalizer-2.4.0/); [source](https://crates.io/api/v1/crates/icu_normalizer/2.4.0/download) |
| `icu_normalizer_data` | 2.4.0 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/icu_normalizer_data-2.4.0/); [source](https://crates.io/api/v1/crates/icu_normalizer_data/2.4.0/download) |
| `icu_properties` | 2.4.0 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/icu_properties-2.4.0/); [source](https://crates.io/api/v1/crates/icu_properties/2.4.0/download) |
| `icu_properties_data` | 2.4.0 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/icu_properties_data-2.4.0/); [source](https://crates.io/api/v1/crates/icu_properties_data/2.4.0/download) |
| `icu_provider` | 2.4.0 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/icu_provider-2.4.0/); [source](https://crates.io/api/v1/crates/icu_provider/2.4.0/download) |
| `ident_case` | 1.0.1 | MIT/Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/ident_case-1.0.1/); [source](https://crates.io/api/v1/crates/ident_case/1.0.1/download) |
| `idna` | 1.1.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/idna-1.1.0/); [source](https://crates.io/api/v1/crates/idna/1.1.0/download) |
| `idna_adapter` | 1.2.2 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/idna_adapter-1.2.2/); [source](https://crates.io/api/v1/crates/idna_adapter/1.2.2/download) |
| `image` | 0.25.10 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/image-0.25.10/); [source](https://crates.io/api/v1/crates/image/0.25.10/download) |
| `indexmap` | 1.9.3 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/indexmap-1.9.3/); [source](https://crates.io/api/v1/crates/indexmap/1.9.3/download) |
| `indexmap` | 2.14.2 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/indexmap-2.14.2/); [source](https://crates.io/api/v1/crates/indexmap/2.14.2/download) |
| `infer` | 0.22.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/infer-0.22.0/); [source](https://crates.io/api/v1/crates/infer/0.22.0/download) |
| `ipnet` | 2.12.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/ipnet-2.12.2/); [source](https://crates.io/api/v1/crates/ipnet/2.12.2/download) |
| `itoa` | 1.0.19 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/itoa-1.0.19/); [source](https://crates.io/api/v1/crates/itoa/1.0.19/download) |
| `javascriptcore-rs` | 1.1.2 | MIT | Other | [files](licenses/cargo/javascriptcore-rs-1.1.2/); [source](https://crates.io/api/v1/crates/javascriptcore-rs/1.1.2/download) |
| `javascriptcore-rs-sys` | 1.1.1 | MIT | Other | [files](licenses/cargo/javascriptcore-rs-sys-1.1.1/); [source](https://crates.io/api/v1/crates/javascriptcore-rs-sys/1.1.1/download) |
| `jiff` | 0.2.43 | Unlicense OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/jiff-0.2.43/); [source](https://crates.io/api/v1/crates/jiff/0.2.43/download) |
| `jiff-core` | 0.1.5 | Unlicense OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/jiff-core-0.1.5/); [source](https://crates.io/api/v1/crates/jiff-core/0.1.5/download) |
| `jiff-static` | 0.2.43 | Unlicense OR MIT | Other | [files](licenses/cargo/jiff-static-0.2.43/); [source](https://crates.io/api/v1/crates/jiff-static/0.2.43/download) |
| `jiff-tzdb` | 0.1.9 | Unlicense OR MIT | Windows x64 | [files](licenses/cargo/jiff-tzdb-0.1.9/); [source](https://crates.io/api/v1/crates/jiff-tzdb/0.1.9/download) |
| `jiff-tzdb-platform` | 0.1.3 | Unlicense OR MIT | Windows x64 | [files](licenses/cargo/jiff-tzdb-platform-0.1.3/); [source](https://crates.io/api/v1/crates/jiff-tzdb-platform/0.1.3/download) |
| `jni` | 0.21.1 | MIT/Apache-2.0 | Other | [files](licenses/cargo/jni-0.21.1/); [source](https://crates.io/api/v1/crates/jni/0.21.1/download) |
| `jni-sys` | 0.3.1 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/jni-sys-0.3.1/); [source](https://crates.io/api/v1/crates/jni-sys/0.3.1/download) |
| `jni-sys` | 0.4.1 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/jni-sys-0.4.1/); [source](https://crates.io/api/v1/crates/jni-sys/0.4.1/download) |
| `jni-sys-macros` | 0.4.1 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/jni-sys-macros-0.4.1/); [source](https://crates.io/api/v1/crates/jni-sys-macros/0.4.1/download) |
| `js-sys` | 0.3.107 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/js-sys-0.3.107/); [source](https://crates.io/api/v1/crates/js-sys/0.3.107/download) |
| `json-patch` | 4.2.0 | MIT/Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/json-patch-4.2.0/); [source](https://crates.io/api/v1/crates/json-patch/4.2.0/download) |
| `jsonptr` | 0.7.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/jsonptr-0.7.1/); [source](https://crates.io/api/v1/crates/jsonptr/0.7.1/download) |
| `keyboard-types` | 0.8.3 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/keyboard-types-0.8.3/); [source](https://crates.io/api/v1/crates/keyboard-types/0.8.3/download) |
| `libappindicator` | 0.9.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/libappindicator-0.9.0/); [source](https://crates.io/api/v1/crates/libappindicator/0.9.0/download) |
| `libappindicator-sys` | 0.9.0 | Apache-2.0 OR MIT | Other | No packaged license file; [source](https://crates.io/api/v1/crates/libappindicator-sys/0.9.0/download) |
| `libc` | 0.2.191 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/libc-0.2.191/); [source](https://crates.io/api/v1/crates/libc/0.2.191/download) |
| `libdbus-sys` | 0.2.7 | Apache-2.0/MIT | Other | [files](licenses/cargo/libdbus-sys-0.2.7/); [source](https://crates.io/api/v1/crates/libdbus-sys/0.2.7/download) |
| `libloading` | 0.7.4 | ISC | Other | [files](licenses/cargo/libloading-0.7.4/); [source](https://crates.io/api/v1/crates/libloading/0.7.4/download) |
| `libredox` | 0.1.25 | MIT | Other | [files](licenses/cargo/libredox-0.1.25/); [source](https://crates.io/api/v1/crates/libredox/0.1.25/download) |
| `linux-raw-sys` | 0.12.1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/linux-raw-sys-0.12.1/); [source](https://crates.io/api/v1/crates/linux-raw-sys/0.12.1/download) |
| `litemap` | 0.8.3 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/litemap-0.8.3/); [source](https://crates.io/api/v1/crates/litemap/0.8.3/download) |
| `lock_api` | 0.4.14 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/lock_api-0.4.14/); [source](https://crates.io/api/v1/crates/lock_api/0.4.14/download) |
| `log` | 0.4.35 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/log-0.4.35/); [source](https://crates.io/api/v1/crates/log/0.4.35/download) |
| `markup5ever` | 0.39.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/markup5ever-0.39.0/); [source](https://crates.io/api/v1/crates/markup5ever/0.39.0/download) |
| `markup5ever` | 0.40.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/markup5ever-0.40.0/); [source](https://crates.io/api/v1/crates/markup5ever/0.40.0/download) |
| `memchr` | 2.8.3 | Unlicense OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/memchr-2.8.3/); [source](https://crates.io/api/v1/crates/memchr/2.8.3/download) |
| `memoffset` | 0.9.1 | MIT | Other | [files](licenses/cargo/memoffset-0.9.1/); [source](https://crates.io/api/v1/crates/memoffset/0.9.1/download) |
| `mime` | 0.3.18 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/mime-0.3.18/); [source](https://crates.io/api/v1/crates/mime/0.3.18/download) |
| `miniz_oxide` | 0.8.9 | MIT OR Zlib OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/miniz_oxide-0.8.9/); [source](https://crates.io/api/v1/crates/miniz_oxide/0.8.9/download) |
| `miniz_oxide` | 0.9.1 | MIT OR Zlib OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/miniz_oxide-0.9.1/); [source](https://crates.io/api/v1/crates/miniz_oxide/0.9.1/download) |
| `mio` | 1.2.4 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/mio-1.2.4/); [source](https://crates.io/api/v1/crates/mio/1.2.4/download) |
| `moxcms` | 0.8.1 | BSD-3-Clause OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/moxcms-0.8.1/); [source](https://crates.io/api/v1/crates/moxcms/0.8.1/download) |
| `muda` | 0.20.0 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/muda-0.20.0/); [source](https://crates.io/api/v1/crates/muda/0.20.0/download) |
| `ndk` | 0.9.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/ndk-0.9.0/); [source](https://crates.io/api/v1/crates/ndk/0.9.0/download) |
| `ndk-context` | 0.1.1 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/ndk-context-0.1.1/); [source](https://crates.io/api/v1/crates/ndk-context/0.1.1/download) |
| `ndk-sys` | 0.6.0+11769913 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/ndk-sys-0.6.0+11769913/); [source](https://crates.io/api/v1/crates/ndk-sys/0.6.0+11769913/download) |
| `new_debug_unreachable` | 1.0.6 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/new_debug_unreachable-1.0.6/); [source](https://crates.io/api/v1/crates/new_debug_unreachable/1.0.6/download) |
| `nix` | 0.31.3 | MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/nix-0.31.3/); [source](https://crates.io/api/v1/crates/nix/0.31.3/download) |
| `num-conv` | 0.2.2 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/num-conv-0.2.2/); [source](https://crates.io/api/v1/crates/num-conv/0.2.2/download) |
| `num-traits` | 0.2.19 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/num-traits-0.2.19/); [source](https://crates.io/api/v1/crates/num-traits/0.2.19/download) |
| `num_enum` | 0.7.6 | BSD-3-Clause OR MIT OR Apache-2.0 | Other | [files](licenses/cargo/num_enum-0.7.6/); [source](https://crates.io/api/v1/crates/num_enum/0.7.6/download) |
| `num_enum_derive` | 0.7.6 | BSD-3-Clause OR MIT OR Apache-2.0 | Other | [files](licenses/cargo/num_enum_derive-0.7.6/); [source](https://crates.io/api/v1/crates/num_enum_derive/0.7.6/download) |
| `objc2` | 0.6.5 | MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-0.6.5/); [source](https://crates.io/api/v1/crates/objc2/0.6.5/download) |
| `objc2-app-kit` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-app-kit-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-app-kit/0.3.2/download) |
| `objc2-cloud-kit` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/objc2-cloud-kit-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-cloud-kit/0.3.2/download) |
| `objc2-core-data` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/objc2-core-data-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-core-data/0.3.2/download) |
| `objc2-core-foundation` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-core-foundation-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-core-foundation/0.3.2/download) |
| `objc2-core-graphics` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-core-graphics-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-core-graphics/0.3.2/download) |
| `objc2-core-image` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/objc2-core-image-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-core-image/0.3.2/download) |
| `objc2-core-location` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/objc2-core-location-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-core-location/0.3.2/download) |
| `objc2-core-text` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/objc2-core-text-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-core-text/0.3.2/download) |
| `objc2-encode` | 4.1.0 | MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-encode-4.1.0/); [source](https://crates.io/api/v1/crates/objc2-encode/4.1.0/download) |
| `objc2-exception-helper` | 0.1.1 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-exception-helper-0.1.1/); [source](https://crates.io/api/v1/crates/objc2-exception-helper/0.1.1/download) |
| `objc2-foundation` | 0.3.2 | MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-foundation-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-foundation/0.3.2/download) |
| `objc2-io-surface` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-io-surface-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-io-surface/0.3.2/download) |
| `objc2-quartz-core` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-quartz-core-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-quartz-core/0.3.2/download) |
| `objc2-security` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-security-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-security/0.3.2/download) |
| `objc2-service-management` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-service-management-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-service-management/0.3.2/download) |
| `objc2-ui-kit` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/objc2-ui-kit-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-ui-kit/0.3.2/download) |
| `objc2-user-notifications` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/objc2-user-notifications-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-user-notifications/0.3.2/download) |
| `objc2-web-kit` | 0.3.2 | Zlib OR Apache-2.0 OR MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/objc2-web-kit-0.3.2/); [source](https://crates.io/api/v1/crates/objc2-web-kit/0.3.2/download) |
| `once_cell` | 1.21.4 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/once_cell-1.21.4/); [source](https://crates.io/api/v1/crates/once_cell/1.21.4/download) |
| `option-ext` | 0.2.0 | MPL-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/option-ext-0.2.0/); [source](https://crates.io/api/v1/crates/option-ext/0.2.0/download) |
| `ordered-stream` | 0.2.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/ordered-stream-0.2.0/); [source](https://crates.io/api/v1/crates/ordered-stream/0.2.0/download) |
| `os_info` | 3.15.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/os_info-3.15.0/); [source](https://crates.io/api/v1/crates/os_info/3.15.0/download) |
| `pango` | 0.18.3 | MIT | Other | [files](licenses/cargo/pango-0.18.3/); [source](https://crates.io/api/v1/crates/pango/0.18.3/download) |
| `pango-sys` | 0.18.0 | MIT | Other | [files](licenses/cargo/pango-sys-0.18.0/); [source](https://crates.io/api/v1/crates/pango-sys/0.18.0/download) |
| `parking` | 2.2.1 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/parking-2.2.1/); [source](https://crates.io/api/v1/crates/parking/2.2.1/download) |
| `parking_lot` | 0.12.5 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/parking_lot-0.12.5/); [source](https://crates.io/api/v1/crates/parking_lot/0.12.5/download) |
| `parking_lot_core` | 0.9.12 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/parking_lot_core-0.9.12/); [source](https://crates.io/api/v1/crates/parking_lot_core/0.9.12/download) |
| `percent-encoding` | 2.3.2 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/percent-encoding-2.3.2/); [source](https://crates.io/api/v1/crates/percent-encoding/2.3.2/download) |
| `phf` | 0.13.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/phf-0.13.1/); [source](https://crates.io/api/v1/crates/phf/0.13.1/download) |
| `phf` | 0.14.0 | MIT | Other | [files](licenses/cargo/phf-0.14.0/); [source](https://crates.io/api/v1/crates/phf/0.14.0/download) |
| `phf_codegen` | 0.13.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/phf_codegen-0.13.1/); [source](https://crates.io/api/v1/crates/phf_codegen/0.13.1/download) |
| `phf_codegen` | 0.14.0 | MIT | Other | [files](licenses/cargo/phf_codegen-0.14.0/); [source](https://crates.io/api/v1/crates/phf_codegen/0.14.0/download) |
| `phf_generator` | 0.13.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/phf_generator-0.13.1/); [source](https://crates.io/api/v1/crates/phf_generator/0.13.1/download) |
| `phf_generator` | 0.14.0 | MIT | Other | [files](licenses/cargo/phf_generator-0.14.0/); [source](https://crates.io/api/v1/crates/phf_generator/0.14.0/download) |
| `phf_macros` | 0.13.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/phf_macros-0.13.1/); [source](https://crates.io/api/v1/crates/phf_macros/0.13.1/download) |
| `phf_macros` | 0.14.0 | MIT | Other | [files](licenses/cargo/phf_macros-0.14.0/); [source](https://crates.io/api/v1/crates/phf_macros/0.14.0/download) |
| `phf_shared` | 0.13.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/phf_shared-0.13.1/); [source](https://crates.io/api/v1/crates/phf_shared/0.13.1/download) |
| `phf_shared` | 0.14.0 | MIT | Other | [files](licenses/cargo/phf_shared-0.14.0/); [source](https://crates.io/api/v1/crates/phf_shared/0.14.0/download) |
| `pin-project-lite` | 0.2.17 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/pin-project-lite-0.2.17/); [source](https://crates.io/api/v1/crates/pin-project-lite/0.2.17/download) |
| `piper` | 0.2.5 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/piper-0.2.5/); [source](https://crates.io/api/v1/crates/piper/0.2.5/download) |
| `pkg-config` | 0.3.34 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/pkg-config-0.3.34/); [source](https://crates.io/api/v1/crates/pkg-config/0.3.34/download) |
| `plist` | 1.10.3 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/plist-1.10.3/); [source](https://crates.io/api/v1/crates/plist/1.10.3/download) |
| `png` | 0.18.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/png-0.18.1/); [source](https://crates.io/api/v1/crates/png/0.18.1/download) |
| `polling` | 3.11.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/polling-3.11.0/); [source](https://crates.io/api/v1/crates/polling/3.11.0/download) |
| `portable-atomic` | 1.15.0 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/portable-atomic-1.15.0/); [source](https://crates.io/api/v1/crates/portable-atomic/1.15.0/download) |
| `portable-atomic-util` | 0.2.8 | Apache-2.0 OR MIT | Other | [files](licenses/cargo/portable-atomic-util-0.2.8/); [source](https://crates.io/api/v1/crates/portable-atomic-util/0.2.8/download) |
| `potential_utf` | 0.1.6 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/potential_utf-0.1.6/); [source](https://crates.io/api/v1/crates/potential_utf/0.1.6/download) |
| `powerfmt` | 0.2.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/powerfmt-0.2.1/); [source](https://crates.io/api/v1/crates/powerfmt/0.2.1/download) |
| `precomputed-hash` | 0.1.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/precomputed-hash-0.1.1/); [source](https://crates.io/api/v1/crates/precomputed-hash/0.1.1/download) |
| `proc-macro-crate` | 1.3.1 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/proc-macro-crate-1.3.1/); [source](https://crates.io/api/v1/crates/proc-macro-crate/1.3.1/download) |
| `proc-macro-crate` | 2.0.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/proc-macro-crate-2.0.2/); [source](https://crates.io/api/v1/crates/proc-macro-crate/2.0.2/download) |
| `proc-macro-crate` | 3.5.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/proc-macro-crate-3.5.0/); [source](https://crates.io/api/v1/crates/proc-macro-crate/3.5.0/download) |
| `proc-macro-error` | 1.0.4 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/proc-macro-error-1.0.4/); [source](https://crates.io/api/v1/crates/proc-macro-error/1.0.4/download) |
| `proc-macro-error-attr` | 1.0.4 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/proc-macro-error-attr-1.0.4/); [source](https://crates.io/api/v1/crates/proc-macro-error-attr/1.0.4/download) |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/proc-macro2-1.0.107/); [source](https://crates.io/api/v1/crates/proc-macro2/1.0.107/download) |
| `pxfm` | 0.1.30 | BSD-3-Clause OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/pxfm-0.1.30/); [source](https://crates.io/api/v1/crates/pxfm/0.1.30/download) |
| `quick-xml` | 0.42.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/quick-xml-0.42.0/); [source](https://crates.io/api/v1/crates/quick-xml/0.42.0/download) |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/quote-1.0.47/); [source](https://crates.io/api/v1/crates/quote/1.0.47/download) |
| `r-efi` | 5.3.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | Other | No packaged license file; [source](https://crates.io/api/v1/crates/r-efi/5.3.0/download) |
| `r-efi` | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | Other | No packaged license file; [source](https://crates.io/api/v1/crates/r-efi/6.0.0/download) |
| `raw-window-handle` | 0.6.2 | MIT OR Apache-2.0 OR Zlib | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/raw-window-handle-0.6.2/); [source](https://crates.io/api/v1/crates/raw-window-handle/0.6.2/download) |
| `redox_syscall` | 0.5.18 | MIT | Other | [files](licenses/cargo/redox_syscall-0.5.18/); [source](https://crates.io/api/v1/crates/redox_syscall/0.5.18/download) |
| `redox_users` | 0.5.3 | MIT | Other | [files](licenses/cargo/redox_users-0.5.3/); [source](https://crates.io/api/v1/crates/redox_users/0.5.3/download) |
| `ref-cast` | 1.0.27 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/ref-cast-1.0.27/); [source](https://crates.io/api/v1/crates/ref-cast/1.0.27/download) |
| `ref-cast-impl` | 1.0.27 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/ref-cast-impl-1.0.27/); [source](https://crates.io/api/v1/crates/ref-cast-impl/1.0.27/download) |
| `regex` | 1.13.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/regex-1.13.1/); [source](https://crates.io/api/v1/crates/regex/1.13.1/download) |
| `regex-automata` | 0.4.18 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/regex-automata-0.4.18/); [source](https://crates.io/api/v1/crates/regex-automata/0.4.18/download) |
| `regex-syntax` | 0.8.11 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/regex-syntax-0.8.11/); [source](https://crates.io/api/v1/crates/regex-syntax/0.8.11/download) |
| `reqwest` | 0.13.5 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/reqwest-0.13.5/); [source](https://crates.io/api/v1/crates/reqwest/0.13.5/download) |
| `rustc-hash` | 2.1.3 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/rustc-hash-2.1.3/); [source](https://crates.io/api/v1/crates/rustc-hash/2.1.3/download) |
| `rustc_version` | 0.4.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/rustc_version-0.4.1/); [source](https://crates.io/api/v1/crates/rustc_version/0.4.1/download) |
| `rustix` | 1.1.6 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/rustix-1.1.6/); [source](https://crates.io/api/v1/crates/rustix/1.1.6/download) |
| `rustversion` | 1.0.23 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/rustversion-1.0.23/); [source](https://crates.io/api/v1/crates/rustversion/1.0.23/download) |
| `same-file` | 1.0.6 | Unlicense/MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/same-file-1.0.6/); [source](https://crates.io/api/v1/crates/same-file/1.0.6/download) |
| `schemars` | 0.8.22 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/schemars-0.8.22/); [source](https://crates.io/api/v1/crates/schemars/0.8.22/download) |
| `schemars` | 0.9.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/schemars-0.9.0/); [source](https://crates.io/api/v1/crates/schemars/0.9.0/download) |
| `schemars` | 1.2.3 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/schemars-1.2.3/); [source](https://crates.io/api/v1/crates/schemars/1.2.3/download) |
| `schemars_derive` | 0.8.22 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/schemars_derive-0.8.22/); [source](https://crates.io/api/v1/crates/schemars_derive/0.8.22/download) |
| `scopeguard` | 1.2.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/scopeguard-1.2.0/); [source](https://crates.io/api/v1/crates/scopeguard/1.2.0/download) |
| `selectors` | 0.38.0 | MPL-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/selectors-0.38.0/); [source](https://crates.io/api/v1/crates/selectors/0.38.0/download) |
| `selectors` | 0.41.0 | MPL-2.0 | Other | [files](licenses/cargo/selectors-0.41.0/); [source](https://crates.io/api/v1/crates/selectors/0.41.0/download) |
| `semver` | 1.0.28 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/semver-1.0.28/); [source](https://crates.io/api/v1/crates/semver/1.0.28/download) |
| `serde` | 1.0.229 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde-1.0.229/); [source](https://crates.io/api/v1/crates/serde/1.0.229/download) |
| `serde-untagged` | 0.1.9 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde-untagged-0.1.9/); [source](https://crates.io/api/v1/crates/serde-untagged/0.1.9/download) |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde_core-1.0.229/); [source](https://crates.io/api/v1/crates/serde_core/1.0.229/download) |
| `serde_derive` | 1.0.229 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde_derive-1.0.229/); [source](https://crates.io/api/v1/crates/serde_derive/1.0.229/download) |
| `serde_derive_internals` | 0.29.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde_derive_internals-0.29.1/); [source](https://crates.io/api/v1/crates/serde_derive_internals/0.29.1/download) |
| `serde_json` | 1.0.154 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde_json-1.0.154/); [source](https://crates.io/api/v1/crates/serde_json/1.0.154/download) |
| `serde_repr` | 0.1.21 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde_repr-0.1.21/); [source](https://crates.io/api/v1/crates/serde_repr/0.1.21/download) |
| `serde_spanned` | 0.6.9 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/serde_spanned-0.6.9/); [source](https://crates.io/api/v1/crates/serde_spanned/0.6.9/download) |
| `serde_spanned` | 1.1.3 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde_spanned-1.1.3/); [source](https://crates.io/api/v1/crates/serde_spanned/1.1.3/download) |
| `serde_with` | 3.24.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde_with-3.24.0/); [source](https://crates.io/api/v1/crates/serde_with/3.24.0/download) |
| `serde_with_macros` | 3.24.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serde_with_macros-3.24.0/); [source](https://crates.io/api/v1/crates/serde_with_macros/3.24.0/download) |
| `serialize-to-javascript` | 0.1.2 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serialize-to-javascript-0.1.2/); [source](https://crates.io/api/v1/crates/serialize-to-javascript/0.1.2/download) |
| `serialize-to-javascript-impl` | 0.1.2 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/serialize-to-javascript-impl-0.1.2/); [source](https://crates.io/api/v1/crates/serialize-to-javascript-impl/0.1.2/download) |
| `servo_arc` | 0.4.3 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/servo_arc-0.4.3/); [source](https://crates.io/api/v1/crates/servo_arc/0.4.3/download) |
| `servo_arc` | 0.5.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/servo_arc-0.5.0/); [source](https://crates.io/api/v1/crates/servo_arc/0.5.0/download) |
| `sha2` | 0.10.9 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/sha2-0.10.9/); [source](https://crates.io/api/v1/crates/sha2/0.10.9/download) |
| `shlex` | 2.0.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/shlex-2.0.1/); [source](https://crates.io/api/v1/crates/shlex/2.0.1/download) |
| `signal-hook-registry` | 1.4.8 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/signal-hook-registry-1.4.8/); [source](https://crates.io/api/v1/crates/signal-hook-registry/1.4.8/download) |
| `simd-adler32` | 0.3.10 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/simd-adler32-0.3.10/); [source](https://crates.io/api/v1/crates/simd-adler32/0.3.10/download) |
| `siphasher` | 1.0.4 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/siphasher-1.0.4/); [source](https://crates.io/api/v1/crates/siphasher/1.0.4/download) |
| `slab` | 0.4.12 | MIT | Other | [files](licenses/cargo/slab-0.4.12/); [source](https://crates.io/api/v1/crates/slab/0.4.12/download) |
| `smallvec` | 1.16.5 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/smallvec-1.16.5/); [source](https://crates.io/api/v1/crates/smallvec/1.16.5/download) |
| `smappservice-rs` | 0.1.3 | MIT | macOS ARM64, macOS x64 | [files](licenses/cargo/smappservice-rs-0.1.3/); [source](https://crates.io/api/v1/crates/smappservice-rs/0.1.3/download) |
| `socket2` | 0.6.5 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/socket2-0.6.5/); [source](https://crates.io/api/v1/crates/socket2/0.6.5/download) |
| `softbuffer` | 0.4.8 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/softbuffer-0.4.8/); [source](https://crates.io/api/v1/crates/softbuffer/0.4.8/download) |
| `soup3` | 0.5.0 | MIT | Other | [files](licenses/cargo/soup3-0.5.0/); [source](https://crates.io/api/v1/crates/soup3/0.5.0/download) |
| `soup3-sys` | 0.5.0 | MIT | Other | [files](licenses/cargo/soup3-sys-0.5.0/); [source](https://crates.io/api/v1/crates/soup3-sys/0.5.0/download) |
| `stable_deref_trait` | 1.2.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/stable_deref_trait-1.2.1/); [source](https://crates.io/api/v1/crates/stable_deref_trait/1.2.1/download) |
| `static_vcruntime` | 3.0.0 | MIT OR Apache-2.0 OR Zlib | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/static_vcruntime-3.0.0/); [source](https://crates.io/api/v1/crates/static_vcruntime/3.0.0/download) |
| `string_cache` | 0.11.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/string_cache-0.11.0/); [source](https://crates.io/api/v1/crates/string_cache/0.11.0/download) |
| `string_cache` | 0.9.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/string_cache-0.9.0/); [source](https://crates.io/api/v1/crates/string_cache/0.9.0/download) |
| `string_cache_codegen` | 0.11.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/string_cache_codegen-0.11.2/); [source](https://crates.io/api/v1/crates/string_cache_codegen/0.11.2/download) |
| `string_cache_codegen` | 0.6.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/string_cache_codegen-0.6.1/); [source](https://crates.io/api/v1/crates/string_cache_codegen/0.6.1/download) |
| `strsim` | 0.11.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/strsim-0.11.1/); [source](https://crates.io/api/v1/crates/strsim/0.11.1/download) |
| `swift-rs` | 1.0.9 | MIT OR Apache-2.0 | macOS ARM64, macOS x64 | [files](licenses/cargo/swift-rs-1.0.9/); [source](https://crates.io/api/v1/crates/swift-rs/1.0.9/download) |
| `syn` | 1.0.109 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/syn-1.0.109/); [source](https://crates.io/api/v1/crates/syn/1.0.109/download) |
| `syn` | 2.0.119 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/syn-2.0.119/); [source](https://crates.io/api/v1/crates/syn/2.0.119/download) |
| `syn` | 3.0.10 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/syn-3.0.10/); [source](https://crates.io/api/v1/crates/syn/3.0.10/download) |
| `sync_wrapper` | 1.0.2 | Apache-2.0 | Other | [files](licenses/cargo/sync_wrapper-1.0.2/); [source](https://crates.io/api/v1/crates/sync_wrapper/1.0.2/download) |
| `synstructure` | 0.14.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/synstructure-0.14.0/); [source](https://crates.io/api/v1/crates/synstructure/0.14.0/download) |
| `system-deps` | 6.2.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/system-deps-6.2.2/); [source](https://crates.io/api/v1/crates/system-deps/6.2.2/download) |
| `tao` | 0.37.2 | Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tao-0.37.2/); [source](https://crates.io/api/v1/crates/tao/0.37.2/download) |
| `tao-macros` | 0.1.4 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/tao-macros-0.1.4/); [source](https://crates.io/api/v1/crates/tao-macros/0.1.4/download) |
| `target-lexicon` | 0.12.16 | Apache-2.0 WITH LLVM-exception | Other | [files](licenses/cargo/target-lexicon-0.12.16/); [source](https://crates.io/api/v1/crates/target-lexicon/0.12.16/download) |
| `tauri` | 2.12.5 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-2.12.5/); [source](https://crates.io/api/v1/crates/tauri/2.12.5/download) |
| `tauri-build` | 2.7.4 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-build-2.7.4/); [source](https://crates.io/api/v1/crates/tauri-build/2.7.4/download) |
| `tauri-codegen` | 2.7.4 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-codegen-2.7.4/); [source](https://crates.io/api/v1/crates/tauri-codegen/2.7.4/download) |
| `tauri-macros` | 2.7.4 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-macros-2.7.4/); [source](https://crates.io/api/v1/crates/tauri-macros/2.7.4/download) |
| `tauri-plugin` | 2.7.2 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-plugin-2.7.2/); [source](https://crates.io/api/v1/crates/tauri-plugin/2.7.2/download) |
| `tauri-plugin-autostart` | 2.7.0 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-plugin-autostart-2.7.0/); [source](https://crates.io/api/v1/crates/tauri-plugin-autostart/2.7.0/download) |
| `tauri-plugin-single-instance` | 2.5.5 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-plugin-single-instance-2.5.5/); [source](https://crates.io/api/v1/crates/tauri-plugin-single-instance/2.5.5/download) |
| `tauri-runtime` | 2.12.2 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-runtime-2.12.2/); [source](https://crates.io/api/v1/crates/tauri-runtime/2.12.2/download) |
| `tauri-runtime-wry` | 2.12.3 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-runtime-wry-2.12.3/); [source](https://crates.io/api/v1/crates/tauri-runtime-wry/2.12.3/download) |
| `tauri-utils` | 2.10.2 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-utils-2.10.2/); [source](https://crates.io/api/v1/crates/tauri-utils/2.10.2/download) |
| `tauri-winres` | 0.3.7 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tauri-winres-0.3.7/); [source](https://crates.io/api/v1/crates/tauri-winres/0.3.7/download) |
| `tempfile` | 3.27.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/tempfile-3.27.0/); [source](https://crates.io/api/v1/crates/tempfile/3.27.0/download) |
| `tendril` | 0.5.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tendril-0.5.1/); [source](https://crates.io/api/v1/crates/tendril/0.5.1/download) |
| `thiserror` | 1.0.69 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/thiserror-1.0.69/); [source](https://crates.io/api/v1/crates/thiserror/1.0.69/download) |
| `thiserror` | 2.0.21 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/thiserror-2.0.21/); [source](https://crates.io/api/v1/crates/thiserror/2.0.21/download) |
| `thiserror-impl` | 1.0.69 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/thiserror-impl-1.0.69/); [source](https://crates.io/api/v1/crates/thiserror-impl/1.0.69/download) |
| `thiserror-impl` | 2.0.21 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/thiserror-impl-2.0.21/); [source](https://crates.io/api/v1/crates/thiserror-impl/2.0.21/download) |
| `time` | 0.3.55 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/time-0.3.55/); [source](https://crates.io/api/v1/crates/time/0.3.55/download) |
| `time-core` | 0.1.9 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/time-core-0.1.9/); [source](https://crates.io/api/v1/crates/time-core/0.1.9/download) |
| `time-macros` | 0.2.32 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/time-macros-0.2.32/); [source](https://crates.io/api/v1/crates/time-macros/0.2.32/download) |
| `tinystr` | 0.8.5 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tinystr-0.8.5/); [source](https://crates.io/api/v1/crates/tinystr/0.8.5/download) |
| `tinyvec` | 1.13.3 | Zlib OR Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tinyvec-1.13.3/); [source](https://crates.io/api/v1/crates/tinyvec/1.13.3/download) |
| `tokio` | 1.54.1 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tokio-1.54.1/); [source](https://crates.io/api/v1/crates/tokio/1.54.1/download) |
| `tokio-util` | 0.7.20 | MIT | Other | [files](licenses/cargo/tokio-util-0.7.20/); [source](https://crates.io/api/v1/crates/tokio-util/0.7.20/download) |
| `toml` | 0.8.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/toml-0.8.2/); [source](https://crates.io/api/v1/crates/toml/0.8.2/download) |
| `toml` | 1.2.1+spec-1.1.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/toml-1.2.1+spec-1.1.0/); [source](https://crates.io/api/v1/crates/toml/1.2.1+spec-1.1.0/download) |
| `toml_datetime` | 0.6.3 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/toml_datetime-0.6.3/); [source](https://crates.io/api/v1/crates/toml_datetime/0.6.3/download) |
| `toml_datetime` | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/toml_datetime-1.1.3+spec-1.1.0/); [source](https://crates.io/api/v1/crates/toml_datetime/1.1.3+spec-1.1.0/download) |
| `toml_edit` | 0.19.15 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/toml_edit-0.19.15/); [source](https://crates.io/api/v1/crates/toml_edit/0.19.15/download) |
| `toml_edit` | 0.20.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/toml_edit-0.20.2/); [source](https://crates.io/api/v1/crates/toml_edit/0.20.2/download) |
| `toml_edit` | 0.25.19+spec-1.1.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/toml_edit-0.25.19+spec-1.1.0/); [source](https://crates.io/api/v1/crates/toml_edit/0.25.19+spec-1.1.0/download) |
| `toml_parser` | 1.1.5+spec-1.1.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/toml_parser-1.1.5+spec-1.1.0/); [source](https://crates.io/api/v1/crates/toml_parser/1.1.5+spec-1.1.0/download) |
| `toml_writer` | 1.2.0+spec-1.1.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/toml_writer-1.2.0+spec-1.1.0/); [source](https://crates.io/api/v1/crates/toml_writer/1.2.0+spec-1.1.0/download) |
| `tower` | 0.5.3 | MIT | Other | [files](licenses/cargo/tower-0.5.3/); [source](https://crates.io/api/v1/crates/tower/0.5.3/download) |
| `tower-http` | 0.6.11 | MIT | Other | [files](licenses/cargo/tower-http-0.6.11/); [source](https://crates.io/api/v1/crates/tower-http/0.6.11/download) |
| `tower-layer` | 0.3.3 | MIT | Other | [files](licenses/cargo/tower-layer-0.3.3/); [source](https://crates.io/api/v1/crates/tower-layer/0.3.3/download) |
| `tower-service` | 0.3.3 | MIT | Other | [files](licenses/cargo/tower-service-0.3.3/); [source](https://crates.io/api/v1/crates/tower-service/0.3.3/download) |
| `tracing` | 0.1.44 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tracing-0.1.44/); [source](https://crates.io/api/v1/crates/tracing/0.1.44/download) |
| `tracing-attributes` | 0.1.31 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tracing-attributes-0.1.31/); [source](https://crates.io/api/v1/crates/tracing-attributes/0.1.31/download) |
| `tracing-core` | 0.1.36 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tracing-core-0.1.36/); [source](https://crates.io/api/v1/crates/tracing-core/0.1.36/download) |
| `tray-icon` | 0.25.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/tray-icon-0.25.1/); [source](https://crates.io/api/v1/crates/tray-icon/0.25.1/download) |
| `try-lock` | 0.2.5 | MIT | Other | [files](licenses/cargo/try-lock-0.2.5/); [source](https://crates.io/api/v1/crates/try-lock/0.2.5/download) |
| `typeid` | 1.0.3 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/typeid-1.0.3/); [source](https://crates.io/api/v1/crates/typeid/1.0.3/download) |
| `typenum` | 1.20.1 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/typenum-1.20.1/); [source](https://crates.io/api/v1/crates/typenum/1.20.1/download) |
| `uds_windows` | 1.2.1 | MIT | Other | [files](licenses/cargo/uds_windows-1.2.1/); [source](https://crates.io/api/v1/crates/uds_windows/1.2.1/download) |
| `unicode-ident` | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/unicode-ident-1.0.27/); [source](https://crates.io/api/v1/crates/unicode-ident/1.0.27/download) |
| `unicode-segmentation` | 1.13.3 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/unicode-segmentation-1.13.3/); [source](https://crates.io/api/v1/crates/unicode-segmentation/1.13.3/download) |
| `url` | 2.5.8 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/url-2.5.8/); [source](https://crates.io/api/v1/crates/url/2.5.8/download) |
| `urlpattern` | 0.6.0 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/urlpattern-0.6.0/); [source](https://crates.io/api/v1/crates/urlpattern/0.6.0/download) |
| `utf8_iter` | 1.1.0 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/utf8_iter-1.1.0/); [source](https://crates.io/api/v1/crates/utf8_iter/1.1.0/download) |
| `utf_types` | 1.0.0 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/utf_types-1.0.0/); [source](https://crates.io/api/v1/crates/utf_types/1.0.0/download) |
| `uuid` | 1.29.2 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/uuid-1.29.2/); [source](https://crates.io/api/v1/crates/uuid/1.29.2/download) |
| `version-compare` | 0.2.1 | MIT | Other | [files](licenses/cargo/version-compare-0.2.1/); [source](https://crates.io/api/v1/crates/version-compare/0.2.1/download) |
| `version_check` | 0.9.5 | MIT/Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/version_check-0.9.5/); [source](https://crates.io/api/v1/crates/version_check/0.9.5/download) |
| `vswhom` | 0.1.0 | MIT | Windows x64 | [files](licenses/cargo/vswhom-0.1.0/); [source](https://crates.io/api/v1/crates/vswhom/0.1.0/download) |
| `vswhom-sys` | 0.1.3 | MIT | Windows x64 | [files](licenses/cargo/vswhom-sys-0.1.3/); [source](https://crates.io/api/v1/crates/vswhom-sys/0.1.3/download) |
| `walkdir` | 2.5.0 | Unlicense/MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/walkdir-2.5.0/); [source](https://crates.io/api/v1/crates/walkdir/2.5.0/download) |
| `want` | 0.3.2 | MIT | Other | [files](licenses/cargo/want-0.3.2/); [source](https://crates.io/api/v1/crates/want/0.3.2/download) |
| `wasi` | 0.11.1+wasi-snapshot-preview1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/wasi-0.11.1+wasi-snapshot-preview1/); [source](https://crates.io/api/v1/crates/wasi/0.11.1+wasi-snapshot-preview1/download) |
| `wasip2` | 1.0.4+wasi-0.2.12 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/wasip2-1.0.4+wasi-0.2.12/); [source](https://crates.io/api/v1/crates/wasip2/1.0.4+wasi-0.2.12/download) |
| `wasm-bindgen` | 0.2.130 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/wasm-bindgen-0.2.130/); [source](https://crates.io/api/v1/crates/wasm-bindgen/0.2.130/download) |
| `wasm-bindgen-futures` | 0.4.80 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/wasm-bindgen-futures-0.4.80/); [source](https://crates.io/api/v1/crates/wasm-bindgen-futures/0.4.80/download) |
| `wasm-bindgen-macro` | 0.2.130 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/wasm-bindgen-macro-0.2.130/); [source](https://crates.io/api/v1/crates/wasm-bindgen-macro/0.2.130/download) |
| `wasm-bindgen-macro-support` | 0.2.130 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/wasm-bindgen-macro-support-0.2.130/); [source](https://crates.io/api/v1/crates/wasm-bindgen-macro-support/0.2.130/download) |
| `wasm-bindgen-shared` | 0.2.130 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/wasm-bindgen-shared-0.2.130/); [source](https://crates.io/api/v1/crates/wasm-bindgen-shared/0.2.130/download) |
| `wasm-streams` | 0.5.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/wasm-streams-0.5.0/); [source](https://crates.io/api/v1/crates/wasm-streams/0.5.0/download) |
| `web-sys` | 0.3.107 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/web-sys-0.3.107/); [source](https://crates.io/api/v1/crates/web-sys/0.3.107/download) |
| `web-time` | 1.1.0 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/web-time-1.1.0/); [source](https://crates.io/api/v1/crates/web-time/1.1.0/download) |
| `web_atoms` | 0.2.6 | MIT OR Apache-2.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/web_atoms-0.2.6/); [source](https://crates.io/api/v1/crates/web_atoms/0.2.6/download) |
| `web_atoms` | 0.3.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/web_atoms-0.3.0/); [source](https://crates.io/api/v1/crates/web_atoms/0.3.0/download) |
| `webkit2gtk` | 2.0.2 | MIT | Other | [files](licenses/cargo/webkit2gtk-2.0.2/); [source](https://crates.io/api/v1/crates/webkit2gtk/2.0.2/download) |
| `webkit2gtk-sys` | 2.0.2 | MIT | Other | [files](licenses/cargo/webkit2gtk-sys-2.0.2/); [source](https://crates.io/api/v1/crates/webkit2gtk-sys/2.0.2/download) |
| `webview2-com` | 0.39.1 | MIT | Windows x64 | [files](licenses/cargo/webview2-com-0.39.1/); [source](https://crates.io/api/v1/crates/webview2-com/0.39.1/download) |
| `webview2-com-macros` | 0.8.1 | MIT | Windows x64 | [files](licenses/cargo/webview2-com-macros-0.8.1/); [source](https://crates.io/api/v1/crates/webview2-com-macros/0.8.1/download) |
| `webview2-com-sys` | 0.39.1 | MIT | Windows x64 | [files](licenses/cargo/webview2-com-sys-0.39.1/); [source](https://crates.io/api/v1/crates/webview2-com-sys/0.39.1/download) |
| `winapi` | 0.3.9 | MIT/Apache-2.0 | Other | [files](licenses/cargo/winapi-0.3.9/); [source](https://crates.io/api/v1/crates/winapi/0.3.9/download) |
| `winapi-i686-pc-windows-gnu` | 0.4.0 | MIT/Apache-2.0 | Other | No packaged license file; [source](https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download) |
| `winapi-util` | 0.1.11 | Unlicense OR MIT | Windows x64 | [files](licenses/cargo/winapi-util-0.1.11/); [source](https://crates.io/api/v1/crates/winapi-util/0.1.11/download) |
| `winapi-x86_64-pc-windows-gnu` | 0.4.0 | MIT/Apache-2.0 | Other | No packaged license file; [source](https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download) |
| `window-vibrancy` | 0.8.1 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/window-vibrancy-0.8.1/); [source](https://crates.io/api/v1/crates/window-vibrancy/0.8.1/download) |
| `windows` | 0.62.2 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-0.62.2/); [source](https://crates.io/api/v1/crates/windows/0.62.2/download) |
| `windows-collections` | 0.3.2 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-collections-0.3.2/); [source](https://crates.io/api/v1/crates/windows-collections/0.3.2/download) |
| `windows-core` | 0.62.2 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-core-0.62.2/); [source](https://crates.io/api/v1/crates/windows-core/0.62.2/download) |
| `windows-future` | 0.3.2 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-future-0.3.2/); [source](https://crates.io/api/v1/crates/windows-future/0.3.2/download) |
| `windows-implement` | 0.60.2 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-implement-0.60.2/); [source](https://crates.io/api/v1/crates/windows-implement/0.60.2/download) |
| `windows-interface` | 0.59.3 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-interface-0.59.3/); [source](https://crates.io/api/v1/crates/windows-interface/0.59.3/download) |
| `windows-link` | 0.2.1 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-link-0.2.1/); [source](https://crates.io/api/v1/crates/windows-link/0.2.1/download) |
| `windows-numerics` | 0.3.1 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-numerics-0.3.1/); [source](https://crates.io/api/v1/crates/windows-numerics/0.3.1/download) |
| `windows-registry` | 0.6.1 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-registry-0.6.1/); [source](https://crates.io/api/v1/crates/windows-registry/0.6.1/download) |
| `windows-result` | 0.4.1 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-result-0.4.1/); [source](https://crates.io/api/v1/crates/windows-result/0.4.1/download) |
| `windows-strings` | 0.5.1 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-strings-0.5.1/); [source](https://crates.io/api/v1/crates/windows-strings/0.5.1/download) |
| `windows-sys` | 0.45.0 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows-sys-0.45.0/); [source](https://crates.io/api/v1/crates/windows-sys/0.45.0/download) |
| `windows-sys` | 0.61.2 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-sys-0.61.2/); [source](https://crates.io/api/v1/crates/windows-sys/0.61.2/download) |
| `windows-targets` | 0.42.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows-targets-0.42.2/); [source](https://crates.io/api/v1/crates/windows-targets/0.42.2/download) |
| `windows-threading` | 0.2.1 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-threading-0.2.1/); [source](https://crates.io/api/v1/crates/windows-threading/0.2.1/download) |
| `windows-version` | 0.1.7 | MIT OR Apache-2.0 | Windows x64 | [files](licenses/cargo/windows-version-0.1.7/); [source](https://crates.io/api/v1/crates/windows-version/0.1.7/download) |
| `windows_aarch64_gnullvm` | 0.42.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows_aarch64_gnullvm-0.42.2/); [source](https://crates.io/api/v1/crates/windows_aarch64_gnullvm/0.42.2/download) |
| `windows_aarch64_msvc` | 0.42.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows_aarch64_msvc-0.42.2/); [source](https://crates.io/api/v1/crates/windows_aarch64_msvc/0.42.2/download) |
| `windows_i686_gnu` | 0.42.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows_i686_gnu-0.42.2/); [source](https://crates.io/api/v1/crates/windows_i686_gnu/0.42.2/download) |
| `windows_i686_msvc` | 0.42.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows_i686_msvc-0.42.2/); [source](https://crates.io/api/v1/crates/windows_i686_msvc/0.42.2/download) |
| `windows_x86_64_gnu` | 0.42.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows_x86_64_gnu-0.42.2/); [source](https://crates.io/api/v1/crates/windows_x86_64_gnu/0.42.2/download) |
| `windows_x86_64_gnullvm` | 0.42.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows_x86_64_gnullvm-0.42.2/); [source](https://crates.io/api/v1/crates/windows_x86_64_gnullvm/0.42.2/download) |
| `windows_x86_64_msvc` | 0.42.2 | MIT OR Apache-2.0 | Other | [files](licenses/cargo/windows_x86_64_msvc-0.42.2/); [source](https://crates.io/api/v1/crates/windows_x86_64_msvc/0.42.2/download) |
| `winnow` | 0.5.40 | MIT | Other | [files](licenses/cargo/winnow-0.5.40/); [source](https://crates.io/api/v1/crates/winnow/0.5.40/download) |
| `winnow` | 1.0.5 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/winnow-1.0.5/); [source](https://crates.io/api/v1/crates/winnow/1.0.5/download) |
| `winreg` | 0.56.0 | MIT | Windows x64 | [files](licenses/cargo/winreg-0.56.0/); [source](https://crates.io/api/v1/crates/winreg/0.56.0/download) |
| `wit-bindgen` | 0.57.1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | Other | [files](licenses/cargo/wit-bindgen-0.57.1/); [source](https://crates.io/api/v1/crates/wit-bindgen/0.57.1/download) |
| `writeable` | 0.6.4 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/writeable-0.6.4/); [source](https://crates.io/api/v1/crates/writeable/0.6.4/download) |
| `wry` | 0.57.2 | Apache-2.0 OR MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/wry-0.57.2/); [source](https://crates.io/api/v1/crates/wry/0.57.2/download) |
| `x11` | 2.21.0 | MIT | Other | [files](licenses/cargo/x11-2.21.0/); [source](https://crates.io/api/v1/crates/x11/2.21.0/download) |
| `x11-dl` | 2.21.0 | MIT | Other | [files](licenses/cargo/x11-dl-2.21.0/); [source](https://crates.io/api/v1/crates/x11-dl/2.21.0/download) |
| `yoke` | 0.8.4 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/yoke-0.8.4/); [source](https://crates.io/api/v1/crates/yoke/0.8.4/download) |
| `yoke-derive` | 0.8.4 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/yoke-derive-0.8.4/); [source](https://crates.io/api/v1/crates/yoke-derive/0.8.4/download) |
| `zbus` | 5.19.0 | MIT | Other | [files](licenses/cargo/zbus-5.19.0/); [source](https://crates.io/api/v1/crates/zbus/5.19.0/download) |
| `zbus_macros` | 5.19.0 | MIT | Other | [files](licenses/cargo/zbus_macros-5.19.0/); [source](https://crates.io/api/v1/crates/zbus_macros/5.19.0/download) |
| `zbus_names` | 4.3.5 | MIT | Other | [files](licenses/cargo/zbus_names-4.3.5/); [source](https://crates.io/api/v1/crates/zbus_names/4.3.5/download) |
| `zcheapstr` | 1.2.0 | MIT | Other | [files](licenses/cargo/zcheapstr-1.2.0/); [source](https://crates.io/api/v1/crates/zcheapstr/1.2.0/download) |
| `zerofrom` | 0.1.8 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/zerofrom-0.1.8/); [source](https://crates.io/api/v1/crates/zerofrom/0.1.8/download) |
| `zerofrom-derive` | 0.1.8 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/zerofrom-derive-0.1.8/); [source](https://crates.io/api/v1/crates/zerofrom-derive/0.1.8/download) |
| `zerotrie` | 0.2.5 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/zerotrie-0.2.5/); [source](https://crates.io/api/v1/crates/zerotrie/0.2.5/download) |
| `zerovec` | 0.11.9 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/zerovec-0.11.9/); [source](https://crates.io/api/v1/crates/zerovec/0.11.9/download) |
| `zerovec-derive` | 0.11.7 | Unicode-3.0 | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/zerovec-derive-0.11.7/); [source](https://crates.io/api/v1/crates/zerovec-derive/0.11.7/download) |
| `zlib-rs` | 0.6.8 | Zlib | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/zlib-rs-0.6.8/); [source](https://crates.io/api/v1/crates/zlib-rs/0.6.8/download) |
| `zmij` | 1.0.23 | MIT | Windows x64, macOS ARM64, macOS x64 | [files](licenses/cargo/zmij-1.0.23/); [source](https://crates.io/api/v1/crates/zmij/1.0.23/download) |
| `zvariant` | 5.15.1 | MIT | Other | [files](licenses/cargo/zvariant-5.15.1/); [source](https://crates.io/api/v1/crates/zvariant/5.15.1/download) |
| `zvariant_derive` | 5.15.1 | MIT | Other | [files](licenses/cargo/zvariant_derive-5.15.1/); [source](https://crates.io/api/v1/crates/zvariant_derive/5.15.1/download) |
| `zvariant_utils` | 4.2.0 | MIT | Other | [files](licenses/cargo/zvariant_utils-4.2.0/); [source](https://crates.io/api/v1/crates/zvariant_utils/4.2.0/download) |

## Audit limits

No GPL-only dependency was found in the locked Cargo or npm metadata. The all-target lock includes `r-efi` with `MIT OR Apache-2.0 OR LGPL-2.1-or-later`, so LGPL is an alternative rather than the only offered license. This audit does not certify legal rights in project-owned contributions or replace a check of the final installer contents. Re-run the inventory after lockfile updates.

Cargo packages with metadata and source links but no retained standalone license text (outside all three intended binary target graphs): `libappindicator-sys 0.9.0`, `r-efi 5.3.0`, `r-efi 6.0.0`, `winapi-i686-pc-windows-gnu 0.4.0`, `winapi-x86_64-pc-windows-gnu 0.4.0`.
