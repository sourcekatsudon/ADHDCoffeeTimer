import { NextResponse } from 'next/server';
import { currentAccount } from '../../../auth';
import { listPaths, mergeHistories, readBlob, responseHeaders, storeReady, userKey, userPrefix, type DeviceHistory } from '../../../lib/store';

export const runtime = 'nodejs';
export async function GET() {
  const account = await currentAccount();
  if (!account) return NextResponse.json({ error: 'Sign in first.' }, { status: 401, headers: responseHeaders });
  if (!storeReady()) return NextResponse.json({ error: 'Private storage is not connected yet.' }, { status: 503, headers: responseHeaders });
  try {
    const prefix = userPrefix(userKey(account.id));
    const names = (await listPaths(prefix)).filter((path) => path.endsWith('.json')).slice(0, 64);
    const records = await Promise.all(names.map((path) => readBlob<DeviceHistory>(path)));
    const devices = records.filter((row): row is DeviceHistory =>
      Boolean(row && row.version === 1 && Array.isArray(row.cells) && row.deviceId && row.updatedAt));
    const history = mergeHistories(devices);
    return NextResponse.json({
      history,
      totalSeconds: history.reduce((a, v) => a + v.seconds, 0),
      devices: devices.map((d) => ({ id: d.deviceId, name: d.deviceName, updatedAt: d.updatedAt })),
      mergeNote: 'Activity from separate PCs is summed and capped at 15 minutes per time cell.',
    }, { headers: responseHeaders });
  } catch {
    return NextResponse.json({ error: 'Could not load your private activity.' }, { status: 503, headers: responseHeaders });
  }
}
