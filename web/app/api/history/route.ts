import { NextResponse } from 'next/server';
import { currentAccount } from '../../../auth';
import { listDocuments, mergeHistories, responseHeaders, storeReady, userKey, userPrefix, type DeviceHistory } from '../../../lib/store';

export const runtime = 'nodejs';
export async function GET() {
  const account = await currentAccount();
  if (!account) return NextResponse.json({ error: 'Sign in first.' }, { status: 401, headers: responseHeaders });
  if (!storeReady()) return NextResponse.json({ error: 'Supabase database is not connected yet.' }, { status: 503, headers: responseHeaders });
  try {
    const prefix = userPrefix(userKey(account.id));
    const records = (await listDocuments<DeviceHistory>(prefix, 64))
      .map(record => record.payload);
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
