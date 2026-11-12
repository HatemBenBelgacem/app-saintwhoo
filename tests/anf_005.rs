//! ANF-005 – Shop: Produkte erfassen und anzeigen, Integrationstests gegen PostgreSQL.
//!
//! Ausführen: `cargo test --features ssr`
//! Reine Formatierungs- und Prüfregeln stehen als Unit-Tests in
//! `src/products/model.rs`. AK-17 (Layout) wird per Screenshot geprüft.
#![cfg(feature = "ssr")]

use app_saintwhoo::products::model::{
    CreateOutcome, DeleteOutcome, ProductImage, ProductInput, ProductStatus, ShopSort, UpdateOutcome,
    format_chf, msg,
};
use app_saintwhoo::products::repo;
use sqlx::PgPool;

fn input(name: &str, sku: &str, price: &str, stock: &str, status: &str) -> ProductInput {
    ProductInput {
        name: name.into(),
        sku: sku.into(),
        short_description: String::new(),
        description: "Erster Absatz.\n\nZweiter Absatz.".into(),
        price: price.into(),
        stock: stock.into(),
        status: status.into(),
        ..Default::default()
    }
}

async fn create(pool: &PgPool, i: &ProductInput) -> i64 {
    match repo::create(pool, i).await.unwrap() {
        CreateOutcome::Created { id } => id,
        o => panic!("erwartet Created, bekommen {o:?}"),
    }
}

async fn active(pool: &PgPool, name: &str, price: &str, stock: &str) -> i64 {
    let sku = name.to_uppercase().replace(' ', "-");
    create(pool, &input(name, &sku, price, stock, "active")).await
}

async fn count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM products").fetch_one(pool).await.unwrap()
}

fn invalid(o: CreateOutcome) -> app_saintwhoo::products::model::FieldErrors {
    match o {
        CreateOutcome::Invalid(e) => e,
        o => panic!("erwartet Invalid, bekommen {o:?}"),
    }
}

#[sqlx::test]
async fn ak1_entwurf_gespeichert_aber_nicht_im_shop(pool: PgPool) {
    let id = create(&pool, &input("T-Shirt", "ts-001", "19.90", "10", "draft")).await;

    let p = repo::find(&pool, id).await.unwrap().unwrap();
    assert_eq!((p.price_cents, p.stock, p.sku.as_str()), (1990, 10, "TS-001"));
    assert_eq!(p.status, ProductStatus::Draft);

    assert_eq!(repo::list(&pool, 1, None).await.unwrap().items.len(), 1);
    assert!(repo::shop(&pool, 1, ShopSort::Newest).await.unwrap().items.is_empty());
}

#[sqlx::test]
async fn ak2_preis_mit_komma(pool: PgPool) {
    let a = create(&pool, &input("A", "A-1", "19,90", "1", "draft")).await;
    let b = create(&pool, &input("B", "B-1", "19.90", "1", "draft")).await;
    let pa = repo::find(&pool, a).await.unwrap().unwrap().price_cents;
    let pb = repo::find(&pool, b).await.unwrap().unwrap().price_cents;
    assert_eq!((pa, pb), (1990, 1990));
}

#[sqlx::test]
async fn ak3_name_leer(pool: PgPool) {
    for name in ["", "   "] {
        let e = invalid(repo::create(&pool, &input(name, "X-1", "1", "1", "draft")).await.unwrap());
        assert_eq!(e.name.as_deref(), Some(msg::NAME_EMPTY));
    }
    assert_eq!(count(&pool).await, 0);
}

#[sqlx::test]
async fn ak4_ungueltiger_preis(pool: PgPool) {
    for (price, expected) in [("-5", msg::PRICE_RANGE), ("abc", msg::PRICE_INVALID), ("19.999", msg::PRICE_RANGE)] {
        let e = invalid(repo::create(&pool, &input("A", "A-1", price, "1", "draft")).await.unwrap());
        assert_eq!(e.price.as_deref(), Some(expected), "{price}");
    }
    assert_eq!(count(&pool).await, 0);
}

