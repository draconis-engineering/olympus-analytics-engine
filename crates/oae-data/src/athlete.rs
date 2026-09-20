use serde::{Deserialize, Serialize};

/// Athlete profile — canonical, persisted to `data/user/profile.json` (Olympus)
/// or to SQLite `athlete` table (OAE). Keep it minimal; zones are derived.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Athlete {
    pub username: String,
    pub weight_kg: f32,
    pub height_cm: f32,
    pub ftp_watts: u16,
    pub max_hr: u16,
    pub resting_hr: Option<u8>,
}

impl Default for Athlete {
    fn default() -> Self {
        Self {
            username: "Rider".to_string(),
            weight_kg: 75.0,
            height_cm: 180.0,
            ftp_watts: 200,
            max_hr: 180,
            resting_hr: None,
        }
    }
}

impl Athlete {
    /// Validate before persisting — `docs/ROADMAP.md Phase 1`.
    pub fn validate(&self) -> Result<(), String> {
        if self.ftp_watts == 0 || self.ftp_watts > 600 {
            return Err(format!("ftp out of range: {}", self.ftp_watts));
        }
        if self.max_hr < 100 || self.max_hr > 230 {
            return Err(format!("max_hr out of range: {}", self.max_hr));
        }
        Ok(())
    }
}
