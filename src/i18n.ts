import catalog from './locales.json';

export const APP_NAME = 'ADHD Coffee Time';
export type Language = 'ja' | 'en' | 'es' | 'fr' | 'de' | 'pt' | 'zh' | 'ko' | 'ru' | 'ar';
export type MessageKey = keyof typeof catalog.messages;
export const LANGUAGES = catalog.languages as Array<{
  id: Language;
  label: string;
  name: string;
  locale: string;
  dir: 'ltr' | 'rtl';
}>;
export function languageInfo(language: Language) {
  return LANGUAGES.find((item) => item.id === language) ?? LANGUAGES[0];
}
export function translate(language: Language) {
  return (key: MessageKey, values: Record<string, string | number> = {}) => {
    const text = catalog.messages[key][language] ?? catalog.messages[key].ja;
    return text.replace(/\{(\w+)\}/g, (placeholder, name: string) =>
      values[name] === undefined ? placeholder : String(values[name]),
    );
  };
}
export function localizeError(error: unknown, language: Language) {
  const raw = String(error);
  const [code, ...details] = raw.split(': ');
  if (Object.hasOwn(catalog.messages, code)) {
    return (
      translate(language)(code as MessageKey) + (details.length ? ` ${details.join(': ')}` : '')
    );
  }
  // OS/library details can be technical; always precede them with a localized message.
  return `${translate(language)('checkSettings')} (${raw})`;
}
