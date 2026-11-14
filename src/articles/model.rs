use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

pub const TITLE_MAX_CHARS: usize = 200;
/// ANF-004
pub const IMAGE_URL_MAX_CHARS: usize = 500;
pub const IMAGE_CREDIT_MAX_CHARS: usize = 200;
/// Einträge pro Seite in der Liste (ANF-002, R-2).
pub const PAGE_SIZE: i64 = 20;
/// Startseite (ANF-003): 1 Aufmacher + 6 Karten + 6 Listeneinträge.
pub const FRONT_GRID: usize = 6;
pub const FRONT_LIST: usize = 6;
pub const FRONT_TOTAL: i64 = 1 + FRONT_GRID as i64 + FRONT_LIST as i64;
/// Maximale Länge des Anrisses in Zeichen.
pub const TEASER_MAX_CHARS: usize = 160;
/// Lesegeschwindigkeit für die Lesezeit.
pub const WORDS_PER_MINUTE: usize = 200;

/// Fehlermeldungen – Texte exakt wie in den Abnahmekriterien von ANF-001.
pub mod msg {
    pub const TITLE_EMPTY: &str = "Bitte einen Titel eingeben.";
    pub const TITLE_TOO_LONG: &str = "Der Titel darf höchstens 200 Zeichen lang sein.";
    pub const BODY_EMPTY: &str = "Bitte einen Text eingeben.";
    pub const DATE_MISSING: &str = "Bitte ein Veröffentlichungsdatum angeben.";
    pub const DATE_INVALID: &str = "Bitte ein gültiges Datum angeben.";
    pub const STATUS_INVALID: &str = "Bitte einen gültigen Status wählen.";
    // ANF-008
    pub const CATEGORY_INVALID: &str = "Bitte eine gültige Rubrik wählen.";
    pub const NEWS_EMPTY: &str = "Noch keine News veröffentlicht.";
    pub const NEWS_NO_MATCH: &str = "Keine News gefunden. Versuch es mit einem anderen Suchbegriff oder einer anderen Rubrik.";
    // ANF-002
    pub const NOT_FOUND: &str = "Dieser Artikel existiert nicht mehr.";
    pub const SAVED: &str = "Änderungen gespeichert.";
    pub const DELETED: &str = "Artikel gelöscht.";
    pub const LIST_EMPTY: &str = "Noch keine Artikel vorhanden.";
    pub const LIST_EMPTY_FILTER: &str = "Keine Artikel mit diesem Status.";
    // ANF-003
    pub const FRONT_EMPTY: &str = "Noch keine Artikel veröffentlicht.";
    // ANF-004
    pub const IMAGE_URL_INVALID: &str = "Bitte eine gültige Bild-Adresse angeben (https://… oder /…).";
    pub const IMAGE_CREDIT_MISSING: &str = "Bitte einen Bildnachweis angeben.";
    pub const IMAGE_CREDIT_TOO_LONG: &str = "Der Bildnachweis darf höchstens 200 Zeichen lang sein.";
}

/// Status eines Artikels. Wird in der DB als Text gespeichert.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArticleStatus {
    #[default]
    Draft,
    Published,
}

impl ArticleStatus {
    pub const ALL: [ArticleStatus; 2] = [ArticleStatus::Draft, ArticleStatus::Published];

    /// Wert in Formular und Datenbank.
    pub fn as_str(self) -> &'static str {
        match self {
            ArticleStatus::Draft => "draft",
            ArticleStatus::Published => "published",
        }
    }

    /// Anzeige-Text.
    pub fn label(self) -> &'static str {
        match self {
            ArticleStatus::Draft => "Entwurf",
            ArticleStatus::Published => "Veröffentlicht",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == value)
    }
}

/// ANF-008: Rubrik eines Artikels (Filter auf der News-Seite).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArticleCategory {
    #[default]
    News,
    Interview,
    Release,
    Event,
    Mix,
}

impl ArticleCategory {
    pub const ALL: [ArticleCategory; 5] = [
        ArticleCategory::News,
        ArticleCategory::Interview,
        ArticleCategory::Release,
        ArticleCategory::Event,
        ArticleCategory::Mix,
    ];

