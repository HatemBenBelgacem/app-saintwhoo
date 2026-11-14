//! Events (ANF-006) – Typen und Prüfregeln. Läuft auf Server und im Browser.

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

/// Bild + Bildnachweis – gleicher Typ und gleiche Regeln wie bei Artikeln (ANF-004).
pub use crate::articles::model::{ArticleImage as EventImage, IMAGE_CREDIT_MAX_CHARS, is_valid_image_url};
use crate::datetime::{add_days, monday_of, start_of};

pub const TITLE_MAX_CHARS: usize = 150;
pub const DESCRIPTION_MAX_CHARS: usize = 10_000;
pub const VENUE_MAX_CHARS: usize = 120;
pub const CITY_MAX_CHARS: usize = 80;
pub const LINEUP_MAX_CHARS: usize = 300;
pub const TICKET_URL_MAX_CHARS: usize = 500;
/// Einträge pro Seite in der Verwaltung (ANF-002, R-2).
pub const PAGE_SIZE: i64 = 20;
/// Länge des Anrisses in der Eventliste.
pub const TEASER_MAX_CHARS: usize = 160;

/// Fehlermeldungen – Texte exakt wie in docs/features/ANF-006.
pub mod msg {
    pub const TITLE_EMPTY: &str = "Bitte einen Titel eingeben.";
    pub const TITLE_TOO_LONG: &str = "Der Titel darf höchstens 150 Zeichen lang sein.";
    pub const DESCRIPTION_EMPTY: &str = "Bitte eine Beschreibung eingeben.";
    pub const DESCRIPTION_TOO_LONG: &str = "Die Beschreibung darf höchstens 10 000 Zeichen lang sein.";
    pub const DATE_MISSING: &str = "Bitte ein Datum angeben.";
    pub const DATE_INVALID: &str = "Bitte ein gültiges Datum angeben.";
    pub const TIME_MISSING: &str = "Bitte eine Uhrzeit angeben.";
    pub const TIME_INVALID: &str = "Bitte eine gültige Uhrzeit angeben (HH:MM).";
    pub const VENUE_EMPTY: &str = "Bitte einen Ort angeben.";
    pub const VENUE_TOO_LONG: &str = "Der Ort darf höchstens 120 Zeichen lang sein.";
    pub const CITY_EMPTY: &str = "Bitte eine Stadt angeben.";
    pub const CITY_TOO_LONG: &str = "Die Stadt darf höchstens 80 Zeichen lang sein.";
    pub const LINEUP_TOO_LONG: &str = "Das Lineup darf höchstens 300 Zeichen lang sein.";
    pub const TICKET_URL_INVALID: &str = "Bitte eine gültige Ticket-Adresse angeben (https://…).";
    pub const TICKET_STATUS_INVALID: &str = "Bitte einen gültigen Ticketstatus wählen.";
    pub use crate::articles::model::msg::{IMAGE_CREDIT_MISSING, IMAGE_CREDIT_TOO_LONG, IMAGE_URL_INVALID};

    pub const NOT_FOUND: &str = "Dieses Event existiert nicht mehr.";
    pub const SAVED: &str = "Event gespeichert.";
    pub const DELETED: &str = "Event gelöscht.";
    pub const LIST_EMPTY: &str = "Noch keine Events vorhanden.";
    pub const PUBLIC_EMPTY: &str = "Zurzeit sind keine Events geplant.";
    pub const PUBLIC_EMPTY_RANGE: &str = "In diesem Zeitraum sind keine Events geplant.";
}

/// Wie viele Tickets es noch gibt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TicketStatus {
    #[default]
    Available,
    Few,
    SoldOut,
}

impl TicketStatus {
    pub const ALL: [TicketStatus; 3] = [TicketStatus::Available, TicketStatus::Few, TicketStatus::SoldOut];

