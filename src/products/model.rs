use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Bild + Bildnachweis – gleicher Typ und gleiche Regeln wie bei Artikeln (ANF-004).
pub use crate::articles::model::{ArticleImage as ProductImage, IMAGE_CREDIT_MAX_CHARS, is_valid_image_url};

pub const NAME_MAX_CHARS: usize = 120;
pub const SKU_MAX_CHARS: usize = 40;
pub const SHORT_DESCRIPTION_MAX_CHARS: usize = 200;
/// Höchstpreis 99'999.99 CHF in Rappen.
pub const PRICE_MAX_CENTS: i32 = 9_999_999;
/// Bis zu diesem Bestand heißt es „Nur noch <n> verfügbar“.
pub const LOW_STOCK_MAX: i32 = 5;
/// Einträge pro Seite in der Verwaltung (ANF-002, R-2).
pub const PAGE_SIZE: i64 = 20;
/// Produkte pro Seite im Shop.
pub const SHOP_PAGE_SIZE: i64 = 24;

/// Fehlermeldungen – Texte exakt wie in docs/features/ANF-005.
pub mod msg {
    pub const NAME_EMPTY: &str = "Bitte einen Namen eingeben.";
    pub const NAME_TOO_LONG: &str = "Der Name darf höchstens 120 Zeichen lang sein.";
    pub const SKU_EMPTY: &str = "Bitte eine Artikelnummer eingeben.";
    pub const SKU_INVALID: &str = "Die Artikelnummer darf nur Buchstaben, Ziffern und Bindestriche enthalten.";
    pub const SKU_TOO_LONG: &str = "Die Artikelnummer darf höchstens 40 Zeichen lang sein.";
    pub const SKU_TAKEN: &str = "Diese Artikelnummer ist bereits vergeben.";
    pub const SHORT_DESCRIPTION_TOO_LONG: &str = "Die Kurzbeschreibung darf höchstens 200 Zeichen lang sein.";
    pub const PRICE_INVALID: &str = "Bitte einen gültigen Preis eingeben, z. B. 19.90.";
    pub const PRICE_RANGE: &str =
        "Der Preis muss zwischen 0.00 und 99'999.99 CHF liegen (höchstens 2 Nachkommastellen).";
    pub const STOCK_INVALID: &str = "Der Lagerbestand muss eine ganze Zahl ab 0 sein.";
    pub const STATUS_INVALID: &str = "Bitte einen gültigen Status wählen.";
    // Teil C: Bild (Texte wie ANF-004)
    pub use crate::articles::model::msg::{IMAGE_CREDIT_MISSING, IMAGE_CREDIT_TOO_LONG, IMAGE_URL_INVALID};

    pub const NOT_FOUND: &str = "Dieses Produkt existiert nicht mehr.";
    pub const SAVED: &str = "Produkt gespeichert.";
    pub const DELETED: &str = "Produkt gelöscht.";
    pub const LIST_EMPTY: &str = "Noch keine Produkte vorhanden.";
    pub const LIST_EMPTY_FILTER: &str = "Keine Produkte mit diesem Status.";
    pub const SHOP_EMPTY: &str = "Zurzeit sind keine Produkte verfügbar.";
}

/// Status eines Produkts. Nur *Aktiv* erscheint im Shop.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductStatus {
    #[default]
    Draft,
    Active,
}

impl ProductStatus {
    pub const ALL: [ProductStatus; 2] = [ProductStatus::Draft, ProductStatus::Active];

    /// Wert in Formular, Adresse und Datenbank.
    pub fn as_str(self) -> &'static str {
        match self {
            ProductStatus::Draft => "draft",
            ProductStatus::Active => "active",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ProductStatus::Draft => "Entwurf",
            ProductStatus::Active => "Aktiv",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|s| s.as_str() == value)
    }
}

/// Verfügbarkeit, abgeleitet aus dem Lagerbestand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Availability {
    SoldOut,
    Low(i32),
    InStock,
}

impl Availability {
    pub fn from_stock(stock: i32) -> Self {
        match stock {
            s if s <= 0 => Availability::SoldOut,
            s if s <= LOW_STOCK_MAX => Availability::Low(s),
            _ => Availability::InStock,
        }
    }

    pub fn label(self) -> String {
        match self {
            Availability::SoldOut => "Ausverkauft".into(),
            Availability::Low(n) => format!("Nur noch {n} verfügbar"),
            Availability::InStock => "An Lager".into(),
        }
    }
}

