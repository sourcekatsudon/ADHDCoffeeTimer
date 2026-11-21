import { NextRequest, NextResponse } from 'next/server';
import { authenticateDeveloperToken } from '../../../../lib/developer-api';
import { aggregateActivity, loadAccountDevices } from '../../../../lib/activity-summary';
import { responseHeaders, storeReady } from '../../../../lib/store';

export const runtime = 'nodejs';
export async function GET(req: NextRequest) {
  const headers = { ...responseHeaders, 'WWW-Authenticate': 'Bearer realm="Focus API"' };
  if (!storeReady()) {
    return NextResponse.json({ error: 'Supabase database is not configured.' }, { status: 503, headers });
  }
  const bearer = req.headers.get('authorization');
  try {
    const auth = await authenticateDeveloperToken(bearer);
    if (!auth) {
      return NextResponse.json({ error: 'Invalid, expired or revoked API token.' }, { status: 401, headers });
    }
    const daysRaw = req.nextUrl.searchParams.get('days') ?? '7';
    const days = Number(daysRaw);
    if (!/^[1-7]$/.test(daysRaw) || !Number.isInteger(days)) {
      return NextResponse.json({ error: 'days must be an integer from 1 to 7.' }, { status: 400, headers: responseHeaders });
    }
    const detail = req.nextUrl.searchParams.get('detail') ?? 'summary';
    if (!['summary', 'slots'].includes(detail)) {
      return NextResponse.json({ error: 'detail must be summary or slots.' }, { status: 400, headers: responseHeaders });
    }
    const deviceId = req.nextUrl.searchParams.get('deviceId') ?? undefined;
    if (deviceId && !/^[a-zA-Z0-9-]{16,64}$/.test(deviceId)) {
      return NextResponse.json({ error: 'Invalid deviceId.' }, { status: 400, headers: responseHeaders });
    }
    const devices = await loadAccountDevices(auth.userKey);
    if (deviceId && !devices.some(d => d.deviceId === deviceId)) {
      return NextResponse.json({ error: 'Device not found.' }, { status: 404, headers: responseHeaders });
    }
    const activity = aggregateActivity(devices, days, {
      includeCells: detail === 'slots', deviceId,
    });
    return NextResponse.json({
      version: '1',
      ...activity,
      deviceCount: activity.devices.length,
    }, { headers: responseHeaders });
  } catch {
    return NextResponse.json({ error: 'Could not retrieve activity.' }, { status: 503, headers: responseHeaders });
  }
}