    /// Wert in Formular und Datenbank.
    pub fn as_str(self) -> &'static str {
        match self {
            TicketStatus::Available => "available",
            TicketStatus::Few => "few",
            TicketStatus::SoldOut => "sold_out",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            TicketStatus::Available => "Tickets verfügbar",
            TicketStatus::Few => "Wenige Tickets",
            TicketStatus::SoldOut => "Ausverkauft",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == value)
    }
}

/// Zeitraum-Filter der Eventliste (`?zeitraum=…`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventRange {
    #[default]
    All,
    Today,
    Week,
    Weekend,
}

impl EventRange {
    pub const ALL: [EventRange; 4] = [EventRange::All, EventRange::Today, EventRange::Week, EventRange::Weekend];

    pub fn as_str(self) -> &'static str {
        match self {
            EventRange::All => "alle",
            EventRange::Today => "heute",
            EventRange::Week => "woche",
            EventRange::Weekend => "wochenende",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            EventRange::All => "Alle",
            EventRange::Today => "Heute",
            EventRange::Week => "Diese Woche",
            EventRange::Weekend => "Wochenende",
        }
    }

    /// Unbekannte Werte → „Alle“.
    pub fn parse(value: &str) -> Self {
        Self::ALL.into_iter().find(|r| r.as_str() == value).unwrap_or_default()
    }

    /// Zeitfenster `[von, bis)` ab heute. `bis = None` heißt offen.
    /// Woche = Montag bis Sonntag, Wochenende = Freitag 00:00 bis Montag 00:00.
    pub fn bounds(self, today: NaiveDate) -> (NaiveDateTime, Option<NaiveDateTime>) {
        let from = start_of(today);
        let next_monday = start_of(add_days(monday_of(today), 7));
        match self {
            EventRange::All => (from, None),
            EventRange::Today => (from, Some(start_of(add_days(today, 1)))),
            EventRange::Week => (from, Some(next_monday)),
            EventRange::Weekend => {
                let friday = start_of(add_days(monday_of(today), 4));
                (from.max(friday), Some(next_monday))
            }
        }
    }
}

/// Rohe Eingaben aus dem Formular.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EventInput {
    pub title: String,
    pub description: String,
    /// `YYYY-MM-DD`
    pub date: String,
    /// `HH:MM`
    pub time: String,
    pub venue: String,
    pub city: String,
    #[serde(default)]
    pub lineup: String,
    #[serde(default)]
    pub ticket_url: String,
    #[serde(default)]
    pub ticket_status: String,
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub image_credit: String,
}

/// Geprüftes Event, bereit zum Speichern.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidEvent {
    pub title: String,
    pub description: String,
    pub starts_at: NaiveDateTime,
    pub venue: String,
    pub city: String,
    pub lineup: String,
    pub ticket_url: Option<String>,
    pub ticket_status: TicketStatus,
    pub image: Option<EventImage>,
}

/// Fehlermeldung pro Feld (`None` = Feld ist in Ordnung).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldErrors {
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
    pub venue: Option<String>,
    pub city: Option<String>,
    pub lineup: Option<String>,
    pub ticket_url: Option<String>,
    pub ticket_status: Option<String>,
    pub image_url: Option<String>,
    pub image_credit: Option<String>,
}

impl FieldErrors {
    pub fn is_empty(&self) -> bool {
        *self == FieldErrors::default()
    }
}

/// Ein gespeichertes Event mit allen Feldern.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub starts_at: NaiveDateTime,
    pub venue: String,
    pub city: String,
    pub lineup: String,
    pub ticket_url: Option<String>,
    pub ticket_status: TicketStatus,
    pub image: Option<EventImage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Kurzform für die Verwaltungsliste.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSummary {
    pub id: i64,
    pub title: String,
    pub starts_at: NaiveDateTime,
    pub city: String,
    pub image: Option<EventImage>,
    pub updated_at: DateTime<Utc>,
}

/// Eine Seite der Verwaltungsliste.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventPage {
    pub items: Vec<EventSummary>,
    pub page: u32,
    pub total_pages: u32,
    pub total: i64,
}

