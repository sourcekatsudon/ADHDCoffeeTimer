-- ADHD Coffee Time: server-only PostgreSQL storage.
-- Next.js authenticates Google accounts; only Vercel holds the Supabase secret.

create table if not exists public.focus_documents (
  key text primary key,
  payload jsonb not null,
  updated_at timestamptz not null default now(),
  constraint focus_documents_key_length check (char_length(key) between 1 and 400),
  constraint focus_documents_object_payload check (jsonb_typeof(payload) = 'object')
);

create index if not exists focus_documents_updated_at_idx
  on public.focus_documents (updated_at);
create index if not exists focus_documents_key_prefix_idx
  on public.focus_documents (key text_pattern_ops);

alter table public.focus_documents enable row level security;

-- This table must never be visible to anonymous or user JWT clients.
revoke all on table public.focus_documents from public, anon, authenticated;
grant select, insert, update, delete on table public.focus_documents to service_role;

comment on table public.focus_documents is
  'Private focus history, hashed credentials and one-time pairing codes. Access via server only.';

-- A row-level lock atomically consumes the one-time code and creates a
-- device credential; two racing redemption requests cannot both succeed.
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
    or coalesce((v_payload->>'userKey') !~ '^[0-9a-f]{64}$', true)
  then
    return;
  end if;

  update public.focus_documents as d
    set payload = jsonb_set(v_payload, '{used}', 'true'::jsonb),
        updated_at = now()
    where d.key = p_pair_key;

  insert into public.focus_documents (key, payload)
    values (p_token_key,
      jsonb_build_object(
        'userKey', v_payload->>'userKey',
        'deviceId', p_device_id,
        'expiresAt', p_expires_at_ms));

  user_key := v_payload->>'userKey';
  return next;
end
$function$;

-- SECURITY DEFINER is necessary for the atomic transaction, but may ONLY
-- be called by the service_role identity from the server.
revoke all on function public.focus_redeem_pairing(text, text, text, bigint)
  from public, anon, authenticated;
grant execute on function public.focus_redeem_pairing(text, text, text, bigint)
  to service_role;
