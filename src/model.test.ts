import { describe, it, expect } from 'vitest';
import { anglePreset, formatClock, formatTime, nextPreset, initial } from './model';
describe('timer interactions', () => {
  it('keeps Japanese and the original large size for existing users', () => {
    expect(initial.settings).toMatchObject({ language: 'ja', timerSize: 'L' });
  });
  it('formats the local date, Japanese weekday and clock with seconds', () => {
    expect(formatClock(new Date(2026, 10, 13, 9, 4, 7))).toEqual({
      era: '令和8年',
      date: '2026/11/13',
      weekday: '金',
      time: '09:04:07',
    });
    expect(formatClock(new Date(2026, 11, 31, 23, 59, 59)).time).toBe('23:59:59');
    expect(formatClock(new Date(2027, 0, 1, 0, 0, 0)).date).toBe('2027/01/01');
    expect(formatClock(new Date(2019, 4, 1)).era).toBe('令和元年');
    expect(formatClock(new Date(2019, 3, 30)).era).toBe('平成31年');
  });
  it('uses ceiling so the final second remains visible', () => {
    expect(formatTime(0.2)).toBe('00:01');
    expect(formatTime(1500)).toBe('25:00');
    expect(formatTime(-1)).toBe('00:00');
  });
  it('steps and clamps wheel presets', () => {
    expect(nextPreset(25, 3)).toBe(30);
    expect(nextPreset(25, -3)).toBe(15);
    expect(nextPreset(5, -1)).toBe(5);
    expect(nextPreset(90, 1)).toBe(90);
  });
  it('maps ring drag positions clockwise from the top', () => {
    expect(anglePreset(0, -1)).toBe(5);
    expect(anglePreset(0, 1)).toBe(25);
    expect(anglePreset(-1, 0)).toBe(60);
  });
});
