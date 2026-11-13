import type { CSSProperties } from 'react';

export const THEME_IDS = [
  'mist-blue',
  'sage-green',
  'lavender',
  'dusty-rose',
  'champagne',
  'sea-glass',
  'smoky-slate',
] as const;
export type ThemeId = (typeof THEME_IDS)[number];
const palettes = [
  ['mist-blue', 'Mist Blue', '#91A9CB', '#F2D2CA'],
  ['sage-green', 'Sage Green', '#95B3A2', '#E9D6AF'],
  ['lavender', 'Lavender', '#ADA0C8', '#F0CADC'],
  ['dusty-rose', 'Dusty Rose', '#C5A0AA', '#F2DCC8'],
  ['champagne', 'Champagne', '#D0BA9C', '#FFF0CD'],
  ['sea-glass', 'Sea Glass', '#8EBFC1', '#D7E8DA'],
  ['smoky-slate', 'Smoky Slate', '#8999AB', '#E7CBBF'],
] as const;
const rgb = (hex: string) => [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16));
function mix(a: string, b: string, amount: number, alpha = 1) {
  const x = rgb(a),
    y = rgb(b);
  return `rgba(${x.map((v, i) => Math.round(v + (y[i] - v) * amount)).join(',')},${alpha})`;
}
export const THEMES = palettes.map(([id, name, base, light]) => ({
  id,
  name,
  base,
  light,
  style: {
    '--surface': mix(base, '#26364b', 0.5, 0.7),
    '--surface-depth': mix(base, '#1b2b40', 0.64, 0.68),
    '--surface-light': mix(base, '#ffffff', 0.35, 0.18),
    '--border': mix(base, '#ffffff', 0.78, 0.65),
    '--accent': light,
    '--accent-soft': mix(light, '#ffffff', 0.05, 0.2),
    '--ring': mix(light, '#fff9eb', 0.22),
    '--glow': light,
    '--glow-soft': mix(light, '#ffffff', 0.04, 0.27),
    '--text': '#fffdfa',
    '--text-muted': '#e3e8efcc',
    '--shadow': mix(base, '#102039', 0.8, 0.4),
    '--hover': mix(base, '#ffffff', 0.48, 0.28),
    '--button': mix(base, '#122b49', 0.78, 0.94),
    '--settings-top': mix(base, '#253b56', 0.62, 0.98),
    '--settings-bottom': mix(base, '#182b43', 0.76, 0.98),
  } as CSSProperties,
}));
export function themeById(id: string) {
  return THEMES.find((t) => t.id === id) ?? THEMES[0];
}
export function applyTheme(element: HTMLElement, id: ThemeId) {
  const theme = themeById(id);
  element.dataset.theme = theme.id;
  for (const [key, value] of Object.entries(theme.style))
    element.style.setProperty(key, String(value));
}
