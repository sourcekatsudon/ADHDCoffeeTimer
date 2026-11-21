import { createHash, randomBytes } from 'node:crypto';
import { createClient, type SupabaseClient } from '@supabase/supabase-js';

// Auth.js owns Google login/session validation. This database client is
// server-side only, uses a private secret key and NEVER appears in the UI.
let dbClient: SupabaseClient | null = null;
const TABLE = 'focus_documents';
export const hash = (value: string) => createHash('sha256').update(value).digest('hex');
export const userKey = (googleSub: string) => hash('google-sub:' + googleSub);
export const randomKey = (n = 32) => randomBytes(n).toString('hex');
export const storeReady = () =>
  Boolean(process.env.SUPABASE_URL && process.env.SUPABASE_SECRET_KEY);

/** Supabase API keys are uninterrupted ASCII strings with no whitespace. */
export function normalizeSupabaseSecret(raw: string): string {
  return raw.replace(/\s+/g, '').replace(/^["']|["']$/g, '');
}

function db(): SupabaseClient {
  if (!storeReady()) throw new Error('Supabase database is not connected.');
  // Lazy initialization prevents next build from crashing before the owner
  // connects the Supabase credentials in Vercel.
  if (!dbClient) {
    dbClient = createClient(
      process.env.SUPABASE_URL!.trim(),
      // API keys never contain whitespace. Pasted keys can gain embedded
      // newlines from line-wrapping; an invalid header would prevent *all*
      // pairing-code writes before Supabase receives a request.
      normalizeSupabaseSecret(process.env.SUPABASE_SECRET_KEY!),
      {
        auth: {
          persistSession: false,
          autoRefreshToken: false,
          detectSessionInUrl: false,
        },
      },
    );
  }
  return dbClient;
}

export const userPrefix = (key: string) => 'focus/v1/users/' + key + '/devices/';
export const devicePath = (key: string, deviceId: string) => userPrefix(key) + deviceId + '.json';
export const pairPath = (code: string) => 'focus/v1/pair/' + hash(code) + '.json';
export const tokenPath = (token: string) => 'focus/v1/token/' + hash(token) + '.json';

export type Cell = { start: number; seconds: number };
export type DeviceHistory = {
  version: 1;
  deviceId: string;
  deviceName: string;
  updatedAt: number;
  cells: Cell[];
};
export type Pair = { userKey: string; expiresAt: number; used: boolean };
export type DeviceToken = { userKey: string; deviceId: string; expiresAt: number };

/** Read a private JSON document, scoped by its opaque key. */
export async function readDocument<T>(key: string): Promise<T | null> {
  const { data, error } = await db().from(TABLE)
    .select('payload').eq('key', key).maybeSingle();
  if (error) throw new Error('Supabase read failed: ' + error.message);
  return data ? (data.payload as T) : null;
}

/** Atomically insert or replace one document by its key. */
export async function writeDocument(key: string, payload: unknown): Promise<void> {
  const { error } = await db().from(TABLE)
    .upsert({ key, payload }, { onConflict: 'key' });
  if (error) throw new Error('Supabase write failed: ' + error.message);
}

/**
 * Minimal server-side connectivity probe for setup diagnostics.
 * No table data, secrets or account identifiers are included in the result.
 */
export async function probeStorage(): Promise<{
  accessible: boolean;
  status: number | null;
  code: string | null;
}> {
  if (!storeReady()) return { accessible: false, status: null, code: 'NOT_CONFIGURED' };
  try {
    const { error, status } = await db().from(TABLE)
      .select('key', { head: true }).limit(1);
    if (error) {
      // Do not log raw PostgREST messages: malformed header errors can
      // literally include part of a secret API key in error.message.
      console.error('[focus/storage] Supabase probe failed', {
        status, code: error.code || 'NO_ERROR_CODE',
        category: storageFailureCategory(error),
      });
    }
    return {
      accessible: !error,
      status: status ?? null,
      code: error?.code || (error ? 'QUERY_FAILED' : null),
    };
  } catch (error) {
    console.error('[focus/storage] Supabase probe threw', {
      category: storageFailureCategory(error),
    });
    return { accessible: false, status: null, code: 'CONNECTION_ERROR' };
  }
}

/** Safe error classification: raw error strings may contain credentials. */
export function storageFailureCategory(error: unknown): string {
  const message = error instanceof Error ? error.message : '';
  if (/invalid header value|headers\.set/i.test(message)) return 'INVALID_SECRET_HEADER';
  if (/invalid api key|invalid jwt|jwt expired/i.test(message)) return 'INVALID_API_KEY';
  if (/permission denied|row.level.security|42501/i.test(message)) return 'DATABASE_PERMISSION';
  if (/failed to fetch|networkerror|fetch failed/i.test(message)) return 'NETWORK_ERROR';
  return 'STORAGE_ERROR';
}

/** A single indexed query for a user's documents replaces Blob list + N reads. */
export async function listDocuments<T>(
  prefix: string, maxResults = 1000,
): Promise<{ key: string; payload: T }[]> {
  if (!/^[a-zA-Z0-9/.-]+$/.test(prefix) || maxResults < 1 || maxResults > 1000) {
    throw new Error('Invalid document prefix or limit');
  }
  const { data, error } = await db().from(TABLE)
    .select('key,payload')
    .like('key', prefix + '%')
    .order('key', { ascending: true })
    .limit(maxResults);
  if (error) throw new Error('Supabase list failed: ' + error.message);
  return (data ?? []) as { key: string; payload: T }[];
}

export async function listDocumentKeys(prefix: string): Promise<string[]> {
  return (await listDocuments(prefix)).map(({ key }) => key);
}

/**
 * A SQL transaction locks and consumes the one-time pairing code.
 * No two concurrent requests can redeem the same code.
 */
export async function redeemPairCode(
  pairKey: string, tokenKey: string, deviceId: string, expiresAt: number,
): Promise<string | null> {
  const { data, error } = await db().rpc('focus_redeem_pairing', {
    p_pair_key: pairKey,
    p_token_key: tokenKey,
    p_device_id: deviceId,
    p_expires_at_ms: expiresAt,
  });
  if (error) throw new Error('Supabase device pairing failed: ' + error.message);
  const owner = (data as { user_key: string }[] | null)?.[0]?.user_key;
  return typeof owner === 'string' && /^[a-f0-9]{64}$/.test(owner) ? owner : null;
}

export const responseHeaders = { 'Cache-Control': 'no-store', 'X-Content-Type-Options': 'nosniff' };
export const deviceHeaders = {
  ...responseHeaders,
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'POST, OPTIONS',
  'Access-Control-Allow-Headers': 'Authorization, Content-Type',
  'Access-Control-Max-Age': '86400',
};

export function mergeHistories(devices: DeviceHistory[]) {
  const bySlot = new Map<number, number>();
  const now = Date.now() / 1000;
  for (const device of devices) {
    for (const cell of device.cells) {
      if (!Number.isFinite(cell.start) || cell.start % 900 !== 0 ||
          !Number.isFinite(cell.seconds) || cell.seconds < 0 || cell.seconds > 900 ||
          cell.start < now - 9 * 86400 || cell.start > now + 900) continue;
      bySlot.set(cell.start, Math.min(900, (bySlot.get(cell.start) ?? 0) + cell.seconds));
    }
  }
  return Array.from(bySlot, ([start, seconds]) => ({ start, seconds })).sort((a, b) => a.start - b.start);
}
