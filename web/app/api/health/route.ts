import { NextResponse } from 'next/server';
import { responseHeaders, storeReady } from '../../../lib/store';
export function GET() {
  return NextResponse.json({
    application: 'ADHD Coffee Time focus history',
    oauthConfigured: Boolean(process.env.AUTH_GOOGLE_ID && process.env.AUTH_GOOGLE_SECRET && process.env.AUTH_SECRET),
    privateStorageConfigured: storeReady(),
  }, { headers: responseHeaders });
}