#[sqlx::test]
async fn ak5_ungueltiger_lagerbestand(pool: PgPool) {
    for stock in ["-1", "2.5"] {
        let e = invalid(repo::create(&pool, &input("A", "A-1", "1", stock, "draft")).await.unwrap());
        assert_eq!(e.stock.as_deref(), Some(msg::STOCK_INVALID), "{stock}");
    }
    assert_eq!(count(&pool).await, 0);
}

#[sqlx::test]
async fn ak6_artikelnummer_doppelt(pool: PgPool) {
    let first = create(&pool, &input("Erstes", "TS-001", "1", "1", "draft")).await;

    // Kleinschreibung zählt als dieselbe Nummer
    let e = invalid(repo::create(&pool, &input("Zweites", " ts-001 ", "1", "1", "draft")).await.unwrap());
    assert_eq!(e.sku.as_deref(), Some(msg::SKU_TAKEN));
    assert_eq!(count(&pool).await, 1);

    // Alle Fehler gleichzeitig: doppelte Nummer + leerer Name
    let e = invalid(repo::create(&pool, &input("", "TS-001", "1", "1", "draft")).await.unwrap());
    assert!(e.name.is_some());
    assert_eq!(e.sku.as_deref(), Some(msg::SKU_TAKEN));

    // Beim Bearbeiten darf das Produkt seine eigene Nummer behalten …
    let out = repo::update(&pool, first, &input("Erstes neu", "TS-001", "2", "1", "draft")).await.unwrap();
    assert_eq!(out, UpdateOutcome::Updated { id: first });

    // … aber nicht die eines anderen übernehmen
    let other = create(&pool, &input("Anderes", "TS-002", "1", "1", "draft")).await;
    match repo::update(&pool, other, &input("Anderes", "ts-001", "1", "1", "draft")).await.unwrap() {
        UpdateOutcome::Invalid(e) => assert_eq!(e.sku.as_deref(), Some(msg::SKU_TAKEN)),
        o => panic!("{o:?}"),
    }
}

#[sqlx::test]
async fn ak7_bearbeiten(pool: PgPool) {
    let id = create(&pool, &input("A", "A-1", "10", "1", "draft")).await;
    let before = repo::find(&pool, id).await.unwrap().unwrap();

    let out = repo::update(&pool, id, &input("A", "A-1", "12.50", "1", "active")).await.unwrap();
    assert_eq!(out, UpdateOutcome::Updated { id });

    let after = repo::find(&pool, id).await.unwrap().unwrap();
    assert_eq!((after.price_cents, after.status), (1250, ProductStatus::Active));
    assert!(after.updated_at > before.updated_at);
    assert_eq!(after.created_at, before.created_at);

    // Ungültige Änderung speichert nichts
    let out = repo::update(&pool, id, &input("", "A-1", "1", "1", "draft")).await.unwrap();
    assert!(matches!(out, UpdateOutcome::Invalid(_)));
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().price_cents, 1250);

    // Unbekannte ID legt nichts an
    let out = repo::update(&pool, 999_999, &input("B", "B-1", "1", "1", "draft")).await.unwrap();
    assert_eq!(out, UpdateOutcome::NotFound);
    assert_eq!(count(&pool).await, 1);
}

#[sqlx::test]
async fn ak8_loeschen(pool: PgPool) {
    let id = create(&pool, &input("A", "A-1", "1", "1", "draft")).await;
    assert_eq!(repo::delete(&pool, id).await.unwrap(), DeleteOutcome::Deleted);
    assert_eq!(repo::find(&pool, id).await.unwrap(), None);
    assert_eq!(repo::delete(&pool, id).await.unwrap(), DeleteOutcome::NotFound);
}

#[sqlx::test]
async fn ak9_shop_zeigt_nur_aktive(pool: PgPool) {
    active(&pool, "Eins", "1", "1").await;
    active(&pool, "Zwei", "1", "1").await;
    create(&pool, &input("Entwurf", "E-1", "1", "1", "draft")).await;

    let shop = repo::shop(&pool, 1, ShopSort::Newest).await.unwrap();
    let names: Vec<_> = shop.items.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(names, ["Zwei", "Eins"]);
    assert_eq!(shop.total, 2);
}