    /// Wert in Formular, Adresse und Datenbank.
    pub fn as_str(self) -> &'static str {
        match self {
            ArticleCategory::News => "news",
            ArticleCategory::Interview => "interview",
            ArticleCategory::Release => "release",
            ArticleCategory::Event => "event",
            ArticleCategory::Mix => "mix",
        }
    }

    /// Einzahl, z. B. über dem Titel einer Meldung.
    pub fn label(self) -> &'static str {
        match self {
            ArticleCategory::News => "Meldung",
            ArticleCategory::Interview => "Interview",
            ArticleCategory::Release => "Release",
            ArticleCategory::Event => "Event",
            ArticleCategory::Mix => "Mix",
        }
    }

    /// Mehrzahl für die Filter-Chips.
    pub fn plural(self) -> &'static str {
        match self {
            ArticleCategory::News => "Meldungen",
            ArticleCategory::Interview => "Interviews",
            ArticleCategory::Release => "Releases",
            ArticleCategory::Event => "Events",
            ArticleCategory::Mix => "Mixe",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.as_str() == value)
    }
}

/// Rohe Eingaben aus dem Formular (alles Strings, wie vom Browser geschickt).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ArticleInput {
    pub title: String,
    pub body: String,
    /// `YYYY-MM-DD` aus `<input type="date">` oder leer.
    pub published_on: String,
    /// `draft` oder `published`.
    pub status: String,
    /// ANF-004: leer = kein Bild
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub image_credit: String,
    /// ANF-008: leer = „Meldung“
    #[serde(default)]
    pub category: String,
}

/// Geprüfter Artikel, bereit zum Speichern.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidArticle {
    pub title: String,
    pub body: String,
    pub published_on: Option<NaiveDate>,
    pub status: ArticleStatus,
    pub image: Option<ArticleImage>,
    pub category: ArticleCategory,
}

/// Bild eines Artikels mit Bildnachweis (ANF-004).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleImage {
    pub url: String,
    pub credit: String,
}

impl ArticleImage {
    /// Aus den DB-Spalten – nur wenn beide gesetzt sind.
    pub fn from_columns(url: Option<String>, credit: Option<String>) -> Option<Self> {
        Some(Self { url: url?, credit: credit? })
    }
}

/// Fehlermeldung pro Feld (`None` = Feld ist in Ordnung).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldErrors {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published_on: Option<String>,
    pub status: Option<String>,
    pub image_url: Option<String>,
    pub image_credit: Option<String>,
    pub category: Option<String>,
}

impl FieldErrors {
    pub fn is_empty(&self) -> bool {
        self.category.is_none()
            && self.title.is_none()
            && self.body.is_none()
            && self.published_on.is_none()
            && self.status.is_none()
            && self.image_url.is_none()
            && self.image_credit.is_none()
    }
}

/// Ergebnis von „Artikel speichern“.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateOutcome {
    Created { id: i64 },
    Invalid(FieldErrors),
}

/// Ein gespeicherter Artikel mit allen Feldern (Detail, Bearbeiten).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published_on: Option<NaiveDate>,
    pub status: ArticleStatus,
    pub image: Option<ArticleImage>,
    pub category: ArticleCategory,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Kurzform für die Liste (ohne Text).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleSummary {
    pub id: i64,
    pub title: String,
    pub published_on: Option<NaiveDate>,
    pub status: ArticleStatus,
    pub updated_at: DateTime<Utc>,
}

/// Eine Seite der Artikelliste.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticlePage {
    pub items: Vec<ArticleSummary>,
    /// Aktuelle Seite (ab 1).
    pub page: u32,
    pub total_pages: u32,
    /// Anzahl Artikel insgesamt (mit Filter).
    pub total: i64,
}

/// Ergebnis von „Artikel bearbeiten“.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateOutcome {
    Updated { id: i64 },
    Invalid(FieldErrors),
    NotFound,
}

/// Ergebnis von „Artikel löschen“.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteOutcome {
    Deleted,
    NotFound,
}

/// Öffentlicher Artikel für die Startseite (ANF-003).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArticleTeaser {
    pub id: i64,
    pub title: String,
    pub teaser: String,
    pub published_on: NaiveDate,
    pub reading_minutes: u32,
    pub image: Option<ArticleImage>,
    pub category: ArticleCategory,
}

/// ANF-008: Eine Seite der News-Liste.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewsPage {
    pub items: Vec<ArticleTeaser>,
    pub page: u32,
    pub total_pages: u32,
    pub total: i64,
}

/// Anriss: Text ohne Zeilenumbrüche, höchstens `max` Zeichen, an einer
/// Wortgrenze gekürzt, mit „…“ wenn gekürzt wurde.
pub fn teaser(body: &str, max: usize) -> String {
    let text = body.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.chars().count() <= max {
        return text;
    }
    // Platz für „…“ lassen
    let cut: String = text.chars().take(max - 1).collect();
    let cut = match cut.rfind(' ') {
        Some(pos) if pos > 0 => &cut[..pos],
        _ => cut.as_str(),
    };
    let cut = cut.trim_end_matches(|c: char| c.is_ascii_punctuation() || c == '–');
    format!("{}…", cut.trim_end())
}