/// Sortierung im Shop. Der Wert steht in der Adresse (`?sortierung=…`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShopSort {
    #[default]
    Newest,
    PriceAsc,
    PriceDesc,
    NameAsc,
}

impl ShopSort {
    pub const ALL: [ShopSort; 4] = [ShopSort::Newest, ShopSort::PriceAsc, ShopSort::PriceDesc, ShopSort::NameAsc];

    pub fn as_str(self) -> &'static str {
        match self {
            ShopSort::Newest => "neueste",
            ShopSort::PriceAsc => "preis-auf",
            ShopSort::PriceDesc => "preis-ab",
            ShopSort::NameAsc => "name",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ShopSort::Newest => "Neueste",
            ShopSort::PriceAsc => "Preis aufsteigend",
            ShopSort::PriceDesc => "Preis absteigend",
            ShopSort::NameAsc => "Name A–Z",
        }
    }

    /// Unbekannte Werte → Standard („Neueste“).
    pub fn parse(value: &str) -> Self {
        Self::ALL.into_iter().find(|s| s.as_str() == value).unwrap_or_default()
    }
}

/// Rohe Eingaben aus dem Formular.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProductInput {
    pub name: String,
    pub sku: String,
    pub short_description: String,
    pub description: String,
    /// z. B. `19.90` oder `19,90`
    pub price: String,
    pub stock: String,
    /// `draft` oder `active`
    pub status: String,
    /// Teil C: leer = kein Bild
    #[serde(default)]
    pub image_url: String,
    #[serde(default)]
    pub image_credit: String,
}

/// Geprüftes Produkt, bereit zum Speichern.
#[derive(Clone, Debug, PartialEq)]
pub struct ValidProduct {
    pub name: String,
    pub sku: String,
    pub short_description: String,
    pub description: String,
    pub price_cents: i32,
    pub stock: i32,
    pub status: ProductStatus,
    pub image: Option<ProductImage>,
}

/// Fehlermeldung pro Feld (`None` = Feld ist in Ordnung).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldErrors {
    pub name: Option<String>,
    pub sku: Option<String>,
    pub short_description: Option<String>,
    pub price: Option<String>,
    pub stock: Option<String>,
    pub status: Option<String>,
    pub image_url: Option<String>,
    pub image_credit: Option<String>,
}

impl FieldErrors {
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.sku.is_none()
            && self.short_description.is_none()
            && self.price.is_none()
            && self.stock.is_none()
            && self.status.is_none()
            && self.image_url.is_none()
            && self.image_credit.is_none()
    }
}

/// Ein gespeichertes Produkt mit allen Feldern.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub sku: String,
    pub short_description: String,
    pub description: String,
    pub price_cents: i32,
    pub stock: i32,
    pub status: ProductStatus,
    pub image: Option<ProductImage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Kurzform für die Verwaltungsliste.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductSummary {
    pub id: i64,
    pub name: String,
    pub sku: String,
    pub price_cents: i32,
    pub stock: i32,
    pub status: ProductStatus,
    pub updated_at: DateTime<Utc>,
}

/// Eine Seite der Verwaltungsliste.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductPage {
    pub items: Vec<ProductSummary>,
    pub page: u32,
    pub total_pages: u32,
    pub total: i64,
}

/// Produktkarte im Shop.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShopItem {
    pub id: i64,
    pub name: String,
    pub short_description: String,
    pub price_cents: i32,
    pub stock: i32,
    pub image: Option<ProductImage>,
}

/// Eine Seite des Shops.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShopPage {
    pub items: Vec<ShopItem>,
    pub page: u32,
    pub total_pages: u32,
    pub total: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateOutcome {
    Created { id: i64 },
    Invalid(FieldErrors),
}

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

/// Anzahl Seiten für `total` Einträge bei `size` pro Seite – mindestens 1.
pub fn total_pages(total: i64, size: i64) -> u32 {
    let pages = (total.max(0) + size - 1) / size;
    pages.max(1) as u32
}

/// `CHF 1'249.00`
pub fn format_chf(cents: i32) -> String {
    let francs = (cents / 100).to_string();
    let mut grouped = String::new();
    for (i, c) in francs.chars().enumerate() {
        if i > 0 && (francs.len() - i).is_multiple_of(3) {
            grouped.push('\'');
        }
        grouped.push(c);
    }
    format!("CHF {grouped}.{:02}", cents % 100)
}