#[sqlx::test]
async fn ak10_preisanzeige(pool: PgPool) {
    active(&pool, "Teuer", "1249", "1").await;
    let item = &repo::shop(&pool, 1, ShopSort::Newest).await.unwrap().items[0];
    assert_eq!(item.price_cents, 124_900);
    assert_eq!(format_chf(item.price_cents), "CHF 1'249.00");
}

#[sqlx::test]
async fn ak11_verfuegbarkeit(pool: PgPool) {
    use app_saintwhoo::products::model::Availability;
    active(&pool, "Leer", "1", "0").await;
    active(&pool, "Wenig", "1", "3").await;
    active(&pool, "Viel", "1", "20").await;
    let shop = repo::shop(&pool, 1, ShopSort::NameAsc).await.unwrap();
    let labels: Vec<_> = shop.items.iter().map(|p| (p.name.as_str(), Availability::from_stock(p.stock).label())).collect();
    assert_eq!(
        labels,
        [
            ("Leer", "Ausverkauft".to_string()),
            ("Viel", "An Lager".to_string()),
            ("Wenig", "Nur noch 3 verfügbar".to_string()),
        ]
    );
}

#[sqlx::test]
async fn ak12_sortierung(pool: PgPool) {
    active(&pool, "Banane", "30", "1").await;
    active(&pool, "apfel", "10", "1").await;
    active(&pool, "Citrus", "20", "1").await;

    let order = |sort| {
        let pool = pool.clone();
        async move {
            repo::shop(&pool, 1, sort).await.unwrap().items.into_iter().map(|p| p.price_cents / 100).collect::<Vec<_>>()
        }
    };
    assert_eq!(order(ShopSort::PriceAsc).await, [10, 20, 30]);
    assert_eq!(order(ShopSort::PriceDesc).await, [30, 20, 10]);
    assert_eq!(order(ShopSort::Newest).await, [20, 10, 30]);
    // Name A–Z unabhängig von Groß-/Kleinschreibung
    assert_eq!(order(ShopSort::NameAsc).await, [10, 30, 20]);
}

#[sqlx::test]
async fn ak13_seiten(pool: PgPool) {
    for i in 0..30 {
        create(&pool, &input(&format!("P{i}"), &format!("P-{i}"), "1", "1", "active")).await;
    }
    let p1 = repo::shop(&pool, 1, ShopSort::Newest).await.unwrap();
    let p2 = repo::shop(&pool, 2, ShopSort::Newest).await.unwrap();
    assert_eq!((p1.items.len(), p2.items.len(), p1.total_pages), (24, 6, 2));
    // zu große Seitenzahl → letzte Seite
    assert_eq!(repo::shop(&pool, 99, ShopSort::Newest).await.unwrap().page, 2);
}

#[sqlx::test]
async fn ak14_leerer_shop(pool: PgPool) {
    create(&pool, &input("Entwurf", "E-1", "1", "1", "draft")).await;
    let shop = repo::shop(&pool, 1, ShopSort::Newest).await.unwrap();
    assert!(shop.items.is_empty());
    assert_eq!((shop.total, shop.page, shop.total_pages), (0, 1, 1));
}

#[sqlx::test]
async fn ak15_produktseite(pool: PgPool) {
    let id = active(&pool, "Tasse", "12", "4").await;
    let p = repo::find_active(&pool, id).await.unwrap().unwrap();
    assert_eq!(p.name, "Tasse");
    assert_eq!(
        app_saintwhoo::articles::model::paragraphs(&p.description),
        ["Erster Absatz.", "Zweiter Absatz."]
    );
}

#[sqlx::test]
async fn ak16_entwurf_oder_unbekannt_ist_404(pool: PgPool) {
    let draft = create(&pool, &input("Entwurf", "E-1", "1", "1", "draft")).await;
    assert_eq!(repo::find_active(&pool, draft).await.unwrap(), None);
    assert_eq!(repo::find_active(&pool, 999_999).await.unwrap(), None);
}