/// Karte in der öffentlichen Eventliste.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventCard {
    pub id: i64,
    pub title: String,
    pub teaser: String,
    pub starts_at: NaiveDateTime,
    pub venue: String,
    pub city: String,
    pub lineup: String,
    pub ticket_url: Option<String>,
    pub ticket_status: TicketStatus,
    pub image: Option<EventImage>,
}

/// Öffentliche Eventliste: kommende Events + das heutige Datum (für „Heute“).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventListing {
    pub today: NaiveDate,
    pub items: Vec<EventCard>,
}

// `FieldErrors` ist bewusst ein flacher Wert und entsteht einmal pro Anfrage – Boxen brächte nichts.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateOutcome {
    Created { id: i64 },
    Invalid(FieldErrors),
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateOutcome {
    Updated { id: i64 },
    Invalid(FieldErrors),
    NotFound,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteOutcome {
    Deleted,
    NotFound,
}

/// Anzahl Seiten für `total` Einträge – mindestens 1.
pub fn total_pages(total: i64) -> u32 {
    let pages = (total.max(0) + PAGE_SIZE - 1) / PAGE_SIZE;
    pages.max(1) as u32
}

/// Ticket-Link: nur `https://`, keine Leerzeichen, höchstens 500 Zeichen.
pub fn is_valid_ticket_url(url: &str) -> bool {
    url.starts_with("https://")
        && url.len() > "https://".len()
        && url.chars().count() <= TICKET_URL_MAX_CHARS
        && !url.chars().any(char::is_whitespace)
}

/// Pflichtfeld einzeilig: getrimmt, 1–`max` Zeichen.
fn required_line(raw: &str, max: usize, empty: &str, too_long: &str, error: &mut Option<String>) -> String {
    let value = raw.trim().to_string();
    if value.is_empty() {
        *error = Some(empty.into());
    } else if value.chars().count() > max {
        *error = Some(too_long.into());
    }
    value
}

/// Prüft alle Regeln aus ANF-006 und sammelt **alle** Fehler auf einmal.
#[allow(clippy::result_large_err)]
pub fn validate(input: &EventInput) -> Result<ValidEvent, FieldErrors> {
    let mut errors = FieldErrors::default();

    let title = required_line(&input.title, TITLE_MAX_CHARS, msg::TITLE_EMPTY, msg::TITLE_TOO_LONG, &mut errors.title);

    // Beschreibung bleibt unverändert (Zeilenumbrüche); nur Leerzeichen = leer
    let description = input.description.clone();
    if description.trim().is_empty() {
        errors.description = Some(msg::DESCRIPTION_EMPTY.into());
    } else if description.chars().count() > DESCRIPTION_MAX_CHARS {
        errors.description = Some(msg::DESCRIPTION_TOO_LONG.into());
    }

    let date = input.date.trim();
    let date = if date.is_empty() {
        errors.date = Some(msg::DATE_MISSING.into());
        None
    } else {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| errors.date = Some(msg::DATE_INVALID.into())).ok()
    };

    let time = input.time.trim();
    let time = if time.is_empty() {
        errors.time = Some(msg::TIME_MISSING.into());
        None
    } else {
        // Browser schicken teils Sekunden mit (`23:00:00`)
        NaiveTime::parse_from_str(time, "%H:%M")
            .or_else(|_| NaiveTime::parse_from_str(time, "%H:%M:%S"))
            .map_err(|_| errors.time = Some(msg::TIME_INVALID.into()))
            .ok()
    };

    let venue = required_line(&input.venue, VENUE_MAX_CHARS, msg::VENUE_EMPTY, msg::VENUE_TOO_LONG, &mut errors.venue);
    let city = required_line(&input.city, CITY_MAX_CHARS, msg::CITY_EMPTY, msg::CITY_TOO_LONG, &mut errors.city);

    let lineup = input.lineup.trim().to_string();
    if lineup.chars().count() > LINEUP_MAX_CHARS {
        errors.lineup = Some(msg::LINEUP_TOO_LONG.into());
    }

    let ticket_url = input.ticket_url.trim();
    let ticket_url = if ticket_url.is_empty() {
        None
    } else {
        if !is_valid_ticket_url(ticket_url) {
            errors.ticket_url = Some(msg::TICKET_URL_INVALID.into());
        }
        Some(ticket_url.to_string())
    };

    // Leer = Standard („Tickets verfügbar“)
    let ticket_status = match input.ticket_status.trim() {
        "" => Some(TicketStatus::default()),
        s => TicketStatus::parse(s),
    };
    if ticket_status.is_none() {
        errors.ticket_status = Some(msg::TICKET_STATUS_INVALID.into());
    }

    // Bild: optional; mit Adresse ist der Bildnachweis Pflicht (ANF-004)
    let image_url = input.image_url.trim();
    let image_credit = input.image_credit.trim();
    let image = if image_url.is_empty() {
        None
    } else {
        if !is_valid_image_url(image_url) {
            errors.image_url = Some(msg::IMAGE_URL_INVALID.into());
        }
        if image_credit.is_empty() {
            errors.image_credit = Some(msg::IMAGE_CREDIT_MISSING.into());
        } else if image_credit.chars().count() > IMAGE_CREDIT_MAX_CHARS {
            errors.image_credit = Some(msg::IMAGE_CREDIT_TOO_LONG.into());
        }
        Some(EventImage { url: image_url.into(), credit: image_credit.into() })
    };

    match (errors.is_empty(), date, time, ticket_status) {
        (true, Some(date), Some(time), Some(ticket_status)) => Ok(ValidEvent {
            title,
            description,
            starts_at: date.and_time(time),
            venue,
            city,
            lineup,
            ticket_url,
            ticket_status,
            image,
        }),
        _ => Err(errors),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid() -> EventInput {
        EventInput {
            title: "Nachtschicht #14".into(),
            description: "Drei DJs, eine Nacht.".into(),
            date: "2026-11-14".into(),
            time: "23:00".into(),
            venue: "Kellerraum 9".into(),
            city: "Berlin".into(),
            lineup: "Mara Voss, Jonas Keller".into(),
            ..Default::default()
        }
    }

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn gueltiges_event() {
        let e = validate(&valid()).unwrap();
        assert_eq!(e.starts_at.to_string(), "2026-11-14 23:00:00");
        assert_eq!(e.ticket_status, TicketStatus::Available);
        assert_eq!(e.ticket_url, None);
    }

    #[test]
    fn ak3_titel_leer() {
        let e = validate(&EventInput { title: "   ".into(), ..valid() }).unwrap_err();
        assert_eq!(e.title.as_deref(), Some(msg::TITLE_EMPTY));
    }

    #[test]
    fn ak4_titel_zu_lang() {
        let e = validate(&EventInput { title: "a".repeat(151), ..valid() }).unwrap_err();
        assert_eq!(e.title.as_deref(), Some(msg::TITLE_TOO_LONG));
        assert!(validate(&EventInput { title: "a".repeat(150), ..valid() }).is_ok());
    }

    #[test]
    fn ak5_beschreibung_leer() {
        let e = validate(&EventInput { description: " \n ".into(), ..valid() }).unwrap_err();
        assert_eq!(e.description.as_deref(), Some(msg::DESCRIPTION_EMPTY));
    }

    #[test]
    fn ak6_ak7_bildregeln() {
        let e = validate(&EventInput { image_url: "/images/events/a.jpg".into(), ..valid() }).unwrap_err();
        assert_eq!(e.image_credit.as_deref(), Some(msg::IMAGE_CREDIT_MISSING));
        for bad in ["ftp://x", "bild.jpg", "/a b.jpg"] {
            let e = validate(&EventInput { image_url: bad.into(), image_credit: "Foto".into(), ..valid() }).unwrap_err();
            assert_eq!(e.image_url.as_deref(), Some(msg::IMAGE_URL_INVALID), "{bad}");
        }
    }

    #[test]
    fn ak8_nachweis_ohne_bild_wird_verworfen() {
        let e = validate(&EventInput { image_credit: "Foto".into(), ..valid() }).unwrap();
        assert_eq!(e.image, None);
    }

    #[test]
    fn ak18_beginn_ort_stadt_pflicht() {
        let e = validate(&EventInput { date: "".into(), time: "".into(), venue: " ".into(), city: "".into(), ..valid() })
            .unwrap_err();
        assert_eq!(e.date.as_deref(), Some(msg::DATE_MISSING));
        assert_eq!(e.time.as_deref(), Some(msg::TIME_MISSING));
        assert_eq!(e.venue.as_deref(), Some(msg::VENUE_EMPTY));
        assert_eq!(e.city.as_deref(), Some(msg::CITY_EMPTY));
    }

    #[test]
    fn ak18_ungueltiges_datum_und_uhrzeit() {
        let e = validate(&EventInput { date: "2026-02-30".into(), time: "25:00".into(), ..valid() }).unwrap_err();
        assert_eq!(e.date.as_deref(), Some(msg::DATE_INVALID));
        assert_eq!(e.time.as_deref(), Some(msg::TIME_INVALID));
        // Sekunden vom Browser sind ok
        assert!(validate(&EventInput { time: "23:00:00".into(), ..valid() }).is_ok());
    }

    #[test]
    fn ak19_ticketlink_nur_https() {
        for bad in ["http://tickets.example", "tickets.example", "https://a b"] {
            let e = validate(&EventInput { ticket_url: bad.into(), ..valid() }).unwrap_err();
            assert_eq!(e.ticket_url.as_deref(), Some(msg::TICKET_URL_INVALID), "{bad}");
        }
        let ok = validate(&EventInput { ticket_url: " https://tickets.example/14 ".into(), ..valid() }).unwrap();
        assert_eq!(ok.ticket_url.as_deref(), Some("https://tickets.example/14"));
    }

    #[test]
    fn ticketstatus() {
        let e = validate(&EventInput { ticket_status: "sold_out".into(), ..valid() }).unwrap();
        assert_eq!(e.ticket_status, TicketStatus::SoldOut);
        let e = validate(&EventInput { ticket_status: "gratis".into(), ..valid() }).unwrap_err();
        assert_eq!(e.ticket_status.as_deref(), Some(msg::TICKET_STATUS_INVALID));
    }

    #[test]
    fn lineup_zu_lang() {
        let e = validate(&EventInput { lineup: "x".repeat(301), ..valid() }).unwrap_err();
        assert_eq!(e.lineup.as_deref(), Some(msg::LINEUP_TOO_LONG));
    }

    #[test]
    fn zeitraeume() {
        // Mittwoch, 11.11.2026
        let mi = d("2026-11-11");
        let at = |s: &str| start_of(d(s));
        assert_eq!(EventRange::All.bounds(mi), (at("2026-11-11"), None));
        assert_eq!(EventRange::Today.bounds(mi), (at("2026-11-11"), Some(at("2026-11-12"))));
        assert_eq!(EventRange::Week.bounds(mi), (at("2026-11-11"), Some(at("2026-11-16"))));
        assert_eq!(EventRange::Weekend.bounds(mi), (at("2026-11-13"), Some(at("2026-11-16"))));
        // Am Samstag beginnt das Wochenende „heute“
        let sa = d("2026-11-14");
        assert_eq!(EventRange::Weekend.bounds(sa), (at("2026-11-14"), Some(at("2026-11-16"))));
        // Sonntag: Woche endet am Montag
        let so = d("2026-11-15");
        assert_eq!(EventRange::Week.bounds(so), (at("2026-11-15"), Some(at("2026-11-16"))));
    }

    #[test]
    fn zeitraum_aus_adresse() {
        assert_eq!(EventRange::parse("wochenende"), EventRange::Weekend);
        assert_eq!(EventRange::parse("quatsch"), EventRange::All);
    }
}
