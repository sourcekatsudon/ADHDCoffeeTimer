import { auth, signIn, signOut } from '../auth';
import Dashboard from './dashboard';
import { storeReady } from '../lib/store';
export const dynamic = 'force-dynamic';
export default async function Home() {
  const user = (await auth())?.user as { id?: string; name?: string | null; email?: string | null } | undefined;
  const ready = Boolean(process.env.AUTH_GOOGLE_ID && process.env.AUTH_GOOGLE_SECRET && process.env.AUTH_SECRET);
  return (
    <main className="page">
      <div className="page-glow" aria-hidden="true" />
      <header className="site-header"><div className="brand"><span className="brand-symbol">☕</span><span>ADHD Coffee Time <b>Focus</b></span></div>
        {user?.id && <form action={async () => { 'use server'; await signOut(); }}><button className="tiny-button">ログアウト</button></form>}
      </header>
      <div className="wrap">
        {!user?.id ? (
          <section className="hero">
            <div className="eyebrow">YOUR FOCUS, ACROSS DEVICES</div>
            <h1>どのPCで作業しても、<br /><span>集中の記録はひとつに。</span></h1>
            <p>ADHD Coffee Timerの作業履歴を、Googleアカウントごとに統合。<br />本体の操作記録やキー入力は送らず、15分ごとの稼働秒数だけを保存します。</p>
            {ready ? <form action={async () => { 'use server'; await signIn('google', { redirectTo: '/' }); }}>
              <button className="google-button" type="submit"><span className="g-letter">G</span> Googleでログイン</button>
            </form> : <div className="setup">Google OAuthの接続待ちです。管理者がGoogle CloudのクライアントIDとシークレットを設定するとログインできます。</div>}
            <p className="hero-fine">あなたの履歴は、あなたのGoogleアカウントでログインしたときだけ表示されます。</p>
          </section>
        ) : (
          <Dashboard name={user.name ?? ''} email={user.email ?? ''} storageConfigured={storeReady()} />
        )}
      </div>
      <footer className="site-footer">ADHD Coffee Time · Private focus history · Made to notice the work you did</footer>
    </main>
  );
}
