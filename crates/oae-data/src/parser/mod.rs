pub mod erg;
pub mod fit;
pub mod zwo;

use std::path::Path;

use crate::activity::Activity;
use crate::workout::Workout;

/// All parsers produce either an `Activity` (recorded) or a `Workout` (planned).
/// Your swim/run markup will add new arms here — the trait keeps the pipeline uniform.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unsupported file type: {0}")]
    Unsupported(String),
    #[error("io: {0}")]
    Io(String),
    #[error("parse: {0}")]
    Parse(String),
}

pub trait ActivityParser {
    fn parse(&self, path: &Path) -> Result<Activity, ParseError>;
}

pub trait WorkoutParser {
    fn parse(&self, path: &Path, ftp: u16) -> Result<Workout, ParseError>;
}

/// Dispatch by extension — `fit → Activity`, `zwo/erg → Workout`.
/// Add your `*.swim` / `*.run` arms here later:
/// ```ignore
/// "swim" => Box::new(parser::swim::SwimParser),
/// ```
pub fn parse_file(path: &Path, ftp: u16) -> Result<Parsed, ParseError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        "fit" => {
            let p = fit::FitParser;
            Ok(Parsed::Activity(p.parse(path)?))
        }
        "zwo" => {
            let p = zwo::ZwoParser;
            Ok(Parsed::Workout(p.parse(path, ftp)?))
        }
        "erg" => {
            let p = erg::ErgParser;
            Ok(Parsed::Workout(p.parse(path, ftp)?))
        }
        other => Err(ParseError::Unsupported(other.to_string())),
    }
}

#[derive(Debug)]
pub enum Parsed {
    Activity(Activity),
    Workout(Workout),
}
