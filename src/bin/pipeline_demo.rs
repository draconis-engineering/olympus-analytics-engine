use std::path::Path;
use oae_data::Pipeline;

fn main() {
    let pipe = Pipeline::new(200);
    println!("=== PIPELINE DEMO ===");
    println!("FTP: 200w\n");

    let fit_path = Path::new("/home/amundgaard/Prosjekter/olympus/data/.fit/ride_20260827_185135.fit");
    match pipe.ingest_activity(fit_path) {
        Ok(act) => {
            println!("FIT OK: {}", fit_path.display());
            println!("  sport: {:?}, source: {}", act.sport, act.source);
            println!("  started_at: {:?}", act.started_at);
            println!("  duration: {}s, distance: {:?}m", act.duration_secs, act.total_distance_m);
            println!("  avg/max power: {:?}/{:?}W, avg/max HR: {:?}/{:?}", act.avg_power, act.max_power, act.avg_hr, act.max_hr);
            println!("  samples: {}", act.samples.len());
            if let Some(s) = act.samples.first() {
                println!("  first sample: t={} power={:?} hr={:?} cad={:?} speed={:?}", s.t_offset_secs, s.power_watts, s.heart_rate_bpm, s.cadence_rpm, s.speed_mps);
            }
        }
        Err(e) => println!("FIT ERR: {e}"),
    }

    println!();

    let zwo_path = Path::new("/home/amundgaard/Prosjekter/olympus/data/workouts/ftp_test_20min.zwo");
    match pipe.ingest_workout(zwo_path) {
        Ok(w) => {
            println!("ZWO OK: {}", zwo_path.display());
            println!("  name: {:?}", w.name);
            println!("  total: {}s, steps: {}, ramp_test: {}", w.total_seconds, w.steps.len(), w.is_ramp_test);
            for step in w.steps.iter().take(4) {
                println!("    {}-{}s @ {}W", step.start_secs, step.end_secs, step.target_power);
            }
        }
        Err(e) => println!("ZWO ERR: {e}"),
    }

    println!();

    let zwo2 = Path::new("/home/amundgaard/Prosjekter/olympus/data/workouts/ramp_test.zwo");
    match pipe.ingest_workout(zwo2) {
        Ok(w) => {
            println!("ZWO RAMP OK: {}", zwo2.display());
            println!("  name: {:?}, total: {}s steps: {} ramp: {}", w.name, w.total_seconds, w.steps.len(), w.is_ramp_test);
        }
        Err(e) => println!("ZWO RAMP ERR: {e}"),
    }

    println!();

    // Demo workouts dir scan
    let count = pipe.ingest_workouts_dir(Path::new("/home/amundgaard/Prosjekter/olympus/data/workouts"));
    println!("Workouts dir: {} files found", count.len());
    for (path, res) in count {
        match res {
            Ok(w) => println!("  OK  {} -> {}s {} steps", path, w.total_seconds, w.steps.len()),
            Err(e) => println!("  ERR {} -> {e}", path),
        }
    }

    println!();

    // Demo SQLite roundtrip
    println!("=== SQLITE PIPELINE: FIT → canonical → SQLite ===");
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("demo.db");
    let conn = oae_storage::open_db(&db_path).unwrap();
    let act = pipe.ingest_activity(fit_path).unwrap();
    let id = oae_storage::save_activity(&conn, &act).unwrap();
    println!("  saved activity id={} to {}", id, db_path.display());
    let loaded = oae_storage::load_activity(&conn, id).unwrap();
    println!("  loaded back: {} samples, avg_power {:?}", loaded.samples.len(), loaded.avg_power);
    println!("  health: {:?}", oae_storage::health_check(&conn).unwrap());

    // Workout SQLite
    let w = pipe.ingest_workout(zwo_path).unwrap();
    let wid = oae_storage::save_workout(&conn, &w).unwrap();
    println!("  saved workout id={} steps={}", wid, w.steps.len());
    println!("  workouts in db: {}", oae_storage::list_workouts(&conn).unwrap().len());
}
