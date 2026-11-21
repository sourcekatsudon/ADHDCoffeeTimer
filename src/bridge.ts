import { invoke, isTauri } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { initial, type Snapshot, type Settings } from './model';
import type { ThemeId } from './themes';

export const native = isTauri();
// Older WKWebView versions do not provide structuredClone. Keep startup
// compatible even when the browser-only preview is never used.
function cloneSnapshot(s: Snapshot): Snapshot {
  return {
    ...s,
    settings: { ...s.settings, position: s.settings.position ? [...s.settings.position] : null },
  };
}
let preview: Snapshot = cloneSnapshot(initial);
const observers = new Set<(s: Snapshot) => void>();
function emitPreview() {
  observers.forEach((f) => f(cloneSnapshot(preview)));
}
export type TimerTick = Pick<Snapshot, 'remainingSecs' | 'status'> & { clockMs: number };

export function applyTimerTick(previous: Snapshot, tick: TimerTick): Snapshot {
  return { ...previous, ...tick };
}

/** Full history/settings snapshots are infrequent. The 1 Hz clock event only
 * changes lightweight scalar fields, retaining the same history reference. */
export async function subscribe(f: (s: Snapshot) => void) {
  if (native) {
    let latest: Snapshot | null = null;
    const publish = (value: Snapshot) => {
      latest = value;
      f(value);
    };
    const offState = await listen<Snapshot>('state', (e) => publish(e.payload));
    let offTick: (() => void) | undefined;
    try {
      offTick = await listen<TimerTick>('timer-tick', (e) => {
        if (!latest) return;
        const { remainingSecs, status, clockMs } = e.payload;
        publish(applyTimerTick(latest, { remainingSecs, status, clockMs }));
      });
      // get_state also supplies the existing persisted activity on startup.
      if (!latest) publish(await invoke<Snapshot>('get_state'));
      return () => { offTick?.(); offState(); };
    } catch (error) {
      offState();
      offTick?.();
      throw error;
    }
  }
  f(cloneSnapshot(preview));
  observers.add(f);
  return () => {
    observers.delete(f);
  };
}
export async function command(kind: string) {
  if (native) return invoke('action', { kind });
  if (kind === 'toggle') preview.status = preview.status === 'running' ? 'paused' : 'running';
  if (kind === 'reset') preview.remainingSecs = preview.durationSecs;
  emitPreview();
}
export async function preset(minutes: number) {
  if (native) return invoke('set_preset', { minutes });
  preview.settings.minutes = minutes;
  preview.settings.lastManualMinutes = minutes;
  preview.remainingSecs = preview.durationSecs = minutes * 60;
  emitPreview();
}
export async function save(settings: Settings) {
  if (native) return invoke('save_settings', { settings });
  const current = preview.settings;
  const changed = current.defaultMinutes !== settings.defaultMinutes;
  preview.settings = {
    ...settings,
    minutes: changed ? settings.defaultMinutes : current.minutes,
    lastManualMinutes: changed ? null : current.lastManualMinutes,
  };
  if (changed) preview.remainingSecs = preview.durationSecs = settings.defaultMinutes * 60;
  emitPreview();
}
export async function clearHistorySlot(start: number) {
  if (native) return invoke('clear_history_slot', { start });
  preview.history = preview.history.filter(item => item.start !== start);
  emitPreview();
}
export async function setTheme(theme: ThemeId) {
  if (native) return invoke('set_theme', { theme });
  preview.settings.theme = theme;
  emitPreview();
}
export async function updateSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
  if (key === 'theme') return setTheme(value as ThemeId);
  if (native) return invoke('update_setting', { key, value });
  return save({ ...preview.settings, [key]: value });
}
export async function onEvent(event: string, fn: () => void) {
  if (native) return listen(event, fn);
  return () => {};
}
