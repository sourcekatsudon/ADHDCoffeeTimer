# Focus Activity API v1

**Base URL:** `https://adhd-coffee-focus.vercel.app`

## Purpose

Query your own Google-account focus history from scripts, internal dashboards, or
other developer tools. Tokens are issued only through the logged-in website.

The API returns **PC-by-PC totals**, **merged totals** and optionally **15-minute
cells**. It does **not** return the current desktop timer countdown, play/pause
state, keystrokes, open applications, browser activity or user credentials.

### Authentication

On the website, sign in with Google and open **開発者API**. Create a
read-only token with a descriptive name and one of 7/30/90/365-day lifetimes.
Up to 10 active tokens are allowed per Google account.

The token is shown **once only**. Store it as a secret in your backend's
environment settings rather than browser code, a URL parameter, a Git commit,
or a shared spreadsheet. It is stored server-side only as a one-way hash
identifier and token metadata. All requests use HTTPS.

```bash
export FOCUS_API_TOKEN='ftk_v1_...'
curl --fail-with-body \
  --header "Authorization: Bearer ${FOCUS_API_TOKEN}" \
  'https://adhd-coffee-focus.vercel.app/api/v1/activity?days=7'
```

A missing, expired, malformed or revoked token returns HTTP `401`.
Unauthenticated requests cannot select another user's account. Never put
developer keys into public client-side JavaScript; CORS is not enabled on
the read-only developer endpoint.

### Endpoint: GET /api/v1/activity

| Query | Default | Meaning |
| --- | --- | --- |
| `days` | `7` | Rolling 1–7 days, in 15-minute UTC buckets |
| `detail` | `summary` | `summary` for totals; `slots` for per-device and merged cells |
| `deviceId` | (none) | Include only that connected PC; returns 404 if absent |

```bash
# Summary by PC and combined total
curl -H "Authorization: Bearer ${FOCUS_API_TOKEN}" \
  'https://adhd-coffee-focus.vercel.app/api/v1/activity?days=7'

# History cells from all PCs
curl -H "Authorization: Bearer ${FOCUS_API_TOKEN}" \
  'https://adhd-coffee-focus.vercel.app/api/v1/activity?days=7&detail=slots'

# Single PC (use an id from response.devices)
curl -H "Authorization: Bearer ${FOCUS_API_TOKEN}" \
  'https://adhd-coffee-focus.vercel.app/api/v1/activity?days=1&deviceId=PC_DEVICE_ID'
```

Example response with fictional numbers:

```json
{
  "version": "1",
  "period": {
    "from": "2026-11-14T00:15:00.000Z",
    "to": "2026-11-21T00:15:00.000Z",
    "days": 7,
    "timezone": "UTC"
  },
  "seconds": 10200,
  "sumDeviceSeconds": 10800,
  "deviceCount": 2,
  "devices": [
    { "id": "aaaaaaaa-aaaa-4aaa-aaaa-aaaaaaaaaaaa", "name": "Windows PC", "updatedAt": 1795100000000, "seconds": 7200 },
    { "id": "bbbbbbbb-bbbb-4bbb-bbbb-bbbbbbbbbbbb", "name": "MacBook", "updatedAt": 1795100000000, "seconds": 3600 }
  ],
  "aggregation": "Each 15-minute UTC cell sums PCs and caps merged seconds at 900; simultaneous PCs may be overcounted."
}
```

- `seconds`: merged time across PCs, capped at 900 seconds for each 15-minute
  bucket; better for a single user's total than simply adding PC hours.
- `sumDeviceSeconds`: raw sum of per-device bucket seconds; may exceed merged
  time if devices were on simultaneously.
- `devices[].seconds`: time measured on that individual PC.
- `detail=slots`: adds `cells: [{start,seconds}]` to the root and each device.
  `start` is the **Unix timestamp in seconds (UTC)**, aligned to a 15-minute
  boundary. Numbers are rounded only for display by clients, not by the API.
- The backend only knows about the **last successful sync** from each PC.
  Native clients normally upload at most once per 30 minutes. The API is not a
  live presence API and cannot infer accurate overlapping intervals when it
  has only quarter-hour totals. Those totals are an approximation.
- Due to historical client retention, only seven days of recent data are
  available, not arbitrary older date ranges.
- Failed network uploads do not delete local records.

### Manage tokens from the website

- `GET /api/tokens` lists names, short prefixes, scopes, expiration and
  revocation status; **never returns full tokens**.
- `POST /api/tokens` creates a token; used by the website and protected by a
  Google session and same-origin check.
- `DELETE /api/tokens/{id}` revokes a token; requires the same user's session
  and origin. A revoked token no longer authenticates new API calls.
- Token management endpoints are **not** callable with developer bearer
  tokens. Only the website login session can issue or revoke keys.

**Prerequisite:** Until Google OAuth and the private Vercel Blob store are
configured for the project (see [README.md](README.md)), issuance, storage
and authenticated API retrieval cannot work end to end.
