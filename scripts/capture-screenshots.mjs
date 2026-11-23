// Run the dev server on 1421, then run this script with Playwright available.
// An optional PLAYWRIGHT_MODULE absolute path keeps screenshot tooling outside
// the application's dependencies. No native Tauri behavior is exercised here.
import { existsSync } from 'node:fs';
import { mkdir, readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const localBrowsers = path.join(root, '.home', 'ms-playwright');
if (!process.env.PLAYWRIGHT_BROWSERS_PATH && existsSync(localBrowsers)) {
  process.env.PLAYWRIGHT_BROWSERS_PATH = localBrowsers;
}

const modulePath = process.env.PLAYWRIGHT_MODULE;
const { chromium } = await import(modulePath ? pathToFileURL(modulePath).href : 'playwright');
const output = path.join(root, 'docs', 'screenshots');
await mkdir(output, { recursive: true });

const previewUrl = (process.env.PREVIEW_URL ?? 'http://127.0.0.1:1421').replace(/\/$/, '');
const distDir = path.join(root, 'dist');
const mimeTypes = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.ico': 'image/x-icon',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.json': 'application/json; charset=utf-8',
};
const expectedThemes = [
  'mist-blue',
  'sage-green',
  'lavender',
  'dusty-rose',
  'champagne',
  'sea-glass',
  'smoky-slate',
];

async function attachDistRoute(page) {
  if (!existsSync(path.join(distDir, 'index.html'))) {
    throw new Error('Loopback preview is blocked and dist/index.html is missing; run `npm run build` first.');
  }
  await page.route(`${previewUrl}/**`, async (route) => {
    const reqUrl = new URL(route.request().url());
    const pathname = reqUrl.pathname === '/' ? '/index.html' : reqUrl.pathname;
    const candidate = path.resolve(distDir, `.${pathname}`);
    if (!candidate.startsWith(distDir) || !existsSync(candidate)) {
      await route.fulfill({ status: 404, body: 'Not found' });
      return;
    }
    const body = await readFile(candidate);
    const ext = path.extname(candidate).toLowerCase();
    await route.fulfill({
      status: 200,
      contentType: mimeTypes[ext] ?? 'application/octet-stream',
      body,
    });
  });
}

