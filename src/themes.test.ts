import { expect, it } from 'vitest';
import { THEMES, THEME_IDS } from './themes';

it('provides the seven requested glass and lighting palettes with complete material tokens', () => {
  expect(THEMES.map((t) => [t.name, t.base, t.light])).toEqual([
    ['Mist Blue', '#91A9CB', '#F2D2CA'],
    ['Sage Green', '#95B3A2', '#E9D6AF'],
    ['Lavender', '#ADA0C8', '#F0CADC'],
    ['Dusty Rose', '#C5A0AA', '#F2DCC8'],
    ['Champagne', '#D0BA9C', '#FFF0CD'],
    ['Sea Glass', '#8EBFC1', '#D7E8DA'],
    ['Smoky Slate', '#8999AB', '#E7CBBF'],
  ]);
  expect(THEMES.map((t) => t.id)).toEqual(THEME_IDS);
  for (const theme of THEMES)
    for (const key of [
      '--surface',
      '--border',
      '--accent',
      '--ring',
      '--glow',
      '--text',
      '--shadow',
      '--hover',
    ])
      expect(theme.style).toHaveProperty(key);
});
