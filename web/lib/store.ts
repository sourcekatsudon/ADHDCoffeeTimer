import { createHash, randomBytes } from 'node:crypto';
import { get, list, put } from '@vercel/blob';

export const hash = (value: string) => createHash('sha256').update(value).digest('hex');
export const userKey = (googleSub: string) => hash('google-sub:' + googleSub);
export const randomKey = (n = 32) => randomBytes(n).toString('hex');
export const storeReady = () => Boolean(process.env.BLOB_READ_WRITE_TOKEN);
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

export async function readBlob<T>(path: string): Promise<T | null> {
  const res = await get(path, { access: 'private' });
  if (!res?.stream) return null;
  const data = await new Response(res.stream).text();
  return JSON.parse(data) as T;
}

export async function writeBlob(path: string, data: unknown): Promise<void> {
  await put(path, JSON.stringify(data), {
    access: 'private',
    addRandomSuffix: false,
    allowOverwrite: true,
    cacheControlMaxAge: 60,
    contentType: 'application/json',
  });
}

export async function listPaths(prefix: string): Promise<string[]> {
  const paths: string[] = [];
  let cursor: string | undefined;
  for (let i = 0; i < 100; i++) {
    const page = await list({ prefix, cursor, limit: 100 });
    paths.push(...page.blobs.map((b) => b.pathname));
    if (!page.hasMore || !page.cursor) break;
    cursor = page.cursor;
  }
  return paths;
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
