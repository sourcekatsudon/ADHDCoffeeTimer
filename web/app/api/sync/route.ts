import { NextRequest, NextResponse } from 'next/server';
import { deviceHeaders, devicePath, readBlob, storeReady, tokenPath, writeBlob, type Cell, type DeviceToken } from '../../../lib/store';

export const runtime = 'nodejs';
export function OPTIONS() { return new NextResponse(null, { status: 204, headers: deviceHeaders }); }
export async function POST(req: NextRequest) {
  if (!storeReady()) return NextResponse.json({ error: 'Private storage not connected.' }, { status: 503, headers: deviceHeaders });
  const bearer = req.headers.get('authorization')?.match(/^Bearer ([a-f0-9]{64})$/i)?.[1];
  if (!bearer) return NextResponse.json({ error: 'Device authorization required.' }, { status: 401, headers: deviceHeaders });
  try {
    const key = await readBlob<DeviceToken>(tokenPath(bearer));
    if (!key || key.expiresAt < Date.now()) {
      return NextResponse.json({ error: 'Device token expired. Pair this PC again.' }, { status: 401, headers: deviceHeaders });
    }
    const data = await req.json().catch(() => null);
    const cells: unknown = data?.history;
    const name = typeof data?.deviceName === 'string' ? data.deviceName.slice(0, 32).replace(/[<>\\]/g, '') : 'PC';
    if (!Array.isArray(cells) || cells.length > 1000) {
      return NextResponse.json({ error: 'Invalid history array.' }, { status: 400, headers: deviceHeaders });
    }
    const now = Math.floor(Date.now() / 1000);
    const validated: Cell[] = [];
    for (const row of cells) {
      const start = row?.start;
      const seconds = row?.seconds;
      if (!Number.isInteger(start) || start % 900 !== 0 || start < now - 9 * 86400 || start > now + 900 ||
          typeof seconds !== 'number' || !Number.isFinite(seconds) || seconds < 0 || seconds > 900) {
        return NextResponse.json({ error: 'Invalid 15-minute activity cell.' }, { status: 400, headers: deviceHeaders });
      }
      validated.push({ start, seconds });
    }
    // Each PC owns only its own snapshot. Sending a new snapshot also propagates
    // deleted cells, without clobbering activity from a different PC.
    const updatedAt = Date.now();
    await writeBlob(devicePath(key.userKey, key.deviceId), {
      version: 1, deviceId: key.deviceId, deviceName: name, updatedAt, cells: validated,
    });
    return NextResponse.json({ ok: true, updatedAt }, { headers: deviceHeaders });
  } catch {
    return NextResponse.json({ error: 'Sync failed. The next attempt will retry.' }, { status: 503, headers: deviceHeaders });
  }
}
