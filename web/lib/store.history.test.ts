import { describe, expect, it } from 'vitest';
import { mergeDeviceSnapshot, type DeviceHistory } from './store';

const TODAY = 1_800_000_000;
const day = 86400;
const at = (daysAgo: number) => Math.floor((TODAY - daysAgo * day) / 900) * 900;
const prior = (...cells: { start: number; seconds: number }[]): DeviceHistory =>
  ({ version: 1, deviceId: 'pc1', deviceName: 'PC 1', updatedAt: 1, cells });

describe('month-long device synchronization', () => {
  it('retains old activity when a legacy eight-day PC overwrites recent data', () => {
    const old = prior({ start: at(20), seconds: 800 }, { start: at(3), seconds: 600 });
    const result = mergeDeviceSnapshot(old, [{ start: at(3), seconds: 300 }], TODAY, 8);
    expect(result).toEqual([{ start: at(20), seconds: 800 }, { start: at(3), seconds: 300 }]);
  });

  it('propagates cell deletion within the reported device window', () => {
    const old = prior({ start: at(20), seconds: 800 }, { start: at(3), seconds: 600 });
    expect(mergeDeviceSnapshot(old, [], TODAY, 8)).toEqual([{ start: at(20), seconds: 800 }]);
    expect(mergeDeviceSnapshot(old, [], TODAY, 32)).toEqual([]);
  });

  it('expires historical records older than the rolling month', () => {
    const old = prior({ start: at(40), seconds: 600 }, { start: at(18), seconds: 700 });
    expect(mergeDeviceSnapshot(old, [], TODAY, 8)).toEqual([{ start: at(18), seconds: 700 }]);
  });
});
