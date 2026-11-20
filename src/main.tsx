import React, { useEffect, useRef, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import { command, native, onEvent, preset, updateSetting, subscribe } from './bridge';
import { applyTheme, THEMES, themeById } from './themes';
import { APP_NAME, LANGUAGES, languageInfo, translate, localizeError, type Language } from './i18n';
import {
  formatClock,
  formatTime,
  nextPreset,
  PRESETS,
  initial,
  type Settings,
  type Snapshot,
  timerDimensions,
  type TimerSize,
} from './model';
import { MiniHistory, HistoryView } from './history';
import { SyncSettings, syncNow } from './sync';
import './style.css';

type IconName = 'coffee' | 'pause' | 'play' | 'check' | 'settings' | 'close' | 'reset' | 'leaf';
const gearOutline =
  Array.from({ length: 32 }, (_, i) => {
    const angle = (i * Math.PI) / 16 - Math.PI / 2 - Math.PI / 32;
    const r = i % 4 < 2 ? 9.5 : 7.6;
    return `${i === 0 ? 'M' : 'L'}${(12 + Math.cos(angle) * r).toFixed(3)},${(12 + Math.sin(angle) * r).toFixed(3)}`;
  }).join(' ') + ' Z';
function Icon({ name, size = 24 }: { name: IconName; size?: number }) {
  const paths: Record<IconName, React.ReactNode> = {
    coffee: (
      <>
        <path d="M5 10h12v5a6 6 0 0 1-12 0zM17 11h1a3 3 0 0 1 0 6h-1M3 22h17M8 2c-3 3 3 3 0 6M12 1c-3 3 3 3 0 6M16 2c-3 3 3 3 0 6" />
      </>
    ),
    pause: (
      <>
        <path d="M8 5v14M16 5v14" />
      </>
    ),
    play: <path d="m8 4 12 8-12 8z" />,
    check: <path d="m5 12 5 5L20 7" />,
    settings: (
      <>
        <path d={gearOutline} />
        <circle cx="12" cy="12" r="3.2" />
      </>
    ),
    close: <path d="m6 6 12 12M18 6 6 18" />,
    reset: (
      <>
        <path d="M3 11a9 9 0 1 0 3-6L3 8M3 3v5h5" />
      </>
    ),
    leaf: (
      <>
        <path d="M20 3C8 2 2 7 5 15c7 4 15-2 15-12Z M4 21l11-12" />
      </>
    ),
  };
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="1.45"
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden="true"
    >
      {paths[name]}
    </svg>
  );
}
function useSnapshot() {
  const [state, setState] = useState<Snapshot | null>(null);
  useEffect(() => {
    let gone = false;
    let off: (() => void) | undefined;
    subscribe((s) => {
      if (!gone) setState(s);
    })
      .then((f) => {
        if (gone) f();
        else off = f;
      })
      .catch(console.error);
    return () => {
      gone = true;
      off?.();
    };
  }, []);
  return state;
}
function DesktopClock({ language, clockMs }: { language: Language; clockMs?: number }) {
  const t = translate(language);
  // In Tauri, reuse the native timer tick: a second JS timer used to force a
  // separate WebView repaint every second, even while the timer was counting.
  const [previewClockMs, setPreviewClockMs] = useState(() => Date.now());
  useEffect(() => {
    if (native) return;
    let timer: ReturnType<typeof setInterval> | undefined;
    const sync = () => {
      clearInterval(timer);
      if (!document.hidden) {
        setPreviewClockMs(Date.now());
        timer = setInterval(() => setPreviewClockMs(Date.now()), 1000);
      }
    };
    sync();
    document.addEventListener('visibilitychange', sync);
    return () => {
      clearInterval(timer);
      document.removeEventListener('visibilitychange', sync);
    };
  }, []);
  const clock = formatClock(new Date(native ? (clockMs ?? Date.now()) : previewClockMs), language);
  return (
    <div className="desktop-clock" aria-label={t('nowLabel', clock)}>
      <div className="clock-date-line">
        {clock.era && <span className="clock-era">{clock.era}</span>}
        <time className="clock-date">{clock.date}</time>
      </div>
      <div className="clock-line">
        <span className="clock-weekday">({clock.weekday})</span>
        <time className="clock-time">{clock.time}</time>
      </div>
    </div>
  );
}

