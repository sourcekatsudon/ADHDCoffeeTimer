// Verifies README screenshot references, high-DPI PNG decoding, seven-theme
// visual distinctness, edge-sharpness gains over legacy WebP assets, and
// generates 100% zoom cropped before/after comparisons under artifacts/.
import { existsSync } from 'node:fs';
import { mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const localBrowsers = path.join(root, '.home', 'ms-playwright');
if (!process.env.PLAYWRIGHT_BROWSERS_PATH && existsSync(localBrowsers)) {
  process.env.PLAYWRIGHT_BROWSERS_PATH = localBrowsers;
}

const modulePath = process.env.PLAYWRIGHT_MODULE;
const { chromium } = await import(modulePath ? pathToFileURL(modulePath).href : 'playwright');

const screenshotsDir = path.join(root, 'docs', 'screenshots');
const artifactsDir = path.join(root, 'artifacts', 'screenshot-verification');
await mkdir(artifactsDir, { recursive: true });

const PNG_SIG = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
function readPngInfo(buf) {
  if (buf.length < 24 || !buf.subarray(0, 8).equals(PNG_SIG)) {
    throw new Error('Invalid PNG signature');
  }
  return {
    width: buf.readUInt32BE(16),
    height: buf.readUInt32BE(20),
    bitDepth: buf[24],
    colorType: buf[25],
  };
}

function extractMarkdownImages(markdown) {
  const matches = [];
  const pattern = /!\[([^\]]*)\]\(([^)]+)\)/g;
  let match;
  while ((match = pattern.exec(markdown)) !== null) {
    matches.push({ alt: match[1], target: match[2] });
  }
  return matches;
}

// 1. Verify README.md and docs/readme/ja.md references and truthful captions.
const docSpecs = [
  {
    file: path.join(root, 'README.md'),
    expectedGallery: 'docs/screenshots/themes-en.png',
    expectedGalleryCount: 2,
    requiredPopups: ['docs/screenshots/timer-popup-en.png', 'docs/screenshots/history-popup-en.png'],
    disclaimerPattern: /browser previews/i,
  },
  {
    file: path.join(root, 'docs', 'readme', 'ja.md'),
    expectedGallery: '../screenshots/themes-ja.png',
    expectedGalleryCount: 1,
    requiredPopups: ['../screenshots/timer-popup-ja.png', '../screenshots/history-popup-ja.png'],
    disclaimerPattern: /ブラウザで描画したプレビュー/,
  },
];

for (const spec of docSpecs) {
  const content = await readFile(spec.file, 'utf8');
  const relDoc = path.relative(root, spec.file);
  if (!spec.disclaimerPattern.test(content)) {
    throw new Error(`${relDoc} is missing truthful browser-preview disclaimer`);
  }
  const images = extractMarkdownImages(content).filter(img => !img.target.startsWith('https://img.shields.io/'));
  const webpRefs = images.filter(img => img.target.endsWith('.webp'));
  if (webpRefs.length > 0) {
    throw new Error(`${relDoc} still references overcompressed WebP screenshot(s): ${JSON.stringify(webpRefs)}`);
  }
  const galleryRefs = images.filter(img => img.target === spec.expectedGallery);
  if (galleryRefs.length !== spec.expectedGalleryCount) {
    throw new Error(`${relDoc} expected ${spec.expectedGalleryCount} reference(s) to ${spec.expectedGallery}, found ${galleryRefs.length}`);
  }
  for (const popup of spec.requiredPopups) {
    if (!images.some(img => img.target === popup)) {
      throw new Error(`${relDoc} missing popup screenshot reference ${popup}`);
    }
  }
  for (const img of images) {
    if (/^https?:\/\//i.test(img.target)) {
      throw new Error(`${relDoc} contains external screenshot hotlink: ${img.target}`);
    }
    const resolved = path.resolve(path.dirname(spec.file), img.target);
    if (!existsSync(resolved)) {
      throw new Error(`${relDoc} broken screenshot reference: ${img.target}`);
    }
  }
  console.log(`PASS markdown check: ${relDoc} (${images.length} local PNG screenshot references)`);
}