/// Lesezeit in Minuten (aufgerundet, mindestens 1).
pub fn reading_minutes(body: &str) -> u32 {
    let words = body.split_whitespace().count();
    words.div_ceil(WORDS_PER_MINUTE).max(1) as u32
}

/// Absätze = durch Leerzeilen getrennte Blöcke.
pub fn paragraphs(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = Vec::new();
    for line in body.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                out.push(current.join("\n"));
                current.clear();
            }
        } else {
            current.push(line.trim_end());
        }
    }
    if !current.is_empty() {
        out.push(current.join("\n"));
    }
    out
}

/// `13. November 2026`
pub fn format_date_long(d: NaiveDate) -> String {
    const MONTHS: [&str; 12] = [
        "Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "September",
        "Oktober", "November", "Dezember",
    ];
    use chrono::Datelike;
    format!("{}. {} {}", d.day(), MONTHS[d.month0() as usize], d.year())
}

/// Anzahl Seiten für `total` Einträge – mindestens 1.
pub fn total_pages(total: i64) -> u32 {
    let pages = (total.max(0) + PAGE_SIZE - 1) / PAGE_SIZE;
    pages.max(1) as u32
}

/// `12.11.2026`
pub fn format_date(d: NaiveDate) -> String {
    d.format("%d.%m.%Y").to_string()
}

/// `12.11.2026 14:05 UTC` – bewusst UTC, damit Server und Browser gleich rendern.
pub fn format_timestamp(t: DateTime<Utc>) -> String {
    t.format("%d.%m.%Y %H:%M UTC").to_string()
}

/// Prüft alle Regeln aus ANF-001 und sammelt **alle** Fehler auf einmal.
// `FieldErrors` ist bewusst ein flacher Wert (eine Meldung pro Feld) und wird
// nur einmal pro Anfrage erzeugt – Boxen brächte hier keinen Vorteil.
#[allow(clippy::result_large_err)]
pub fn validate(input: &ArticleInput) -> Result<ValidArticle, FieldErrors> {
    let mut errors = FieldErrors::default();

    // Titel: getrimmt, 1–200 Zeichen
    let title = input.title.trim().to_string();
    if title.is_empty() {
        errors.title = Some(msg::TITLE_EMPTY.into());
    } else if title.chars().count() > TITLE_MAX_CHARS {
        errors.title = Some(msg::TITLE_TOO_LONG.into());
    }

    // Text: nicht nur Leerzeichen; Inhalt bleibt unverändert (Zeilenumbrüche!)
    if input.body.trim().is_empty() {
        errors.body = Some(msg::BODY_EMPTY.into());
    }

    // Status
    let status = ArticleStatus::parse(input.status.trim());
    if status.is_none() {
        errors.status = Some(msg::STATUS_INVALID.into());
    }

    // Datum: optional, aber Pflicht bei „Veröffentlicht“
    let date_raw = input.published_on.trim();
    let published_on = if date_raw.is_empty() {
        None
    } else {
        match NaiveDate::parse_from_str(date_raw, "%Y-%m-%d") {
            Ok(d) => Some(d),
            Err(_) => {
                errors.published_on = Some(msg::DATE_INVALID.into());
                None
            }
        }
    };
    if status == Some(ArticleStatus::Published)
        && date_raw.is_empty()
        && errors.published_on.is_none()
    {
        errors.published_on = Some(msg::DATE_MISSING.into());
    }

    // Bild (ANF-004): optional; mit Adresse ist der Bildnachweis Pflicht
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
        Some(ArticleImage { url: image_url.into(), credit: image_credit.into() })
    };

    // Rubrik (ANF-008): leer = Standard
    let category = match input.category.trim() {
        "" => Some(ArticleCategory::default()),
        c => ArticleCategory::parse(c),
    };
    if category.is_none() {
        errors.category = Some(msg::CATEGORY_INVALID.into());
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(ValidArticle {
        title,
        body: input.body.clone(),
        published_on,
        status: status.expect("geprüft"),
        image,
        category: category.expect("geprüft"),
    })
}