const browser = await chromium.launch({ headless: true, args: ['--no-sandbox'] });
const context = await browser.newContext({ deviceScaleFactor: 2, timezoneId: 'Asia/Tokyo', reducedMotion: 'reduce' });
const page = await context.newPage();
const errors = [];
page.on('pageerror', error => errors.push(String(error)));
const frozen = new Date('2026-11-14T10:08:00+09:00');
await page.clock.install({ time: frozen });
await page.clock.pauseAt(frozen);
const cases = [
  { name: 'themes', view: 'gallery', width: 1600, height: 700 },
  { name: 'timer', view: 'showcase&panel=timer', width: 760, height: 700 },
  { name: 'timer-popup', view: 'showcase&panel=timer-popup', width: 900, height: 700 },
  { name: 'history-popup', view: 'showcase&panel=history-popup', width: 1100, height: 700 },
  { name: 'settings', view: 'showcase&panel=settings', width: 800, height: 1650 },
  { name: 'notification', view: 'showcase&panel=notification', width: 960, height: 520 },
];
let sourceMode = 'live-dev-server';
try {
  for (const language of ['en', 'ja']) {
    for (const item of cases) {
      await page.setViewportSize({ width: item.width, height: item.height });
      const targetUrl = `${previewUrl}/?view=${item.view}&lang=${language}`;
      try {
        await page.goto(targetUrl, { waitUntil: 'load' });
      } catch (error) {
        if (sourceMode === 'live-dev-server' && String(error).includes('ERR_ACCESS_DENIED')) {
          sourceMode = 'dist-route-fallback';
          console.log(`Loopback TCP denied by sandbox (${previewUrl}); serving compiled Vite production bundle from dist/ via Playwright route.`);
          await attachDistRoute(page);
          await page.goto(targetUrl, { waitUntil: 'load' });
        } else {
          throw error;
        }
      }
      await page.locator(item.name === 'settings' ? '.settings-page' : item.name === 'notification' ? '.break-card' : item.name === 'history-popup' ? '.history-popup' : '.capsule').first().waitFor();
      await page.evaluate(() => document.fonts.ready);
      const measurement = await page.evaluate(() => {
        const rectsWithinViewport = (selector) => [...document.querySelectorAll(selector)].every((el) => {
          const r = el.getBoundingClientRect();
          return r.width > 0 && r.height > 0 && r.left >= 0 && r.top >= 0 && r.right <= innerWidth && r.bottom <= innerHeight;
        });
        return {
          horizontalOverflow: document.documentElement.scrollWidth > innerWidth,
          verticalOverflow: document.documentElement.scrollHeight > innerHeight,
          capsules: document.querySelectorAll('.capsule').length,
          themes: [...document.querySelectorAll('.release-theme')].map(el => el.getAttribute('data-theme')),
          captions: [...document.querySelectorAll('.release-theme h2')].map(el => el.textContent?.trim() ?? ''),
          captionsInBounds: rectsWithinViewport('.release-theme h2'),
          capsulesInBounds: rectsWithinViewport('.capsule'),
          remaining: [...document.querySelectorAll('.time')].map(el => el.textContent),
          clocks: [...document.querySelectorAll('.clock-time')].map(el => el.textContent),
          miniHistories: document.querySelectorAll('.history-mini').length,
          miniHistoryRows: document.querySelectorAll('.history-mini .history-mini-row').length,
          litMiniCells: document.querySelectorAll('.history-mini-cell[data-level]:not([data-level="0"])').length,
          popupRows: document.querySelectorAll('.history-popup .history-popup-row').length,
          litPopupCells: document.querySelectorAll('.history-popup-cell[data-level]:not([data-level="0"])').length,
          dateFitsPill: [...document.querySelectorAll('.capsule')].every((capsule) => {
            const c = capsule.getBoundingClientRect();
            const r = c.width / 2;
            const parts = [...capsule.querySelectorAll('.clock-date-line > *')];
            if (!parts.length) return false;
            const left = Math.min(...parts.map(el => el.getBoundingClientRect().left));
            const right = Math.max(...parts.map(el => el.getBoundingClientRect().right));
            const top = Math.min(...parts.map(el => el.getBoundingClientRect().top));
            const bottom = Math.max(...parts.map(el => el.getBoundingClientRect().bottom));
            const inkTop = top + (bottom - top) * 0.25;
            const dy = Math.max(0, r - (inkTop - c.top));
            const chordHalf = Math.sqrt(Math.max(0, r * r - dy * dy));
            const center = c.left + r;
            return left >= center - chordHalf && right <= center + chordHalf;
          }),
          popupFooterInBounds: (() => {
            const popup = document.querySelector('.history-popup');
            const footer = document.querySelector('.history-popup-footer');
            if (!popup || !footer) return true;
            const p = popup.getBoundingClientRect();
            const f = footer.getBoundingClientRect();
            return f.bottom <= p.bottom - 1 && f.top >= p.top;
          })(),
        };
      });
      if (measurement.horizontalOverflow || measurement.verticalOverflow) throw new Error(`Clipping in ${item.name}-${language}: ${JSON.stringify(measurement)}`);
      if (!measurement.capsulesInBounds && measurement.capsules > 0) throw new Error(`Capsule clipped in ${item.name}-${language}: ${JSON.stringify(measurement)}`);
      if (measurement.capsules > 0 && !measurement.dateFitsPill) throw new Error(`Clock date clipped by capsule pill in ${item.name}-${language}: ${JSON.stringify(measurement)}`);
      if (!measurement.popupFooterInBounds) throw new Error(`History popup footer clipped in ${item.name}-${language}: ${JSON.stringify(measurement)}`);
      if (item.name === 'themes') {
        if (
          measurement.capsules !== 7 ||
          JSON.stringify(measurement.themes) !== JSON.stringify(expectedThemes) ||
          measurement.captions.length !== 7 ||
          measurement.captions.some(text => !text) ||
          !measurement.captionsInBounds ||
          measurement.remaining.some(value => value !== '24:32') ||
          measurement.clocks.some(value => value !== '10:08:00') ||
          measurement.miniHistories !== 7 ||
          measurement.miniHistoryRows !== 21 ||
          measurement.litMiniCells < 7
        ) {
          throw new Error(`Inconsistent gallery (${language}): ${JSON.stringify(measurement)}`);
        }
        const selector = page.locator('.release-gallery-selector .selector');
        if (await selector.count() !== 1 || await selector.locator('button').count() !== 7 || await selector.locator('button[aria-pressed="true"]').innerText().then(text => !text.startsWith('25'))) {
          throw new Error('Gallery must show exactly one preset selector with 25 minutes selected');
        }
      }
      if (item.name === 'history-popup' && (measurement.popupRows !== 7 || measurement.litPopupCells < 10)) {
        throw new Error(`Incomplete history popup (${language}): ${JSON.stringify(measurement)}`);
      }
      await page.screenshot({ path: path.join(output, `${item.name}-${language}.png`), animations: 'disabled' });
      console.log(`${item.name}-${language}.png ${item.width * 2}×${item.height * 2} [${sourceMode}] ${JSON.stringify({ capsules: measurement.capsules, miniHistories: measurement.miniHistories, litMiniCells: measurement.litMiniCells, popupRows: measurement.popupRows, litPopupCells: measurement.litPopupCells })}`);
    }
  }
  if (errors.length) throw new Error(errors.join('\n'));
} finally {
  await browser.close();
}
