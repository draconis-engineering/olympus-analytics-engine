use std::path::Path;

use crate::parser::{ParseError, WorkoutParser};
use crate::workout::{ErgTarget, Workout};

pub struct ErgParser;

impl WorkoutParser for ErgParser {
    fn parse(&self, path: &Path, _ftp: u16) -> Result<Workout, ParseError> {
        // Direct port of `olympus/src/erg.rs::load_erg_workout`.
        let content = std::fs::read_to_string(path).map_err(|e| ParseError::Io(e.to_string()))?;
        let mut targets = Vec::new();
        let mut current: Option<ErgTarget> = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                if let Some(t) = current.take() {
                    if t.duration_seconds > 0 { targets.push(t); }
                }
                continue;
            }
            let Some((key, value)) = line.split_once(':') else { continue; };
            let key = key.trim();
            let Ok(value) = value.trim().parse::<u16>() else { continue; };
            match key {
                "TARGET_POWER" => {
                    current = Some(ErgTarget { target_power: value, duration_seconds: 0, rest_power: 0, rest_duration: 0 });
                }
                "DURATION" => { if let Some(ref mut t) = current { t.duration_seconds = value; } }
                "REST_POWER" => { if let Some(ref mut t) = current { t.rest_power = value; } }
                "REST_DURATION" => { if let Some(ref mut t) = current { t.rest_duration = value; } }
                _ => {}
            }
        }
        if let Some(t) = current { if t.duration_seconds > 0 { targets.push(t); } }

        if targets.is_empty() {
            return Err(ParseError::Parse("No valid steps found in ERG".into()));
        }

        let mut workout = Workout::from_targets(&targets);
        workout.source_file = Some(path.to_string_lossy().to_string());
        // Name from filename
        workout.name = path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string());
        Ok(workout)
    }
}
