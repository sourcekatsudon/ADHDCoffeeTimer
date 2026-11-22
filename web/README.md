# ADHD Coffee Time — Google account dashboard (Supabase Postgres)

Public site: https://adhd-coffee-focus.vercel.app

## Architecture

- **Next.js on Vercel**: Google OAuth (Auth.js), account dashboard, device
  pairing API, read-only developer token API.
- **Supabase PostgreSQL**: private JSONB documents, one document per PC's
  rolling activity history, per-code one-time pairing records and hashed
  token verifiers. No Vercel Blob is used.
- **Desktop Tauri client**: unchanged sync endpoint and device token protocol.
  Records locally; only linked PCs upload their measured 15-minute time cells.
- **Google remains the identity provider**. Supabase Auth is not used:
  authenticated user identity is handled by the Vercel backend.

The database intentionally uses one small table rather than introducing a
complex relational migration. This preserves the previously implemented
account-scoping and allows a low-query-volume migration from Blob. Device
record overwrites are atomic upserts, and pairing uses a PostgreSQL function
with a row lock to guarantee single use even if requests race.

## Required one-time setup

### 1. Create a Supabase project

Open https://supabase.com/dashboard and create/select a project near your
Vercel Functions region. In **SQL Editor**, run the full contents of:

[`supabase/migrations/0001_focus_documents.sql`](supabase/migrations/0001_focus_documents.sql)

This creates `public.focus_documents`, a prefix-lookup index, denies
`anon` and `authenticated` roles, enables RLS, and installs the atomic
`focus_redeem_pairing` function callable only by `service_role`.

Do **not** add anonymous row access policies to this table. All data access
must be mediated by the authorized Next.js server routes.

### 2. Connect Supabase secrets to Vercel

In Vercel → project `adhd-coffee-focus` → Settings → Environment Variables,
add these **server-only** variables for Production (Preview if wanted):

| Name | Value |
| --- | --- |
| `SUPABASE_URL` | `https://<project-ref>.supabase.co` |
| `SUPABASE_SECRET_KEY` | Supabase **secret** API key (`sb_secret_...`) |

Find both under Supabase Dashboard → Connect or Project Settings → API Keys.
Prefer the new **secret** key, not the legacy `service_role` JWT.
Never use `NEXT_PUBLIC_` for this key, never commit it to GitHub,
and never enter it into the Tauri desktop app.

The connector previously lacked permission to provision a Blob store, and
there is currently no connected Supabase admin action. Creating the Supabase
project, executing SQL and installing the secret must be done in the owner's
Supabase/Vercel account. The app cannot create these credentials itself.

### 3. Configure Google OAuth (same as before)

Visit https://console.cloud.google.com/apis/credentials and configure
Google Auth Platform's OAuth consent screen. Create a **Web application**
OAuth 2.0 client:

- Authorized JavaScript origin: `https://adhd-coffee-focus.vercel.app`
- Authorized redirect URI:
  `https://adhd-coffee-focus.vercel.app/api/auth/callback/google`
- Only `openid email profile` scopes. When testing, add your account
  as an allowed test user.

Set `AUTH_GOOGLE_ID` and `AUTH_GOOGLE_SECRET` in Vercel.
`AUTH_SECRET` is already present in the Vercel project.

### 4. Redeploy and verify

Redeploy the production build after adding environment variables, then visit
`https://adhd-coffee-focus.vercel.app/api/health`. Both
`oauthConfigured` and `privateStorageConfigured` should be `true`,
and `storageBackend` should be `supabase-postgres`.

Sign in, create a pairing code, pair a first PC in the Tauri Settings, use
**Sync now**, then check the account dashboard. Pair a second PC with a new
code. Finally issue a developer token to test `/api/v1/activity`.
A green health check only verifies the environment variable presence; the
above pairing and read/write checks verify actual database permissions.

## How data is stored

One row in `focus_documents` is keyed by an opaque path; the value is JSONB.
- `focus/v1/users/<hashed-google-sub>/devices/<device-id>.json`:
  current device snapshot, refreshed by each upload.
- `focus/v1/pair/<sha256-of-code>.json`: short-lived single-use pairing.
- `focus/v1/token/<sha256-of-device-token>.json`: per-PC write token.
- `focus/v1/users/<hashed-google-sub>/developer-tokens/<id>.json`:
  developer token metadata.
- `focus/v1/developer-verifiers/<sha256-of-secret>.json`:
  read-only developer key verifier.

Only the server gets `SUPABASE_SECRET_KEY`; the frontend and desktop never
talk directly to Supabase. User identity is derived from the Google subject,
never from a user-supplied email, device ID, or query parameter.

The server uses indexed Supabase queries for per-account device/token lists
instead of fetching and listing many Blob objects. One upsert replaces a
device snapshot. The PostgreSQL function consumes a pairing code and issues
the device token in one transaction, preventing concurrent code reuse.
Disconnected PCs stop uploading, but previously uploaded entries remain
until overwritten/removed; adding account-level deletion is future work.

Each PC keeps local history even if offline. History is uploaded only after
the device has been explicitly paired and Focus History is enabled.
Network failures do not destroy local history. The account dashboard merges up to 30 days of 15-minute cells, capping each slot at 900 seconds.
Overlapping devices are approximate because only aggregated slot seconds
are available, not second-by-second overlap data.

## Validation and tests

```bash
cd web
npm install
cp .env.example .env.local
npm test
npm run build
```

The test suite checks per-PC aggregation and API token behavior; production
pairing and DB isolation need a configured Supabase test project to verify.

## Developer API

Logged-in users can create, list and revoke read-only developer tokens.
See [API.md](API.md) for the endpoints and examples.
Web routes: `/api/auth/[...nextauth]`, `/api/pair`,
`/api/device/redeem`, `/api/sync`, `/api/history`,
`/api/tokens`, `/api/tokens/[id]`, `/api/v1/activity`,
and `/api/health`.

## Migration note

The earlier implementation used Vercel Blob. As that private store was not
connected when the dashboard/API were built, no live Blob activity records
are expected to exist. This migration therefore **switches the backend**
rather than copying user data. If a Blob store was later connected and used,
preserve its contents and migrate before deleting it.

## 7日間／30日間の集中履歴

Webダッシュボードの作業記録は、**7日間**と**30日間**の表示切替に対応します。
96個の15分セルは、0:00から23:45までの1日分で、24:00はセル列の右端を示す目盛りです。
1日内のセル間隔と日ごとの行間は以前の約半分にしています。

PCアプリの保存期間を32日に延長し、Vercelの同期APIも32日分（最大3,500セル）を受け付けます。
Supabase PostgreSQLは1台ごとに端末履歴を保存し、Webは日別に統合します。
旧バージョンのPCから8日程度の短い履歴が届いても、同期範囲より古い蓄積済みの記録を保護します。
ただし、新PC版で送信した範囲内で消去したセルは、次回同期時にクラウド側からも消えます。

**注意：以前のPC版がすでに破棄した古い記録は復元できません。**
今後30日分を保持するには、各PCに新しいWindows版をインストールしてください。
