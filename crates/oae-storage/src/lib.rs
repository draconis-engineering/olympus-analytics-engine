use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub use oae_core::config::{candidate_paths, resolve_olympus_db_path};

// ---------------------------------------------------------------------------
// Legacy Olympus-compatible types (kept for `cargo run` health check)
// ---------------------------------------------------------------------------

/// One stored session row, mirrors `olympus/src/data.rs::StoredSession`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSession {
    pub id: i64,
    pub filename: String,
    pub total_distance: f64,
    pub total_calories: f64,
    pub avg_speed: f64,
    pub max_speed: f64,
    pub max_heart_rate: i64,
    pub avg_heart_rate: i64,
    pub max_power: i64,
    pub avg_power: i64,
    pub recorded_at: String,
}

fn map_session(row: &Row) -> rusqlite::Result<StoredSession> {
    Ok(StoredSession {
        id: row.get(0)?,
        filename: row.get(1)?,
        total_distance: row.get(2)?,
        total_calories: row.get(3)?,
        avg_speed: row.get(4)?,
        max_speed: row.get(5)?,
        max_heart_rate: row.get(6)?,
        avg_heart_rate: row.get(7)?,
        max_power: row.get(8)?,
        avg_power: row.get(9)?,
        recorded_at: row.get(10)?,
    })
}

/// Per-second sample (for power/HR charts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub t: i64,
    pub power: i64,
    pub cadence: i64,
    pub heart_rate: i64,
    pub speed: f64,
}

/// Resolve the DB path using oae-core config.
pub fn default_db_path() -> PathBuf {
    resolve_olympus_db_path()
}

/// Open the Olympus DB. Creates parent dirs if needed, ensures schema
/// (idempotent) — same DDL as `olympus/src/data.rs::init_db`.
/// Also creates OAE-native tables (`oae_workouts`, `oae_activities` view).
pub fn open_db(path: &Path) -> rusqlite::Result<Connection> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(path)?;

    // Olympus-compatible tables (so OAE reads Olympus DB directly)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS fit_sessions (
            id INTEGER PRIMARY KEY,
            filename TEXT NOT NULL,
            total_distance REAL NOT NULL,
            total_calories REAL NOT NULL,
            avg_speed REAL NOT NULL,
            max_speed REAL NOT NULL,
            max_heart_rate INTEGER NOT NULL,
            avg_heart_rate INTEGER NOT NULL,
            max_power INTEGER NOT NULL,
            avg_power INTEGER NOT NULL,
            recorded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS samples (
            id INTEGER PRIMARY KEY,
            session_id INTEGER NOT NULL REFERENCES fit_sessions(id) ON DELETE CASCADE,
            t INTEGER NOT NULL,
            power INTEGER NOT NULL,
            cadence INTEGER NOT NULL,
            heart_rate INTEGER NOT NULL,
            speed REAL NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_samples_session ON samples(session_id)",
        [],
    )?;

    // OAE-native: curated workouts (ZWO/ERG + your future *.swim/*.run)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS oae_workouts (
            id INTEGER PRIMARY KEY,
            name TEXT,
            description TEXT,
            source_file TEXT NOT NULL UNIQUE,
            total_seconds INTEGER NOT NULL,
            is_ramp_test INTEGER NOT NULL DEFAULT 0,
            steps_json TEXT NOT NULL,
            imported_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // OAE-native: generic activities index (optional cache over fit_sessions)
    // We keep fit_sessions as primary for compat; this is a thin view.
    // No extra migration needed for Phase 1.

    let user_version: i64 = conn
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap_or(0);
    if user_version < 2 {
        let _ = conn.pragma_update(None, "user_version", 2);
    }

    Ok(conn)
}

/// Open using the resolved default path.
pub fn open_default() -> rusqlite::Result<Connection> {
    let path = default_db_path();
    open_db(&path)
}

/// Health check: can we open and query the DB?
pub fn health_check(conn: &Connection) -> rusqlite::Result<(usize, usize)> {
    let sessions: i64 = conn.query_row("SELECT COUNT(*) FROM fit_sessions", [], |r| r.get(0))?;
    let samples: i64 = conn.query_row("SELECT COUNT(*) FROM samples", [], |r| r.get(0))?;
    Ok((sessions as usize, samples as usize))
}

/// List recent sessions, newest first.
pub fn list_sessions(conn: &Connection, limit: usize) -> rusqlite::Result<Vec<StoredSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, filename, total_distance, total_calories, avg_speed, max_speed,
                max_heart_rate, avg_heart_rate, max_power, avg_power, recorded_at
         FROM fit_sessions ORDER BY id DESC LIMIT ?1",
    )?;
    let rows = stmt.query_map([limit as i64], map_session)?;
    rows.collect()
}

