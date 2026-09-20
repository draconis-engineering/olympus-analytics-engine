use crate::activity::{Activity, ActivitySample};

/// Pipeline normalization — `docs/ROADMAP.md Phase 1`.
/// FIT quirks live here, not in parsers or storage.

/// FIT speed is stored as uint16 scaled 1000 → m/s.
pub fn fit_speed_to_mps(raw: u16) -> f32 {
    raw as f32 / 1000.0
}

pub fn fit_speed_raw_to_mps(raw: f32) -> f32 {
    raw // already float in some files — keep as-is
}

/// Clamp power/HR to plausible ranges; return None if bogus.
pub fn normalize_power(w: Option<u16>) -> Option<u16> {
    match w {
        Some(v) if v <= 3000 => Some(v),
        _ => None,
    }
}

pub fn normalize_hr(bpm: Option<u8>) -> Option<u8> {
    match bpm {
        Some(v) if (30..=250).contains(&v) => Some(v),
        _ => None,
    }
}

/// Sort samples by offset and fill gaps. Idempotent.
pub fn normalize_activity(activity: &mut Activity) {
    // Sort by offset (parsers should already, but be safe).
    activity.samples.sort_by_key(|s| s.t_offset_secs);

    // Deduplicate same t_offset (keep last).
    let mut dedup: Vec<ActivitySample> = Vec::new();
    for s in std::mem::take(&mut activity.samples) {
        if let Some(last) = dedup.last_mut() {
            if last.t_offset_secs == s.t_offset_secs {
                *last = s;
                continue;
            }
        }
        dedup.push(s);
    }
    activity.samples = dedup;

    // Derive duration if missing.
    if activity.duration_secs == 0 {
        if let Some(last) = activity.samples.last() {
            activity.duration_secs = last.t_offset_secs;
        }
    }

    // Derive summary if missing (avg/max).
    if !activity.samples.is_empty() {
        let powers: Vec<u16> = activity
            .samples
            .iter()
            .filter_map(|s| s.power_watts)
            .collect();
        if !powers.is_empty() && activity.avg_power.is_none() {
            let sum: u32 = powers.iter().map(|&v| v as u32).sum();
            activity.avg_power = Some((sum / powers.len() as u32) as u16);
            activity.max_power = powers.iter().copied().max();
        }
        let hrs: Vec<u8> = activity
            .samples
            .iter()
            .filter_map(|s| s.heart_rate_bpm)
            .collect();
        if !hrs.is_empty() && activity.avg_hr.is_none() {
            let sum: u32 = hrs.iter().map(|&v| v as u32).sum();
            activity.avg_hr = Some((sum / hrs.len() as u32) as u8);
            activity.max_hr = hrs.iter().copied().max();
        }
    }

    // Basic validation clamping.
    for s in &mut activity.samples {
        s.power_watts = normalize_power(s.power_watts);
        s.heart_rate_bpm = normalize_hr(s.heart_rate_bpm);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fit_speed() {
        assert!((fit_speed_to_mps(1000) - 1.0).abs() < 1e-6);
        assert!((fit_speed_to_mps(30_000) - 30.0).abs() < 1e-6);
    }
}
