use std::path::{Path, PathBuf};

/// Resolve the Olympus SQLite database path.
///
/// Priority:
/// 1. $OLYMPUS_DB env var (exact path, ~ expanded)
/// 2. ~/olympus/data/olympus.db  (user expectation)
/// 3. ~/Prosjekter/olympus/data/olympus.db  (actual repo location on this machine)
/// 4. ../olympus/data/olympus.db  (relative to oae, i.e. sibling checkout)
/// 5. data/olympus.db  (local to OAE checkout, for fresh installs)
///
/// Returns the first candidate that exists; if none exist, returns #2 as the
/// canonical default so the caller can create it or report missing.
pub fn resolve_olympus_db_path() -> PathBuf {
    if let Ok(env_path) = std::env::var("OLYMPUS_DB") {
        let p = expand_tilde(&env_path);
        if !env_path.is_empty() {
            return p;
        }
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let candidates = [
        PathBuf::from(format!("{home}/olympus/data/olympus.db")),
        PathBuf::from(format!("{home}/Prosjekter/olympus/data/olympus.db")),
        PathBuf::from("../olympus/data/olympus.db"),
        PathBuf::from("data/olympus.db"),
        PathBuf::from(format!("{home}/olympus-analytics-engine/data/olympus.db")),
    ];

    for cand in &candidates {
        if cand.exists() {
            return cand.clone();
        }
    }

    // Default to the user-expected location
    candidates[0].clone()
}

/// Expand a leading `~/` using $HOME.
pub fn expand_tilde(p: &str) -> PathBuf {
    if let Some(rest) = p.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(p)
}

/// All candidate paths (for diagnostics / health endpoint).
pub fn candidate_paths() -> Vec<PathBuf> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    vec![
        expand_tilde(&std::env::var("OLYMPUS_DB").unwrap_or_default()),
        PathBuf::from(format!("{home}/olympus/data/olympus.db")),
        PathBuf::from(format!("{home}/Prosjekter/olympus/data/olympus.db")),
        PathBuf::from("../olympus/data/olympus.db"),
        PathBuf::from("data/olympus.db"),
    ]
    .into_iter()
    .filter(|p| !p.as_os_str().is_empty())
    .collect()
}

/// Check if a path exists and is a file.
pub fn db_exists(path: &Path) -> bool {
    path.is_file()
}
