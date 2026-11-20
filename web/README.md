# ADHD Coffee Time — account dashboard

Production URL: https://adhd-coffee-focus.vercel.app

## Required one-time account setup

The Next.js dashboard and native sync API are implemented and deployed, but
Google OAuth and private storage will not work until these resources are configured.

1. Open [Google Cloud Console](https://console.cloud.google.com/apis/credentials),
   select your project and configure **Google Auth Platform** (OAuth consent screen).
   Configure a **Web application OAuth 2.0 client** with:
   - Authorized JavaScript origin: `https://adhd-coffee-focus.vercel.app`
   - Authorized redirect URI: `https://adhd-coffee-focus.vercel.app/api/auth/callback/google`
   - Scopes: `openid`, `email`, `profile` (no Drive or Gmail access).
   Add your Google account as a test user if the consent app is in Testing mode.
2. In Vercel → project `adhd-coffee-focus` → Settings → Environment Variables,
   set `AUTH_GOOGLE_ID` and `AUTH_GOOGLE_SECRET` for **Production** (and Preview,
   if desired). `AUTH_SECRET` has already been set on the connected project.
   Never paste client secrets or blob tokens into the repository or chat.
3. In the same Vercel project, open **Storage → Create Database → Blob → Private**.
   Connect the new private Blob store to `adhd-coffee-focus` so Vercel adds
   `BLOB_READ_WRITE_TOKEN`. Do not create a **Public** Blob store for user history.
4. Redeploy the **production** build, then visit
   `https://adhd-coffee-focus.vercel.app/api/health`: both flags must be true.
   Sign in with Google, issue a pairing code, connect PC #1 and PC #2, and upload
   their histories. The website displays the two devices together.

**Connector limitation:** The connected Vercel integration can create the project
and deploy code but returned HTTP 403 for Blob store creation. The owner must
create and connect the private store in the Vercel dashboard.

## Data model and boundaries

- Auth.js v5 signs in with Google, uses stateless encrypted/signed sessions, and
  derives a stable account namespace from the provider's Google subject ID.
- The backend keeps token verifiers and per-PC activity snapshots in a **private**
  Blob store. No plaintext Google email is used in object path names.
- A 16-character one-time pairing code issued by the logged-in website lets each
  desktop app obtain a random device-scoped bearer token.
- The desktop sends only 15-minute slot timestamps and counted seconds. No
  keystrokes, window titles, processes, or URLs are uploaded.
- Devices upload a **full snapshot of their own seven-day history**; an upload from
  one PC cannot overwrite another PC's record. Clearing a local slot removes it
  from that PC's next uploaded snapshot.
- The web aggregator adds seconds across PCs, capped at 900 per slot. Without
  second-level source data, overlapping activity may be overcounted within a slot.
- The client uploads automatically at most once per 30 minutes, plus user-requested
  manual uploads, to control Hobby Blob write-operation usage.
- The device token is stored in the native app's local WebView storage, not a
  cloud plaintext setting. Keep the PC profile secure. Disconnecting locally
  prevents future uploads but does not erase previously uploaded history.
- Device tokens expire after 365 days; issue a new pairing code to re-pair.

## Development

```bash
cd web
npm install
cp .env.example .env.local
npm run dev
```

Server routes: `/api/auth/[...nextauth]`, `/api/pair`, `/api/device/redeem`,
`/api/sync`, `/api/history`, `/api/tokens`, `/api/tokens/[id]`,
`/api/v1/activity`, and `/api/health`.
All private history reads require Google session auth. Device writes use
bearer tokens and cross-origin CORS for the installed Tauri client.

## Read-only developer tokens

After logging in, open the **開発者API** panel to create a time-limited, read-only
account token or revoke it. The raw secret is displayed exactly once. API keys
are not PC pairing tokens and cannot be used to upload, edit, or delete history.

See [API.md](API.md) for authentication, query parameters, response schema,
usage examples, expiration, and revocation.
