import { NextRequest, NextResponse } from 'next/server';
import { probeStorage, responseHeaders, storeReady } from '../../../lib/store';

export const runtime = 'nodejs';
export async function GET(req: NextRequest) {
  // Only query Supabase when explicitly asked; ordinary health pings remain
  // free of database read traffic.
  const check = req.nextUrl.searchParams.get('check') === 'storage';
  const diagnostic = check ? await probeStorage() : null;
  return NextResponse.json({
    application: 'ADHD Coffee Time focus history',
    oauthConfigured: Boolean(process.env.AUTH_GOOGLE_ID && process.env.AUTH_GOOGLE_SECRET && process.env.AUTH_SECRET),
    privateStorageConfigured: storeReady(),
    storageBackend: 'supabase-postgres',
    ...(diagnostic ? { storageDiagnostic: diagnostic } : {}),
  }, { headers: responseHeaders });
}
