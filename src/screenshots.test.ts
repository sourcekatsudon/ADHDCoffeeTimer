import { describe, expect, it } from 'vitest';

const PNG_SIGNATURE = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

type NodeFs = {
  existsSync(path: string): boolean;
  readFileSync(path: string): Uint8Array;
  readFileSync(path: string, encoding: 'utf8'): string;
};
type NodePath = {
  dirname(p: string): string;
  join(...paths: string[]): string;
  resolve(...paths: string[]): string;
};
type NodeUrl = {
  fileURLToPath(url: string): string;
};

async function nodeModules() {
  const fsId = 'node:fs';
  const pathId = 'node:path';
  const urlId = 'node:url';
  const [fs, path, url] = await Promise.all([
    import(/* @vite-ignore */ fsId) as Promise<NodeFs>,
    import(/* @vite-ignore */ pathId) as Promise<NodePath>,
    import(/* @vite-ignore */ urlId) as Promise<NodeUrl>,
  ]);
  const root = path.resolve(path.dirname(url.fileURLToPath(import.meta.url)), '..');
  return { fs, path, root };
}

function readPngHeader(bytes: Uint8Array) {
  expect(bytes.byteLength).toBeGreaterThan(26);
  for (let i = 0; i < PNG_SIGNATURE.length; i++) {
    expect(bytes[i]).toBe(PNG_SIGNATURE[i]);
  }
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  return {
    bytes: bytes.byteLength,
    width: view.getUint32(16, false),
    height: view.getUint32(20, false),
    bitDepth: bytes[24],
    colorType: bytes[25],
  };
}

function localMarkdownImages(markdown: string) {
  return [...markdown.matchAll(/!\[([^\]]*)\]\(([^)]+)\)/g)]
    .map(([, alt, target]) => ({ alt, target }))
    .filter(({ target }) => !target.startsWith('https://img.shields.io/'));
}

describe('README screenshot references and high-DPI PNG assets', () => {
  it('links README.md and docs/readme/ja.md to local lossless PNG screenshots with truthful browser-preview captions', async () => {
    const { fs, path, root } = await nodeModules();
    const enPath = path.join(root, 'README.md');
    const jaPath = path.join(root, 'docs', 'readme', 'ja.md');
    const enDoc = fs.readFileSync(enPath, 'utf8');
    const jaDoc = fs.readFileSync(jaPath, 'utf8');

    expect(enDoc).toMatch(/browser previews/i);
    expect(jaDoc).toMatch(/ブラウザで描画したプレビュー/);

    const enImages = localMarkdownImages(enDoc);
    const jaImages = localMarkdownImages(jaDoc);

    expect(enImages.filter((i) => i.target === 'docs/screenshots/themes-en.png')).toHaveLength(2);
    expect(enImages.some((i) => i.target === 'docs/screenshots/timer-popup-en.png')).toBe(true);
    expect(enImages.some((i) => i.target === 'docs/screenshots/history-popup-en.png')).toBe(true);
    expect(enImages.some((i) => i.target.endsWith('.webp'))).toBe(false);

    expect(jaImages.filter((i) => i.target === '../screenshots/themes-ja.png')).toHaveLength(1);
    expect(jaImages.some((i) => i.target === '../screenshots/timer-popup-ja.png')).toBe(true);
    expect(jaImages.some((i) => i.target === '../screenshots/history-popup-ja.png')).toBe(true);
    expect(jaImages.some((i) => i.target.endsWith('.webp'))).toBe(false);

    for (const [docPath, images] of [
      [enPath, enImages],
      [jaPath, jaImages],
    ] as const) {
      for (const img of images) {
        expect(img.target).not.toMatch(/^https?:\/\//i);
        expect(fs.existsSync(path.resolve(path.dirname(docPath), img.target))).toBe(true);
      }
    }
  });

  it('stores 2x high-DPI lossless PNG gallery and popup captures with exact expected dimensions', async () => {
    const { fs, path, root } = await nodeModules();
    const expected: Array<[string, number, number]> = [
      ['themes-en.png', 3200, 1400],
      ['themes-ja.png', 3200, 1400],
      ['timer-popup-en.png', 1800, 1400],
      ['timer-popup-ja.png', 1800, 1400],
      ['history-popup-en.png', 2200, 1400],
      ['history-popup-ja.png', 2200, 1400],
    ];
    for (const [name, width, height] of expected) {
      const header = readPngHeader(fs.readFileSync(path.join(root, 'docs', 'screenshots', name)));
      expect(header.width).toBe(width);
      expect(header.height).toBe(height);
      expect(header.bitDepth).toBe(8);
      expect(header.bytes).toBeGreaterThan(500_000);
    }
  });

  it('includes macOS system font fallback before CJK fallback in style.css to avoid capsule/popup clipping', async () => {
    const { fs, path, root } = await nodeModules();
    const css = fs.readFileSync(path.join(root, 'src', 'style.css'), 'utf8');
    expect(css).toMatch(/'Segoe UI',\s*-apple-system,\s*BlinkMacSystemFont/);
  });
});
