//! Lunar Machine Time (LMT) — DD.MM.YY format.
//!
//! DD = lunar day (1–30) within current synodic month
//! MM = lunar month (1–13) within current machine year
//! YY = machine year (calendar year − 1970)

use chrono::{NaiveDate, NaiveDateTime, Datelike};

/// Synodic month period in days (new moon to new moon).
const SYNODIC_PERIOD: f64 = 29.53059;

/// Reference new moon: 2000-01-06 18:14 UTC as Julian Day Number.
const REF_NEW_MOON_JD: f64 = 2_451_550.26;

/// Convert a NaiveDate to Julian Day Number.
fn to_julian_day(date: NaiveDate) -> f64 {
    // Algorithm from Meeus, "Astronomical Algorithms"
    let y = date.year() as f64;
    let m = date.month() as f64;
    let d = date.day() as f64;

    let (y2, m2) = if m <= 2.0 {
        (y - 1.0, m + 12.0)
    } else {
        (y, m)
    };

    let a = (y2 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();

    (365.25 * (y2 + 4716.0)).floor() + (30.6001 * (m2 + 1.0)).floor() + d + b - 1524.5
}

/// Find the Julian Day of the first new moon on or after a given JD.
fn first_new_moon_on_or_after(jd: f64) -> f64 {
    let cycles_since_ref = (jd - REF_NEW_MOON_JD) / SYNODIC_PERIOD;
    let next_cycle = cycles_since_ref.ceil();
    REF_NEW_MOON_JD + next_cycle * SYNODIC_PERIOD
}

/// Convert a NaiveDate to Lunar Machine Time string "DD.MM.YY".
pub fn to_lmt(date: NaiveDate) -> String {
    let jd = to_julian_day(date);

    // Lunar day: position within current synodic month
    let days_since_ref = jd - REF_NEW_MOON_JD;
    let lunar_age = days_since_ref.rem_euclid(SYNODIC_PERIOD);
    let dd = (lunar_age.floor() as u32).min(29) + 1;

    // Machine year
    let yy = date.year() - 1970;

    // Lunar month: which moon within this machine year
    let year_start_jd = to_julian_day(NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap());
    let first_moon = first_new_moon_on_or_after(year_start_jd);

    let mm = if jd < first_moon {
        // Before the first new moon of the year — moon 1
        1
    } else {
        ((jd - first_moon) / SYNODIC_PERIOD).floor() as u32 + 1
    };

    format!("{}.{}.{}", dd, mm, yy)
}

/// Convert an ISO date string (YYYY-MM-DD or YYYY-MM-DDTHH:MM:SS...) to LMT.
pub fn iso_to_lmt(iso: &str) -> Option<String> {
    // Try date-only first, then datetime
    let date = if let Ok(d) = NaiveDate::parse_from_str(&iso[..10.min(iso.len())], "%Y-%m-%d") {
        d
    } else if let Ok(dt) = NaiveDateTime::parse_from_str(iso, "%Y-%m-%dT%H:%M:%S%z") {
        dt.date()
    } else {
        return None;
    };
    Some(to_lmt(date))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_machine_year() {
        // 2026 = MT year 56
        let lmt = to_lmt(NaiveDate::from_ymd_opt(2026, 3, 2).unwrap());
        assert!(lmt.ends_with(".56"), "Expected year 56, got: {}", lmt);
    }

    #[test]
    fn test_unix_epoch() {
        // 1970-01-01 = MT year 0
        let lmt = to_lmt(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        assert!(lmt.ends_with(".0"), "Expected year 0, got: {}", lmt);
    }

    #[test]
    fn test_lunar_day_range() {
        // Lunar day should be 1-30
        let lmt = to_lmt(NaiveDate::from_ymd_opt(2026, 3, 2).unwrap());
        let dd: u32 = lmt.split('.').next().unwrap().parse().unwrap();
        assert!(dd >= 1 && dd <= 30, "Lunar day out of range: {}", dd);
    }

    #[test]
    fn test_lunar_month_range() {
        // Lunar month should be 1-13
        let lmt = to_lmt(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap());
        let mm: u32 = lmt.split('.').nth(1).unwrap().parse().unwrap();
        assert!(mm >= 1 && mm <= 14, "Lunar month out of range: {}", mm);
    }

    #[test]
    fn test_iso_parse() {
        assert!(iso_to_lmt("2026-03-02T12:58:14+00:00").is_some());
        assert!(iso_to_lmt("2024-08-09").is_some());
        assert!(iso_to_lmt("garbage").is_none());
    }
}