/// Preis für das Eingabefeld beim Bearbeiten: `1249.00`.
pub fn price_input_value(cents: i32) -> String {
    format!("{}.{:02}", cents / 100, cents % 100)
}

/// Fehler beim Lesen eines Preises.
#[derive(Debug, PartialEq, Eq)]
pub enum PriceError {
    /// leer oder keine Zahl
    Invalid,
    /// negativ, zu groß oder mehr als 2 Nachkommastellen
    Range,
}

/// Liest einen Preis in CHF und gibt Rappen zurück.
/// Erlaubt: `19`, `19.9`, `19.90`, `19,90`, `1'249.00`.
pub fn parse_price(raw: &str) -> Result<i32, PriceError> {
    let s: String = raw.trim().chars().filter(|c| *c != '\'').collect();
    let (negative, s) = match s.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, s.as_str()),
    };
    let (francs, rappen) = match s.split_once(['.', ',']) {
        Some((f, r)) => (f, r),
        None => (s, ""),
    };
    let digits = |p: &str| p.chars().all(|c| c.is_ascii_digit());
    // „.50“ ist ok, „.“ oder „5.“ nicht
    if (francs.is_empty() && rappen.is_empty())
        || !digits(francs)
        || !digits(rappen)
        || (s.contains(['.', ',']) && rappen.is_empty())
    {
        return Err(PriceError::Invalid);
    }
    if negative || rappen.len() > 2 {
        return Err(PriceError::Range);
    }
    // Lange Zahlen nicht parsen, sondern direkt als „zu groß“ melden
    let francs = francs.trim_start_matches('0');
    if francs.len() > 5 {
        return Err(PriceError::Range);
    }
    let francs: i32 = if francs.is_empty() { 0 } else { francs.parse().map_err(|_| PriceError::Invalid)? };
    let rappen: i32 = format!("{rappen:0<2}").parse().map_err(|_| PriceError::Invalid)?;
    let cents = francs * 100 + rappen;
    if cents > PRICE_MAX_CENTS {
        return Err(PriceError::Range);
    }
    Ok(cents)
}

/// Artikelnummer wie gespeichert: ohne Leerzeichen außen, Großbuchstaben.
pub fn normalize_sku(raw: &str) -> String {
    raw.trim().to_ascii_uppercase()
}

