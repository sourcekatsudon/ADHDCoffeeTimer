// Run the dev server on 1421, then run this script with Playwright available.
// An optional PLAYWRIGHT_MODULE absolute path keeps screenshot tooling outside
// the application's dependencies. No native Tauri behavior is exercised here.
import { mkdir } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const modulePath = process.env.PLAYWRIGHT_MODULE;
const { chromium } = await import(modulePath ? pathToFileURL(modulePath).href : 'playwright');
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const output = path.join(root, 'docs', 'screenshots');
await mkdir(output, { recursive: true });
const browser = await chromium.launch({ channel: 'chrome', headless: true });
const context = await browser.newContext({ deviceScaleFactor: 2, timezoneId: 'Asia/Tokyo', reducedMotion: 'reduce' });
const page = await context.newPage();
const errors = [];
page.on('pageerror', error => errors.push(String(error)));
const frozen = new Date('2026-11-14T10:08:00+09:00');
await page.clock.install({ time: frozen });
await page.clock.pauseAt(frozen);
const cases = [
  { name: 'themes', view: 'gallery', width: 1400, height: 620 },
  { name: 'timer', view: 'showcase&panel=timer', width: 720, height: 620 },
  { name: 'settings', view: 'showcase&panel=settings', width: 760, height: 1120 },
  { name: 'notification', view: 'showcase&panel=notification', width: 960, height: 520 },
];
try {
  for (const language of ['en', 'ja']) {
    for (const item of cases) {
      await page.setViewportSize({ width: item.width, height: item.height });
      await page.goto(`${process.env.PREVIEW_URL ?? 'http://127.0.0.1:1421'}/?view=${item.view}&lang=${language}`);
      await page.locator(item.name === 'settings' ? '.settings-page' : item.name === 'notification' ? '.break-card' : '.capsule').first().waitFor();
      await page.evaluate(() => document.fonts.ready);
      const measurement = await page.evaluate(() => ({
        horizontalOverflow: document.documentElement.scrollWidth > innerWidth,
        verticalOverflow: document.documentElement.scrollHeight > innerHeight,
        capsules: document.querySelectorAll('.capsule').length,
        remaining: [...document.querySelectorAll('.time')].map(el => el.textContent),
        clocks: [...document.querySelectorAll('.clock-time')].map(el => el.textContent),
      }));
      if (measurement.horizontalOverflow || measurement.verticalOverflow) throw new Error(`Clipping in ${item.name}-${language}: ${JSON.stringify(measurement)}`);
      if (item.name === 'themes' && (measurement.capsules !== 7 || measurement.remaining.some(value => value !== '24:32') || measurement.clocks.some(value => value !== '10:08:00'))) throw new Error(`Inconsistent gallery: ${JSON.stringify(measurement)}`);
      await page.screenshot({ path: path.join(output, `${item.name}-${language}.png`), animations: 'disabled' });
      console.log(`${item.name}-${language}.png ${item.width * 2}×${item.height * 2} ${JSON.stringify(measurement)}`);
    }
  }
  if (errors.length) throw new Error(errors.join('\n'));
} finally {
  await browser.close();
}
