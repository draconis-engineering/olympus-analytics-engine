use std::fs::File;
use std::path::Path;

use chrono::{DateTime, TimeZone, Utc};

use crate::activity::{Activity, ActivitySample, SourceType, Sport};
use crate::parser::{ActivityParser, ParseError};

pub struct FitParser;

impl ActivityParser for FitParser {
    fn parse(&self, path: &Path) -> Result<Activity, ParseError> {
        let mut file = File::open(path).map_err(|e| ParseError::Io(e.to_string()))?;
        let mut records = fitparser::from_reader(&mut file).map_err(|e| ParseError::Parse(e.to_string()))?;

        // Olympus files write arch=0x01 (big) but data is little — fitparser sees
        // Session/Record as Value(5120). If we got no Session/Record, retry patched.
        let has_session = records.iter().any(|r| r.kind() == fitparser::profile::MesgNum::Session);
        let has_record = records.iter().any(|r| r.kind() == fitparser::profile::MesgNum::Record);
        if !has_session && !has_record && !records.is_empty() {
            // Patch file bytes: flip arch 0x01→0x00 on every definition header and fix CRC.
            if let Ok(bytes) = std::fs::read(path) {
                let mut patched = bytes.clone();
                for i in 0..patched.len().saturating_sub(6) {
                    let h = patched[i];
                    if (h & 0x40) != 0 && (h & 0x80) == 0 && patched[i + 1] == 0x00 && patched[i + 2] == 0x01 {
                        patched[i + 2] = 0x00; // big→little
                    }
                }
                // Recompute FIT data CRC (last 2 bytes) over data section.
                if patched.len() >= 14 {
                    let data_size = u32::from_le_bytes([patched[4], patched[5], patched[6], patched[7]]) as usize;
                    let header_len = patched[0] as usize; // 14
                    if header_len == 14 && patched.len() >= header_len + data_size + 2 {
                        let crc = fit_crc_slice(&patched[header_len..header_len + data_size]);
                        let crc_bytes = crc.to_le_bytes();
                        patched[header_len + data_size] = crc_bytes[0];
                        patched[header_len + data_size + 1] = crc_bytes[1];
                    }
                }
                if let Ok(retry) = fitparser::from_reader(&mut std::io::Cursor::new(patched)) {
                    records = retry;
                }
            }
        }

        let mut activity = Activity {
            sport: Sport::Cycling,
            source: SourceType::Fit,
            source_file: Some(path.to_string_lossy().to_string()),
            ..Default::default()
        };

        // 1) Session summary (distance, calories, avg/max) — same as `olympus/src/data.rs::_parse_fit_file`.
        if let Some(session) = records
            .iter()
            .find(|r| r.kind() == fitparser::profile::MesgNum::Session)
        {
            for field in session.fields() {
                match field.name() {
                    "total_distance" => {
                        if let fitparser::Value::Float32(v) = field.value() {
                            activity.total_distance_m = Some(*v);
                        }
                    }
                    "total_calories" => {
                        if let fitparser::Value::UInt16(v) = field.value() {
                            activity.total_calories = Some(*v);
                        }
                    }
                    "avg_speed" => {
                        if let fitparser::Value::UInt16(v) = field.value() {
                            activity.avg_speed_mps = Some(*v as f32 / 1000.0);
                        }
                    }
                    "max_speed" => {
                        if let fitparser::Value::UInt16(v) = field.value() {
                            activity.max_speed_mps = Some(*v as f32 / 1000.0);
                        }
                    }
                    "avg_power" => {
                        if let fitparser::Value::UInt16(v) = field.value() {
                            activity.avg_power = Some(*v);
                        }
                    }
                    "max_power" => {
                        if let fitparser::Value::UInt16(v) = field.value() {
                            activity.max_power = Some(*v);
                        }
                    }
                    "avg_heart_rate" => {
                        if let fitparser::Value::UInt8(v) = field.value() {
                            activity.avg_hr = Some(*v);
                        }
                    }
                    "max_heart_rate" => {
                        if let fitparser::Value::UInt8(v) = field.value() {
                            activity.max_hr = Some(*v);
                        }
                    }
                    "timestamp" => {
                        if let fitparser::Value::Timestamp(v) = field.value() {
                            activity.started_at = Some(Utc.timestamp_opt(v.timestamp(), 0).single().unwrap_or_else(Utc::now));
                        }
                    }
                    _ => {}
                }
            }
        }

        // 2) Per-record samples (Record mesg) — power/hr/cadence/speed/distance.
        // We keep it tolerant: not every FIT has every field.
        let mut t0: Option<DateTime<Utc>> = None;
        let mut samples: Vec<ActivitySample> = Vec::new();

        for rec in records.iter().filter(|r| r.kind() == fitparser::profile::MesgNum::Record) {
            let mut ts: Option<DateTime<Utc>> = None;
            let mut power: Option<u16> = None;
            let mut hr: Option<u8> = None;
            let mut cad: Option<u8> = None;
            let mut speed: Option<f32> = None;
            let mut dist: Option<f32> = None;
            let mut lat: Option<f64> = None;
            let mut lon: Option<f64> = None;
            let mut alt: Option<f32> = None;

            for f in rec.fields() {
                match f.name() {
                    "timestamp" => {
                        if let fitparser::Value::Timestamp(v) = f.value() {
                            let dt = Utc.timestamp_opt(v.timestamp(), 0).single().unwrap_or_else(Utc::now);
                            ts = Some(dt);
                            if t0.is_none() {
                                t0 = Some(dt);
                                if activity.started_at.is_none() {
                                    activity.started_at = Some(dt);
                                }
                            }
                        }
                    }
                    "power" => {
                        if let fitparser::Value::UInt16(v) = f.value() { power = Some(*v); }
                    }
                    "heart_rate" => {
                        if let fitparser::Value::UInt8(v) = f.value() { hr = Some(*v); }
                    }
                    "cadence" => {
                        if let fitparser::Value::UInt8(v) = f.value() { cad = Some(*v); }
                    }
                    "speed" => {
                        match f.value() {
                            fitparser::Value::UInt16(v) => speed = Some(*v as f32 / 1000.0),
                            fitparser::Value::Float32(v) => speed = Some(*v),
                            _ => {}
                        }
                    }
                    "distance" => {
                        if let fitparser::Value::Float32(v) = f.value() { dist = Some(*v); }
                    }
                    "position_lat" => {
                        if let fitparser::Value::SInt32(v) = f.value() { lat = Some(*v as f64 / 11_938_030.0); } // semicircles → deg
                    }
                    "position_long" => {
                        if let fitparser::Value::SInt32(v) = f.value() { lon = Some(*v as f64 / 11_938_030.0); }
                    }
                    "altitude" => {
                        // FIT altitude: uint16 scaled 5 + 500 offset, or float
                        match f.value() {
                            fitparser::Value::UInt16(v) => alt = Some(*v as f32 / 5.0 - 500.0),
                            fitparser::Value::Float32(v) => alt = Some(*v),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            // Only push if we got at least one metric.
            if power.is_some() || hr.is_some() || cad.is_some() || speed.is_some() || dist.is_some() {
                let offset = match (ts, t0) {
                    (Some(a), Some(b)) => (a.timestamp() - b.timestamp()).max(0) as u32,
                    _ => samples.len() as u32,
                };
                samples.push(ActivitySample {
                    t_offset_secs: offset,
                    timestamp: ts,
                    power_watts: power,
                    heart_rate_bpm: hr,
                    cadence_rpm: cad,
                    speed_mps: speed,
                    distance_m: dist,
                    lat,
                    lon,
                    altitude_m: alt,
                });
            }
        }

        // Fallback: ensure offsets are monotonic if timestamps were missing.
        for (i, s) in samples.iter_mut().enumerate() {
            if s.timestamp.is_none() && s.t_offset_secs == 0 {
                s.t_offset_secs = i as u32;
            }
        }

        activity.samples = samples;
        if let Some(last) = activity.samples.last() {
            activity.duration_secs = last.t_offset_secs;
        }
        // If no samples but session gave us a duration-like distance, keep 0 and let normalize derive.

        crate::normalize::normalize_activity(&mut activity);
        activity.validate().map_err(ParseError::Parse)?;
        Ok(activity)
    }
}

const CRC_TABLE: [u16; 16] = [
    0x0000, 0xCC01, 0xD801, 0x1400, 0xF001, 0x3C00, 0x2800, 0xE401, 0xA001, 0x6C00, 0x7800, 0xB401,
    0x5000, 0x9C01, 0x8801, 0x4400,
];
fn fit_crc_step(crc: u16, byte: u8) -> u16 {
    let mut tmp = CRC_TABLE[(crc & 0xF) as usize];
    let mut crc = (crc >> 4) & 0x0FFF;
    crc ^= tmp ^ CRC_TABLE[(byte & 0xF) as usize];
    tmp = CRC_TABLE[(crc & 0xF) as usize];
    crc = (crc >> 4) & 0x0FFF;
    crc ^ tmp ^ CRC_TABLE[((byte >> 4) & 0xF) as usize]
}
fn fit_crc_slice(data: &[u8]) -> u16 {
    let mut crc = 0u16;
    for &b in data {
        crc = fit_crc_step(crc, b);
    }
    crc
}
