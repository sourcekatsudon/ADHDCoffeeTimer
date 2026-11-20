import React, { useMemo, useState } from 'react';
import { command, clearHistorySlot } from './bridge';
import type { Snapshot } from './model';

const QUARTER = 900;
const HOURS = [0, 6, 12, 18, 24];
const localDate = (dayOffset: number): Date => {
  const day = new Date();
  day.setHours(0, 0, 0, 0);
  day.setDate(day.getDate() - dayOffset);
  return day;
};
const hourLabel = (date: Date, lang: string) =>
  lang === 'ja'
    ? (date.getMonth() + 1) + '/' + date.getDate() + ' ' + '日月火水木金土'[date.getDay()]
    : new Intl.DateTimeFormat(lang, { month: 'numeric', day: 'numeric' }).format(date);
const heading = (lang: string, count = 3) => lang === 'ja' ? `作業の履歴（${count}日間）` : `Focus history (${count} days)`;

type Slot = { start: number; seconds: number };
function useHistory(state: Snapshot, dayCount: number) {
  return useMemo(() => {
    const values = new Map(state.history.map((entry) => [entry.start, entry.seconds]));
    return Array.from({ length: dayCount }, (_, i) => {
      const date = localDate(i);
      const midnight = date.getTime() / 1000;
      const slots: Slot[] = Array.from({ length: 96 }, (_, index) => {
        const local = new Date(date);
        local.setHours(Math.floor(index / 4), (index % 4) * 15, 0, 0);
        const start = local.getTime() / 1000;
        return { start, seconds: values.get(start) ?? 0 };
      });
      return { date, midnight, slots };
    });
  }, [state.history, dayCount]);
}
function intensity(seconds: number, max: number) {
  return seconds <= 0 ? 0 : Math.max(1, Math.min(4, Math.ceil((seconds / max) * 4)));
}

export function MiniHistory({ state, studio = false }: { state: Snapshot; studio?: boolean }) {
  const days = useHistory(state, 3);
  if (!state.settings.historyEnabled) return null;
  return (
    <button
      type="button"
      className="history-mini"
      aria-label={heading(state.settings.language)}
      title={state.settings.language === 'ja' ? 'マウスを重ねると履歴を拡大' : 'Hover to expand history'}
      onMouseEnter={() => { if (!studio) void command('history-show'); }}
      onMouseLeave={() => { if (!studio) void command('history-leave'); }}
      onFocus={() => { if (!studio) void command('history-show'); }}
      onBlur={() => { if (!studio) void command('history-leave'); }}
      onClick={() => { if (!studio) void command('history-show'); }}
    >
      <span className="history-mini-axis"><span />{HOURS.map(h => <span key={h}>{h}</span>)}</span>
      {days.map((day, index) => (
        <span className="history-mini-row" key={day.midnight}>
          <span className="history-mini-day">{state.settings.language === 'ja' ? ['今日','昨日','一昨日'][index] : ['Now','-1d','-2d'][index]}</span>
          <span className="history-mini-cells">
            {Array.from({ length: 48 }, (_, hourHalf) => {
              const secs = day.slots[hourHalf * 2].seconds + day.slots[hourHalf * 2 + 1].seconds;
              return <span className="history-mini-cell" data-level={intensity(secs, 1800)} key={hourHalf} />;
            })}
          </span>
        </span>
      ))}
    </button>
  );
}

export function HistoryView({ state }: { state: Snapshot }) {
  const [selected, setSelected] = useState<Slot | null>(null);
  const [error, setError] = useState('');
  const days = useHistory(state, 7);
  const lang = state.settings.language;
  const totalToday = Math.round(days[0].slots.reduce((acc, slot) => acc + slot.seconds, 0) / 60);
  const todaySummary = lang === 'ja'
    ? '今日 ' + Math.floor(totalToday / 60) + '時間' + totalToday % 60 + '分'
    : 'Today ' + Math.floor(totalToday / 60) + 'h ' + totalToday % 60 + 'm';
  const selectedDate = selected ? new Date(selected.start * 1000) : null;
  const clearedLabel = lang === 'ja' ? '光るセルをクリックすると記録を消去' : 'Click a lit cell to clear';
  return (
    <main
      className="history-popup glass"
      onMouseEnter={() => void command('history-show')}
      onMouseLeave={() => void command('history-leave')}
      dir="ltr"
    >
      <header className="history-popup-heading">
        <div><strong>{heading(lang, 7)}</strong><p>{todaySummary} · {lang === 'ja' ? 'タイマーの実稼働時間' : 'Actual timer running time'}</p></div>
        <button className="history-popup-close" aria-label={lang === 'ja' ? '閉じる' : 'Close'} onClick={() => void command('history-leave')}>×</button>
      </header>
      <div className="history-popup-axis"><span />{HOURS.map(h => <span key={h}>{h}</span>)}</div>
      <div className="history-popup-days">
        {days.map((day) => (
          <div className="history-popup-row" key={day.midnight}>
            <div className="history-popup-day">{hourLabel(day.date, lang)}</div>
            <div className="history-popup-cells">
              {day.slots.map((slot, index) => (
                <button
                  type="button"
                  key={state.rewardSlot === slot.start ? slot.start + '-reward-' + state.completed : slot.start}
                  className={'history-popup-cell' + (state.rewardSlot === slot.start ? ' just-completed' : '')}
                  data-level={intensity(slot.seconds, QUARTER)}
                  aria-label={hourLabel(day.date, lang) + ' ' + String(Math.floor(index / 4)).padStart(2, '0') + ':' + String(index % 4 * 15).padStart(2, '0') + ' ' + Math.round(slot.seconds) + 's'}
                  title={slot.seconds > 0 ? Math.round(slot.seconds / 60) + ' min · ' + Math.round(slot.seconds / QUARTER * 100) + '%' : ''}
                  onMouseEnter={() => setSelected(slot)}
                  onFocus={() => setSelected(slot)}
                  onClick={() => {
                    if (slot.seconds <= 0) return;
                    clearHistorySlot(slot.start).catch(e => setError(String(e)));
                  }}
                />
              ))}
            </div>
          </div>
        ))}
      </div>
      <footer className="history-popup-footer">
        <span>{selected && selected.seconds > 0 && selectedDate
          ? String(selectedDate.getHours()).padStart(2,'0') + ':' + String(selectedDate.getMinutes()).padStart(2,'0') + ' · ' + Math.round(selected.seconds / 60) + (lang === 'ja' ? ' 分' : ' min') + ' (' + Math.round(selected.seconds / QUARTER * 100) + '%)'
          : clearedLabel}</span>
        <span className="history-popup-legend">{[0, 1, 2, 3, 4].map(level => <i key={level} data-level={level} />)}</span>
      </footer>
      {error && <div className="history-popup-error" role="alert">{error}</div>}
    </main>
  );
}
