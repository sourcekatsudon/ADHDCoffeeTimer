import { expect, it } from 'vitest';
import catalog from './locales.json';
import { LANGUAGES, translate, localizeError } from './i18n';
import { formatClock, timerDimensions } from './model';

it('has complete translations with matching interpolation values in all ten languages', () => {
  expect(LANGUAGES).toHaveLength(10);
  for (const [key, translations] of Object.entries(catalog.messages)) {
    const placeholders = (text: string) => [...text.matchAll(/\{\w+\}/g)].map((m) => m[0]).sort();
    for (const language of LANGUAGES) {
      const value = translations[language.id];
      expect(value, `${key}/${language.id}`).toBeTruthy();
      expect(placeholders(value), `${key}/${language.id}`).toEqual(placeholders(translations.ja));
    }
  }
});
it('translates timer, settings, notification and saved error codes immediately', () => {
  expect(translate('en')('running')).toBe('Time to focus');
  expect(translate('es')('settings')).toBe('Ajustes');
  expect(translate('fr')('elapsed', { minutes: 25 })).toBe('25 min écoulées');
  expect(localizeError('errorSize', 'en')).toBe('Invalid timer size.');
  expect(localizeError('errorSize', 'ja')).toBe('タイマーのサイズが不正です。');
  expect(localizeError('errorAutostart: OS error', 'de')).toContain('OS error');
});
it('formats weekdays and calendars for the selected language', () => {
  const date = new Date(2026, 10, 14, 9, 4, 7);
  expect(formatClock(date, 'en')).toMatchObject({ era: '', weekday: 'Sat', time: '09:04:07' });
  expect(formatClock(date, 'ko').weekday).toBe('토');
  expect(formatClock(date, 'ja').era).toBe('令和8年');
});
it('scales both timer dimensions by exactly 40%, 70% and 100%', () => {
  for (const [size, scale] of [
    ['S', 0.4],
    ['M', 0.7],
    ['L', 1],
  ] as const) {
    const dimensions = timerDimensions(size);
    expect(dimensions.scale).toBe(scale);
    expect(dimensions.width).toBeCloseTo(172 * scale);
    expect(dimensions.height).toBeCloseTo(474 * scale);
  }
});
