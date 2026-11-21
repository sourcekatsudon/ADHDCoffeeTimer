-- ADHD Coffee Time: server-only Supabase PostgreSQL storage
-- Run once in Supabase Dashboard > SQL Editor, on the intended project.
--
-- Auth.js Google sessions and per-device tokens continue to be validated by
-- the Next.js Vercel server. Only that server uses SUPABASE_SECRET_KEY.
-- No browser/desktop client ever receives a Supabase key.

create table if not exists public.focus_documents (
  key text primary key,
  payload jsonb not null,
  updated_at timestamptz not null default now(),
  constraint focus_documents_key_length check (char_length(key) between 1 and 400),
  constraint focus_documents_object_payload check (jsonb_typeof(payload) = 'object')
);

create index if not exists focus_documents_updated_at_idx
  on public.focus_documents (updated_at);

alter table public.focus_documents enable row level security;
-- Explicitly deny access to the table through anonymous / end-user keys.
revoke all on table public.focus_documents from public, anon, authenticated;
grant select, insert, update, delete on table public.focus_documents to service_role;

comment on table public.focus_documents is
  'Private short-lived pairing codes, hashed token verifiers and per-device '
  'focus history snapshots. Access only through the Vercel backend.';

-- Atomic, one-time pairing code redemption. A row-level lock makes concurrent
-- redemption requests deterministic: only one device gets a token.
create or replace function public.focus_redeem_pairing(
  p_pair_key text,
  p_token_key text,
  p_device_id text,
  p_expires_at_ms bigint
)
returns table (user_key text)
language plpgsql
security definer
set search_path = ''
as $function$
declare
  v_payload jsonb;
  v_now_ms bigint := floor(extract(epoch from clock_timestamp()) * 1000)::bigint;
begin
  -- The Node backend hashes the raw code/token before calling this function.
  if p_pair_key !~ '^focus/v1/pair/[0-9a-f]{64}\.json$'
    or p_token_key !~ '^focus/v1/token/[0-9a-f]{64}\.json$'
    or p_device_id !~ '^[a-zA-Z0-9-]{16,64}$'
    or p_expires_at_ms <= v_now_ms
    or p_expires_at_ms > v_now_ms + (366::bigint * 86400000)
  then
    return;
  end if;

  select d.payload into v_payload
  from public.focus_documents as d
  where d.key = p_pair_key
  for update;

  if not found
    or coalesce((v_payload->>'used')::boolean, true)
    or coalesce((v_payload->>'expiresAt')::bigint, 0) <= v_now_ms
    or (v_payload->>'userKey') !~ '^[0-9a-f]{64}$'
  then
    return;
  end if;

  update public.focus_documents as d
  set payload = jsonb_set(v_payload, '{used}', 'true'::jsonb),
      updated_at = now()
  where d.key = p_pair_key;

  insert into public.focus_documents (key, payload)
  values (
    p_token_key,
    jsonb_build_object(
      'userKey', v_payload->>'userKey',
      'deviceId', p_device_id,
      'expiresAt', p_expires_at_ms
    )
  );

  user_key := v_payload->>'userKey';
  return next;
end
$function$;

revoke all on function public.focus_redeem_pairing(text, text, text, bigint)
  from public, anon, authenticated;
grant execute on function public.focus_redeem_pairing(text, text, text, bigint)
  to service_role;

-- Optional: run infrequently if storage grows. Device snapshots overwrite by
-- device id; only pairing and token entries can accumulate indefinitely.
-- Archive/revocation policy may be added later without altering app code.
