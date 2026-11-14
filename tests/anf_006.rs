//! ANF-006 – Eventliste, Integrationstests gegen PostgreSQL.
//!
//! Ausführen: `cargo test --features ssr`
//! Reine Prüfregeln und Zeiträume stehen als Unit-Tests in
//! `src/events/model.rs`. AK-16 (Layout) wird im Browser geprüft.
#![cfg(feature = "ssr")]

use app_saintwhoo::events::model::{
    CreateOutcome, DeleteOutcome, EventImage, EventInput, EventRange, FieldErrors, TicketStatus,
    UpdateOutcome, msg,
};
use app_saintwhoo::events::repo;
use chrono::NaiveDate;
use sqlx::PgPool;

fn input(title: &str, date: &str, time: &str) -> EventInput {
    EventInput {
        title: title.into(),
        description: "Erster Absatz.\n\nZweiter Absatz.".into(),
        date: date.into(),
        time: time.into(),
        venue: "Kellerraum 9".into(),
        city: "Zürich".into(),
        lineup: "Mara Voss, Jonas Keller".into(),
        ..Default::default()
    }
}

fn day(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

async fn create(pool: &PgPool, i: &EventInput) -> i64 {
    match repo::create(pool, i).await.unwrap() {
        CreateOutcome::Created { id } => id,
        o => panic!("erwartet Created, bekommen {o:?}"),
    }
}

fn invalid(o: CreateOutcome) -> FieldErrors {
    match o {
        CreateOutcome::Invalid(e) => e,
        o => panic!("erwartet Invalid, bekommen {o:?}"),
    }
}

async fn count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM events").fetch_one(pool).await.unwrap()
}

#[sqlx::test]
async fn ak1_event_mit_bild_in_verwaltung_und_eventliste(pool: PgPool) {
    let i = EventInput {
        image_url: "/images/events/a.jpg".into(),
        image_credit: "Foto: Test (CC0)".into(),
        ticket_url: "https://tickets.example/14".into(),
        ticket_status: "few".into(),
        ..input("Nachtschicht #14", "2026-11-14", "23:00")
    };
    let id = create(&pool, &i).await;
    let image = Some(EventImage { url: "/images/events/a.jpg".into(), credit: "Foto: Test (CC0)".into() });

    let e = repo::find(&pool, id).await.unwrap().unwrap();
    assert_eq!(e.image, image);
    assert_eq!(e.starts_at.to_string(), "2026-11-14 23:00:00");
    assert_eq!((e.venue.as_str(), e.city.as_str()), ("Kellerraum 9", "Zürich"));
    assert_eq!(e.ticket_status, TicketStatus::Few);
    assert_eq!(e.ticket_url.as_deref(), Some("https://tickets.example/14"));

    assert_eq!(repo::list(&pool, 1).await.unwrap().items[0].image, image);
    let listing = repo::upcoming(&pool, EventRange::All, day("2026-11-14")).await.unwrap();
    assert_eq!(listing.items[0].image, image);
}

#[sqlx::test]
async fn ak2_ohne_bild(pool: PgPool) {
    let id = create(&pool, &input("Ohne Bild", "2026-11-14", "23:00")).await;
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().image, None);
}

#[sqlx::test]
async fn ak3_ak5_pflichtfelder_nichts_gespeichert(pool: PgPool) {
    let e = invalid(repo::create(&pool, &EventInput { title: "  ".into(), description: "".into(), ..input("x", "2026-11-14", "23:00") }).await.unwrap());
    assert_eq!(e.title.as_deref(), Some(msg::TITLE_EMPTY));
    assert_eq!(e.description.as_deref(), Some(msg::DESCRIPTION_EMPTY));
    assert_eq!(count(&pool).await, 0);
}

#[sqlx::test]
async fn ak9_bearbeiten_aktualisiert_updated_at(pool: PgPool) {
    let id = create(&pool, &input("Alt", "2026-11-14", "23:00")).await;
    let before = repo::find(&pool, id).await.unwrap().unwrap();

    let out = repo::update(&pool, id, &input("Neu", "2026-11-20", "22:00")).await.unwrap();
    assert_eq!(out, UpdateOutcome::Updated { id });

    let after = repo::find(&pool, id).await.unwrap().unwrap();
    assert_eq!(after.title, "Neu");
    assert_eq!(after.starts_at.to_string(), "2026-11-20 22:00:00");
    assert_eq!(after.created_at, before.created_at);
    assert!(after.updated_at > before.updated_at);
}

