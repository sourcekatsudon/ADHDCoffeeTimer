import { listPaths, readBlob, userPrefix, type Cell, type DeviceHistory } from './store';

export type AccountDevice = {
  id: string;
  name: string;
  updatedAt: number;
  seconds: number;
  cells?: Cell[];
};
export type AccountActivity = {
  period: { from: string; to: string; days: number; timezone: 'UTC' };
  seconds: number;
  sumDeviceSeconds: number;
  devices: AccountDevice[];
  cells?: Cell[];
  aggregation: string;
};

export async function loadAccountDevices(userKey: string): Promise<DeviceHistory[]> {
  if (!/^[a-f0-9]{64}$/.test(userKey)) throw new Error('Invalid user key');
  const paths = (await listPaths(userPrefix(userKey)))
    .filter(path => path.endsWith('.json')).slice(0, 64);
  const records = await Promise.all(paths.map(path => readBlob<DeviceHistory>(path)));
  return records.filter((record): record is DeviceHistory =>
    !!record && record.version === 1 &&
    typeof record.deviceId === 'string' && Array.isArray(record.cells) &&
    Number.isFinite(record.updatedAt) && typeof record.deviceName === 'string');
}

/** All servers use UTC boundaries. The desktop can group the returned cells in a user's timezone. */
export function aggregateActivity(
  devices: DeviceHistory[],
  days: number,
  options: { includeCells?: boolean; deviceId?: string; now?: number } = {},
): AccountActivity {
  if (!Number.isInteger(days) || days < 1 || days > 7) throw new Error('days must be 1..7');
  const now = options.now ?? Date.now();
  // A rolling N-day window, aligned to 15-minute UTC buckets to avoid
  // incorrectly counting partial buckets at either boundary.
  const last = Math.floor(now / 1000 / 900) * 900;
  const first = last - days * 86400 + 900;
  const selected = options.deviceId
    ? devices.filter(d => d.deviceId === options.deviceId)
    : devices;
  const merged = new Map<number, number>();
  const resultDevices = selected.map(device => {
    const cells: Cell[] = [];
    for (const cell of device.cells) {
      if (!Number.isInteger(cell.start) || cell.start % 900 !== 0 ||
          !Number.isFinite(cell.seconds) || cell.seconds < 0 || cell.seconds > 900 ||
          cell.start < first || cell.start > last) continue;
      cells.push({ start: cell.start, seconds: cell.seconds });
      merged.set(cell.start, Math.min(900, (merged.get(cell.start) ?? 0) + cell.seconds));
    }
    cells.sort((a, b) => a.start - b.start);
    return {
      id: device.deviceId, name: device.deviceName,
      updatedAt: device.updatedAt,
      seconds: cells.reduce((sum, c) => sum + c.seconds, 0),
      ...(options.includeCells ? { cells } : {}),
    };
  });
  const cells = Array.from(merged, ([start, seconds]) => ({ start, seconds }))
    .sort((a, b) => a.start - b.start);
  return {
    period: {
      from: new Date(first * 1000).toISOString(),
      to: new Date((last + 900) * 1000).toISOString(),
      days, timezone: 'UTC',
    },
    seconds: cells.reduce((sum, c) => sum + c.seconds, 0),
    sumDeviceSeconds: resultDevices.reduce((sum, d) => sum + d.seconds, 0),
    devices: resultDevices,
    ...(options.includeCells ? { cells } : {}),
    aggregation: 'Each 15-minute UTC cell sums PCs and caps merged seconds at 900; simultaneous PCs may be overcounted.',
  };
}
