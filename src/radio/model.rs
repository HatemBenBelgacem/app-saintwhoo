//! Radio (ANF-007) – Typen und Prüfregeln. Läuft auf Server und im Browser.

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::articles::model::is_valid_image_url;

pub const TITLE_MAX_CHARS: usize = 120;
pub const HOST_MAX_CHARS: usize = 120;
/// Längste Sendung: 12 Stunden.
pub const SHOW_MAX_HOURS: i64 = 12;
pub const MIX_MAX_MINUTES: i32 = 600;
pub const AUDIO_URL_MAX_CHARS: usize = 500;
/// Höchstens so viele Mixe im Archiv anzeigen.
pub const ARCHIVE_LIMIT: i64 = 50;

/// Fehlermeldungen – Texte exakt wie in docs/features/ANF-007.
pub mod msg {
    pub const TITLE_EMPTY: &str = "Bitte einen Titel eingeben.";
    pub const TITLE_TOO_LONG: &str = "Der Titel darf höchstens 120 Zeichen lang sein.";
    pub const HOST_EMPTY: &str = "Bitte einen DJ angeben.";
    pub const HOST_TOO_LONG: &str = "Der Name darf höchstens 120 Zeichen lang sein.";
    pub const DATE_MISSING: &str = "Bitte ein Datum angeben.";
    pub const DATE_INVALID: &str = "Bitte ein gültiges Datum angeben.";
    pub const TIME_MISSING: &str = "Bitte eine Uhrzeit angeben.";
    pub const TIME_INVALID: &str = "Bitte eine gültige Uhrzeit angeben (HH:MM).";
    pub const SAME_TIME: &str = "Beginn und Ende dürfen nicht gleich sein.";
    pub const TOO_LONG: &str = "Eine Sendung darf höchstens 12 Stunden dauern.";
    pub const DURATION_INVALID: &str = "Bitte eine Dauer zwischen 1 und 600 Minuten angeben.";
    pub const AUDIO_URL_MISSING: &str = "Bitte die Adresse der Aufnahme angeben.";
    pub const AUDIO_URL_INVALID: &str = "Bitte eine gültige Adresse angeben (https://… oder /…).";

    pub const SHOW_SAVED: &str = "Sendung gespeichert.";
    pub const SHOW_DELETED: &str = "Sendung gelöscht.";
    pub const MIX_SAVED: &str = "Mix gespeichert.";
    pub const MIX_DELETED: &str = "Mix gelöscht.";
    pub const NOT_FOUND: &str = "Dieser Eintrag existiert nicht mehr.";

    pub const SCHEDULE_EMPTY: &str = "Heute sind keine Sendungen geplant.";
    pub const ARCHIVE_EMPTY: &str = "Noch keine Mixe im Archiv.";
    pub const STREAM_OFF: &str = "Der Livestream ist zurzeit nicht verfügbar.";
}

/// Ansicht der Radioseite (`?ansicht=archiv`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum RadioView {
    #[default]
    Live,
    Archive,
}

impl RadioView {
    pub fn parse(value: &str) -> Self {
        if value == "archiv" { RadioView::Archive } else { RadioView::Live }
    }
}

