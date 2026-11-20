import { describe, expect, it } from 'vitest';
import { renderToStaticMarkup } from 'react-dom/server';
import { MiniHistory, HistoryView } from './history';
import { initial } from './model';

describe('focus history presentation', () => {
  it('keeps exactly three activity rows with no space-consuming heading', () => {
    const markup = renderToStaticMarkup(<MiniHistory state={initial} />);
    expect((markup.match(/class="history-mini-row"/g) ?? []).length).toBe(3);
    expect((markup.match(/class="history-mini-cell"/g) ?? []).length).toBe(144);
    expect(markup).not.toContain('history-mini-title');
    expect(markup).toContain('aria-label="作業の履歴（3日間）"');
  });

  it('marks only the earned quarter-hour cell for the completion animation', () => {
    const today = new Date();
    today.setHours(14, 0, 0, 0);
    const start = today.getTime() / 1000;
    const state = {
      ...initial,
      completed: 2,
      rewardSlot: start,
      history: [{ start, seconds: 720 }],
    };
    const markup = renderToStaticMarkup(<HistoryView state={state} />);
    expect((markup.match(/history-popup-row"/g) ?? []).length).toBe(3);
    expect((markup.match(/class="history-popup-cell/g) ?? []).length).toBe(288);
    expect((markup.match(/just-completed/g) ?? []).length).toBe(1);
    expect(markup).toContain('data-level="4"');
  });
});
