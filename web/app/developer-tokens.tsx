'use client';

import { useCallback, useEffect, useState } from 'react';

type ApiTokenMeta = {
  id: string;
  name: string;
  scope: 'activity:read';
  prefix: string;
  createdAt: number;
  expiresAt: number;
  revokedAt: number | null;
};
type TokensResponse = { tokens?: ApiTokenMeta[]; error?: string };

const formatDate = (ms: number) =>
  new Date(ms).toLocaleString('ja-JP', { year: 'numeric', month: '2-digit', day: '2-digit' });

export default function DeveloperTokens({ storageConfigured }: { storageConfigured: boolean }) {
  const [tokens, setTokens] = useState<ApiTokenMeta[]>([]);
  const [name, setName] = useState('開発用トークン');
  const [days, setDays] = useState(90);
  const [secret, setSecret] = useState('');
  const [error, setError] = useState('');
  const [busy, setBusy] = useState(false);
  const [loaded, setLoaded] = useState(false);

  const refresh = useCallback(async () => {
    if (!storageConfigured) return;
    const response = await fetch('/api/tokens', { cache: 'no-store' });
    const body = await response.json() as TokensResponse;
    if (!response.ok) throw new Error(body.error ?? 'トークンの取得に失敗しました');
    setTokens(body.tokens ?? []);
  }, [storageConfigured]);

  useEffect(() => {
    if (!storageConfigured) { setLoaded(true); return; }
    void refresh().catch(e => setError(String(e))).finally(() => setLoaded(true));
  }, [refresh, storageConfigured]);

  async function createToken() {
    setBusy(true); setError('');
    try {
      const response = await fetch('/api/tokens', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, expiresInDays: days }),
      });
      const data = await response.json() as { token?: string; error?: string };
      if (!response.ok || !data.token) throw new Error(data.error ?? '発行に失敗しました');
      // The secret remains only in this component's state. The backend never
      // returns it again, not even from the token listing endpoint.
      setSecret(data.token);
      await refresh();
    } catch (e) { setError(e instanceof Error ? e.message : String(e)); }
    finally { setBusy(false); }
  }

  async function revokeToken(token: ApiTokenMeta) {
    if (!window.confirm('「' + token.name + '」のAPIアクセスを無効にしますか？')) return;
    setBusy(true); setError('');
    try {
      const response = await fetch('/api/tokens/' + encodeURIComponent(token.id), { method: 'DELETE' });
      const data = await response.json() as { error?: string };
      if (!response.ok) throw new Error(data.error ?? '失効処理に失敗しました');
      await refresh();
    } catch (e) { setError(e instanceof Error ? e.message : String(e)); }
    finally { setBusy(false); }
  }

  return <section className="panel developer-api-panel">
    <header className="panel-heading">
      <div>
        <h2>開発者API</h2>
        <p>自作ツールから、このGoogleアカウントのPC別・合計稼働時間を取得できます。読み取り専用です。</p>
      </div>
      <span className="panel-label">API v1</span>
    </header>

    <div className="api-panel-grid">
      <div>
        <h3>アクセストークンを発行</h3>
        <p>PCごとの同期キーとは別物です。トークンを共有した相手は稼働時間を閲覧できるため、必要な相手だけに渡してください。</p>
        <div className="api-issue-controls">
          <label>用途・名前
            <input maxLength={48} value={name} onChange={e => setName(e.target.value)}
              autoComplete="off" placeholder="例：自分の分析スクリプト" />
          </label>
          <label>有効期間
            <select value={days} onChange={e => setDays(Number(e.target.value))}>
              <option value={7}>7日</option><option value={30}>30日</option>
              <option value={90}>90日</option><option value={365}>365日</option>
            </select>
          </label>
          <button type="button" className="primary-button" onClick={() => void createToken()}
            disabled={busy || !storageConfigured || !name.trim()}>発行する</button>
        </div>
        {secret && <div className="api-secret" role="status">
          <strong>新しいトークン（表示は今回限り）</strong>
          <code>{secret}</code>
          <div className="api-secret-actions">
            <button type="button" className="tiny-button" onClick={() => void navigator.clipboard.writeText(secret)}>コピー</button>
            <button type="button" className="tiny-button" onClick={() => setSecret('')}>閉じる</button>
          </div>
          <small>再表示はできません。紛失した場合は新しいトークンを発行してください。</small>
        </div>}
      </div>

      <div>
        <h3>発行済みトークン</h3>
        {!loaded ? <p>取得中…</p> : tokens.length === 0 ? <p>まだ発行されていません。</p> :
        <div className="api-tokens-list">{tokens.map(token => {
          const inactive = !!token.revokedAt || token.expiresAt <= Date.now();
          return <div className="api-token-row" key={token.id}>
            <div>
              <strong>{token.name}</strong>
              <small>{token.prefix}… · {token.scope}</small>
              <small>有効期限：{formatDate(token.expiresAt)} {inactive ? '（失効）' : ''}</small>
            </div>
            {!inactive && <button type="button" className="tiny-button" disabled={busy}
              onClick={() => void revokeToken(token)}>失効</button>}
          </div>;
        })}</div>}
      </div>
    </div>
    {error && <div className="warning" role="alert">{error}</div>}
    <div className="api-usage">
      <strong>APIの使い方</strong>
      <p>読み取りにはHTTP Bearerトークンを指定します。サーバー側の処理から呼ぶことを推奨します。</p>
      <pre>{'curl -H "Authorization: Bearer ${FOCUS_API_TOKEN}" "https://adhd-coffee-focus.vercel.app/api/v1/activity?days=7"'}</pre>
      <p>全PCの統合時間・PC別合計を返します。<code>detail=slots</code> で15分刻みのデータ、<code>deviceId=...</code> で1台だけ取得できます。</p>
      <a href="https://github.com/sourcekatsudon/ADHDCoffeeTimer/blob/feat/focus-history-reward/web/API.md"
         target="_blank" rel="noopener noreferrer">API仕様書を開く ↗</a>
    </div>
  </section>;
}
