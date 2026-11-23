# README screenshot verification

The gallery and popup assets are lossless PNG captures of the real Vite/React browser presentation views, not native desktop screenshots. The English and Japanese galleries show seven themes with the same 25-minute selection, 24:32 countdown, 10:08:00 clock and three-day activity cells.

| Asset | Previous WebP | New PNG |
| --- | --- | --- |
| English gallery | 760 x 332, 5,636 bytes | 3200 x 1400, 3,608,141 bytes |
| Japanese gallery | 760 x 332, 5,544 bytes | 3200 x 1400, 3,630,162 bytes |

[English 100% crop comparison](comparisons/compare-themes-en-100pct.png), [Japanese comparison](comparisons/compare-themes-ja-100pct.png), [English history popup](comparisons/compare-history-popup-en-100pct.png), [Japanese history popup](comparisons/compare-history-popup-ja-100pct.png), and [decoded-image report](comparisons/report.json) record the inspected evidence. Legacy images are explicitly upscaled for the side-by-side comparison; the after crop uses native pixels. File size or a gradient metric alone is not proof of visual quality. The text, ring edges, theme colors and activity cells were also visually reviewed.

## Reproduction

Use Node 24 and Playwright 1.65.1. From the repository root:

```sh
npm ci
npm install --no-save playwright@1.65.1
export PLAYWRIGHT_BROWSERS_PATH="$PWD/.home/ms-playwright"
npm exec -- playwright install chromium
npm run build
npm run dev -- --port 1421
```

Keep that server running and use another terminal with the same browser-path environment:

```sh
node scripts/capture-screenshots.mjs
node scripts/verify-screenshots.mjs
npm test
npm run build
```

The capture uses a 2x device scale, fixed clock, seven-theme/time checks, font/glyph and popup-content bounds checks. The verifier decodes all 12 PNGs in Chromium and checks local Markdown links, dimensions, color distinction and crop evidence. If a terminal sandbox blocks loopback access, the capture script can route the actual freshly compiled dist bundle into Playwright; this is still a real application render and is not a native OS test. The parent also verifies with a live Vite server.

Typography corrections are scoped to `.release-gallery` and `.release-showcase`, the browser-only presentation views. The native desktop typography rules retain their original values. No merge, release or deployment is implied by this screenshot PR.