/// `https://…` oder ein Pfad im Projekt (`/…`, aber nicht `//…`),
/// ohne Leerzeichen, höchstens 500 Zeichen. Gleiche Regel wie in der DB.
pub fn is_valid_image_url(url: &str) -> bool {
    let scheme_ok = url.starts_with("https://") || (url.starts_with('/') && !url.starts_with("//"));
    scheme_ok
        && url.len() > 1
        && !url.chars().any(char::is_whitespace)
        && url.chars().count() <= IMAGE_URL_MAX_CHARS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(title: &str, body: &str, date: &str, status: &str) -> ArticleInput {
        ArticleInput {
            title: title.into(),
            body: body.into(),
            published_on: date.into(),
            status: status.into(),
            ..Default::default()
        }
    }

    fn with_image(url: &str, credit: &str) -> ArticleInput {
        ArticleInput {
            image_url: url.into(),
            image_credit: credit.into(),
            ..input("Titel", "Text", "", "draft")
        }
    }

    #[test]
    fn anf004_ak1_bild_mit_nachweis() {
        let a = validate(&with_image(" /images/artikel/a.jpg ", " Foto: X ")).unwrap();
        assert_eq!(a.image, Some(ArticleImage { url: "/images/artikel/a.jpg".into(), credit: "Foto: X".into() }));
        assert!(validate(&with_image("https://example.org/b.jpg", "Foto: Y")).is_ok());
    }

    #[test]
    fn anf004_ak2_nachweis_fehlt() {
        let e = validate(&with_image("/a.jpg", "  ")).unwrap_err();
        assert_eq!(e.image_credit.as_deref(), Some(msg::IMAGE_CREDIT_MISSING));
    }

    #[test]
    fn anf004_ak3_ungueltige_adresse() {
        for url in ["ftp://x/a.jpg", "bild.jpg", "http://x/a.jpg", "//x/a.jpg", "/mit leerzeichen.jpg", "/", "javascript:alert(1)"] {
            let e = validate(&with_image(url, "Foto")).unwrap_err();
            assert_eq!(e.image_url.as_deref(), Some(msg::IMAGE_URL_INVALID), "{url}");
        }
        let lang = format!("https://x.org/{}", "a".repeat(500));
        assert!(validate(&with_image(&lang, "Foto")).is_err());
    }

    #[test]
    fn anf004_ak4_nachweis_zu_lang() {
        assert!(validate(&with_image("/a.jpg", &"ä".repeat(200))).is_ok());
        let e = validate(&with_image("/a.jpg", &"a".repeat(201))).unwrap_err();
        assert_eq!(e.image_credit.as_deref(), Some(msg::IMAGE_CREDIT_TOO_LONG));
    }

    #[test]
    fn anf004_ak5_nachweis_ohne_bild_wird_ignoriert() {
        let a = validate(&with_image("", "Foto: niemand")).unwrap();
        assert_eq!(a.image, None);
    }

    #[test]
    fn ak1_entwurf_ohne_datum_ist_gueltig() {
        let a = validate(&input("Titel", "Text", "", "draft")).unwrap();
        assert_eq!(a.status, ArticleStatus::Draft);
        assert_eq!(a.published_on, None);
    }

    #[test]
    fn ak2_veroeffentlicht_mit_datum_ist_gueltig() {
        let a = validate(&input("Titel", "Text", "2026-11-12", "published")).unwrap();
        assert_eq!(a.status, ArticleStatus::Published);
        assert_eq!(a.published_on, NaiveDate::from_ymd_opt(2026, 11, 12));
    }

    #[test]
    fn ak3_veroeffentlicht_ohne_datum_ist_ungueltig() {
        let e = validate(&input("Titel", "Text", "", "published")).unwrap_err();
        assert_eq!(e.published_on.as_deref(), Some(msg::DATE_MISSING));
    }

    #[test]
    fn ak4_titel_leer_oder_nur_leerzeichen() {
        for t in ["", "   ", "\t\n"] {
            let e = validate(&input(t, "Text", "", "draft")).unwrap_err();
            assert_eq!(e.title.as_deref(), Some(msg::TITLE_EMPTY));
        }
    }

    #[test]
    fn ak5_titel_laenge_grenzen() {
        // genau 200 Zeichen (mit Umlauten = mehr Bytes) ist ok
        let ok = "ä".repeat(200);
        assert!(validate(&input(&ok, "Text", "", "draft")).is_ok());
        let e = validate(&input(&"a".repeat(201), "Text", "", "draft")).unwrap_err();
        assert_eq!(e.title.as_deref(), Some(msg::TITLE_TOO_LONG));
    }

    #[test]
    fn ak6_text_leer_oder_nur_leerzeichen() {
        for b in ["", "  ", "\n\n"] {
            let e = validate(&input("Titel", b, "", "draft")).unwrap_err();
            assert_eq!(e.body.as_deref(), Some(msg::BODY_EMPTY));
        }
    }

    #[test]
    fn titel_wird_getrimmt_text_bleibt_unveraendert() {
        let a = validate(&input("  Titel  ", "Zeile 1\nZeile 2\n", "", "draft")).unwrap();
        assert_eq!(a.title, "Titel");
        assert_eq!(a.body, "Zeile 1\nZeile 2\n");
    }

    #[test]
    fn alle_fehler_werden_gleichzeitig_gemeldet() {
        let e = validate(&input("", "", "", "published")).unwrap_err();
        assert!(e.title.is_some() && e.body.is_some() && e.published_on.is_some());
    }

    #[test]
    fn ungueltiges_datum_und_status() {
        let e = validate(&input("T", "B", "2026-02-30", "archiviert")).unwrap_err();
        assert_eq!(e.published_on.as_deref(), Some(msg::DATE_INVALID));
        assert_eq!(e.status.as_deref(), Some(msg::STATUS_INVALID));
    }

    #[test]
    fn anf002_seitenanzahl() {
        assert_eq!(total_pages(0), 1);
        assert_eq!(total_pages(20), 1);
        assert_eq!(total_pages(21), 2);
        assert_eq!(total_pages(25), 2);
        assert_eq!(total_pages(40), 2);
        assert_eq!(total_pages(41), 3);
    }

    #[test]
    fn anf002_formatierung() {
        let d = NaiveDate::from_ymd_opt(2026, 1, 5).unwrap();
        assert_eq!(format_date(d), "05.01.2026");
        let t = d.and_hms_opt(9, 7, 0).unwrap().and_utc();
        assert_eq!(format_timestamp(t), "05.01.2026 09:07 UTC");
    }

    #[test]
    fn anf003_ak4_anriss() {
        let kurz = "Kurzer\nText";
        assert_eq!(teaser(kurz, 160), "Kurzer Text");

        let lang = "Wort ".repeat(100); // 500 Zeichen
        let t = teaser(&lang, 160);
        assert!(t.chars().count() <= 160, "zu lang: {}", t.chars().count());
        assert!(t.ends_with("Wort…"), "endet nicht an Wortgrenze: {t}");

        // Umlaute (mehrere Bytes) dürfen nicht zum Absturz führen
        let umlaute = "Äpfel öffnen Übermut ".repeat(20);
        assert!(teaser(&umlaute, 160).chars().count() <= 160);

        // Satzzeichen vor „…“ werden entfernt
        assert_eq!(teaser("Eins zwei, drei vier", 12), "Eins zwei…");
    }

    #[test]
    fn anf003_ak5_lesezeit() {
        assert_eq!(reading_minutes(""), 1);
        assert_eq!(reading_minutes("ein Wort"), 1);
        assert_eq!(reading_minutes(&"wort ".repeat(200)), 1);
        assert_eq!(reading_minutes(&"wort ".repeat(201)), 2);
        assert_eq!(reading_minutes(&"wort ".repeat(450)), 3);
    }

    #[test]
    fn anf003_absaetze() {
        let p = paragraphs("Erster\nnoch erster\n\n\nZweiter\n  \nDritter\n");
        assert_eq!(p, ["Erster\nnoch erster", "Zweiter", "Dritter"]);
        assert!(paragraphs("  \n ").is_empty());
    }

    #[test]
    fn anf003_langes_datum() {
        let d = NaiveDate::from_ymd_opt(2026, 3, 5).unwrap();
        assert_eq!(format_date_long(d), "5. März 2026");
    }

    #[test]
    fn zukunftsdatum_erlaubt() {
        assert!(validate(&input("T", "B", "2099-01-01", "published")).is_ok());
    }

    #[test]
    fn anf008_rubrik() {
        let base = ArticleInput {
            title: "T".into(),
            body: "Text".into(),
            published_on: "2026-11-14".into(),
            status: "published".into(),
            ..Default::default()
        };
        assert_eq!(validate(&base).unwrap().category, ArticleCategory::News);
        let a = validate(&ArticleInput { category: "interview".into(), ..base.clone() }).unwrap();
        assert_eq!(a.category, ArticleCategory::Interview);
        let e = validate(&ArticleInput { category: "klatsch".into(), ..base }).unwrap_err();
        assert_eq!(e.category.as_deref(), Some(msg::CATEGORY_INVALID));
    }
}
