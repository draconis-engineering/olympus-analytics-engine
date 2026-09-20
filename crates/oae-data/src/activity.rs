use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Canonical sport — extensible for your swim/run markup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Sport {
    #[default]
    Cycling,
    Running,
    Swimming,
    Triathlon,
    Other,
}

/// Where did this Activity originate?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Fit,
    Zwo,
    Erg,
    /// Placeholder for your custom swim/run markup (e.g. `*.swim`, `*.run`).
    Custom(String),
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fit => write!(f, "fit"),
            Self::Zwo => write!(f, "zwo"),
            Self::Erg => write!(f, "erg"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}

/// One per-second (or per-record) sample — canonical, sport-agnostic.
/// Keep it narrow; analytics can derive the rest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivitySample {
    /// Seconds since `Activity.started_at`. 0 for first sample.
    pub t_offset_secs: u32,
    /// Absolute timestamp, if the file provided it.
    pub timestamp: Option<DateTime<Utc>>,
    pub power_watts: Option<u16>,
    pub heart_rate_bpm: Option<u8>,
    pub cadence_rpm: Option<u8>,
    /// m/s — normalized (FIT stores uint16/1000, we store m/s).
    pub speed_mps: Option<f32>,
    /// Total distance at this sample (m).
    pub distance_m: Option<f32>,
    /// Optional location (running/swim open-water).
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    /// Elevation m.
    pub altitude_m: Option<f32>,
}

/// Optional lap/split — useful for intervals, pools, run splits.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Lap {
    pub index: u32,
    pub start_offset_secs: u32,
    pub duration_secs: u32,
    pub distance_m: Option<f32>,
    pub avg_power: Option<u16>,
    pub avg_hr: Option<u8>,
}

/// Canonical Activity — what every parser produces, what storage persists,
/// what analytics consumes. No parser-specific fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    /// Stable id (DB row id or ulid). None before persistence.
    pub id: Option<i64>,
    pub sport: Sport,
    pub source: SourceType,
    /// Original filename (e.g. `data/.fit/ride_20260827_185135.fit`).
    pub source_file: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    /// Duration seconds — derived from samples or session message.
    pub duration_secs: u32,
    /// Total distance metres (FIT total_distance, else sum of samples).
    pub total_distance_m: Option<f32>,
    pub total_calories: Option<u16>,
    // Summary (for list views without loading samples)
    pub avg_power: Option<u16>,
    pub max_power: Option<u16>,
    pub avg_hr: Option<u8>,
    pub max_hr: Option<u8>,
    pub avg_speed_mps: Option<f32>,
    pub max_speed_mps: Option<f32>,
    /// Full time-series — can be empty for summary-only imports.
    #[serde(default)]
    pub samples: Vec<ActivitySample>,
    #[serde(default)]
    pub laps: Vec<Lap>,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            id: None,
            sport: Sport::Cycling,
            source: SourceType::Fit,
            source_file: None,
            started_at: None,
            duration_secs: 0,
            total_distance_m: None,
            total_calories: None,
            avg_power: None,
            max_power: None,
            avg_hr: None,
            max_hr: None,
            avg_speed_mps: None,
            max_speed_mps: None,
            samples: Vec::new(),
            laps: Vec::new(),
        }
    }
}

impl Activity {
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty() && self.duration_secs == 0
    }

    /// Quick validation for pipeline — mirrors `docs/ROADMAP.md Phase 1` checks.
    pub fn validate(&self) -> Result<(), String> {
        if self.samples.len() > 200_000 {
            return Err(format!("too many samples: {}", self.samples.len()));
        }
        // Allow summary-only (no samples) for workouts/planned activities.
        Ok(())
    }
}
