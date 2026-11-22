import { NextRequest, NextResponse } from 'next/server';
import { currentAccount } from '../../../../auth';
import { revokeDeveloperToken } from '../../../../lib/developer-api';
import { responseHeaders, storeReady, userKey } from '../../../../lib/store';

export const runtime = 'nodejs';
export async function DELETE(
  request: NextRequest,
  context: { params: Promise<{ id: string }> },
) {
  const account = await currentAccount();
  if (!account) return NextResponse.json({ error: 'Google sign-in required.' }, { status: 401, headers: responseHeaders });
  if (request.headers.get('origin') !== request.nextUrl.origin) {
    return NextResponse.json({ error: 'Cross-site token deletion forbidden.' }, { status: 403, headers: responseHeaders });
  }
  if (!storeReady()) return NextResponse.json({ error: 'Supabase database not connected.' }, { status: 503, headers: responseHeaders });
  const { id } = await context.params;
  try {
    const revoked = await revokeDeveloperToken(userKey(account.id), id);
    if (!revoked) return NextResponse.json({ error: 'Token not found.' }, { status: 404, headers: responseHeaders });
    return NextResponse.json({ revoked: true, id }, { headers: responseHeaders });
  } catch {
    return NextResponse.json({ error: 'Could not revoke token.' }, { status: 503, headers: responseHeaders });
  }
}
