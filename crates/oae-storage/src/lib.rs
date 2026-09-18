use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub use oae_core::config::{candidate_paths, resolve_olympus_db_path};

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

/// Open the Olympus DB (read-only if possible). Creates parent dirs if needed
/// for fresh installs, but does not overwrite an existing DB.
pub fn open_db(path: &Path) -> rusqlite::Result<Connection> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let conn = Connection::open(path)?;

    // Ensure schema exists (idempotent) — same DDL as Olympus.
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
}