// ---------- Sendungen ----------

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ShowInput {
    pub title: String,
    pub host: String,
    /// `YYYY-MM-DD`
    pub date: String,
    /// `HH:MM`
    pub start: String,
    /// `HH:MM` – früher als `start` heißt: endet am nächsten Tag
    pub end: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidShow {
    pub title: String,
    pub host: String,
    pub starts_at: NaiveDateTime,
    pub ends_at: NaiveDateTime,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ShowErrors {
    pub title: Option<String>,
    pub host: Option<String>,
    pub date: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

impl ShowErrors {
    pub fn is_empty(&self) -> bool {
        *self == ShowErrors::default()
    }
}

/// Eine Sendung im Sendeplan.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Show {
    pub id: i64,
    pub title: String,
    pub host: String,
    pub starts_at: NaiveDateTime,
    pub ends_at: NaiveDateTime,
}

impl Show {
    /// Läuft die Sendung zum Zeitpunkt `now`?
    pub fn is_on_air(&self, now: NaiveDateTime) -> bool {
        self.starts_at <= now && now < self.ends_at
    }

    /// `02:00 – 04:00`
    pub fn time_range(&self) -> String {
        format!("{} – {}", self.starts_at.format("%H:%M"), self.ends_at.format("%H:%M"))
    }
}

// ---------- Mixe ----------

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MixInput {
    pub title: String,
    pub host: String,
    pub duration_minutes: String,
    pub audio_url: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidMix {
    pub title: String,
    pub host: String,
    pub duration_minutes: i32,
    pub audio_url: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MixErrors {
    pub title: Option<String>,
    pub host: Option<String>,
    pub duration_minutes: Option<String>,
    pub audio_url: Option<String>,
}

impl MixErrors {
    pub fn is_empty(&self) -> bool {
        *self == MixErrors::default()
    }
}

/// Ein aufgezeichnetes Set im Archiv.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mix {
    pub id: i64,
    pub title: String,
    pub host: String,
    pub duration_minutes: i32,
    pub audio_url: String,
}

// ---------- Ergebnisse ----------

/// Live-Ansicht: was gerade läuft + Sendeplan für heute.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveInfo {
    /// Adresse des Livestreams (`RADIO_STREAM_URL`), `None` = nicht eingerichtet
    pub stream_url: Option<String>,
    pub now: NaiveDateTime,
    pub today: Vec<Show>,
}

impl LiveInfo {
    pub fn on_air(&self) -> Option<&Show> {
        self.today.iter().find(|s| s.is_on_air(self.now))
    }
}

/// Verwaltung: kommende Sendungen und alle Mixe.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadioOverview {
    pub shows: Vec<Show>,
    pub mixes: Vec<Mix>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ShowOutcome {
    Created { id: i64 },
    Invalid(ShowErrors),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MixOutcome {
    Created { id: i64 },
    Invalid(MixErrors),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteOutcome {
    Deleted,
    NotFound,
}

// ---------- Hilfen ----------

/// `2 Std. 30 Min.`, `45 Min.`, `2 Std.`
pub fn format_duration(minutes: i32) -> String {
    let (h, m) = (minutes / 60, minutes % 60);
    match (h, m) {
        (0, m) => format!("{m} Min."),
        (h, 0) => format!("{h} Std."),
        (h, m) => format!("{h} Std. {m} Min."),
    }
}

fn required_line(raw: &str, max: usize, empty: &str, too_long: &str, error: &mut Option<String>) -> String {
    let value = raw.trim().to_string();
    if value.is_empty() {
        *error = Some(empty.into());
    } else if value.chars().count() > max {
        *error = Some(too_long.into());
    }
    value
}

fn parse_time(raw: &str, error: &mut Option<String>) -> Option<NaiveTime> {
    let raw = raw.trim();
    if raw.is_empty() {
        *error = Some(msg::TIME_MISSING.into());
        return None;
    }
    NaiveTime::parse_from_str(raw, "%H:%M")
        .or_else(|_| NaiveTime::parse_from_str(raw, "%H:%M:%S"))
        .map_err(|_| *error = Some(msg::TIME_INVALID.into()))
        .ok()
}

/// Prüft eine Sendung. Ende vor Beginn = Sendung über Mitternacht.
#[allow(clippy::result_large_err)]
pub fn validate_show(input: &ShowInput) -> Result<ValidShow, ShowErrors> {
    let mut errors = ShowErrors::default();
    let title = required_line(&input.title, TITLE_MAX_CHARS, msg::TITLE_EMPTY, msg::TITLE_TOO_LONG, &mut errors.title);
    let host = required_line(&input.host, HOST_MAX_CHARS, msg::HOST_EMPTY, msg::HOST_TOO_LONG, &mut errors.host);

    let date = input.date.trim();
    let date = if date.is_empty() {
        errors.date = Some(msg::DATE_MISSING.into());
        None
    } else {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| errors.date = Some(msg::DATE_INVALID.into())).ok()
    };
    let start = parse_time(&input.start, &mut errors.start);
    let end = parse_time(&input.end, &mut errors.end);

    let times = match (date, start, end) {
        (Some(d), Some(s), Some(e)) => {
            let starts_at = d.and_time(s);
            let mut ends_at = d.and_time(e);
            if e == s {
                errors.end = Some(msg::SAME_TIME.into());
            } else if e < s {
                ends_at += Duration::days(1);
            }
            if ends_at - starts_at > Duration::hours(SHOW_MAX_HOURS) {
                errors.end = Some(msg::TOO_LONG.into());
            }
            Some((starts_at, ends_at))
        }
        _ => None,
    };

    match (errors.is_empty(), times) {
        (true, Some((starts_at, ends_at))) => Ok(ValidShow { title, host, starts_at, ends_at }),
        _ => Err(errors),
    }
}

/// Prüft einen Mix fürs Archiv.
#[allow(clippy::result_large_err)]
pub fn validate_mix(input: &MixInput) -> Result<ValidMix, MixErrors> {
    let mut errors = MixErrors::default();
    let title = required_line(&input.title, TITLE_MAX_CHARS, msg::TITLE_EMPTY, msg::TITLE_TOO_LONG, &mut errors.title);
    let host = required_line(&input.host, HOST_MAX_CHARS, msg::HOST_EMPTY, msg::HOST_TOO_LONG, &mut errors.host);

    let duration = input.duration_minutes.trim();
    let duration_minutes = match duration.parse::<i32>() {
        Ok(n) if (1..=MIX_MAX_MINUTES).contains(&n) && !duration.starts_with('+') => n,
        _ => {
            errors.duration_minutes = Some(msg::DURATION_INVALID.into());
            0
        }
    };

    // Gleiche Regeln wie Bild-Adressen: https://… oder /…, ohne Leerzeichen
    let audio_url = input.audio_url.trim().to_string();
    if audio_url.is_empty() {
        errors.audio_url = Some(msg::AUDIO_URL_MISSING.into());
    } else if !is_valid_image_url(&audio_url) || audio_url.chars().count() > AUDIO_URL_MAX_CHARS {
        errors.audio_url = Some(msg::AUDIO_URL_INVALID.into());
    }

    if errors.is_empty() { Ok(ValidMix { title, host, duration_minutes, audio_url }) } else { Err(errors) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn show() -> ShowInput {
        ShowInput {
            title: "Nachtschicht #14".into(),
            host: "Mara Voss".into(),
            date: "2026-11-14".into(),
            start: "02:00".into(),
            end: "04:00".into(),
        }
    }

    fn mix() -> MixInput {
        MixInput {
            title: "Acid Morning #12".into(),
            host: "Lea Stern".into(),
            duration_minutes: "90".into(),
            audio_url: "/audio/acid-morning-12.mp3".into(),
        }
    }

    #[test]
    fn sendung_gueltig() {
        let s = validate_show(&show()).unwrap();
        assert_eq!(s.starts_at.to_string(), "2026-11-14 02:00:00");
        assert_eq!(s.ends_at.to_string(), "2026-11-14 04:00:00");
    }

    #[test]
    fn sendung_ueber_mitternacht() {
        let s = validate_show(&ShowInput { start: "23:00".into(), end: "01:00".into(), ..show() }).unwrap();
        assert_eq!(s.ends_at.to_string(), "2026-11-15 01:00:00");
    }

    #[test]
    fn sendung_gleiche_zeit_und_zu_lang() {
        let e = validate_show(&ShowInput { end: "02:00".into(), ..show() }).unwrap_err();
        assert_eq!(e.end.as_deref(), Some(msg::SAME_TIME));
        let e = validate_show(&ShowInput { start: "06:00".into(), end: "19:00".into(), ..show() }).unwrap_err();
        assert_eq!(e.end.as_deref(), Some(msg::TOO_LONG));
        assert!(validate_show(&ShowInput { start: "06:00".into(), end: "18:00".into(), ..show() }).is_ok());
    }

    #[test]
    fn sendung_pflichtfelder() {
        let e = validate_show(&ShowInput {
            title: " ".into(),
            host: "".into(),
            date: "".into(),
            start: "".into(),
            end: "25:00".into(),
        })
        .unwrap_err();
        assert_eq!(e.title.as_deref(), Some(msg::TITLE_EMPTY));
        assert_eq!(e.host.as_deref(), Some(msg::HOST_EMPTY));
        assert_eq!(e.date.as_deref(), Some(msg::DATE_MISSING));
        assert_eq!(e.start.as_deref(), Some(msg::TIME_MISSING));
        assert_eq!(e.end.as_deref(), Some(msg::TIME_INVALID));
    }

    #[test]
    fn mix_gueltig_und_ungueltig() {
        assert_eq!(validate_mix(&mix()).unwrap().duration_minutes, 90);
        for bad in ["0", "601", "abc", "+5", "-1", ""] {
            let e = validate_mix(&MixInput { duration_minutes: bad.into(), ..mix() }).unwrap_err();
            assert_eq!(e.duration_minutes.as_deref(), Some(msg::DURATION_INVALID), "{bad}");
        }
        let e = validate_mix(&MixInput { audio_url: "".into(), ..mix() }).unwrap_err();
        assert_eq!(e.audio_url.as_deref(), Some(msg::AUDIO_URL_MISSING));
        for bad in ["ftp://x", "mix.mp3", "/a b.mp3", "//evil.example/x"] {
            let e = validate_mix(&MixInput { audio_url: bad.into(), ..mix() }).unwrap_err();
            assert_eq!(e.audio_url.as_deref(), Some(msg::AUDIO_URL_INVALID), "{bad}");
        }
    }

    #[test]
    fn dauer_text() {
        assert_eq!(format_duration(45), "45 Min.");
        assert_eq!(format_duration(120), "2 Std.");
        assert_eq!(format_duration(90), "1 Std. 30 Min.");
    }

    #[test]
    fn on_air() {
        let s = Show {
            id: 1,
            title: "x".into(),
            host: "y".into(),
            starts_at: "2026-11-14T02:00:00".parse().unwrap(),
            ends_at: "2026-11-14T04:00:00".parse().unwrap(),
        };
        assert!(s.is_on_air("2026-11-14T02:00:00".parse().unwrap()));
        assert!(s.is_on_air("2026-11-14T03:59:00".parse().unwrap()));
        assert!(!s.is_on_air("2026-11-14T04:00:00".parse().unwrap()));
        assert_eq!(s.time_range(), "02:00 – 04:00");
    }
}
