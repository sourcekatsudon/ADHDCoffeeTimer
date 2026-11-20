use serde::{Deserialize, Serialize};

pub const PRESETS: [u32; 7] = [5, 10, 15, 25, 30, 60, 90];
pub const THEMES: [&str; 7] = [
    "mist-blue",
    "sage-green",
    "lavender",
    "dusty-rose",
    "champagne",
    "sea-glass",
    "smoky-slate",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub minutes: u32,
    pub default_minutes: u32,
    pub last_manual_minutes: Option<u32>,
    pub theme: String,
    pub language: String,
    pub timer_size: String,
    pub idle_pause_secs: u32,
    pub idle_reset_secs: u32,
    pub sound: bool,
    pub opacity: f64,
    pub autostart: bool,
    pub auto_detect: bool,
    pub timer_visible: bool,
    pub history_enabled: bool,
    pub position: Option<(i32, i32)>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            minutes: 25,
            default_minutes: 25,
            last_manual_minutes: None,
            theme: "mist-blue".into(),
            language: "ja".into(),
            timer_size: "L".into(),
            idle_pause_secs: 90,
            idle_reset_secs: 600,
            sound: false,
            opacity: 0.82,
            autostart: true,
            auto_detect: true,
            timer_visible: true,
            history_enabled: true,
            position: None,
        }
    }
}
impl Settings {
    pub fn timer_scale(&self) -> f64 {
        match self.timer_size.as_str() {
            "S" => 0.4,
            "M" => 0.7,
            _ => 1.,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if !crate::i18n::LANGUAGES.contains(&self.language.as_str()) {
            return Err("errorLanguage".into());
        }
        if !["S", "M", "L"].contains(&self.timer_size.as_str()) {
            return Err("errorSize".into());
        }
        if !THEMES.contains(&self.theme.as_str()) {
            return Err("errorTheme".into());
        }
        if !PRESETS.contains(&self.minutes)
            || !PRESETS.contains(&self.default_minutes)
            || self
                .last_manual_minutes
                .is_some_and(|m| !PRESETS.contains(&m))
        {
            return Err("errorDuration".into());
        }
        if !(5..=1800).contains(&self.idle_pause_secs)
            || self.idle_reset_secs < self.idle_pause_secs + 5
            || self.idle_reset_secs > 14400
        {
            return Err("errorIdle".into());
        }
        // Displayed strength is opacity * 150%; legacy 1.0 stays visually 150%.
        if !self.opacity.is_finite() || !(0.3..=(200. / 150.)).contains(&self.opacity) {
            return Err("errorOpacity".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Waiting,
    Running,
    Away,
    Paused,
    Unavailable,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub settings: Settings,
    pub remaining_secs: f64,
    pub duration_secs: f64,
    pub status: Status,
    pub completed: u64,
    pub history: Vec<crate::history::HistoryCell>,
    pub reward_slot: Option<i64>,
    pub debug: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub remaining: f64,
    pub manual_paused: bool,
    pub completed: u64,
}

pub struct Engine {
    pub settings: Settings,
    pub remaining: f64,
    pub status: Status,
    pub completed: u64,
    pub manual_paused: bool,
    manual_running: bool,
    debug_duration: Option<f64>,
    last_idle: f64,
    awaiting_input: bool,
    pub error: Option<String>,
    pub charged_secs: f64,
}
impl Engine {
    pub fn new(
        mut settings: Settings,
        saved: Option<Checkpoint>,
        debug_duration: Option<f64>,
    ) -> Self {
        if let Some(minutes) = settings.last_manual_minutes {
            settings.minutes = minutes;
        }
        let duration = debug_duration.unwrap_or(settings.minutes as f64 * 60.0);
        let mut e = Self {
            settings,
            remaining: duration,
            status: Status::Waiting,
            completed: 0,
            manual_paused: false,
            manual_running: false,
            debug_duration,
            last_idle: f64::INFINITY,
            awaiting_input: true,
            error: None,
            charged_secs: 0.0,
        };
        if let Some(c) = saved {
            if c.remaining.is_finite() && c.remaining > 0.0 {
                e.remaining = c.remaining.min(duration);
            }
            e.manual_paused = c.manual_paused;
            e.completed = c.completed;
            if e.manual_paused {
                e.status = Status::Paused;
            }
        }
        e
    }
    pub fn duration(&self) -> f64 {
        self.debug_duration
            .unwrap_or(self.settings.minutes as f64 * 60.0)
    }
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            settings: self.settings.clone(),
            remaining_secs: self.remaining,
            duration_secs: self.duration(),
            status: self.status,
            completed: self.completed,
            history: Vec::new(),
            reward_slot: None,
            debug: self.debug_duration.is_some(),
            error: self.error.clone(),
        }
    }
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            remaining: self.remaining,
            manual_paused: self.manual_paused,
            completed: self.completed,
        }
    }
    pub fn reset(&mut self) {
        self.remaining = self.duration();
    }
    pub fn toggle(&mut self) {
        if self.status == Status::Running {
            self.manual_paused = true;
            self.manual_running = false;
            self.status = Status::Paused;
        } else {
            self.manual_paused = false;
            self.manual_running = true;
            self.awaiting_input = false;
            self.status = Status::Running;
        }
    }
    pub fn update(&mut self, settings: Settings) {
        let changed = self.settings.minutes != settings.minutes;
        if !self.settings.auto_detect && settings.auto_detect {
            self.manual_paused = false;
            self.manual_running = false;
            self.awaiting_input = true;
            self.status = Status::Waiting;
        }
        self.settings = settings;
        if changed {
            self.reset();
        }
    }
    pub fn select_manual(&mut self, minutes: u32) {
        let mut settings = self.settings.clone();
        settings.minutes = minutes;
        settings.last_manual_minutes = Some(minutes);
        self.update(settings);
    }
    pub fn apply_preferences(&mut self, mut settings: Settings) {
        settings.theme = self.settings.theme.clone();
        if settings.default_minutes != self.settings.default_minutes {
            settings.minutes = settings.default_minutes;
            settings.last_manual_minutes = None;
        } else {
            settings.minutes = self.settings.minutes;
            settings.last_manual_minutes = self.settings.last_manual_minutes;
        }
        self.update(settings);
    }
    pub fn set_theme(&mut self, theme: String) -> Result<(), String> {
        if !THEMES.contains(&theme.as_str()) {
            return Err("errorTheme".into());
        }
        self.settings.theme = theme;
        Ok(())
    }
    /// Called at 1 Hz with monotonic elapsed time and a single OS idle-age sample.
    /// No key codes, cursor positions, or input events enter this state machine.
    pub fn tick(&mut self, elapsed: f64, idle: Option<f64>, locked: bool) -> bool {
        self.charged_secs = 0.0;
        let Some(idle) = idle.filter(|i| i.is_finite() && *i >= 0.0) else {
            self.status = if self.manual_paused {
                Status::Paused
            } else {
                Status::Unavailable
            };
            self.awaiting_input = true;
            return false;
        };
        let input = idle < 1.5 || (self.last_idle.is_finite() && idle + 0.25 < self.last_idle);
        self.last_idle = idle;
        // A scheduler gap is a suspend/hibernation hint. Never charge it as work.
        if elapsed > 5.0 || locked {
            self.awaiting_input = true;
            self.status = if self.manual_paused {
                Status::Paused
            } else {
                Status::Away
            };
            if idle >= self.settings.idle_reset_secs as f64
                || elapsed >= self.settings.idle_reset_secs as f64
            {
                self.reset();
            }
            return false;
        }
        if self.settings.auto_detect && idle >= self.settings.idle_reset_secs as f64 {
            self.reset();
            self.awaiting_input = true;
            self.status = if self.manual_paused {
                Status::Paused
            } else {
                Status::Waiting
            };
            return false;
        }
        if self.manual_paused {
            self.status = Status::Paused;
            return false;
        }
        if self.settings.auto_detect && idle >= self.settings.idle_pause_secs as f64 {
            self.status = Status::Away;
            self.awaiting_input = true;
            return false;
        }
        if self.awaiting_input {
            if !input {
                return false;
            }
            self.awaiting_input = false;
            // The first sample after resuming establishes a baseline.
            self.status = if self.settings.auto_detect || self.manual_running {
                Status::Running
            } else {
                Status::Waiting
            };
            return false;
        }
        if !self.settings.auto_detect && !self.manual_running {
            self.status = Status::Waiting;
            return false;
        }
        self.status = Status::Running;
        let before = self.remaining;
        self.remaining = (self.remaining - elapsed.max(0.0)).max(0.0);
        self.charged_secs = before - self.remaining;
        // Keep a completed cycle pending during inactivity, rather than notifying an empty desk.
        if self.remaining <= 0.0 && input {
            self.completed += 1;
            self.reset();
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn language_and_size_defaults_preserve_legacy_settings() {
        let settings: super::Settings = serde_json::from_str(r#"{"minutes":30}"#).unwrap();
        let json = serde_json::to_value(settings).unwrap();
        assert_eq!(json["language"], "ja");
        assert_eq!(json["timerSize"], "L");
        assert_eq!(json["historyEnabled"], true);
    }

    #[test]
    fn language_and_size_survive_restart_without_resetting_the_timer() {
        let mut e = engine();
        e.select_manual(30);
        e.remaining = 777.;
        e.toggle();
        e.toggle();
        let mut json = serde_json::to_value(&e.settings).unwrap();
        json["language"] = "ar".into();
        json["timerSize"] = "S".into();
        e.apply_preferences(serde_json::from_value(json).unwrap());
        let restored = super::Engine::new(
            serde_json::from_value(serde_json::to_value(&e.settings).unwrap()).unwrap(),
            Some(e.checkpoint()),
            None,
        );
        let json = serde_json::to_value(&restored.settings).unwrap();
        assert_eq!(json["language"], "ar");
        assert_eq!(json["timerSize"], "S");
        assert_eq!(restored.remaining, 777.);
        assert!(restored.manual_paused);
    }

    #[test]
    fn rejects_unknown_languages_and_sizes() {
        for (key, value) in [("language", "unknown"), ("timerSize", "XL")] {
            let mut json = serde_json::to_value(super::Settings::default()).unwrap();
            json[key] = value.into();
            let settings: super::Settings = serde_json::from_value(json).unwrap();
            assert!(settings.validate().is_err());
        }
    }
    #[test]
    fn distribution_defaults_are_silent_with_autostart_enabled() {
        let settings = super::Settings::default();
        assert!(!settings.sound);
        assert!(settings.autostart);
        assert!(settings.timer_visible);
        settings.validate().unwrap();
    }
    use super::*;
    #[test]
    fn every_theme_persists_without_changing_the_current_cycle() {
        let mut e = engine();
        e.select_manual(30);
        e.toggle();
        e.toggle();
        e.remaining = 777.;
        for theme in THEMES {
            e.set_theme(theme.into()).unwrap();
            assert_eq!(e.remaining, 777.);
            assert_eq!(e.status, Status::Paused);
            assert_eq!(e.settings.minutes, 30);
            let settings: Settings =
                serde_json::from_str(&serde_json::to_string(&e.settings).unwrap()).unwrap();
            let restored = Engine::new(settings, Some(e.checkpoint()), None);
            assert_eq!(restored.settings.theme, theme);
            assert_eq!(restored.remaining, 777.);
            assert!(restored.manual_paused);
        }
        assert!(e.set_theme("unknown".into()).is_err());
    }
    #[test]
    fn manual_duration_survives_restart_separately_from_default() {
        let mut e = engine();
        assert!(!e.settings.sound);
        let mut prefs = e.settings.clone();
        prefs.default_minutes = 30;
        e.apply_preferences(prefs);
        assert_eq!(e.remaining, 1800.);
        e.select_manual(10);
        e.remaining = 456.;
        e.toggle();
        e.toggle();
        let json = serde_json::to_string(&e.settings).unwrap();
        let restored = Engine::new(
            serde_json::from_str(&json).unwrap(),
            Some(e.checkpoint()),
            None,
        );
        assert_eq!(restored.settings.default_minutes, 30);
        assert_eq!(restored.settings.last_manual_minutes, Some(10));
        assert_eq!(restored.settings.minutes, 10);
        assert_eq!(restored.remaining, 456.);
        assert!(restored.manual_paused);
    }
    #[test]
    fn unrelated_preferences_keep_manual_duration_and_default_change_applies() {
        let mut e = engine();
        e.select_manual(60);
        e.remaining = 2000.;
        let mut prefs = e.settings.clone();
        prefs.sound = true;
        prefs.minutes = 5;
        prefs.last_manual_minutes = Some(5);
        e.apply_preferences(prefs);
        assert_eq!(e.settings.minutes, 60);
        assert_eq!(e.settings.last_manual_minutes, Some(60));
        assert_eq!(e.remaining, 2000.);
        let mut prefs = e.settings.clone();
        prefs.default_minutes = 15;
        e.apply_preferences(prefs);
        assert_eq!(e.settings.minutes, 15);
        assert_eq!(e.settings.last_manual_minutes, None);
        assert_eq!(e.remaining, 900.);
    }
    #[test]
    fn legacy_settings_keep_the_previously_selected_duration() {
        let settings: Settings = serde_json::from_str(r#"{"minutes":30}"#).unwrap();
        let e = Engine::new(settings, None, None);
        assert_eq!(e.settings.minutes, 30);
        assert_eq!(e.duration(), 1800.);
        assert_eq!(e.settings.default_minutes, 25);
    }
    fn engine() -> Engine {
        Engine::new(Settings::default(), None, None)
    }
    #[test]
    fn starts_only_on_input() {
        let mut e = engine();
        e.tick(1., Some(50.), false);
        assert_eq!(e.status, Status::Waiting);
        e.tick(1., Some(0.), false);
        e.tick(1., Some(0.), false);
        assert_eq!(e.remaining, 1499.);
    }
    #[test]
    fn short_idle_counts_then_pauses_and_resumes() {
        let mut e = engine();
        e.tick(1., Some(0.), false);
        for i in 1..90 {
            e.tick(1., Some(i as f64), false);
        }
        assert_eq!(e.remaining, 1411.);
        e.tick(1., Some(90.), false);
        assert_eq!(e.status, Status::Away);
        e.tick(1., Some(120.), false);
        assert_eq!(e.remaining, 1411.);
        e.tick(1., Some(0.), false);
        e.tick(1., Some(0.), false);
        assert_eq!(e.remaining, 1410.);
    }
    #[test]
    fn long_idle_resets() {
        let mut e = engine();
        e.toggle();
        e.tick(1., Some(0.), false);
        e.tick(1., Some(600.), false);
        assert_eq!(e.remaining, 1500.);
        assert_eq!(e.status, Status::Waiting);
        e.tick(1., Some(0.), false);
        assert_eq!(e.status, Status::Running);
    }
    #[test]
    fn manual_pause_has_priority() {
        let mut e = engine();
        e.toggle();
        e.toggle();
        for idle in [0., 90., 600., 0.] {
            e.tick(1., Some(idle), false);
            assert_eq!(e.status, Status::Paused);
        }
        e.toggle();
        e.tick(1., Some(0.), false);
        assert_eq!(e.status, Status::Running);
    }
    #[test]
    fn preset_preserves_manual_pause() {
        let mut e = engine();
        e.toggle();
        e.toggle();
        let mut s = e.settings.clone();
        s.minutes = 5;
        e.update(s);
        assert_eq!(e.remaining, 300.);
        assert!(e.manual_paused);
    }
    #[test]
    fn notifies_once_per_cycle_and_never_when_idle() {
        let mut e = Engine::new(Settings::default(), None, Some(5.));
        e.tick(1., Some(0.), false);
        for _ in 0..5 {
            assert!(!e.tick(1., Some(8.), false));
        }
        assert_eq!(e.remaining, 0.);
        assert!(e.tick(1., Some(0.), false));
        assert!(!e.tick(1., Some(0.), false));
        assert_eq!(e.completed, 1);
    }
    #[test]
    fn suspend_lock_and_unavailable_do_not_count() {
        let mut e = engine();
        e.toggle();
        e.tick(10000., Some(0.), false);
        assert_eq!(e.remaining, 1500.);
        e.tick(1., Some(0.), true);
        assert_eq!(e.remaining, 1500.);
        e.tick(1., None, false);
        assert_eq!(e.status, Status::Unavailable);
        e.tick(1., Some(20.), false);
        assert_eq!(e.remaining, 1500.);
        e.tick(1., Some(0.), false);
        assert_eq!(e.remaining, 1500.);
    }
    #[test]
    fn long_suspend_resets_even_if_wake_input_clears_idle_age() {
        let mut e = engine();
        e.toggle();
        e.tick(1., Some(0.), false);
        assert_eq!(e.remaining, 1499.);
        assert!(!e.tick(601., Some(0.), false));
        assert_eq!(e.remaining, 1500.);
        assert_eq!(e.status, Status::Away);
        e.tick(1., Some(0.), false);
        assert_eq!(e.remaining, 1500.);
        assert_eq!(e.status, Status::Running);
    }
    #[test]
    fn restore_never_counts_offline_time() {
        let mut e = Engine::new(
            Settings::default(),
            Some(Checkpoint {
                remaining: 42.,
                manual_paused: false,
                completed: 2,
            }),
            None,
        );
        e.tick(1., Some(30.), false);
        assert_eq!(e.remaining, 42.);
        e.tick(1., Some(0.), false);
        assert_eq!(e.remaining, 42.);
    }
    #[test]
    fn auto_off_requires_start_and_reenable_clears_pause() {
        let mut e = engine();
        let mut s = e.settings.clone();
        s.auto_detect = false;
        e.update(s);
        e.tick(1., Some(0.), false);
        e.tick(1., Some(0.), false);
        assert_eq!(e.status, Status::Waiting);
        e.toggle();
        e.tick(1., Some(0.), false);
        assert_eq!(e.remaining, 1499.);
        e.toggle();
        let mut s = e.settings.clone();
        s.auto_detect = true;
        e.update(s);
        e.tick(1., Some(0.), false);
        assert_eq!(e.status, Status::Running);
    }
}
