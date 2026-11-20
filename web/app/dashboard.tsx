'use client';
import { useCallback, useEffect, useMemo, useState } from 'react';
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
function dayCells(date: Date, lookup: Map<number, number>) {
  return Array.from({ length: 96 }, (_, i) => {
    const start = new Date(date);
    start.setHours(Math.floor(i / 4), (i % 4) * 15, 0, 0);
    const unix = start.getTime() / 1000;
    const seconds = lookup.get(unix) ?? 0;
    return { start: unix, seconds };
  });
}
export default function Dashboard({ name, email, storageConfigured }: {
  name: string; email: string; storageConfigured: boolean;
}) {
  const [record, setRecord] = useState<HistoryResult>({ history: [], totalSeconds: 0, devices: [] });
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(true);
  const [linking, setLinking] = useState(false);
  const [code, setCode] = useState('');
  const [expiry, setExpiry] = useState(0);
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
  const days = useMemo(() => {
    const lookup = new Map(record.history.map(v => [v.start, v.seconds]));
    return Array.from({ length: 7 }, (_, i) => {
      const day = new Date();
      day.setHours(0, 0, 0, 0);
      day.setDate(day.getDate() - i);
      const cells = dayCells(day, lookup);
      return { day, cells, seconds: cells.reduce((a, v) => a + v.seconds, 0) };
    });
  }, [record.history]);
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
        <div className="metric"><span>直近7日間</span><strong>{formatMins(days.reduce((a, d) => a + d.seconds, 0))}</strong></div>
        <div className="metric"><span>連携しているPC</span><strong>{record.devices.length}<small> 台</small></strong></div>
      </div>
      {!storageConfigured && <div className="warning">クラウド保存領域が未接続です。VercelプロジェクトにPrivate Blobを接続してください。</div>}
      {error && <div className="warning" role="alert">{error}</div>}
      <section className="panel">
        <header className="panel-heading"><div><h2>作業の記録</h2><p>直近7日間 · 15分ごとの集中履歴 · 重複するPCは1枠15分を上限に集計</p></div><span className="panel-label">WEEKLY</span></header>
        <div className="activity-scroll"><div className="activity">
          <div className="ticks"><span />{[0, 6, 12, 18, 24].map(h => <span key={h}>{h}:00</span>)}</div>
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
      <p className="privacy-note">🔒 入力内容・アプリ名・閲覧サイトは送信しません。GoogleアカウントのIDをハッシュ化して、PCごとの稼働時間だけをPrivate Blobに保存します。</p>
    </div>
  );
}