// 2. Verify high-DPI PNG metadata & decode all assets in Chromium for pixel inspection.
const expectedAssets = [
  { name: 'themes-en.png', legacy: 'themes-en.webp', width: 3200, height: 1400, isGallery: true },
  { name: 'themes-ja.png', legacy: 'themes-ja.webp', width: 3200, height: 1400, isGallery: true },
  { name: 'timer-popup-en.png', legacy: 'timer-popup-en.webp', width: 1800, height: 1400, isGallery: false },
  { name: 'timer-popup-ja.png', legacy: 'timer-popup-ja.webp', width: 1800, height: 1400, isGallery: false },
  { name: 'history-popup-en.png', legacy: 'history-popup-en.webp', width: 2200, height: 1400, isGallery: false },
  { name: 'history-popup-ja.png', legacy: 'history-popup-ja.webp', width: 2200, height: 1400, isGallery: false },
  { name: 'timer-en.png', legacy: null, width: 1520, height: 1400, isGallery: false },
  { name: 'timer-ja.png', legacy: null, width: 1520, height: 1400, isGallery: false },
  { name: 'settings-en.png', legacy: null, width: 1600, height: 3300, isGallery: false },
  { name: 'settings-ja.png', legacy: null, width: 1600, height: 3300, isGallery: false },
  { name: 'notification-en.png', legacy: null, width: 1920, height: 1040, isGallery: false },
  { name: 'notification-ja.png', legacy: null, width: 1920, height: 1040, isGallery: false },
];

const browser = await chromium.launch({ headless: true, args: ['--no-sandbox'] });
const context = await browser.newContext({ deviceScaleFactor: 1 });
const page = await context.newPage();

async function analyzeImage(filePath) {
  const buf = await readFile(filePath);
  const ext = path.extname(filePath).toLowerCase();
  const mime = ext === '.webp' ? 'image/webp' : 'image/png';
  const dataUrl = `data:${mime};base64,${buf.toString('base64')}`;
  return await page.evaluate(async (url) => {
    const img = new Image();
    img.src = url;
    await img.decode();
    const canvas = document.createElement('canvas');
    canvas.width = img.naturalWidth;
    canvas.height = img.naturalHeight;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(img, 0, 0);
    const { data, width, height } = ctx.getImageData(0, 0, canvas.width, canvas.height);
    // Compute normalized high-frequency edge sharpness across the center band.
    let gradSum = 0;
    let count = 0;
    const x0 = Math.floor(width * 0.08);
    const x1 = Math.floor(width * 0.92);
    const y0 = Math.floor(height * 0.12);
    const y1 = Math.floor(height * 0.88);
    const lum = (x, y) => {
      const i = (y * width + x) * 4;
      return 0.299 * data[i] + 0.587 * data[i + 1] + 0.114 * data[i + 2];
    };
    for (let y = y0; y < y1 - 1; y += 2) {
      for (let x = x0; x < x1 - 1; x += 2) {
        const dx = Math.abs(lum(x + 1, y) - lum(x, y));
        const dy = Math.abs(lum(x, y + 1) - lum(x, y));
        gradSum += dx + dy;
        count++;
      }
    }
    return {
      width,
      height,
      meanGradient: Number((gradSum / Math.max(1, count)).toFixed(3)),
    };
  }, dataUrl);
}

