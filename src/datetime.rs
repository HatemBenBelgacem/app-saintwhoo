//! Datum und Uhrzeit auf Deutsch (Server und Browser).
//!
//! Zeiten von Events und Sendungen sind Ortszeit (Europe/Zurich) und werden
//! ohne Zeitzone gespeichert (`TIMESTAMP`, in Rust `NaiveDateTime`).

use chrono::{Datelike, Days, NaiveDate, NaiveDateTime, NaiveTime, Weekday};

const WEEKDAYS: [&str; 7] = ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"];
const MONTHS: [&str; 12] = ["Jan", "Feb", "März", "Apr", "Mai", "Juni", "Juli", "Aug", "Sept", "Okt", "Nov", "Dez"];

/// `Sa`
pub fn weekday_short(d: NaiveDate) -> &'static str {
    WEEKDAYS[d.weekday().num_days_from_monday() as usize]
}

/// `Nov`
pub fn month_short(d: NaiveDate) -> &'static str {
    MONTHS[d.month0() as usize]
}

/// `23:00`
pub fn format_time(t: NaiveTime) -> String {
    t.format("%H:%M").to_string()
}

/// `Sa, 14. Nov · 23:00 Uhr`
pub fn format_when(dt: NaiveDateTime) -> String {
    let d = dt.date();
    format!("{}, {}. {} · {} Uhr", weekday_short(d), d.day(), month_short(d), format_time(dt.time()))
}

/// Montag der Woche von `d`.
pub fn monday_of(d: NaiveDate) -> NaiveDate {
    d.week(Weekday::Mon).first_day()
}

/// `d` plus `n` Tage.
pub fn add_days(d: NaiveDate, n: u64) -> NaiveDate {
    d.checked_add_days(Days::new(n)).unwrap_or(d)
}

/// Mitternacht am Tag `d`.
pub fn start_of(d: NaiveDate) -> NaiveDateTime {
    d.and_time(NaiveTime::MIN)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dt(s: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").unwrap()
    }

    #[test]
    fn format_when_wie_im_design() {
        assert_eq!(format_when(dt("2026-11-14 23:00")), "Sa, 14. Nov · 23:00 Uhr");
        assert_eq!(format_when(dt("2026-03-02 07:05")), "Mo, 2. März · 07:05 Uhr");
    }

    #[test]
    fn montag_der_woche() {
        let sa = NaiveDate::from_ymd_opt(2026, 11, 14).unwrap();
        assert_eq!(monday_of(sa), NaiveDate::from_ymd_opt(2026, 11, 9).unwrap());
        let mo = NaiveDate::from_ymd_opt(2026, 11, 9).unwrap();
        assert_eq!(monday_of(mo), mo);
    }
}
