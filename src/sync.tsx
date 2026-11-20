import { useState } from 'react';
import { emit } from '@tauri-apps/api/event';
import type { Snapshot } from './model';
import { native } from './bridge';

export const CLOUD_BASE = 'https://adhd-coffee-focus.vercel.app';
const KEY = 'adhd-coffee-sync-v1';
const DEVICE_KEY = 'adhd-coffee-sync-device-v1';
const LAST_KEY = 'adhd-coffee-sync-last-v1';
const MIN_INTERVAL_MS = 30 * 60 * 1000; // One upload per 30 minutes, only on linked PCs

type Connection = { token: string; deviceId: string; expiresAt: number };
const stored = (): Connection | null => {
  try {
    const value = JSON.parse(localStorage.getItem(KEY) ?? 'null') as Connection | null;
    return value && typeof value.token === 'string' && typeof value.deviceId === 'string' ? value : null;
  } catch { return null; }
};
function deviceId() {
  let id = localStorage.getItem(DEVICE_KEY);
  if (!id) {
    id = crypto.randomUUID();
    localStorage.setItem(DEVICE_KEY, id);
  }
  return id;
}
const asError = async (response: Response): Promise<Error> => {
  const data = await response.json().catch(() => null) as { error?: string } | null;
  return new Error(data?.error ?? 'HTTP ' + response.status);
};
export async function pairPc(code: string): Promise<void> {
  const response = await fetch(CLOUD_BASE + '/api/device/redeem', {
    method: 'POST', headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ code: code.trim().toUpperCase(), deviceId: deviceId() }),
  });
  if (!response.ok) throw await asError(response);
  const data = await response.json() as Connection;
  if (!/^[a-f0-9]{64}$/i.test(data.token)) throw new Error('Invalid device token');
  localStorage.setItem(KEY, JSON.stringify(data));
  localStorage.removeItem(LAST_KEY);
  if (native) void emit('focus-sync-link-changed').catch(() => {});
}
export function disconnectPc() {
  localStorage.removeItem(KEY);
  localStorage.removeItem(LAST_KEY);
  if (native) void emit('focus-sync-link-changed').catch(() => {});
}
export function connectedPc() {
  const c = stored();
  return c !== null && c.expiresAt > Date.now();
}
export function lastSyncAt(): number {
  return Number(localStorage.getItem(LAST_KEY) ?? '0') || 0;
}

let sending = false;
export async function syncNow(state: Snapshot, force = false): Promise<boolean> {
  if (!native || !state.settings.historyEnabled) return false;
  const connection = stored();
  if (!connection || Date.now() > connection.expiresAt) return false;
  if (sending) return false;
  if (!force && Date.now() - lastSyncAt() < MIN_INTERVAL_MS) return false;
  sending = true;
  try {
    const history = state.history.filter((row) =>
      row.start >= Math.floor(Date.now() / 1000) - 9 * 86400 && row.seconds > 0);
    const response = await fetch(CLOUD_BASE + '/api/sync', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', Authorization: 'Bearer ' + connection.token },
      body: JSON.stringify({
        deviceId: connection.deviceId,
        deviceName: localStorage.getItem(DEVICE_KEY + '-name') || 'Windows PC',
        history,
      }),
    });
    if (!response.ok) throw await asError(response);
    localStorage.setItem(LAST_KEY, String(Date.now()));
    return true;
  } finally { sending = false; }
}

export function SyncSettings({ state }: { state: Snapshot }) {
  const japanese = state.settings.language === 'ja';
  const [paired, setPaired] = useState(() => connectedPc());
  const [last, setLast] = useState(() => lastSyncAt());
  const [code, setCode] = useState('');
  const [name, setName] = useState(() => localStorage.getItem(DEVICE_KEY + '-name') || 'Windows PC');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const [message, setMessage] = useState('');
  async function act(operation: () => Promise<void>) {
    setBusy(true); setError(''); setMessage('');
    try { await operation(); }
    catch (e) { setError(e instanceof Error ? e.message : String(e)); }
    finally { setPaired(connectedPc()); setLast(lastSyncAt()); setBusy(false); }
  }
  return <section className="setting-group sync-setting">
    <div className="setting-row">
      <div>
        <span>{japanese ? 'クラウド同期（複数PC）' : 'Cloud sync (multiple PCs)'}</span>
        <small>{japanese ? 'Googleアカウントごとに作業履歴を統合。連携したPCだけ同期します。' : 'Merge focus history by Google account; opt-in on each PC.'}</small>
      </div>
      <span className={'sync-state' + (paired ? ' connected' : '')}>{paired ? (japanese ? '連携済み' : 'Linked') : (japanese ? '未接続' : 'Not linked')}</span>
    </div>
    <div className="sync-website"><a href={CLOUD_BASE} target="_blank" rel="noreferrer">
      {japanese ? 'Googleでログインして連携コードを発行 ↗' : 'Sign in with Google & get pairing code ↗'}
    </a><button type="button" className="sound-preview" onClick={() => void navigator.clipboard.writeText(CLOUD_BASE)}>
      {japanese ? 'URLコピー' : 'Copy URL'}
    </button></div>
    {!paired ? <>
      <div className="sync-connect-row">
        <label htmlFor="sync-code">{japanese ? '連携コード' : 'Pairing code'}</label>
        <input id="sync-code" autoComplete="off" spellCheck={false} inputMode="text"
          maxLength={16} placeholder="1234ABCD5678EF90" value={code}
          onChange={e => setCode(e.target.value.toUpperCase().replace(/[^0-9A-F]/g, ''))} />
        <button className="sound-preview" type="button" disabled={busy || !native || code.length !== 16}
          onClick={() => void act(async () => {
            await pairPc(code);
            setMessage(japanese ? '連携しました。' : 'Connected.');
            await syncNow(state, true);
            setLast(lastSyncAt());
            setCode('');
          })}>{busy ? '…' : (japanese ? '連携する' : 'Connect')}</button>
      </div>
    </> : <>
      <div className="sync-connect-row">
        <label htmlFor="sync-name">{japanese ? 'このPCの名前' : 'PC name'}</label>
        <input id="sync-name" maxLength={32} value={name} onChange={e => { setName(e.target.value); localStorage.setItem(DEVICE_KEY + '-name', e.target.value); }} />
      </div>
      <div className="sync-actions">
        <button className="sound-preview" type="button" disabled={busy || !state.settings.historyEnabled}
          onClick={() => void act(async () => {
            const sent = await syncNow(state, true);
            if (!sent) throw new Error('Could not start sync.');
            setMessage(japanese ? '同期しました。' : 'Synced.');
          })}>{japanese ? '今すぐ同期' : 'Sync now'}</button>
        <button className="sound-preview" type="button" onClick={() => {
          disconnectPc(); setPaired(false); setMessage(japanese ? 'このPCの連携を解除しました。' : 'Disconnected this PC.');
        }}>{japanese ? '連携解除' : 'Disconnect'}</button>
      </div>
      <small>{japanese ? '最終同期：' : 'Last synced: '}{last ? new Date(last).toLocaleString() : (japanese ? '未同期' : 'Never')}</small>
    </>}
    {!state.settings.historyEnabled && <small className="sync-error">{japanese ? '作業記録OFF中は同期しません。' : 'History is OFF, so sync is paused.'}</small>}
    {error && <small className="sync-error" role="alert">{error}</small>}
    {message && <small className="sync-message">{message}</small>}
    {!native && <small>{japanese ? 'クラウド同期はWindows/macOSアプリで利用できます。' : 'Cloud sync is available in the native app.'}</small>}
  </section>;
}
