import { NextResponse } from 'next/server';
import { currentAccount } from '../../../auth';
import { pairPath, randomKey, responseHeaders, storeReady, userKey, writeBlob } from '../../../lib/store';

export const runtime = 'nodejs';
export async function POST() {
  const account = await currentAccount();
  if (!account) return NextResponse.json({ error: 'Sign in with Google first.' }, { status: 401, headers: responseHeaders });
  if (!storeReady()) return NextResponse.json({ error: 'Private storage not connected yet.' }, { status: 503, headers: responseHeaders });
  const code = randomKey(8).toUpperCase();
  const expiresAt = Date.now() + 10 * 60 * 1000;
  try {
    await writeBlob(pairPath(code), { userKey: userKey(account.id), expiresAt, used: false });
    return NextResponse.json({ code, expiresAt }, { headers: responseHeaders });
  } catch {
    return NextResponse.json({ error: 'Could not create a pairing code.' }, { status: 503, headers: responseHeaders });
  }
}
