use serde::{Deserialize, Serialize};

/// Canonical workout — what ZWO/ERG (and later your *.swim/*.run) produce.
/// This is *planned* work, not a recorded `Activity`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Workout {
    /// Friendly name from <workout_file><name> (ZWO) or filename.
    pub name: Option<String>,
    pub description: Option<String>,
    /// Ordered steps — absolute start/end in seconds from workout start.
    pub steps: Vec<WorkoutStep>,
    pub total_seconds: u32,
    /// True for ramp FTP tests (see `olympus/src/erg.rs:1`).
    pub is_ramp_test: bool,
    /// Original file, for lineage.
    pub source_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkoutStep {
    pub start_secs: u32,
    pub end_secs: u32,
    /// Target power watts (resolved vs FTP if source was fractional).
    pub target_power: u16,
    /// Optional cadence target (for run/swim later).
    pub target_cadence: Option<u8>,
    /// Free-form label (e.g. "Warmup", "Interval 1").
    pub label: Option<String>,
}

impl Workout {
    pub fn from_targets(targets: &[ErgTarget]) -> Self {
        let mut steps = Vec::new();
        let mut t: u32 = 0;
        for target in targets {
            if target.duration_seconds > 0 {
                steps.push(WorkoutStep {
                    start_secs: t,
                    end_secs: t + target.duration_seconds as u32,
                    target_power: target.target_power,
                    target_cadence: None,
                    label: None,
                });
                t += target.duration_seconds as u32;
            }
            if target.rest_duration > 0 {
                steps.push(WorkoutStep {
                    start_secs: t,
                    end_secs: t + target.rest_duration as u32,
                    target_power: target.rest_power,
                    target_cadence: None,
                    label: None,
                });
                t += target.rest_duration as u32;
            }
        }
        Self {
            name: None,
            description: None,
            total_seconds: t,
            steps,
            is_ramp_test: false,
            source_file: None,
        }
    }

    pub fn step_at(&self, elapsed_secs: u32) -> Option<&WorkoutStep> {
        self.steps
            .iter()
            .find(|s| elapsed_secs >= s.start_secs && elapsed_secs < s.end_secs)
    }

    pub fn is_finished(&self, elapsed_secs: u32) -> bool {
        elapsed_secs >= self.total_seconds
    }
}

/// Intermediate parsed target before expanding into `WorkoutStep`s.
/// Mirrors `olympus/src/erg.rs::ErgTarget` so we can reuse that logic verbatim.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ErgTarget {
    pub target_power: u16,
    pub duration_seconds: u16,
    pub rest_power: u16,
    pub rest_duration: u16,
}