#[sqlx::test]
async fn ak18_datenbank_lehnt_ungueltiges_ab(pool: PgPool) {
    let bad: [(&str, &str, i32, i32, &str); 7] = [
        ("Neg. Preis", "A-1", -1, 0, "draft"),
        ("Zu teuer", "A-2", 10_000_000, 0, "draft"),
        ("Neg. Bestand", "A-3", 0, -1, "draft"),
        ("Kleinbuchstaben", "a-4", 0, 0, "draft"),
        ("Leerzeichen", "A 5", 0, 0, "draft"),
        (" Nicht getrimmt", "A-6", 0, 0, "draft"),
        ("Status", "A-7", 0, 0, "archived"),
    ];
    for (name, sku, price, stock, status) in bad {
        let res = sqlx::query(
            "INSERT INTO products (name, sku, price_cents, stock, status) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(name)
        .bind(sku)
        .bind(price)
        .bind(stock)
        .bind(status)
        .execute(&pool)
        .await;
        assert!(res.is_err(), "DB hätte ablehnen müssen: {name}");
    }

    // doppelte Artikelnummer direkt in der DB
    let insert = "INSERT INTO products (name, sku, price_cents) VALUES ('X', 'DUP-1', 100)";
    sqlx::query(insert).execute(&pool).await.unwrap();
    assert!(sqlx::query(insert).execute(&pool).await.is_err());
    assert_eq!(count(&pool).await, 1);
}

// ---------- Teil C: Produktbild ----------

fn with_image(url: &str, credit: &str) -> ProductInput {
    ProductInput { image_url: url.into(), image_credit: credit.into(), ..input("Bild", "B-1", "1", "1", "active") }
}

#[sqlx::test]
async fn ak19_bild_wird_gespeichert_und_im_shop_geliefert(pool: PgPool) {
    let id = create(&pool, &with_image("/images/produkte/a.jpg", "Foto: Test (CC0)")).await;
    let expected = Some(ProductImage { url: "/images/produkte/a.jpg".into(), credit: "Foto: Test (CC0)".into() });

    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().image, expected);
    assert_eq!(repo::find_active(&pool, id).await.unwrap().unwrap().image, expected);
    assert_eq!(repo::shop(&pool, 1, ShopSort::Newest).await.unwrap().items[0].image, expected);

    // Bild beim Bearbeiten entfernen
    let out = repo::update(&pool, id, &with_image("", "")).await.unwrap();
    assert_eq!(out, UpdateOutcome::Updated { id });
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().image, None);
}

#[sqlx::test]
async fn ak20_ungueltiges_bild_speichert_nichts(pool: PgPool) {
    let e = invalid(repo::create(&pool, &with_image("/a.jpg", "")).await.unwrap());
    assert_eq!(e.image_credit.as_deref(), Some(msg::IMAGE_CREDIT_MISSING));
    let e = invalid(repo::create(&pool, &with_image("bild.jpg", "Foto")).await.unwrap());
    assert_eq!(e.image_url.as_deref(), Some(msg::IMAGE_URL_INVALID));
    assert_eq!(count(&pool).await, 0);
}

#[sqlx::test]
async fn ak21_datenbank_lehnt_ungueltige_bilddaten_ab(pool: PgPool) {
    let bad: [(Option<&str>, Option<&str>); 4] = [
        (Some("/a.jpg"), None),
        (None, Some("Foto")),
        (Some("ftp://x/a.jpg"), Some("F")),
        (Some("/mit leer.jpg"), Some("F")),
    ];
    for (i, (url, credit)) in bad.into_iter().enumerate() {
        let res = sqlx::query(
            "INSERT INTO products (name, sku, price_cents, image_url, image_credit) VALUES ('X', $1, 100, $2, $3)",
        )
        .bind(format!("IMG-{i}"))
        .bind(url)
        .bind(credit)
        .execute(&pool)
        .await;
        assert!(res.is_err(), "DB hätte ablehnen müssen: {url:?} / {credit:?}");
    }
    assert_eq!(count(&pool).await, 0);
}
