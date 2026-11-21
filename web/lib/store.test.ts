import { describe, expect, it } from 'vitest';
import { normalizeSupabaseSecret, storageFailureCategory } from './store';

describe('Supabase credentials', () => {
  it('removes pasted line breaks without changing the API key contents', () => {
    expect(normalizeSupabaseSecret('sb_secret_abc\n123\r\nXYZ'))
      .toBe('sb_secret_abc123XYZ');
    expect(normalizeSupabaseSecret('  "sb_secret_abc\n123"  '))
      .toBe('sb_secret_abc123');
    expect(normalizeSupabaseSecret('sb_secret_clean'))
      .toBe('sb_secret_clean');
  });

  it('classifies invalid-header errors without exposing the secret in logs', () => {
    const sensitive = 'sb_secret_test_very_sensitive_value';
    const error = new Error('TypeError: Headers.set: "' + sensitive + '\nnext" is an invalid header value.');
    expect(storageFailureCategory(error)).toBe('INVALID_SECRET_HEADER');
  });
});
