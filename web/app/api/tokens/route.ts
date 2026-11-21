import { NextRequest, NextResponse } from 'next/server';
import { currentAccount } from '../../../auth';
import {
  issueDeveloperToken, listDeveloperTokens, newDeveloperToken, publicToken,
} from '../../../lib/developer-api';
import { responseHeaders, storeReady, userKey } from '../../../lib/store';

export const runtime = 'nodejs';
const response = (body: unknown, status = 200) =>
  NextResponse.json(body, { status, headers: responseHeaders });

export async function GET() {
  const account = await currentAccount();
  if (!account) return response({ error: 'Google sign-in required.' }, 401);
  if (!storeReady()) return response({ error: 'Supabase database not connected.' }, 503);
  try {
    const items = await listDeveloperTokens(userKey(account.id));
    return response({ tokens: items.map(publicToken) });
  } catch {
    return response({ error: 'Could not list tokens.' }, 503);
  }
}
export async function POST(req: NextRequest) {
  const account = await currentAccount();
  if (!account) return response({ error: 'Google sign-in required.' }, 401);
  if (req.headers.get('origin') !== req.nextUrl.origin) {
    return response({ error: 'Cross-site token creation forbidden.' }, 403);
  }
  if (!storeReady()) return response({ error: 'Supabase database not connected.' }, 503);
  const body = await req.json().catch(() => null);
  const name = body?.name;
  const days = body?.expiresInDays ?? 90;
  if (typeof name !== 'string' || ![7, 30, 90, 365].includes(days)) {
    return response({ error: 'Specify a name and expiration of 7, 30, 90 or 365 days.' }, 400);
  }
  try {
    const owner = userKey(account.id);
    const current = await listDeveloperTokens(owner);
    if (current.filter(t => !t.revokedAt && t.expiresAt > Date.now()).length >= 10) {
      return response({ error: 'Maximum 10 active developer tokens; revoke one first.' }, 409);
    }
    const { secret, record } = newDeveloperToken(name, owner, days);
    await issueDeveloperToken(record);
    return response({ token: secret, details: publicToken(record) }, 201);
  } catch (error) {
    if (error instanceof Error && /Token name/.test(error.message)) {
      return response({ error: error.message }, 400);
    }
    return response({ error: 'Could not create token.' }, 503);
  }
}