function Timer({ state, studio = false }: { state: Snapshot; studio?: boolean }) {
  const t = translate(state.settings.language);
  const dimensions = timerDimensions(state.settings.timerSize, state.settings.historyEnabled);
  const [hint, setHint] = useState(false);
  const [error, setError] = useState('');
  const wheelAt = useRef(0);
  type Origin = { x: number; y: number; scale: number };
  type Gesture = {
    id: number;
    x: number;
    y: number;
    lastX: number;
    lastY: number;
    active: boolean;
    finished: boolean;
    origin: Promise<Origin>;
  };
  const gesture = useRef<Gesture | null>(null);
  const suppressClick = useRef(false);
  const [dragging, setDragging] = useState(false);
  const progress = Math.max(0, Math.min(1, state.remainingSecs / state.durationSecs));
  const change = (m: number) => {
    preset(m).catch((e) => setError(String(e)));
    setHint(true);
  };
  useEffect(() => {
    if (!hint) return;
    const id = setTimeout(() => setHint(false), 1800);
    return () => clearTimeout(id);
  }, [hint, state.settings.minutes]);
  function wheel(e: React.WheelEvent) {
    e.preventDefault();
    const now = performance.now();
    if (now - wheelAt.current < 170 || Math.abs(e.deltaY) < 2) return;
    wheelAt.current = now;
    change(nextPreset(state.settings.minutes, e.deltaY));
  }
  function beginMove(e: React.PointerEvent<HTMLElement>) {
    if (!native || e.button !== 0) return;
    const origin = invoke<Origin>('get_drag_origin');
    origin.catch((error) => setError(String(error)));
    gesture.current = {
      id: e.pointerId,
      x: e.screenX,
      y: e.screenY,
      lastX: e.screenX,
      lastY: e.screenY,
      active: false,
      finished: false,
      origin,
    };
  }
  function move(e: React.PointerEvent<HTMLElement>) {
    const g = gesture.current;
    if (!g || g.id !== e.pointerId || !(e.buttons & 1)) return;
    g.lastX = e.screenX;
    g.lastY = e.screenY;
    if (!g.active && Math.hypot(g.lastX - g.x, g.lastY - g.y) < 4) return;
    if (!g.active) {
      g.active = true;
      setDragging(true);
      e.currentTarget.setPointerCapture(e.pointerId);
    }
    e.preventDefault();
    const dx = g.lastX - g.x,
      dy = g.lastY - g.y;
    g.origin
      .then((o) => {
        if (!g.finished)
          return invoke('move_timer', {
            x: Math.round(o.x + dx * o.scale),
            y: Math.round(o.y + dy * o.scale),
            finished: false,
          });
      })
      .catch((e) => setError(String(e)));
  }
  function finishMove(e: React.PointerEvent<HTMLElement>) {
    const g = gesture.current;
    if (!g || g.id !== e.pointerId) return;
    gesture.current = null;
    g.finished = true;
    if (!g.active) return;
    setDragging(false);
    suppressClick.current = true;
    if (e.currentTarget.hasPointerCapture(e.pointerId))
      e.currentTarget.releasePointerCapture(e.pointerId);
    g.origin
      .then((o) =>
        invoke('move_timer', {
          x: Math.round(o.x + (g.lastX - g.x) * o.scale),
          y: Math.round(o.y + (g.lastY - g.y) * o.scale),
          finished: true,
        }),
      )
      .catch((e) => setError(String(e)));
    setTimeout(() => {
      suppressClick.current = false;
    }, 0);
  }
  return (
    <div
      className="timer-frame"
      style={{ width: dimensions.width, height: dimensions.height }}
      dir="ltr"
    >
      <div
        className={`timer-shell ${studio ? 'studio-timer' : ''} ${!state.settings.historyEnabled ? 'no-history' : ''}`}
        onWheel={wheel}
        style={{ transform: `scale(${dimensions.scale})`, transformOrigin: 'top left' }}
        dir={languageInfo(state.settings.language).dir}
      >
        <section
          className={`capsule glass ${state.status === 'paused' ? 'is-paused' : ''} ${dragging ? 'is-dragging' : ''}`}
          style={
            {
              '--opacity': state.settings.opacity,
              '--glass-density': Math.max(0, (state.settings.opacity - 1) * 3),
            } as React.CSSProperties
          }
          onContextMenu={(e) => {
            e.preventDefault();
            command('settings');
          }}
          aria-label={t('timerLabel', { app: APP_NAME })}
          onPointerDownCapture={beginMove}
          onPointerMoveCapture={move}
          onPointerUpCapture={finishMove}
          onPointerCancel={finishMove}
          onLostPointerCapture={finishMove}
          onClickCapture={(e) => {
            if (suppressClick.current) {
              e.preventDefault();
              e.stopPropagation();
              suppressClick.current = false;
            }
          }}
        >
          <DesktopClock language={state.settings.language} clockMs={state.clockMs} />
          <button
            className="ring-button"
            aria-label={t('ringLabel', { minutes: state.settings.minutes })}
            onMouseEnter={() => {
              if (!gesture.current?.active) {
                command('selector');
                setHint(true);
              }
            }}
            onKeyDown={(e) => {
              if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
                e.preventDefault();
                change(
                  nextPreset(
                    state.settings.minutes,
                    ['ArrowUp', 'ArrowRight'].includes(e.key) ? 1 : -1,
                  ),
                );
              }
            }}
            onDoubleClick={() => {
              command('selector');
              setHint(true);
            }}
          >
            <svg viewBox="0 0 112 112" className="progress-ring" aria-hidden="true">
              <defs>
                <filter id="ring-bloom" x="-60%" y="-60%" width="220%" height="220%">
                  <feGaussianBlur stdDeviation="3.4" />
                </filter>
              </defs>
              <circle className="ring-track" cx="56" cy="56" r="47" />
              <circle
                className={`ring-aura ${state.status === 'running' ? 'running' : ''}`}
                cx="56"
                cy="56"
                r="47"
                pathLength="1"
                strokeDasharray="1"
                strokeDashoffset={1 - progress}
              />
              <circle
                className={`ring-fill ${state.status === 'running' ? 'running' : ''}`}
                cx="56"
                cy="56"
                r="47"
                pathLength="1"
                strokeDasharray="1"
                strokeDashoffset={1 - progress}
              />
              <circle
                className={`ring-highlight ${state.status === 'running' ? 'running' : ''}`}
                cx="56"
                cy="56"
                r="47"
                pathLength="1"
                strokeDasharray="1"
                strokeDashoffset={1 - progress}
              />
            </svg>
            <span className="ring-icon">
              <Icon name="coffee" size={34} />
            </span>
          </button>
          <time
            className="time"
            dir="ltr"
            aria-label={t('remainingLabel', { time: formatTime(state.remainingSecs) })}
          >
            {formatTime(state.remainingSecs)}
          </time>
          <span
            className="timer-status"
            title={
              state.error ? localizeError(state.error, state.settings.language) : t(state.status)
            }
          >
            {state.error ? t('checkSettings') : t(state.status)}
          </span>
          <MiniHistory state={state} studio={studio} />
          <div className="timer-controls">
            <button
              className="pause-button"
              title={t(state.status === 'running' ? 'pause' : 'startResume')}
              aria-label={t(state.status === 'running' ? 'pause' : 'startResume')}
              onClick={() => command('toggle').catch((e) => setError(String(e)))}
            >
              <Icon name={state.status === 'running' ? 'pause' : 'play'} size={22} />
            </button>
            <button
              className="capsule-settings"
              title={t('settings')}
              aria-label={t('openSettings')}
              onClick={() => command('settings')}
            >
              <Icon name="settings" size={24} />
            </button>
          </div>
          {error && (
            <span
              className="timer-error"
              role="alert"
              title={localizeError(error, state.settings.language)}
            >
              {t('saveFailed')}
            </span>
          )}
        </section>
        {!native && hint && !studio && (
          <div className="preview-selector">
            <Selector state={state} />
          </div>
        )}
      </div>
    </div>
  );
}
function Selector({ state }: { state: Snapshot }) {
  const t = translate(state.settings.language);
  const timer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const hovering = useRef(false);
  function schedule(delay = 1900) {
    clearTimeout(timer.current);
    if (!hovering.current) timer.current = setTimeout(() => command('hide-selector'), delay);
  }
  useEffect(() => {
    let off: (() => void) | undefined;
    let gone = false;
    onEvent('selector-show', () => schedule()).then((f) => {
      if (gone) f();
      else off = f;
    });
    schedule();
    return () => {
      gone = true;
      clearTimeout(timer.current);
      off?.();
    };
  }, []);
  return (
    <nav
      className="selector glass"
      aria-label={t('presets')}
      onMouseEnter={() => {
        hovering.current = true;
        clearTimeout(timer.current);
      }}
      onMouseLeave={() => {
        hovering.current = false;
        schedule(800);
      }}
    >
      {PRESETS.map((m) => (
        <button
          key={m}
          aria-pressed={m === state.settings.minutes}
          className={m === state.settings.minutes ? 'selected' : ''}
          onClick={() => preset(m)}
        >
          {m}
          <small>{t('minutesUnit')}</small>
        </button>
      ))}
    </nav>
  );
}
function BreakNotification({ state }: { state: Snapshot }) {
  const t = translate(state.settings.language);
  const [cycle, setCycle] = useState(0);
  const [leaving, setLeaving] = useState(false);
  useEffect(() => {
    let gone = false;
    const offs: (() => void)[] = [];
    Promise.all([
      onEvent('break', () => {
        setLeaving(false);
        setCycle((c) => c + 1);
      }),
      onEvent('break-hide', () => setLeaving(true)),
    ]).then((fs) => {
      if (gone) fs.forEach((f) => f());
      else offs.push(...fs);
    });
    return () => {
      gone = true;
      offs.forEach((f) => f());
    };
  }, []);
  return (
    <section
      key={cycle}
      className={`break-card glass ${leaving ? 'leaving' : ''}`}
      aria-label={t('breakNotice')}
    >
      <div className="coffee-glow">
        <Icon name="coffee" size={72} />
      </div>
      <h1>
        <span className="break-elapsed">{t('elapsed', { minutes: state.settings.minutes })}</span>
        <span className="break-space">{t('rest')}</span>
      </h1>
      <button
        className="dismiss-button"
        aria-label={t('dismiss')}
        onClick={() => command('dismiss')}
      >
        <Icon name="check" size={29} />
      </button>
    </section>
  );
}
function Toggle({
  checked,
  onChange,
  label,
}: {
  checked: boolean;
  onChange: (v: boolean) => void;
  label: string;
}) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={label}
      className={`switch ${checked ? 'on' : ''}`}
      onClick={() => onChange(!checked)}
    >
      <span />
    </button>
  );
}
function SettingsView({ state }: { state: Snapshot }) {
  const [draft, setDraft] = useState<Partial<Settings>>({});
  const [error, setError] = useState('');
  const [fieldErrors, setFieldErrors] = useState<Partial<Record<keyof Settings, string>>>({});
  const [acknowledged, acknowledge] = useState(0);
  const revisions = useRef<Partial<Record<keyof Settings, number>>>({});
  const accepted = useRef<Partial<Record<keyof Settings, number>>>({});
  useEffect(() => {
    // Keep optimistic slider edits until the native state acknowledges them.
    setDraft((current) =>
      Object.fromEntries(
        Object.entries(current).filter(
          ([key, value]) =>
            accepted.current[key as keyof Settings] !== revisions.current[key as keyof Settings] ||
            !Object.is(state.settings[key as keyof Settings], value),
        ),
      ),
    );
  }, [state.settings, acknowledged]);
  const s = { ...state.settings, ...draft };
  const t = translate(s.language);
  const set = <K extends keyof Settings>(key: K, value: Settings[K]) => {
    setDraft((current) => ({ ...current, [key]: value }));
    setFieldErrors((current) => ({ ...current, [key]: '' }));
    const revision = (revisions.current[key] ?? 0) + 1;
    revisions.current[key] = revision;
    if (typeof value === 'number' && !Number.isFinite(value)) {
      setFieldErrors((current) => ({
        ...current,
        [key]: 'numericError',
      }));
      return;
    }
    updateSetting(key, value)
      .then(() => {
        accepted.current[key] = Math.max(accepted.current[key] ?? 0, revision);
        acknowledge((n) => n + 1);
      })
      .catch((e) => {
        if (revisions.current[key] === revision)
          setFieldErrors((current) => ({ ...current, [key]: String(e) }));
      });
  };
  return (
    <main className="settings-page" lang={s.language} dir={languageInfo(s.language).dir}>
      <header className="settings-heading">
        <h1 className="settings-title">{t('settings')}</h1>
        <div className="language-buttons" role="group" aria-label={t('language')} dir="ltr">
          {LANGUAGES.map((language) => (
            <button
              type="button"
              key={language.id}
              lang={language.id}
              aria-label={language.name}
              title={language.name}
              aria-pressed={s.language === language.id}
              className={s.language === language.id ? 'active' : ''}
              onClick={() => set('language', language.id)}
            >
              {language.label}
            </button>
          ))}
        </div>
      </header>
      <p className="setting-hint">{t('autosave')}</p>
      {!native && <p className="preview-note">{t('browserNote')}</p>}
      {state.debug && (
        <p className="preview-note">{t('debugNote', { seconds: state.durationSecs })}</p>
      )}
      {state.error && (
        <p className="form-error" role="alert">
          {localizeError(state.error, s.language)}
        </p>
      )}
      <form onSubmit={(e) => e.preventDefault()}>
        <section className="setting-group focus-history-setting">
          <div className="setting-row">
            <div>
              <span>{s.language === 'ja' ? '作業記録の表示' : 'Focus history display'}</span>
              <small>{s.language === 'ja'
                ? 'タイマー本体に直近3日間、拡大画面に直近7日間の履歴を表示。OFFで記録と終了時の演出も停止します。'
                : 'Show 3 days on the timer and 7 in the popup. OFF also stops recording and the completion reward.'}</small>
            </div>
            <div className="focus-history-choice" role="group" aria-label={s.language === 'ja' ? '作業記録の表示を切り替える' : 'Toggle focus history'}>
              <button type="button" className={s.historyEnabled ? 'active' : ''} aria-pressed={s.historyEnabled}
                onClick={() => set('historyEnabled', true)}>ON</button>
              <button type="button" className={!s.historyEnabled ? 'active' : ''} aria-pressed={!s.historyEnabled}
                onClick={() => set('historyEnabled', false)}>OFF</button>
            </div>
          </div>
        </section>
        <SyncSettings state={state} />
        <section className="setting-group">
          <div className="setting-block">
            <label id="duration-label">
              {t('duration')}
              <small>{t('durationHint')}</small>
            </label>
            <div className="preset-grid" role="group" aria-labelledby="duration-label">
              {PRESETS.map((m) => (
                <button
                  type="button"
                  key={m}
                  aria-pressed={s.defaultMinutes === m}
                  className={s.defaultMinutes === m ? 'active' : ''}
                  onClick={() => set('defaultMinutes', m)}
                >
                  {m}
                  <small>{t('minutesUnit')}</small>
                </button>
              ))}
            </div>
            <p className="setting-hint">
              {t('durationChange')}
              <br />
              {t('durationRestore')}
            </p>
          </div>
          <div className="setting-row">
            <div>
              <span>{t('autoDetect')}</span>
              <small>{t('autoDetectHint')}</small>
            </div>
            <Toggle
              checked={s.autoDetect}
              onChange={(v) => set('autoDetect', v)}
              label={t('autoDetect')}
            />
          </div>
          <div className="setting-row compact">
            <label htmlFor="idle-pause">{t('idlePause')}</label>
            <div className="number-field">
              <input
                id="idle-pause"
                type="number"
                min="5"
                max="1800"
                required
                value={Number.isFinite(s.idlePauseSecs) ? s.idlePauseSecs : ''}
                onChange={(e) => set('idlePauseSecs', e.currentTarget.valueAsNumber)}
              />
              <span>{t('afterSeconds')}</span>
            </div>
          </div>
          <div className="setting-row compact">
            <label htmlFor="idle-reset">{t('idleReset')}</label>
            <div className="number-field">
              <input
                id="idle-reset"
                type="number"
                min={
                  ((Number.isFinite(s.idlePauseSecs)
                    ? s.idlePauseSecs
                    : state.settings.idlePauseSecs) +
                    5) /
                  60
                }
                max="240"
                step="any"
                required
                value={
                  Number.isFinite(s.idleResetSecs) ? Number((s.idleResetSecs / 60).toFixed(2)) : ''
                }
                onChange={(e) =>
                  set('idleResetSecs', Math.round(e.currentTarget.valueAsNumber * 60))
                }
              />
              <span>{t('afterMinutes')}</span>
            </div>
          </div>
        </section>
        <section className="setting-group">
          <div className="theme-setting">
            <span>{t('theme')}</span>
            <div className="theme-swatches" role="group" aria-label={t('theme')}>
              {THEMES.map((theme) => (
                <button
                  key={theme.id}
                  type="button"
                  title={t(theme.id)}
                  aria-label={t(theme.id)}
                  aria-pressed={s.theme === theme.id}
                  className={`theme-swatch ${s.theme === theme.id ? 'selected' : ''}`}
                  style={{
                    background: `linear-gradient(145deg, ${theme.light}, ${theme.base} 58%, ${theme.base}cc)`,
                  }}
                  onClick={() => set('theme', theme.id)}
                >
                  {s.theme === theme.id && <Icon name="check" size={14} />}
                </button>
              ))}
            </div>
            <small>{t(s.theme)}</small>
          </div>
          <div className="setting-row">
            <div>
              <span>{t('sound')}</span>
              <small>{t('soundHint')}</small>
            </div>
            <div className="sound-controls">
              <button
                type="button"
                className="sound-preview"
                onClick={() => command('preview-sound').catch((e) => setError(String(e)))}
              >
                {t('listen')}
              </button>
              <Toggle checked={s.sound} onChange={(v) => set('sound', v)} label={t('sound')} />
            </div>
          </div>
          <div className="setting-row">
            <span>{t('breakNotice')}</span>
            <button
              type="button"
              className="sound-preview"
              onClick={() => command('preview-notification').catch((e) => setError(String(e)))}
            >
              {t('previewNotice')}
            </button>
          </div>
          <div className="setting-row">
            <label htmlFor="opacity">{t('opacity')}</label>
            <div className="range-field">
              <input
                id="opacity"
                type="range"
                min="45"
                max="200"
                step="1"
                value={s.opacity * 150}
                onChange={(e) => set('opacity', Number(e.target.value) / 150)}
              />
              <output>{Math.round(s.opacity * 150)}%</output>
            </div>
          </div>
          <div className="setting-row timer-size-row">
            <div>
              <span id="timer-size-label">{t('size')}</span>
              <small dir="ltr">{t('sizeHint')}</small>
            </div>
            <div className="size-buttons" role="group" aria-labelledby="timer-size-label" dir="ltr">
              {(['S', 'M', 'L'] as TimerSize[]).map((size) => (
                <button
                  type="button"
                  key={size}
                  aria-pressed={s.timerSize === size}
                  title={t(`size${size}`)}
                  aria-label={t(`size${size}`)}
                  className={s.timerSize === size ? 'active' : ''}
                  onClick={() => set('timerSize', size)}
                >
                  {size}
                </button>
              ))}
            </div>
          </div>
          <div className="setting-row compact">
            <span>{t('timerVisible')}</span>
            <Toggle
              checked={s.timerVisible}
              onChange={(v) => set('timerVisible', v)}
              label={t('timerVisible')}
            />
          </div>
          <div className="setting-row compact">
            <span>{t('autostart')}</span>
            <Toggle
              checked={s.autostart}
              onChange={(v) => set('autostart', v)}
              label={t('autostart')}
            />
          </div>
          <div className="position-row">
            <span>{t('positionHint')}</span>
            <button type="button" onClick={() => command('reposition')}>
              {t('reposition')} <span>↗</span>
            </button>
          </div>
        </section>
        {error && (
          <p className="form-error" role="alert">
            {localizeError(error, s.language)}
          </p>
        )}
        {Object.entries(fieldErrors)
          .filter(([, message]) => message)
          .map(([key, message]) => (
            <p key={key} className="form-error" role="alert">
              {localizeError(message, s.language)}
            </p>
          ))}
      </form>
    </main>
  );
}
function App() {
  const snapshot = useSnapshot();
  const latestSnapshot = useRef<Snapshot | null>(null);
  useEffect(() => { latestSnapshot.current = snapshot; }, [snapshot]);
  const params = new URLSearchParams(location.search);
  const view = params.get('view') ?? (native ? 'timer' : 'studio');
  // These browser-only compositions render the production UI for documentation.
  const presentation = !native && (view === 'gallery' || view === 'showcase');
  const state = snapshot && presentation ? {
    ...snapshot,
    remainingSecs: 1472,
    status: 'running' as const,
    settings: {
      ...snapshot.settings,
      language: params.get('lang') === 'en' ? 'en' as const : 'ja' as const,
      timerSize: 'L' as const,
    },
  } : snapshot;
  const language = state?.settings.language ?? 'ja';
  const t = translate(language);
  // Use only the main timer WebView for periodic uploads, to avoid duplicate
  // writes when Settings and History popups are open at the same time.
  useEffect(() => {
    if (!native || view !== 'timer') return;
    const send = () => {
      const current = latestSnapshot.current;
      if (current) void syncNow(current).catch(() => {
        // Offline and auth failures are retried at the next cycle; the user
        // can see the error immediately via "Sync now" in Settings.
      });
    };
    const first = window.setTimeout(send, 15_000);
    const interval = window.setInterval(send, 120_000);
    return () => { clearTimeout(first); clearInterval(interval); };
  }, [view]);
  useEffect(() => {
    document.body.dataset.view = view;
  }, [view]);
  useEffect(() => {
    applyTheme(document.documentElement, state?.settings.theme ?? 'mist-blue');
  }, [state?.settings.theme]);
  useEffect(() => {
    document.documentElement.lang = language;
    document.documentElement.dir = languageInfo(language).dir;
    document.title = view === 'settings' ? `${APP_NAME} — ${t('settings')}` : APP_NAME;
  }, [language, view]);
  if (!state) return null;
  if (!native && view === 'gallery') return (
    <main className="release-gallery">
      {THEMES.map((theme) => (
        <section className="release-theme" key={theme.id} style={theme.style} data-theme={theme.id}>
          {theme.id === THEMES[0].id && (
            <div className="release-gallery-selector">
              <Selector state={state} />
            </div>
          )}
          <Timer state={{ ...state, settings: { ...state.settings, theme: theme.id } }} studio />
          <h2>{t(theme.id)}</h2>
        </section>
      ))}
    </main>
  );
  if (!native && view === 'showcase') {
    const panel = params.get('panel') ?? 'timer';
    return (
      <main className={`release-showcase release-showcase-${panel}`}>
        {panel === 'settings' ? <SettingsView state={state} />
          : panel === 'notification' ? <BreakNotification state={state} />
          : <Timer state={state} studio />}
      </main>
    );
  }
  if (!native && view === 'theme-review') {
    const requested = new URLSearchParams(location.search).get('theme') ?? 'all';
    const themes = requested === 'all' ? THEMES : [themeById(requested)];
    const sample = {
      ...initial,
      settings: state.settings,
      remainingSecs: 1472,
      status: 'running' as const,
    };
    return (
      <main className={`theme-review ${themes.length > 1 ? 'all' : ''}`}>
        {themes.map((t) => (
          <section className="theme-review-item" key={t.id} style={t.style} data-theme={t.id}>
            <h2>{translate(language)(t.id)}</h2>
            <div className="theme-review-pair">
              <div className="review-background light">
                <span>{translate(language)('lightBackground')}</span>
                <Timer
                  state={{ ...sample, settings: { ...sample.settings, theme: t.id } }}
                  studio
                />
              </div>
              <div className="review-background dark">
                <span>{translate(language)('darkBackground')}</span>
                <Timer
                  state={{ ...sample, settings: { ...sample.settings, theme: t.id } }}
                  studio
                />
              </div>
            </div>
          </section>
        ))}
      </main>
    );
  }
  if (view === 'settings') return <SettingsView state={state} />;
  if (view === 'notification') return <BreakNotification state={state} />;
  if (view === 'selector') return <Selector state={state} />;
  if (view === 'history') return <HistoryView state={state} />;
  if (view === 'studio')
    return (
      <main className="studio">
        <div className="studio-heading">
          <span className="eyebrow">{APP_NAME}</span>
          <h2>{t('studioTitle')}</h2>
          <p>{t('studioSubtitle')}</p>
          <a href="?view=settings">
            {t('previewSettings')} <span>↗</span>
          </a>
        </div>
        <div className="studio-notification">
          <BreakNotification state={state} />
        </div>
        <div className="studio-controls">
          <Selector state={state} />
          <Timer studio state={state} />
        </div>
        <div className="studio-foot">
          <span>{t('tagline')}</span>
          <span>{t('browserNote')}</span>
        </div>
      </main>
    );
  return <Timer state={state} />;
}
createRoot(document.getElementById('root')!).render(<App />);
