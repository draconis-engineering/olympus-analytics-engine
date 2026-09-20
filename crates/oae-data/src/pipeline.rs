use std::path::Path;

use crate::activity::Activity;
use crate::parser::{parse_file, ParseError, Parsed};
use crate::workout::Workout;

/// Unified pipeline error — what `oae-api` will map to HTTP 400/500.
#[derive(Debug, thiserror::Error)]
pub enum IngestError {
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("validation: {0}")]
    Validation(String),
    #[error("io: {0}")]
    Io(String),
}

pub type IngestResult<T> = Result<T, IngestError>;

/// `Workout file → canonical struct` pipeline.
/// Keeps file I/O, parsing, and normalization in one place so `oae-storage`
/// and `oae-api` never touch `fitparser`/`xml` directly.
///
/// Your swim/run markup: add `*.swim` / `*.run` arms in `parser/mod.rs::parse_file`,
/// then they flow through here automatically.
pub struct Pipeline {
    /// Athlete FTP for resolving fractional ZWO powers.
    pub ftp: u16,
}

impl Pipeline {
    pub fn new(ftp: u16) -> Self { Self { ftp } }

    /// Parse any supported file into the canonical enum.
    /// Dispatches on extension: `.fit` → `Activity`, `.zwo`/`.erg` → `Workout`.
    pub fn ingest_file(&self, path: &Path) -> IngestResult<Parsed> {
        let parsed = parse_file(path, self.ftp).map_err(IngestError::Parse)?;
        match &parsed {
            Parsed::Activity(a) => {
                a.validate().map_err(IngestError::Validation)?;
            }
            Parsed::Workout(w) => {
                if w.steps.is_empty() {
                    return Err(IngestError::Validation("workout has no steps".into()));
                }
            }
        }
        Ok(parsed)
    }

    /// Convenience: parse strictly as Activity (error if file was a Workout).
    pub fn ingest_activity(&self, path: &Path) -> IngestResult<Activity> {
        match self.ingest_file(path)? {
            Parsed::Activity(a) => Ok(a),
            Parsed::Workout(_) => Err(IngestError::Validation(format!("{} is a workout, not an activity", path.display()))),
        }
    }

    /// Convenience: parse strictly as Workout.
    pub fn ingest_workout(&self, path: &Path) -> IngestResult<Workout> {
        match self.ingest_file(path)? {
            Parsed::Workout(w) => Ok(w),
            Parsed::Activity(_) => Err(IngestError::Validation(format!("{} is an activity, not a workout", path.display()))),
        }
    }

    /// Scan a directory (e.g. `data/workouts`) and ingest all supported workouts.
    /// Mirrors `olympus/src/data.rs::list_workout_files` but returns canonical `Workout`s.
    pub fn ingest_workouts_dir(&self, dir: &Path) -> Vec<(String, IngestResult<Workout>)> {
        let mut out = Vec::new();
        let Ok(read_dir) = std::fs::read_dir(dir) else { return out; };
        for entry in read_dir.flatten() {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase()).unwrap_or_default();
            if !matches!(ext.as_str(), "zwo" | "erg") { continue; }
            let res = self.ingest_workout(&path);
            out.push((path.to_string_lossy().to_string(), res));
        }
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsupported_extension() {
        let p = Pipeline::new(200);
        let err = p.ingest_file(Path::new("foo.swim")).unwrap_err();
        assert!(matches!(err, IngestError::Parse(_)));
    }
}