async function analyzeGalleryThemes(filePath) {
  const buf = await readFile(filePath);
  const dataUrl = `data:image/png;base64,${buf.toString('base64')}`;
  return await page.evaluate(async (url) => {
    const img = new Image();
    img.src = url;
    await img.decode();
    const canvas = document.createElement('canvas');
    canvas.width = img.naturalWidth;
    canvas.height = img.naturalHeight;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(img, 0, 0);
    // Sample the upper body of each of the 7 evenly spaced capsules (after the preset selector).
    const startX = Math.round(canvas.width * 0.175);
    const stepX = Math.round(canvas.width * 0.119);
    const sampleY = Math.round(canvas.height * 0.34);
    const swatches = [];
    for (let i = 0; i < 7; i++) {
      const cx = startX + i * stepX;
      const patch = ctx.getImageData(cx - 20, sampleY - 20, 40, 40).data;
      let r = 0, g = 0, b = 0;
      const n = patch.length / 4;
      for (let j = 0; j < patch.length; j += 4) {
        r += patch[j];
        g += patch[j + 1];
        b += patch[j + 2];
      }
      swatches.push({
        index: i + 1,
        r: Math.round(r / n),
        g: Math.round(g / n),
        b: Math.round(b / n),
      });
    }
    let minPairwiseDelta = Infinity;
    for (let i = 0; i < swatches.length; i++) {
      for (let j = i + 1; j < swatches.length; j++) {
        const delta = Math.hypot(
          swatches[i].r - swatches[j].r,
          swatches[i].g - swatches[j].g,
          swatches[i].b - swatches[j].b,
        );
        if (delta < minPairwiseDelta) minPairwiseDelta = delta;
      }
    }
    return { swatches, minPairwiseDelta: Number(minPairwiseDelta.toFixed(2)) };
  }, dataUrl);
}

async function createBeforeAfterCrop({ beforePath, afterPath, cropNorm, outPath, title }) {
  const beforeUrl = `data:image/webp;base64,${(await readFile(beforePath)).toString('base64')}`;
  const afterUrl = `data:image/png;base64,${(await readFile(afterPath)).toString('base64')}`;
  const pngBase64 = await page.evaluate(async ({ beforeUrl, afterUrl, cropNorm, title }) => {
    const load = async (src) => {
      const img = new Image();
      img.src = src;
      await img.decode();
      return img;
    };
    const [before, after] = await Promise.all([load(beforeUrl), load(afterUrl)]);
    const ax = Math.round(after.naturalWidth * cropNorm.x);
    const ay = Math.round(after.naturalHeight * cropNorm.y);
    const aw = Math.round(after.naturalWidth * cropNorm.w);
    const ah = Math.round(after.naturalHeight * cropNorm.h);

    const bx = Math.round(before.naturalWidth * cropNorm.x);
    const by = Math.round(before.naturalHeight * cropNorm.y);
    const bw = Math.round(before.naturalWidth * cropNorm.w);
    const bh = Math.round(before.naturalHeight * cropNorm.h);

    const pad = 24;
    const header = 52;
    const gap = 24;
    const canvas = document.createElement('canvas');
    canvas.width = pad * 2 + aw * 2 + gap;
    canvas.height = header + ah + pad;
    const ctx = canvas.getContext('2d');
    ctx.fillStyle = '#18202c';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = '#eef2f7';
    ctx.font = '600 18px -apple-system, BlinkMacSystemFont, sans-serif';
    ctx.fillText(`BEFORE (legacy WebP ${before.naturalWidth}x${before.naturalHeight}, upscaled to match) — ${title}`, pad, 34);
    ctx.fillText(`AFTER (new 2x PNG ${after.naturalWidth}x${after.naturalHeight}, 100% native pixels)`, pad + aw + gap, 34);

    // Draw legacy crop scaled up to the exact same visual area to expose real compression loss.
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(before, bx, by, bw, bh, pad, header, aw, ah);
    // Draw new PNG crop at exact 1:1 native pixels (100% zoom).
    ctx.drawImage(after, ax, ay, aw, ah, pad + aw + gap, header, aw, ah);
    return canvas.toDataURL('image/png').replace(/^data:image\/png;base64,/, '');
  }, { beforeUrl, afterUrl, cropNorm, title });
  await writeFile(outPath, Buffer.from(pngBase64, 'base64'));
}

