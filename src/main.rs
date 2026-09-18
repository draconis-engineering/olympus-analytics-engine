/*
 * Olympus Analytics Engine | Main Entrypoint - DRACONIS ENGINEERING
 * Copyright (C) 2026 Simon Stordal Amundgård
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see http://www.gnu.org/licenses.
 */

use std::path::PathBuf;

fn main() {
    println!("Starting Olympus Analytics Engine...");
    println!();

    // Resolve the Olympus SQLite path (mirrors Olympus data.rs logic)
    let db_path: PathBuf = oae_core::resolve_olympus_db_path();
    let candidates = oae_core::candidate_paths();

    println!("  DB candidates:");
    for cand in &candidates {
        let exists = if cand.exists() { "exists" } else { "missing" };
        let marker = if *cand == db_path { "→" } else { " " };
        println!("    {marker} {}  [{exists}]", cand.display());
    }
    println!();
    println!("  Resolved DB: {}", db_path.display());

    // Try to open the DB and report health
    match oae_storage::open_db(&db_path) {
        Ok(conn) => {
            match oae_storage::health_check(&conn) {
                Ok((sessions, samples)) => {
                    println!("  ✓ Database   (SQLite) — {} sessions, {} samples", sessions, samples);
                    if sessions > 0 {
                        match oae_storage::list_sessions(&conn, 5) {
                            Ok(list) => {
                                println!();
                                println!("  Recent sessions (newest 5):");
                                for s in &list {
                                    println!(
                                        "    #{} {} | {:.2} km | {}W avg / {}W max | {} bpm | {}",
                                        s.id,
                                        s.recorded_at,
                                        s.total_distance,
                                        s.avg_power,
                                        s.max_power,
                                        s.avg_heart_rate,
                                        s.filename
                                    );
                                }
                            }
                            Err(e) => println!("  ✗ list_sessions failed: {e}"),
                        }
                    } else {
                        println!("  (no sessions yet — ride with Olympus to generate data)");
                    }
                }
                Err(e) => println!("  ✗ Database health check failed: {e}"),
            }
        }
        Err(e) => {
            println!("  ✗ Database   — failed to open {}: {e}", db_path.display());
            println!("    Hint: set $OLYMPUS_DB or ensure ~/olympus/data/olympus.db exists");
            println!("    Olympus creates it at data/olympus.db on first ride (see olympus/src/data.rs:160)");
        }
    }

    println!();
    println!("  ✓ DracoLIX   (stub)");
    println!("  ✓ Analytics  (planned)");
    println!("  ✓ API        (planned — /api/v1/health will expose this check)");
    println!("  ✓ Web interface (Vite dev: http://localhost:5173/)");
    println!();
    println!("Opening http://localhost:43100 (planned) — for now run:");
    println!("  cd web && npm run dev");
    println!();
    println!("Early development — architecture subject to change. See docs/README.md");
}
