import { NextResponse } from 'next/server';
import { currentAccount } from '../../../auth';
import { pairPath, randomKey, responseHeaders, storageFailureCategory, storeReady, userKey, writeDocument } from '../../../lib/store';

export const runtime = 'nodejs';
export async function POST() {
  const account = await currentAccount();
  if (!account) return NextResponse.json({ error: 'Sign in with Google first.' }, { status: 401, headers: responseHeaders });
  if (!storeReady()) return NextResponse.json({ error: 'Supabase storage not connected yet.' }, { status: 503, headers: responseHeaders });
  const code = randomKey(8).toUpperCase();
  const expiresAt = Date.now() + 10 * 60 * 1000;
  try {
    await writeDocument(pairPath(code), { userKey: userKey(account.id), expiresAt, used: false });
    return NextResponse.json({ code, expiresAt }, { headers: responseHeaders });
  } catch (error) {
    // The browser must not receive the user's account or Supabase secrets.
    // Keep the database error message in Vercel's server logs for diagnosis.
    console.error('[focus/pair] Supabase write failed', {
      category: storageFailureCategory(error),
    });
    return NextResponse.json({
      error: 'Could not create a pairing code. Please check the Supabase connection.',
    }, { status: 503, headers: responseHeaders });
  }
}