const report = [];
try {
  for (const asset of expectedAssets) {
    const fullPath = path.join(screenshotsDir, asset.name);
    const buf = await readFile(fullPath);
    const pngInfo = readPngInfo(buf);
    if (pngInfo.width !== asset.width || pngInfo.height !== asset.height) {
      throw new Error(`${asset.name} dimensions ${pngInfo.width}x${pngInfo.height} !== expected ${asset.width}x${asset.height}`);
    }
    const decoded = await analyzeImage(fullPath);
    if (decoded.width !== asset.width || decoded.height !== asset.height) {
      throw new Error(`${asset.name} Chromium decode mismatch`);
    }
    const entry = {
      asset: asset.name,
      afterBytes: (await stat(fullPath)).size,
      afterDims: `${decoded.width}x${decoded.height}`,
      afterGradient: decoded.meanGradient,
    };
    if (asset.legacy) {
      const legacyPath = path.join(screenshotsDir, asset.legacy);
      const legacyInfo = await analyzeImage(legacyPath);
      entry.beforeAsset = asset.legacy;
      entry.beforeBytes = (await stat(legacyPath)).size;
      entry.beforeDims = `${legacyInfo.width}x${legacyInfo.height}`;
      entry.beforeGradient = legacyInfo.meanGradient;
      if (decoded.width < legacyInfo.width * 2) {
        throw new Error(`${asset.name} width ${decoded.width} is not at least 2x legacy ${legacyInfo.width}`);
      }
    }
    if (asset.isGallery) {
      const themeCheck = await analyzeGalleryThemes(fullPath);
      entry.minThemeColorDelta = themeCheck.minPairwiseDelta;
      if (themeCheck.minPairwiseDelta < 4) {
        throw new Error(`${asset.name} theme capsules not visually distinct: ${JSON.stringify(themeCheck)}`);
      }
    }
    report.push(entry);
    console.log(`PASS image check: ${JSON.stringify(entry)}`);
  }

  // 3. Generate 100% zoom before/after comparison crops for visual review.
  const crops = [
    {
      beforePath: path.join(screenshotsDir, 'themes-en.webp'),
      afterPath: path.join(screenshotsDir, 'themes-en.png'),
      cropNorm: { x: 0.04, y: 0.07, w: 0.34, h: 0.87 },
      outPath: path.join(artifactsDir, 'compare-themes-en-100pct.png'),
      title: 'EN Gallery (Selector + Mist Blue + Sage Green)',
    },
    {
      beforePath: path.join(screenshotsDir, 'themes-ja.webp'),
      afterPath: path.join(screenshotsDir, 'themes-ja.png'),
      cropNorm: { x: 0.04, y: 0.07, w: 0.34, h: 0.87 },
      outPath: path.join(artifactsDir, 'compare-themes-ja-100pct.png'),
      title: 'JA Gallery (Selector + ミストブルー + セージグリーン)',
    },
    {
      beforePath: path.join(screenshotsDir, 'history-popup-en.webp'),
      afterPath: path.join(screenshotsDir, 'history-popup-en.png'),
      cropNorm: { x: 0.35, y: 0.34, w: 0.48, h: 0.34 },
      outPath: path.join(artifactsDir, 'compare-history-popup-en-100pct.png'),
      title: 'EN 7-Day History Popup',
    },
    {
      beforePath: path.join(screenshotsDir, 'history-popup-ja.webp'),
      afterPath: path.join(screenshotsDir, 'history-popup-ja.png'),
      cropNorm: { x: 0.35, y: 0.34, w: 0.48, h: 0.34 },
      outPath: path.join(artifactsDir, 'compare-history-popup-ja-100pct.png'),
      title: 'JA 7-Day History Popup',
    },
  ];
  for (const crop of crops) {
    await createBeforeAfterCrop(crop);
    console.log(`WROTE 100% zoom crop comparison: ${path.relative(root, crop.outPath)}`);
  }

  await writeFile(
    path.join(artifactsDir, 'report.json'),
    JSON.stringify({ verifiedAt: new Date().toISOString(), assets: report }, null, 2),
  );
  console.log(`ALL ${report.length} screenshot checks passed.`);
} finally {
  await browser.close();
}
