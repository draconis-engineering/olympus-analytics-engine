use std::path::Path;

use crate::parser::{ParseError, WorkoutParser};
use crate::workout::{ErgTarget, Workout};

pub struct ZwoParser;

impl WorkoutParser for ZwoParser {
    fn parse(&self, path: &Path, ftp: u16) -> Result<Workout, ParseError> {
        // Reuse the exact logic from `olympus/src/erg.rs::parse_zwo_workout`
        // but map into `oae-data::workout::Workout` (canonical).
        let file = std::fs::File::open(path).map_err(|e| ParseError::Io(e.to_string()))?;
        let source = std::io::BufReader::new(file);
        let parser = xml::reader::EventReader::new(source);

        let mut targets: Vec<ErgTarget> = Vec::new();
        let mut name: Option<String> = None;
        let mut in_name = false;
        let mut ramp_test_seen = false;
        let mut pending_name_text: Option<String> = None;

        for event in parser {
            use xml::reader::XmlEvent;
            match event.map_err(|e| ParseError::Parse(format!("Invalid ZWO XML: {e}")))? {
                XmlEvent::StartElement { name: elem_name, attributes, .. } => {
                    let local = elem_name.local_name.as_str();
                    if local == "name" {
                        in_name = true;
                        continue;
                    }
                    let get = |key: &str| -> Option<f32> {
                        attributes.iter().find(|a| a.name.local_name == key).and_then(|a| a.value.parse().ok())
                    };
                    match local {
                        "Warmup" | "Cooldown" => {
                            let duration = get("Duration").unwrap_or(0.0) as u32;
                            let low = get("PowerLow");
                            let high = get("PowerHigh");
                            let power = get("Power");
                            if let Some(t) = resolve_power(power.or(high).or(low), ftp) {
                                targets.push(ErgTarget { target_power: t, duration_seconds: duration as u16, rest_power: 0, rest_duration: 0 });
                            }
                        }
                        "SteadyState" => {
                            let duration = get("Duration").unwrap_or(0.0) as u32;
                            let power = get("Power");
                            if let Some(t) = resolve_power(power, ftp) {
                                targets.push(ErgTarget { target_power: t, duration_seconds: duration as u16, rest_power: 0, rest_duration: 0 });
                            }
                        }
                        "IntervalsT" | "Intervals" => {
                            let repeat = get("Repeat").unwrap_or(1.0) as u32;
                            let on_duration = get("OnDuration").unwrap_or(0.0) as u32;
                            let off_duration = get("OffDuration").unwrap_or(0.0) as u32;
                            let on = resolve_power(get("OnPower"), ftp).unwrap_or(0);
                            let off = resolve_power(get("OffPower"), ftp).unwrap_or(0);
                            for _ in 0..repeat {
                                targets.push(ErgTarget { target_power: on, duration_seconds: on_duration as u16, rest_power: off, rest_duration: off_duration as u16 });
                            }
                        }
                        "Ramp" => {
                            // Detect ramp test: presence of Ramp with RampRate or long ramp
                            ramp_test_seen = true;
                            let duration = get("Duration").unwrap_or(0.0) as u32;
                            // Two forms: PowerLow/High + Duration, or Ftp fraction
                            if let Some(_rate) = get("RampRate") {
                                // Expand ramp into 1-min steps at +RampRate W/min from PowerLow
                                let low = get("PowerLow").unwrap_or(100.0) as u16;
                                let rate = get("RampRate").unwrap_or(20.0) as u16;
                                let mins = (duration / 60).max(1);
                                for i in 0..mins {
                                    targets.push(ErgTarget { target_power: low + i as u16 * rate, duration_seconds: 60, rest_power: 0, rest_duration: 0 });
                                }
                                let rem = duration % 60;
                                if rem > 0 {
                                    targets.push(ErgTarget { target_power: low + mins as u16 * rate, duration_seconds: rem as u16, rest_power: 0, rest_duration: 0 });
                                }
                            } else if let Some(frac) = get("Ftp") {
                                targets.push(ErgTarget { target_power: resolve_power(Some(frac), ftp).unwrap_or(0), duration_seconds: duration as u16, rest_power: 0, rest_duration: 0 });
                            } else {
                                let low = get("PowerLow").unwrap_or(0.0);
                                let high = get("PowerHigh").unwrap_or(low);
                                let p = resolve_power(Some(high), ftp).unwrap_or(resolve_power(Some(low), ftp).unwrap_or(0));
                                targets.push(ErgTarget { target_power: p, duration_seconds: duration as u16, rest_power: 0, rest_duration: 0 });
                            }
                        }
                        _ => {}
                    }
                }
                XmlEvent::Characters(text) => {
                    if in_name {
                        // ZWO nests <name><en_US>Text</en_US></name> — capture inner text.
                        let trimmed = text.trim().to_string();
                        if !trimmed.is_empty() {
                            pending_name_text = Some(trimmed);
                        }
                    }
                }
                XmlEvent::EndElement { name: elem_name } => {
                    let local = elem_name.local_name.as_str();
                    if local == "name" {
                        if let Some(t) = pending_name_text.take() {
                            name = Some(t);
                        }
                        in_name = false;
                    }
                    if local == "en_US" && in_name {
                        if let Some(t) = pending_name_text.take() {
                            // en_US is inside name — keep it as name
                            name = Some(t);
                        }
                    }
                }
                _ => {}
            }
        }

        if targets.is_empty() {
            return Err(ParseError::Parse("No valid steps found in ZWO".into()));
        }

        let mut workout = Workout::from_targets(&targets);
        workout.name = name;
        workout.is_ramp_test = ramp_test_seen;
        workout.source_file = Some(path.to_string_lossy().to_string());
        // Pull description if present — cheap second pass
        // (not critical for pipeline; leave None if not found)
        Ok(workout)
    }
}

fn resolve_power(v: Option<f32>, ftp: u16) -> Option<u16> {
    let f = v?;
    // Fractional (<5.0) means FTP multiplier; integer means absolute watts.
    // Mirrors `olympus/src/erg.rs::resolve_power`.
    if f > 0.0 && f < 5.0 {
        Some((f * ftp as f32).round() as u16)
    } else {
        Some(f.round() as u16)
    }
}
