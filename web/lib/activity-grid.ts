/** Seven- or thirty-day view in the user's local calendar timezone. */
export type ActivityRange = 7 | 30;
export type ActivityCell = { start: number; seconds: number };
export type ActivityDay = { day: Date; cells: ActivityCell[]; seconds: number };
export const CELLS_PER_DAY = 96;

export function buildActivityDays(
  history: ActivityCell[], range: ActivityRange, current = new Date(),
): ActivityDay[] {
  const lookup = new Map(history.map(cell => [cell.start, cell.seconds]));
  const midnight = new Date(current);
  midnight.setHours(0, 0, 0, 0);
  return Array.from({ length: range }, (_, index) => {
    const day = new Date(midnight);
    day.setDate(midnight.getDate() - index);
    const cells = Array.from({ length: CELLS_PER_DAY }, (_, slot) => {
      const time = new Date(day);
      time.setHours(Math.floor(slot / 4), (slot % 4) * 15, 0, 0);
      const start = time.getTime() / 1000;
      return { start, seconds: lookup.get(start) ?? 0 };
    });
    return { day, cells, seconds: cells.reduce((sum, cell) => sum + cell.seconds, 0) };
  });
}
