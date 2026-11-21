import { NextRequest, NextResponse } from 'next/server';
import { deviceHeaders, pairPath, randomKey, redeemPairCode, storeReady, tokenPath } from '../../../../lib/store';

export const runtime = 'nodejs';
export function OPTIONS() { return new NextResponse(null, { status: 204, headers: deviceHeaders }); }
export async function POST(req: NextRequest) {
  if (!storeReady()) return NextResponse.json({ error: 'Supabase storage not connected.' }, { status: 503, headers: deviceHeaders });
  const body = await req.json().catch(() => null);
  const code = typeof body?.code === 'string' ? body.code.trim().toUpperCase() : '';
  const deviceId = typeof body?.deviceId === 'string' ? body.deviceId : '';
  if (!/^[0-9A-F]{16}$/.test(code) || !/^[a-zA-Z0-9-]{16,64}$/.test(deviceId)) {
    return NextResponse.json({ error: 'Invalid pairing code or PC identifier.' }, { status: 400, headers: deviceHeaders });
  }
  try {
    const token = randomKey();
    const expiresAt = Date.now() + 365 * 86400_000;
    const owner = await redeemPairCode(pairPath(code), tokenPath(token), deviceId, expiresAt);
    if (!owner) {
      return NextResponse.json(
        { error: 'Code expired or already used. Create a new code in the dashboard.' },
        { status: 401, headers: deviceHeaders },
      );
    }
    return NextResponse.json({ token, deviceId, expiresAt }, { headers: deviceHeaders });
  } catch {
    return NextResponse.json({ error: 'Could not finish device pairing.' }, { status: 503, headers: deviceHeaders });
  }
}
