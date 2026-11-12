//! ANF-001 – Integrationstests gegen PostgreSQL.
//!
//! Ausführen: `cargo test --features ssr`
//! `#[sqlx::test]` legt pro Test eine eigene Datenbank an (braucht `DATABASE_URL`
//! mit dem Recht, Datenbanken zu erstellen) und führt die Migrationen aus.
#![cfg(feature = "ssr")]

use app_saintwhoo::articles::model::{ArticleInput, ArticleStatus, CreateOutcome, msg};
use app_saintwhoo::articles::repo;
use chrono::NaiveDate;
use sqlx::PgPool;

fn input(title: &str, body: &str, date: &str, status: &str) -> ArticleInput {
    ArticleInput {
        title: title.into(),
        body: body.into(),
        published_on: date.into(),
        status: status.into(),
        ..Default::default()
    }
}

async fn count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM articles")
        .fetch_one(pool)
        .await
        .unwrap()
}

fn created_id(outcome: CreateOutcome) -> i64 {
    match outcome {
        CreateOutcome::Created { id } => id,
        other => panic!("erwartet Created, bekommen: {other:?}"),
    }
}

#[sqlx::test]
async fn ak1_entwurf_ohne_datum_wird_gespeichert(pool: PgPool) {
    let id = created_id(repo::create(&pool, &input("Titel", "Text", "", "draft")).await.unwrap());
    let row = repo::find(&pool, id).await.unwrap().unwrap();
    assert_eq!(row.status, ArticleStatus::Draft);
    assert_eq!(row.published_on, None);
}

#[sqlx::test]
async fn ak2_veroeffentlicht_mit_datum_wird_gespeichert(pool: PgPool) {
    let out = repo::create(&pool, &input("Titel", "Text", "2026-11-12", "published")).await;
    let row = repo::find(&pool, created_id(out.unwrap())).await.unwrap().unwrap();
    assert_eq!(row.status, ArticleStatus::Published);
    assert_eq!(row.published_on, NaiveDate::from_ymd_opt(2026, 11, 12));
}

#[sqlx::test]
async fn ak3_veroeffentlicht_ohne_datum_speichert_nichts(pool: PgPool) {
    let out = repo::create(&pool, &input("Titel", "Text", "", "published")).await.unwrap();
    match out {
        CreateOutcome::Invalid(e) => assert_eq!(e.published_on.as_deref(), Some(msg::DATE_MISSING)),
        other => panic!("erwartet Invalid, bekommen: {other:?}"),
    }
    assert_eq!(count(&pool).await, 0);
}

#[sqlx::test]
async fn ak4_ak6_leere_felder_speichern_nichts(pool: PgPool) {
    for i in [input("   ", "Text", "", "draft"), input("Titel", " \n ", "", "draft")] {
        let out = repo::create(&pool, &i).await.unwrap();
        assert!(matches!(out, CreateOutcome::Invalid(_)));
    }
    assert_eq!(count(&pool).await, 0);
}

#[sqlx::test]
async fn ak8_daten_bleiben_unveraendert(pool: PgPool) {
    let body = "Grüße aus Zürich!\n\nZweiter Absatz mit „Anführungszeichen“ und Emoji 🎉\n";
    let title = "Ärger über Öl & Übermut";
    let out = repo::create(&pool, &input(title, body, "2026-12-24", "published")).await;
    let row = repo::find(&pool, created_id(out.unwrap())).await.unwrap().unwrap();
    assert_eq!(row.title, title);
    assert_eq!(row.body, body);
    assert_eq!(row.published_on, NaiveDate::from_ymd_opt(2026, 12, 24));
    assert_eq!(row.status, ArticleStatus::Published);
}

/// AK-9: Auch ohne Formular (direkter Aufruf) werden die Regeln geprüft –
/// und die Datenbank lehnt ungültige Daten zusätzlich selbst ab.
#[sqlx::test]
async fn ak9_server_und_datenbank_lehnen_ungueltige_daten_ab(pool: PgPool) {
    let out = repo::create(&pool, &input(&"x".repeat(201), "Text", "", "draft")).await.unwrap();
    assert!(matches!(out, CreateOutcome::Invalid(ref e) if e.title.as_deref() == Some(msg::TITLE_TOO_LONG)));

    // Direktes SQL an der Validierung vorbei → CHECK-Constraints greifen
    let bad = [
        ("", "Text", None::<NaiveDate>, "draft"),
        ("Titel", "   ", None, "draft"),
        ("Titel", "Text", None, "published"),
        ("Titel", "Text", None, "archiviert"),
    ];
    for (t, b, d, s) in bad {
        let res = sqlx::query("INSERT INTO articles (title, body, published_on, status) VALUES ($1,$2,$3,$4)")
            .bind(t).bind(b).bind(d).bind(s)
            .execute(&pool)
            .await;
        assert!(res.is_err(), "DB hätte ablehnen müssen: {t:?} {b:?} {s:?}");
    }
    assert_eq!(count(&pool).await, 0);
}
