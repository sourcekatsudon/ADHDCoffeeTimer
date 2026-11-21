'use client';
import { useCallback, useEffect, useMemo, useState } from 'react';
import DeveloperTokens from './developer-tokens';
import { buildActivityDays, type ActivityRange } from '../lib/activity-grid';
type Cell = { start: number; seconds: number };
type Device = { id: string; name: string; updatedAt: number };
type HistoryResult = { history: Cell[]; totalSeconds: number; devices: Device[]; error?: string };
const formatMins = (seconds: number) => {
  const mins = Math.round(seconds / 60);
  return Math.floor(mins / 60) + '時間 ' + (mins % 60) + '分';
};
const dateLabel = (date: Date) =>
  (date.getMonth() + 1) + '/' + date.getDate() + ' ' + '日月火水木金土'[date.getDay()];
const n = (number: number) => String(number).padStart(2, '0');
export default function Dashboard({ name, email, storageConfigured }: {
  name: string; email: string; storageConfigured: boolean;
}) {
  const [record, setRecord] = useState<HistoryResult>({ history: [], totalSeconds: 0, devices: [] });
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(true);
  const [linking, setLinking] = useState(false);
  const [code, setCode] = useState('');
  const [expiry, setExpiry] = useState(0);
  const [range, setRange] = useState<ActivityRange>(7);
  const reload = useCallback(async () => {
    try {
      const response = await fetch('/api/history', { cache: 'no-store' });
      const result = await response.json() as HistoryResult;
      if (!response.ok) throw new Error(result.error ?? '履歴の取得に失敗しました');
      setRecord(result);
      setError('');
    } catch (e) { setError(String(e instanceof Error ? e.message : e)); }
    finally { setLoading(false); }
  }, []);
  useEffect(() => {
    if (!storageConfigured) { setLoading(false); return; }
    void reload();
    const id = setInterval(() => void reload(), 120_000);
    return () => clearInterval(id);
  }, [reload, storageConfigured]);
  const days = useMemo(() => buildActivityDays(record.history, range), [record.history, range]);
  const today = days[0]?.seconds ?? 0;
  const issueCode = async () => {
    setLinking(true);
    try {
      const res = await fetch('/api/pair', { method: 'POST' });
      const data = await res.json() as { code?: string; expiresAt?: number; error?: string };
      if (!res.ok || !data.code) throw new Error(data.error ?? '連携コードを作れませんでした');
      setCode(data.code);
      setExpiry(data.expiresAt ?? 0);
      setError('');
    } catch (e) { setError(String(e instanceof Error ? e.message : e)); }
    finally { setLinking(false); }
  };
  return (
    <div className="dashboard">
      <div className="greeting"><div><div className="eyebrow">YOUR FOCUS ACTIVITY</div>
        <h1>おかえりなさい<span className="user-name">{name ? '、' + name : ''}</span></h1>
        <p>{email} · このGoogleアカウントに連携したPCの履歴</p></div>
        <button className="tiny-button" onClick={() => void reload()} disabled={loading}>更新 <span aria-hidden="true">↻</span></button>
      </div>
      <div className="metrics">
        <div className="metric"><span>今日の集中時間</span><strong>{formatMins(today)}</strong></div>
        <div className="metric"><span>直近{range}日間</span><strong>{formatMins(days.reduce((a, d) => a + d.seconds, 0))}</strong></div>
        <div className="metric"><span>連携しているPC</span><strong>{record.devices.length}<small> 台</small></strong></div>
      </div>
      {!storageConfigured && <div className="warning">クラウド保存領域が未接続です。VercelプロジェクトにSupabase PostgreSQLを接続してください。</div>}
      {error && <div className="warning" role="alert">{error}</div>}
      <section className="panel">
        <header className="panel-heading"><div><h2>作業の記録</h2><p>直近{range}日間 · 15分ごとの集中履歴 · 重複するPCは1枠15分を上限に集計</p></div>
          <div className="panel-actions">
            <div className="range-switch" role="group" aria-label="作業記録の表示期間">
              <button type="button" className={range === 7 ? 'active' : ''} aria-pressed={range === 7} onClick={() => setRange(7)}>7日間</button>
              <button type="button" className={range === 30 ? 'active' : ''} aria-pressed={range === 30} onClick={() => setRange(30)}>30日間</button>
            </div>
            <span className="panel-label">{range === 30 ? 'MONTHLY' : 'WEEKLY'}</span>
          </div>
        </header>
        <div className="activity-scroll"><div className="activity">
          <div className="ticks" aria-hidden="true">
            <span />
            <div className="tick-track">{[0, 6, 12, 18, 24].map(h =>
              <span key={h} style={{ left: (h / 24 * 100) + '%' }}>{h}:00</span>
            )}</div>
            <span />
          </div>
          {days.map((day, index) => <div className="activity-row" key={day.day.getTime()}>
            <div className="day-label"><b>{dateLabel(day.day)}</b><small>{index === 0 ? '今日' : index === 1 ? '昨日' : ''}</small></div>
            <div className="cell-row">{day.cells.map((cell, i) =>
              <span key={cell.start} className="cell" data-level={cell.seconds ? Math.max(1, Math.ceil(cell.seconds / 225)) : 0}
                title={dateLabel(day.day) + ' ' + n(Math.floor(i / 4)) + ':' + n(i % 4 * 15) + ' · ' + Math.round(cell.seconds / 60) + '分'} />
            )}</div>
            <div className="day-total">{formatMins(day.seconds)}</div>
          </div>)}
        </div></div>
        <div className="legend"><span>{loading ? '読み込み中…' : '各セルにカーソルを重ねると分単位で確認できます'}</span><div className="legend-cells">{[0,1,2,3,4].map(x => <i key={x} data-level={x} />)}</div></div>
      </section>
      <div className="bottom-grid">
        <section className="panel"><h2>PCを追加する</h2><p>Windows版アプリの「設定 → クラウド同期」にある連携コード欄へ入力してください。GoogleのパスワードをPCアプリへ渡す必要はありません。</p>
          {!code ? <button className="primary-button" disabled={linking || !storageConfigured} onClick={() => void issueCode()}>{linking ? '発行中…' : '連携コードを発行'}</button> :
            <div className="pairing">
              <div className="pair-code">{code}</div>
              <button className="tiny-button" onClick={() => void navigator.clipboard.writeText(code)}>コピー</button>
              <small>{new Date(expiry).toLocaleTimeString('ja-JP', { hour:'2-digit', minute:'2-digit' })} まで有効 · 1回だけ使用可能</small>
              <button className="text-button" onClick={() => void issueCode()}>新しいコードを発行</button>
            </div>}
        </section>
        <section className="panel"><h2>連携中のPC</h2>{record.devices.length ? <div className="devices">
            {record.devices.map(d => <div className="device" key={d.id}><span className="device-icon">▣</span><div><b>{d.name}</b><small>最終同期：{new Date(d.updatedAt).toLocaleString('ja-JP')}</small></div></div>)}
          </div> : <p>まだPCが連携されていません。連携すると、作業履歴がここに表示されます。</p>}
        </section>
      </div>
      <DeveloperTokens storageConfigured={storageConfigured} />
      <p className="privacy-note">🔒 入力内容・アプリ名・閲覧サイトは送信しません。GoogleアカウントのIDをハッシュ化して、PCごとの稼働時間だけをSupabase PostgreSQLに保存します。</p>
    </div>
  );
}
