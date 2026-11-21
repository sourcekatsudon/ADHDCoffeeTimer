import { describe, expect, it } from 'vitest';
import { buildActivityDays, CELLS_PER_DAY } from './activity-grid';

describe('focus history calendar', () => {
  const now = new Date(2026, 10, 21, 21, 0);
  it('switches between seven days and thirty local calendar days', () => {
    const week = buildActivityDays([], 7, now);
    const month = buildActivityDays([], 30, now);
    expect(week).toHaveLength(7);
    expect(month).toHaveLength(30);
    expect(month.reduce((count, day) => count + day.cells.length, 0)).toBe(30 * 96);
    expect(month[29].day.getFullYear()).toBe(2026);
    expect(month[29].day.getMonth()).toBe(9);
    expect(month[29].day.getDate()).toBe(23);
  });

  it('ends at 23:45; 24:00 is the right edge, not another cell', () => {
    const day = buildActivityDays([], 7, now)[0];
    expect(day.cells).toHaveLength(CELLS_PER_DAY);
    const last = new Date(day.cells[CELLS_PER_DAY - 1].start * 1000);
    expect(last.getHours()).toBe(23);
    expect(last.getMinutes()).toBe(45);
    expect(day.cells.every(cell => new Date(cell.start * 1000).getDate() === 21)).toBe(true);
  });

  it('shows activity older than one week when viewing one month', () => {
    const time = new Date(2026, 10, 6, 14, 15);
    const cell = { start: time.getTime() / 1000, seconds: 375 };
    const week = buildActivityDays([cell], 7, now);
    const month = buildActivityDays([cell], 30, now);
    expect(week.every(day => day.seconds === 0)).toBe(true);
    expect(month.find(day => day.day.getDate() === 6)?.seconds).toBe(375);
  });
});