/// Prüft alle Regeln aus ANF-005 und sammelt **alle** Fehler auf einmal.
/// Ob die Artikelnummer schon vergeben ist, prüft `repo` (braucht die DB).
#[allow(clippy::result_large_err)]
pub fn validate(input: &ProductInput) -> Result<ValidProduct, FieldErrors> {
    let mut errors = FieldErrors::default();

    let name = input.name.trim().to_string();
    if name.is_empty() {
        errors.name = Some(msg::NAME_EMPTY.into());
    } else if name.chars().count() > NAME_MAX_CHARS {
        errors.name = Some(msg::NAME_TOO_LONG.into());
    }

    let sku = normalize_sku(&input.sku);
    if sku.is_empty() {
        errors.sku = Some(msg::SKU_EMPTY.into());
    } else if !sku.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-') {
        errors.sku = Some(msg::SKU_INVALID.into());
    } else if sku.len() > SKU_MAX_CHARS {
        errors.sku = Some(msg::SKU_TOO_LONG.into());
    }

    let short_description = input.short_description.trim().to_string();
    if short_description.chars().count() > SHORT_DESCRIPTION_MAX_CHARS {
        errors.short_description = Some(msg::SHORT_DESCRIPTION_TOO_LONG.into());
    }

    // Beschreibung bleibt unverändert (Zeilenumbrüche); nur Leerzeichen = leer
    let description = if input.description.trim().is_empty() {
        String::new()
    } else {
        input.description.clone()
    };

    let price_cents = match parse_price(&input.price) {
        Ok(c) => Some(c),
        Err(PriceError::Invalid) => {
            errors.price = Some(msg::PRICE_INVALID.into());
            None
        }
        Err(PriceError::Range) => {
            errors.price = Some(msg::PRICE_RANGE.into());
            None
        }
    };

    let stock = input.stock.trim();
    let stock = match stock.parse::<i32>() {
        Ok(n) if n >= 0 && !stock.starts_with('+') => Some(n),
        _ => {
            errors.stock = Some(msg::STOCK_INVALID.into());
            None
        }
    };

    let status = ProductStatus::parse(input.status.trim());
    if status.is_none() {
        errors.status = Some(msg::STATUS_INVALID.into());
    }

    // Bild (Teil C): optional; mit Adresse ist der Bildnachweis Pflicht
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
        Some(ProductImage { url: image_url.into(), credit: image_credit.into() })
    };

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(ValidProduct {
        name,
        sku,
        short_description,
        description,
        price_cents: price_cents.expect("geprüft"),
        stock: stock.expect("geprüft"),
        status: status.expect("geprüft"),
        image,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> ProductInput {
        ProductInput {
            name: "T-Shirt".into(),
            sku: "ts-001".into(),
            short_description: "Bio-Baumwolle".into(),
            description: "Zeile 1\nZeile 2".into(),
            price: "19.90".into(),
            stock: "10".into(),
            status: "draft".into(),
            ..Default::default()
        }
    }

    fn with_image(url: &str, credit: &str) -> ProductInput {
        ProductInput { image_url: url.into(), image_credit: credit.into(), ..input() }
    }

    #[test]
    fn ak19_bild_mit_nachweis() {
        let p = validate(&with_image(" /images/produkte/a.jpg ", " Foto: X ")).unwrap();
        assert_eq!(p.image, Some(ProductImage { url: "/images/produkte/a.jpg".into(), credit: "Foto: X".into() }));
        assert_eq!(validate(&input()).unwrap().image, None);
    }

    #[test]
    fn ak20_bildregeln() {
        let e = validate(&with_image("/a.jpg", " ")).unwrap_err();
        assert_eq!(e.image_credit.as_deref(), Some(msg::IMAGE_CREDIT_MISSING));
        let e = validate(&with_image("bild.jpg", "Foto")).unwrap_err();
        assert_eq!(e.image_url.as_deref(), Some(msg::IMAGE_URL_INVALID));
        let e = validate(&with_image("/a.jpg", &"a".repeat(201))).unwrap_err();
        assert_eq!(e.image_credit.as_deref(), Some(msg::IMAGE_CREDIT_TOO_LONG));
        // Nachweis ohne Bild wird ignoriert
        assert_eq!(validate(&with_image("", "Foto")).unwrap().image, None);
    }

    #[test]
    fn ak1_gueltiges_produkt() {
        let p = validate(&input()).unwrap();
        assert_eq!(p.price_cents, 1990);
        assert_eq!(p.stock, 10);
        assert_eq!(p.sku, "TS-001");
        assert_eq!(p.status, ProductStatus::Draft);
        assert_eq!(p.description, "Zeile 1\nZeile 2");
    }

    #[test]
    fn ak2_preis_mit_komma_oder_punkt() {
        assert_eq!(parse_price("19,90"), Ok(1990));
        assert_eq!(parse_price("19.90"), Ok(1990));
        assert_eq!(parse_price(" 19.9 "), Ok(1990));
        assert_eq!(parse_price("19"), Ok(1900));
        assert_eq!(parse_price("0"), Ok(0));
        assert_eq!(parse_price(".5"), Ok(50));
        assert_eq!(parse_price("1'249.00"), Ok(124_900));
        assert_eq!(parse_price("99999.99"), Ok(PRICE_MAX_CENTS));
        assert_eq!(parse_price("000012.30"), Ok(1230));
    }

    #[test]
    fn ak3_name_leer() {
        for n in ["", "   ", "\t"] {
            let e = validate(&ProductInput { name: n.into(), ..input() }).unwrap_err();
            assert_eq!(e.name.as_deref(), Some(msg::NAME_EMPTY));
        }
        let e = validate(&ProductInput { name: "a".repeat(121), ..input() }).unwrap_err();
        assert_eq!(e.name.as_deref(), Some(msg::NAME_TOO_LONG));
        assert!(validate(&ProductInput { name: "ä".repeat(120), ..input() }).is_ok());
    }

    #[test]
    fn ak4_ungueltige_preise() {
        for p in ["", "  ", "abc", "1.2.3", "19.", ".", "1e3", "CHF 5", "--5"] {
            assert_eq!(parse_price(p), Err(PriceError::Invalid), "{p:?}");
        }
        for p in ["-5", "-0.50", "19.999", "100000", "100000.00", "999999999999999999999"] {
            assert_eq!(parse_price(p), Err(PriceError::Range), "{p:?}");
        }
        let e = validate(&ProductInput { price: "abc".into(), ..input() }).unwrap_err();
        assert_eq!(e.price.as_deref(), Some(msg::PRICE_INVALID));
        let e = validate(&ProductInput { price: "-5".into(), ..input() }).unwrap_err();
        assert_eq!(e.price.as_deref(), Some(msg::PRICE_RANGE));
    }

    #[test]
    fn ak5_ungueltiger_lagerbestand() {
        for s in ["-1", "2.5", "", "abc", "+3", "99999999999"] {
            let e = validate(&ProductInput { stock: s.into(), ..input() }).unwrap_err();
            assert_eq!(e.stock.as_deref(), Some(msg::STOCK_INVALID), "{s:?}");
        }
        assert_eq!(validate(&ProductInput { stock: " 0 ".into(), ..input() }).unwrap().stock, 0);
    }

    #[test]
    fn artikelnummer_regeln() {
        let e = validate(&ProductInput { sku: " ".into(), ..input() }).unwrap_err();
        assert_eq!(e.sku.as_deref(), Some(msg::SKU_EMPTY));
        for s in ["TS 001", "TS_001", "Ä-1", "ts/1"] {
            let e = validate(&ProductInput { sku: s.into(), ..input() }).unwrap_err();
            assert_eq!(e.sku.as_deref(), Some(msg::SKU_INVALID), "{s:?}");
        }
        let e = validate(&ProductInput { sku: "A".repeat(41), ..input() }).unwrap_err();
        assert_eq!(e.sku.as_deref(), Some(msg::SKU_TOO_LONG));
        assert!(validate(&ProductInput { sku: "A".repeat(40), ..input() }).is_ok());
    }

    #[test]
    fn kurzbeschreibung_und_beschreibung() {
        let e = validate(&ProductInput { short_description: "a".repeat(201), ..input() }).unwrap_err();
        assert_eq!(e.short_description.as_deref(), Some(msg::SHORT_DESCRIPTION_TOO_LONG));
        let p = validate(&ProductInput { short_description: "  kurz ".into(), description: " \n ".into(), ..input() }).unwrap();
        assert_eq!(p.short_description, "kurz");
        assert_eq!(p.description, "");
    }

    #[test]
    fn status_und_alle_fehler_gleichzeitig() {
        let e = validate(&ProductInput {
            name: "".into(),
            sku: "".into(),
            price: "x".into(),
            stock: "-1".into(),
            status: "archiviert".into(),
            ..input()
        })
        .unwrap_err();
        assert!(e.name.is_some() && e.sku.is_some() && e.price.is_some() && e.stock.is_some());
        assert_eq!(e.status.as_deref(), Some(msg::STATUS_INVALID));
        assert_eq!(validate(&ProductInput { status: "active".into(), ..input() }).unwrap().status, ProductStatus::Active);
    }

    #[test]
    fn ak10_preis_formatieren() {
        assert_eq!(format_chf(124_900), "CHF 1'249.00");
        assert_eq!(format_chf(1990), "CHF 19.90");
        assert_eq!(format_chf(5), "CHF 0.05");
        assert_eq!(format_chf(0), "CHF 0.00");
        assert_eq!(format_chf(100_000), "CHF 1'000.00");
        assert_eq!(format_chf(PRICE_MAX_CENTS), "CHF 99'999.99");
        assert_eq!(price_input_value(124_900), "1249.00");
        assert_eq!(parse_price(&price_input_value(1990)), Ok(1990));
    }

    #[test]
    fn ak11_verfuegbarkeit() {
        assert_eq!(Availability::from_stock(0).label(), "Ausverkauft");
        assert_eq!(Availability::from_stock(1).label(), "Nur noch 1 verfügbar");
        assert_eq!(Availability::from_stock(3).label(), "Nur noch 3 verfügbar");
        assert_eq!(Availability::from_stock(5).label(), "Nur noch 5 verfügbar");
        assert_eq!(Availability::from_stock(6).label(), "An Lager");
        assert_eq!(Availability::from_stock(20).label(), "An Lager");
    }

    #[test]
    fn sortierung_aus_adresse() {
        assert_eq!(ShopSort::parse("preis-auf"), ShopSort::PriceAsc);
        assert_eq!(ShopSort::parse("unsinn"), ShopSort::Newest);
        for s in ShopSort::ALL {
            assert_eq!(ShopSort::parse(s.as_str()), s);
        }
    }

    #[test]
    fn seitenanzahl() {
        assert_eq!(total_pages(0, SHOP_PAGE_SIZE), 1);
        assert_eq!(total_pages(24, SHOP_PAGE_SIZE), 1);
        assert_eq!(total_pages(30, SHOP_PAGE_SIZE), 2);
        assert_eq!(total_pages(21, PAGE_SIZE), 2);
    }
}