/// Load samples for one session.
pub fn session_samples(conn: &Connection, session_id: i64) -> rusqlite::Result<Vec<Sample>> {
    let mut stmt = conn.prepare(
        "SELECT t, power, cadence, heart_rate, speed FROM samples WHERE session_id = ?1 ORDER BY t",
    )?;
    let rows = stmt.query_map([session_id], |row| {
        Ok(Sample {
            t: row.get(0)?,
            power: row.get(1)?,
            cadence: row.get(2)?,
            heart_rate: row.get(3)?,
            speed: row.get(4)?,
        })
    })?;
    rows.collect()
}

// ---------------------------------------------------------------------------
// Canonical pipeline: Activity ↔ SQLite (via oae-data::Activity)
// ---------------------------------------------------------------------------

/// Persist a canonical `Activity` (from `Pipeline::ingest_activity`) into
/// `fit_sessions`+`samples`. Returns the new row id. This is the write side
/// of the pipeline: `*.fit → Activity → SQLite → API`.
pub fn save_activity(conn: &Connection, activity: &oae_data::Activity) -> rusqlite::Result<i64> {
    let filename = activity
        .source_file
        .clone()
        .unwrap_or_else(|| "unknown.fit".to_string());
    let distance = activity.total_distance_m.unwrap_or(0.0) as f64;
    let calories = activity.total_calories.unwrap_or(0) as f64;
    let avg_speed = activity.avg_speed_mps.unwrap_or(0.0) as f64;
    let max_speed = activity.max_speed_mps.unwrap_or(0.0) as f64;
    let max_hr = activity.max_hr.unwrap_or(0) as i64;
    let avg_hr = activity.avg_hr.unwrap_or(0) as i64;
    let max_pw = activity.max_power.unwrap_or(0) as i64;
    let avg_pw = activity.avg_power.unwrap_or(0) as i64;

    conn.execute(
        "INSERT INTO fit_sessions (filename, total_distance, total_calories, avg_speed, max_speed, max_heart_rate, avg_heart_rate, max_power, avg_power)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (filename, distance, calories, avg_speed, max_speed, max_hr, avg_hr, max_pw, avg_pw),
    )?;
    let id = conn.last_insert_rowid();

    if !activity.samples.is_empty() {
        let mut stmt = conn.prepare(
            "INSERT INTO samples (session_id, t, power, cadence, heart_rate, speed) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;
        for s in &activity.samples {
            // Canonical → legacy: we store t_offset as absolute-ish `t` for now.
            // If timestamp exists, use its epoch; otherwise offset.
            let t = s.timestamp.map(|dt| dt.timestamp()).unwrap_or(s.t_offset_secs as i64);
            stmt.execute((
                id,
                t,
                s.power_watts.unwrap_or(0) as i64,
                s.cadence_rpm.unwrap_or(0) as i64,
                s.heart_rate_bpm.unwrap_or(0) as i64,
                s.speed_mps.unwrap_or(0.0) as f64,
            ))?;
        }
    }
    Ok(id)
}

/// Load a session as canonical `Activity` (read side of pipeline).
pub fn load_activity(conn: &Connection, id: i64) -> rusqlite::Result<oae_data::Activity> {
    let session = conn.query_row(
        "SELECT id, filename, total_distance, total_calories, avg_speed, max_speed, max_heart_rate, avg_heart_rate, max_power, avg_power, recorded_at
         FROM fit_sessions WHERE id = ?1",
        [id],
        map_session,
    )?;

    let samples = session_samples(conn, id)?
        .into_iter()
        .enumerate()
        .map(|(i, s)| oae_data::ActivitySample {
            t_offset_secs: i as u32,
            timestamp: None,
            power_watts: Some(s.power as u16),
            heart_rate_bpm: Some(s.heart_rate as u8),
            cadence_rpm: Some(s.cadence as u8),
            speed_mps: Some(s.speed as f32),
            distance_m: None,
            lat: None,
            lon: None,
            altitude_m: None,
        })
        .collect::<Vec<_>>();

    // Parse recorded_at → DateTime
    let started_at = chrono::NaiveDateTime::parse_from_str(&session.recorded_at, "%Y-%m-%d %H:%M:%S")
        .ok()
        .and_then(|naive| naive.and_utc().into());

    let activity = oae_data::Activity {
        id: Some(session.id),
        sport: oae_data::Sport::Cycling,
        source: oae_data::SourceType::Fit,
        source_file: Some(session.filename),
        started_at,
        duration_secs: samples.last().map(|s| s.t_offset_secs).unwrap_or(0),
        total_distance_m: Some(session.total_distance as f32),
        total_calories: Some(session.total_calories as u16),
        avg_power: Some(session.avg_power as u16),
        max_power: Some(session.max_power as u16),
        avg_hr: Some(session.avg_heart_rate as u8),
        max_hr: Some(session.max_heart_rate as u8),
        avg_speed_mps: Some(session.avg_speed as f32),
        max_speed_mps: Some(session.max_speed as f32),
        samples,
        laps: Vec::new(),
    };
    Ok(activity)
}

// ---------------------------------------------------------------------------
// Canonical pipeline: Workout ↔ SQLite
// ---------------------------------------------------------------------------

/// Persist a canonical `Workout` (from `Pipeline::ingest_workout`) into `oae_workouts`.
/// Upserts on `source_file` so re-imports are idempotent.
pub fn save_workout(conn: &Connection, workout: &oae_data::Workout) -> rusqlite::Result<i64> {
    let source = workout
        .source_file
        .clone()
        .unwrap_or_else(|| "unknown.zwo".to_string());
    let steps_json = serde_json::to_string(&workout.steps).unwrap_or_else(|_| "[]".to_string());
    conn.execute(
        "INSERT INTO oae_workouts (name, description, source_file, total_seconds, is_ramp_test, steps_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(source_file) DO UPDATE SET
            name=excluded.name,
            description=excluded.description,
            total_seconds=excluded.total_seconds,
            is_ramp_test=excluded.is_ramp_test,
            steps_json=excluded.steps_json",
        (
            workout.name.clone(),
            workout.description.clone(),
            source,
            workout.total_seconds as i64,
            if workout.is_ramp_test { 1 } else { 0 },
            steps_json,
        ),
    )?;
    Ok(conn.last_insert_rowid())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredWorkout {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub source_file: String,
    pub total_seconds: i64,
    pub is_ramp_test: bool,
    pub steps: Vec<oae_data::WorkoutStep>,
}

pub fn list_workouts(conn: &Connection) -> rusqlite::Result<Vec<StoredWorkout>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, source_file, total_seconds, is_ramp_test, steps_json FROM oae_workouts ORDER BY name",
    )?;
    let rows = stmt.query_map([], |row| {
        let json: String = row.get(6)?;
        let steps: Vec<oae_data::WorkoutStep> = serde_json::from_str(&json).unwrap_or_default();
        Ok(StoredWorkout {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            source_file: row.get(3)?,
            total_seconds: row.get(4)?,
            is_ramp_test: row.get::<_, i64>(5)? != 0,
            steps,
        })
    })?;
    rows.collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn test_db() -> (tempfile::TempDir, Connection) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.db");
        let conn = open_db(&path).unwrap();
        (dir, conn)
    }

    #[test]
    fn open_creates_schema() {
        let (_dir, conn) = test_db();
        let (sessions, samples) = health_check(&conn).unwrap();
        assert_eq!(sessions, 0);
        assert_eq!(samples, 0);
    }

    #[test]
    fn list_empty() {
        let (_dir, conn) = test_db();
        assert!(list_sessions(&conn, 10).unwrap().is_empty());
    }

    #[test]
    fn save_and_load_activity_roundtrip() {
        let (_dir, conn) = test_db();
        let activity = oae_data::Activity {
            source_file: Some("test.fit".into()),
            total_distance_m: Some(1000.0),
            total_calories: Some(100),
            avg_power: Some(150),
            max_power: Some(200),
            avg_hr: Some(140),
            max_hr: Some(160),
            avg_speed_mps: Some(8.0),
            max_speed_mps: Some(10.0),
            samples: vec![oae_data::ActivitySample {
                t_offset_secs: 0,
                timestamp: None,
                power_watts: Some(150),
                heart_rate_bpm: Some(140),
                cadence_rpm: Some(90),
                speed_mps: Some(8.0),
                distance_m: None,
                lat: None,
                lon: None,
                altitude_m: None,
            }],
            ..Default::default()
        };
        let id = save_activity(&conn, &activity).unwrap();
        let loaded = load_activity(&conn, id).unwrap();
        assert_eq!(loaded.samples.len(), 1);
        assert_eq!(loaded.avg_power, Some(150));
    }

    #[test]
    fn workout_upsert() {
        let (_dir, conn) = test_db();
        let w = oae_data::Workout {
            name: Some("Test".into()),
            source_file: Some("a.zwo".into()),
            total_seconds: 600,
            steps: vec![oae_data::WorkoutStep { start_secs: 0, end_secs: 600, target_power: 200, target_cadence: None, label: None }],
            ..Default::default()
        };
        save_workout(&conn, &w).unwrap();
        save_workout(&conn, &w).unwrap(); // upsert, no duplicate
        let list = list_workouts(&conn).unwrap();
        assert_eq!(list.len(), 1);
    }
}
