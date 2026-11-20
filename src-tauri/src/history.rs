//! A locally stored, privacy-preserving record of actual timer running time.
//! No application titles, URLs, key presses, or other user activity are stored.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const SLOT_SECONDS: i64 = 15 * 60;
// Keep a full seven calendar days even across timezone / daylight-saving boundaries.
const RETAIN_SECONDS: i64 = 8 * 24 * 60 * 60;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct WorkHistory {
    /// Unix epoch, floored to a 15-minute boundary -> measured running seconds.
    slots: BTreeMap<i64, f64>,
    /// Explicitly cleared slots must not light up again during an active cycle.
    removed: BTreeSet<i64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryCell {
    pub start: i64,
    pub seconds: f64,
}

impl WorkHistory {
    pub fn visible(&self, now: i64) -> Vec<HistoryCell> {
        self.slots
            .iter()
            .filter(|(start, seconds)| {
                **start >= now - RETAIN_SECONDS
                    && **start <= now + SLOT_SECONDS
                    && **seconds > 0.0
                    && seconds.is_finite()
                    && !self.removed.contains(start)
            })
            .map(|(start, seconds)| HistoryCell {
                start: *start,
                seconds: *seconds,
            })
            .collect()
    }

    pub fn record(&mut self, ended_at: f64, elapsed: f64) -> Option<i64> {
        if !ended_at.is_finite()
            || !elapsed.is_finite()
            || !(0.0..=5.0).contains(&elapsed)
            || elapsed == 0.0
        {
            return None;
        }
        // Distribute real counted time across the 15-minute boundary.
        let mut cursor = ended_at - elapsed;
        let mut last_slot = None;
        while cursor < ended_at - 1e-8 {
            let slot = (cursor.floor() as i64).div_euclid(SLOT_SECONDS) * SLOT_SECONDS;
            let next = (slot + SLOT_SECONDS) as f64;
            let until = ended_at.min(next);
            let secs = until - cursor;
            if secs <= 0.0 {
                break;
            }
            last_slot = Some(slot);
            if !self.removed.contains(&slot) {
                let entry = self.slots.entry(slot).or_default();
                *entry = (*entry + secs).min(SLOT_SECONDS as f64);
            }
            cursor = until;
        }
        self.prune(ended_at as i64);
        last_slot
    }

    /// A completion can arrive on a later scheduler tick with no counted
    /// seconds in that exact tick. Find a still-recorded, recent active slot.
    pub fn recent_slot(&self, now: i64) -> Option<i64> {
        self.visible(now)
            .into_iter()
            .filter(|cell| cell.start <= now && cell.start + SLOT_SECONDS >= now - SLOT_SECONDS)
            .map(|cell| cell.start)
            .max()
    }

    pub fn contains_slot(&self, start: i64) -> bool {
        self.slots
            .get(&start)
            .is_some_and(|seconds| seconds.is_finite() && *seconds > 0.0)
            && !self.removed.contains(&start)
    }

    pub fn clear(&mut self, start: i64, now: i64) -> Result<(), String> {
        if start.rem_euclid(SLOT_SECONDS) != 0
            || start < now - RETAIN_SECONDS
            || start > now + SLOT_SECONDS
        {
            return Err("errorHistorySlot".into());
        }
        self.slots.remove(&start);
        self.removed.insert(start);
        self.prune(now);
        Ok(())
    }

    fn prune(&mut self, now: i64) {
        let oldest = now - RETAIN_SECONDS;
        self.slots.retain(|slot, _| *slot >= oldest);
        self.removed.retain(|slot| *slot >= oldest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crossing_a_quarter_hour_allocates_each_real_second_once() {
        let mut history = WorkHistory::default();
        let slot = history.record(901.0, 3.0);
        assert_eq!(slot, Some(900));
        assert_eq!(history.slots.get(&0), Some(&2.0));
        assert_eq!(history.slots.get(&900), Some(&1.0));
        assert!((history.visible(901).iter().map(|s| s.seconds).sum::<f64>() - 3.).abs() < 1e-8);
    }

    #[test]
    fn clearing_a_running_slot_remains_cleared_and_survives_json_roundtrip() {
        let mut h = WorkHistory::default();
        h.record(1000.0, 2.0);
        h.clear(900, 1000).unwrap();
        let mut reloaded: WorkHistory =
            serde_json::from_str(&serde_json::to_string(&h).unwrap()).unwrap();
        reloaded.record(1001.0, 1.0);
        assert!(reloaded.visible(1001).is_empty());
    }

    #[test]
    fn invalid_slots_or_scheduler_gaps_are_not_recorded() {
        let mut h = WorkHistory::default();
        assert!(h.record(1000.0, 120.0).is_none());
        assert!(h.clear(901, 1000).is_err());
        assert!(h.visible(1000).is_empty());
    }

    #[test]
    fn a_full_seven_days_of_recorded_activity_remains_visible() {
        let mut h = WorkHistory::default();
        h.record(1_000_000.0, 1.0);
        assert!(h.contains_slot(999_900));
        let after_seven_days = 1_000_000 + 7 * 24 * 60 * 60;
        assert!(h.visible(after_seven_days).iter().any(|cell| cell.start == 999_900));
        assert_eq!(h.recent_slot(after_seven_days), None);
        h.record((1_000_000 + 9 * 24 * 60 * 60) as f64, 1.0);
        assert!(!h.contains_slot(999_900));
    }

    #[test]
    fn a_cleared_cell_is_never_selected_for_completion_reward() {
        let mut h = WorkHistory::default();
        h.record(2_000.0, 1.0);
        assert_eq!(h.recent_slot(2_000), Some(1_800));
        h.clear(1_800, 2_000).unwrap();
        assert_eq!(h.recent_slot(2_000), None);
        assert!(!h.contains_slot(1_800));
    }

    #[test]
    fn caps_at_15_minutes_and_expires_old_entries() {
        let mut h = WorkHistory::default();
        for second in 1..=1000 {
            h.record(second as f64, 1.);
        }
        assert_eq!(h.slots.get(&0), Some(&900.));
        h.record((RETAIN_SECONDS + 2000) as f64, 1.);
        assert!(!h.slots.contains_key(&0));
    }
}
