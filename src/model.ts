export const PRESETS = [5, 10, 15, 25, 30, 60, 90] as const;
import type { ThemeId } from './themes';
import { languageInfo, type Language } from './i18n';
export const TIMER_SCALES = { S: 0.4, M: 0.7, L: 1 } as const;
export type TimerSize = keyof typeof TIMER_SCALES;
export function timerDimensions(size: TimerSize) {
  const scale = TIMER_SCALES[size];
  return { width: 172 * scale, height: 474 * scale, scale };
}
export type Settings = {
  language: Language;
  timerSize: TimerSize;
  minutes: number;
  defaultMinutes: number;
  lastManualMinutes: number | null;
  theme: ThemeId;
  idlePauseSecs: number;
  idleResetSecs: number;
  sound: boolean;
  opacity: number;
  autostart: boolean;
  autoDetect: boolean;
  timerVisible: boolean;
  position: [number, number] | null;
};
export type Snapshot = {
  settings: Settings;
  remainingSecs: number;
  durationSecs: number;
  status: 'waiting' | 'running' | 'away' | 'paused' | 'unavailable';
  completed: number;
  debug: boolean;
  error: string | null;
};
export const initial: Snapshot = {
  settings: {
    language: 'ja',
    timerSize: 'L',
    minutes: 25,
    defaultMinutes: 25,
    lastManualMinutes: null,
    theme: 'mist-blue',
    idlePauseSecs: 90,
    idleResetSecs: 600,
    sound: false,
    opacity: 0.82,
    autostart: true,
    autoDetect: true,
    timerVisible: true,
    position: null,
  },
  remainingSecs: 1500,
  durationSecs: 1500,
  status: 'waiting',
  completed: 0,
  debug: false,
  error: null,
};
export function formatTime(seconds: number) {
  const s = Math.max(0, Math.ceil(seconds));
  return `${String(Math.floor(s / 60)).padStart(2, '0')}:${String(s % 60).padStart(2, '0')}`;
}

export function formatClock(date: Date, language: Language = 'ja') {
  const pad = (n: number) => String(n).padStart(2, '0');
  const locale = languageInfo(language).locale;
  return {
    era:
      language === 'ja'
        ? new Intl.DateTimeFormat('ja-JP-u-ca-japanese', { era: 'long', year: 'numeric' }).format(
            date,
          )
        : '',
    date:
      language === 'ja'
        ? `${date.getFullYear()}/${pad(date.getMonth() + 1)}/${pad(date.getDate())}`
        : new Intl.DateTimeFormat(locale, {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            calendar: 'gregory',
          }).format(date),
    weekday:
      language === 'ja'
        ? ['日', '月', '火', '水', '木', '金', '土'][date.getDay()]
        : new Intl.DateTimeFormat(locale, { weekday: 'short' }).format(date),
    time: `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`,
  };
}
export function nextPreset(current: number, direction: number): number {
  const index = PRESETS.indexOf(current as (typeof PRESETS)[number]);
  return PRESETS[Math.max(0, Math.min(PRESETS.length - 1, index + Math.sign(direction)))];
}
export function anglePreset(x: number, y: number): number {
  const angle = (Math.atan2(y, x) + Math.PI / 2 + Math.PI * 2) % (Math.PI * 2);
  return PRESETS[Math.min(6, Math.round((angle / (Math.PI * 2)) * 6))];
}
