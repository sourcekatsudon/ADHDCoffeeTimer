import { describe, expect, it } from 'vitest';
import { newDeveloperToken, publicToken, TOKEN_SCOPE } from './developer-api';

const user = 'b'.repeat(64);
describe('read-only developer API keys', () => {
  it('creates high-entropy, short-lived, account-scoped keys', () => {
    const a = newDeveloperToken('Analytics automation', user, 30, 1000);
    const b = newDeveloperToken('Analytics automation', user, 30, 1000);
    expect(a.secret).toMatch(/^ftk_v1_[a-f0-9]{64}$/);
    expect(a.secret).not.toBe(b.secret);
    expect(a.record.userKey).toBe(user);
    expect(a.record.expiresAt).toBe(1000 + 30 * 86400_000);
    expect(a.record.scope).toBe(TOKEN_SCOPE);
    expect(a.record.verifierPath).not.toContain(a.secret);
    expect(JSON.stringify(publicToken(a.record))).not.toContain(a.secret);
  });
  it('rejects invalid expiries and blank or long names', () => {
    expect(() => newDeveloperToken('test', user, 366)).toThrow();
    expect(() => newDeveloperToken('', user, 30)).toThrow();
    expect(() => newDeveloperToken('x'.repeat(100), user, 30)).toThrow();
    expect(() => newDeveloperToken('test', 'invalid', 30)).toThrow();
  });
});