#[sqlx::test]
async fn ak10_bild_entfernen(pool: PgPool) {
    let with = EventInput { image_url: "/images/events/a.jpg".into(), image_credit: "Foto".into(), ..input("Bild", "2026-11-14", "23:00") };
    let id = create(&pool, &with).await;
    repo::update(&pool, id, &input("Bild", "2026-11-14", "23:00")).await.unwrap();
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().image, None);
}

#[sqlx::test]
async fn ak11_loeschen(pool: PgPool) {
    let id = create(&pool, &input("Weg", "2026-11-14", "23:00")).await;
    assert_eq!(repo::delete(&pool, id).await.unwrap(), DeleteOutcome::Deleted);
    assert_eq!(repo::delete(&pool, id).await.unwrap(), DeleteOutcome::NotFound);
    assert_eq!(repo::find(&pool, id).await.unwrap(), None);
}

#[sqlx::test]
async fn ak12_eventliste_naechstes_zuerst_vergangene_ausgeblendet(pool: PgPool) {
    create(&pool, &input("Gestern", "2026-11-13", "23:00")).await;
    create(&pool, &input("Später", "2026-12-05", "23:30")).await;
    create(&pool, &input("Heute früh", "2026-11-14", "02:00")).await;
    create(&pool, &input("Nächste Woche", "2026-11-20", "22:00")).await;

    let titles: Vec<_> = repo::upcoming(&pool, EventRange::All, day("2026-11-14"))
        .await
        .unwrap()
        .items
        .into_iter()
        .map(|e| e.title)
        .collect();
    assert_eq!(titles, ["Heute früh", "Nächste Woche", "Später"]);
}

#[sqlx::test]
async fn ak13_keine_events(pool: PgPool) {
    let listing = repo::upcoming(&pool, EventRange::All, day("2026-11-14")).await.unwrap();
    assert!(listing.items.is_empty());
    assert_eq!(listing.today, day("2026-11-14"));
}

#[sqlx::test]
async fn ak15_unbekannte_id(pool: PgPool) {
    assert_eq!(repo::find(&pool, 999_999).await.unwrap(), None);
}

#[sqlx::test]
async fn ak17_datenbank_lehnt_ungueltige_werte_ab(pool: PgPool) {
    let insert = "INSERT INTO events (title, description, starts_at, venue, city, ticket_url, ticket_status, image_url, image_credit)
                  VALUES ($1, $2, '2026-11-14 23:00', $3, 'Zürich', $4, $5, $6, $7)";
    // Titel, Beschreibung, Ort, Ticket-Adresse, Ticketstatus, Bild, Bildnachweis
    type Case<'a> = (&'a str, &'a str, &'a str, Option<&'a str>, &'a str, Option<&'a str>, Option<&'a str>);
    let cases: [Case; 6] = [
        ("", "Text", "Club", None, "available", None, None),
        ("Titel", "  ", "Club", None, "available", None, None),
        ("Titel", "Text", "", None, "available", None, None),
        ("Titel", "Text", "Club", Some("http://x"), "available", None, None),
        ("Titel", "Text", "Club", None, "gratis", None, None),
        ("Titel", "Text", "Club", None, "available", Some("/a.jpg"), None),
    ];
    for (title, desc, venue, ticket, status, img, credit) in cases {
        let res = sqlx::query(insert)
            .bind(title)
            .bind(desc)
            .bind(venue)
            .bind(ticket)
            .bind(status)
            .bind(img)
            .bind(credit)
            .execute(&pool)
            .await;
        assert!(res.is_err(), "sollte abgelehnt werden: {title:?} {desc:?} {venue:?} {ticket:?} {status:?} {img:?}");
    }
}

#[sqlx::test]
async fn ak20_zeitraum_filter(pool: PgPool) {
    // Mittwoch, 11.11.2026
    create(&pool, &input("Mittwoch", "2026-11-11", "21:00")).await;
    create(&pool, &input("Freitag", "2026-11-13", "23:00")).await;
    create(&pool, &input("Sonntag", "2026-11-15", "06:00")).await;
    create(&pool, &input("Montag", "2026-11-16", "00:00")).await;

    let titles = |range| {
        let pool = pool.clone();
        async move {
            repo::upcoming(&pool, range, day("2026-11-11"))
                .await
                .unwrap()
                .items
                .into_iter()
                .map(|e| e.title)
                .collect::<Vec<_>>()
        }
    };
    assert_eq!(titles(EventRange::Today).await, ["Mittwoch"]);
    assert_eq!(titles(EventRange::Week).await, ["Mittwoch", "Freitag", "Sonntag"]);
    assert_eq!(titles(EventRange::Weekend).await, ["Freitag", "Sonntag"]);
    assert_eq!(titles(EventRange::All).await.len(), 4);
}
