import { describe, expect, it } from 'vitest';
import { aggregateActivity } from './activity-summary';
import type { DeviceHistory } from './store';

const now = Date.UTC(2026, 10, 21, 0, 0, 0);
const slot = now / 1000 - 900;
const ids = [
  'aaaaaaaa-aaaa-4aaa-aaaa-aaaaaaaaaaaa',
  'bbbbbbbb-bbbb-4bbb-bbbb-bbbbbbbbbbbb',
] as const;
const make = (id: string, seconds: number): DeviceHistory => ({
  version: 1, deviceId: id, deviceName: id, updatedAt: now,
  cells: [{ start: slot, seconds }],
});

describe('account aggregation', () => {
  it('shows individual totals and caps overlapping PCs to 15 minutes', () => {
    const result = aggregateActivity([make(ids[0], 700), make(ids[1], 500)], 1, { now });
    expect(result.deviceCount).toBeUndefined(); // API adds this display field
    expect(result.devices.map(d => d.seconds)).toEqual([700, 500]);
    expect(result.sumDeviceSeconds).toBe(1200);
    expect(result.seconds).toBe(900);
    expect(result.cells).toBeUndefined();
    expect(result.period.timezone).toBe('UTC');
  });

  it('supports filtering one PC and returning detailed cells', () => {
    const result = aggregateActivity([make(ids[0], 700), make(ids[1], 500)], 1,
      { now, deviceId: ids[1], includeCells: true });
    expect(result.devices).toHaveLength(1);
    expect(result.seconds).toBe(500);
    expect(result.devices[0].cells).toEqual([{ start: slot, seconds: 500 }]);
    expect(result.cells).toEqual([{ start: slot, seconds: 500 }]);
  });

  it('ignores invalid slots outside the allowed period and rejects longer ranges', () => {
    const device = make(ids[0], 500);
    device.cells.push({ start: slot - 15 * 86400, seconds: 600 });
    device.cells.push({ start: slot + 1, seconds: 600 });
    device.cells.push({ start: slot, seconds: 1200 });
    expect(aggregateActivity([device], 1, { now }).seconds).toBe(500);
    expect(() => aggregateActivity([device], 8, { now })).toThrow('days must be 1..7');
  });
});
