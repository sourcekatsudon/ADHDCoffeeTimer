import { expect, it, vi } from 'vitest';
import type { Snapshot } from './model';

vi.mock('@tauri-apps/api/core', () => ({ isTauri: () => false, invoke: vi.fn() }));

it('boots on a WebView without structuredClone and isolates subscriber state', async () => {
  vi.stubGlobal('structuredClone', undefined);
  let off: (() => void) | undefined;
  try {
    const bridge = await import('./bridge');
    const snapshots: Snapshot[] = [];
    off = await bridge.subscribe((s) => snapshots.push(s));
    snapshots[0].settings.minutes = 90;
    await bridge.command('toggle');
    expect(snapshots[1].settings.minutes).toBe(25);
    expect(snapshots[1].status).toBe('running');
  } finally {
    off?.();
    vi.unstubAllGlobals();
  }
});

it('native timer ticks preserve large history and settings references', async () => {
  const { initial } = await import('./model');
  const { applyTimerTick } = await import('./bridge');
  const history = [{ start: 900, seconds: 17 }];
  const previous: Snapshot = { ...initial, history };
  const next = applyTimerTick(previous, {
    remainingSecs: 1499,
    status: 'running',
    clockMs: 1790000000000,
  });
  expect(next.remainingSecs).toBe(1499);
  expect(next.status).toBe('running');
  expect(next.clockMs).toBe(1790000000000);
  expect(next.history).toBe(history);
  expect(next.settings).toBe(previous.settings);
});
