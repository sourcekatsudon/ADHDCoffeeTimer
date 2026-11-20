import { randomKey, hash, listPaths, readBlob, writeBlob } from './store';

export const TOKEN_SCOPE = 'activity:read' as const;
const TOKEN_RE = /^ftk_v1_[a-f0-9]{64}$/;
const USER_KEY_RE = /^[a-f0-9]{64}$/;
const ID_RE = /^[a-f0-9]{24}$/;

export type DeveloperToken = {
  version: 1;
  id: string;
  name: string;
  userKey: string;
  scope: typeof TOKEN_SCOPE;
  prefix: string;
  verifierPath: string;
  createdAt: number;
  expiresAt: number;
  revokedAt: number | null;
};
type Verifier = {
  version: 1;
  userKey: string;
  id: string;
  scope: typeof TOKEN_SCOPE;
  expiresAt: number;
  revokedAt: number | null;
};

export const developerTokenPrefix = (userKey: string) =>
  'focus/v1/users/' + userKey + '/developer-tokens/';
export const developerTokenPath = (userKey: string, id: string) =>
  developerTokenPrefix(userKey) + id + '.json';
const verifierPathFor = (secret: string) =>
  'focus/v1/developer-verifiers/' + hash('developer-token:' + secret) + '.json';

export function publicToken(record: DeveloperToken) {
  const { id, name, scope, prefix, createdAt, expiresAt, revokedAt } = record;
  return { id, name, scope, prefix, createdAt, expiresAt, revokedAt };
}

export function newDeveloperToken(name: string, userKey: string, days: number, now = Date.now()) {
  if (!USER_KEY_RE.test(userKey) || !Number.isInteger(days) || days < 1 || days > 365) {
    throw new Error('Invalid account or expiration');
  }
  const trimmed = name.trim();
  if (!trimmed || trimmed.length > 48 || /[<>\x00-\x1f]/.test(trimmed)) {
    throw new Error('Token name must be 1–48 characters');
  }
  const secret = 'ftk_v1_' + randomKey(32);
  const record: DeveloperToken = {
    version: 1,
    id: randomKey(12),
    name: trimmed,
    userKey,
    scope: TOKEN_SCOPE,
    prefix: secret.slice(0, 15),
    verifierPath: verifierPathFor(secret),
    createdAt: now,
    expiresAt: now + days * 86400_000,
    revokedAt: null,
  };
  return { secret, record };
}

export async function listDeveloperTokens(userKey: string): Promise<DeveloperToken[]> {
  if (!USER_KEY_RE.test(userKey)) throw new Error('Invalid account');
  const paths = (await listPaths(developerTokenPrefix(userKey)))
    .filter(path => /\/[a-f0-9]{24}\.json$/.test(path))
    .slice(0, 50);
  const rows = await Promise.all(paths.map(path => readBlob<DeveloperToken>(path)));
  return rows
    .filter((row): row is DeveloperToken =>
      !!row && row.version === 1 && row.userKey === userKey &&
      ID_RE.test(row.id) && row.scope === TOKEN_SCOPE &&
      row.verifierPath.startsWith('focus/v1/developer-verifiers/'))
    .sort((a, b) => b.createdAt - a.createdAt);
}

export async function issueDeveloperToken(record: DeveloperToken) {
  // Only a one-way verifier path and metadata are persisted. The plaintext
  // secret is never put into storage or returned to later list requests.
  const verifier: Verifier = {
    version: 1, userKey: record.userKey, id: record.id,
    scope: TOKEN_SCOPE, expiresAt: record.expiresAt, revokedAt: null,
  };
  await writeBlob(record.verifierPath, verifier);
  await writeBlob(developerTokenPath(record.userKey, record.id), record);
}

export async function revokeDeveloperToken(userKey: string, id: string): Promise<boolean> {
  if (!USER_KEY_RE.test(userKey) || !ID_RE.test(id)) return false;
  const path = developerTokenPath(userKey, id);
  const record = await readBlob<DeveloperToken>(path);
  if (!record || record.userKey !== userKey || record.id !== id ||
      !record.verifierPath.startsWith('focus/v1/developer-verifiers/')) return false;
  if (record.revokedAt) return true;
  const existing = await readBlob<Verifier>(record.verifierPath);
  if (existing?.userKey === userKey && existing.id === id) {
    // Make the verifier unusable BEFORE updating the visible metadata.
    await writeBlob(record.verifierPath, { ...existing, revokedAt: Date.now() });
  }
  await writeBlob(path, { ...record, revokedAt: Date.now() });
  return true;
}

export async function authenticateDeveloperToken(bearer: string | null) {
  const matched = bearer?.match(/^Bearer (ftk_v1_[a-f0-9]{64})$/);
  if (!matched || !TOKEN_RE.test(matched[1])) return null;
  const verifier = await readBlob<Verifier>(verifierPathFor(matched[1]));
  if (!verifier || verifier.version !== 1 || verifier.scope !== TOKEN_SCOPE ||
      !USER_KEY_RE.test(verifier.userKey) || !ID_RE.test(verifier.id) ||
      !!verifier.revokedAt || verifier.expiresAt <= Date.now()) return null;
  return { userKey: verifier.userKey, tokenId: verifier.id };
}
