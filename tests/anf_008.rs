//! ANF-008 – News mit Suche und Rubrik, Integrationstests gegen PostgreSQL.
//!
//! Ausführen: `cargo test --features ssr`
#![cfg(feature = "ssr")]

use app_saintwhoo::articles::model::{ArticleCategory, ArticleInput, CreateOutcome, UpdateOutcome};
use app_saintwhoo::articles::repo;
use sqlx::PgPool;

fn input(title: &str, body: &str, category: &str) -> ArticleInput {
    ArticleInput {
        title: title.into(),
        body: body.into(),
        published_on: "2026-11-01".into(),
        status: "published".into(),
        category: category.into(),
        ..Default::default()
    }
}

async fn create(pool: &PgPool, i: &ArticleInput) -> i64 {
    match repo::create(pool, i).await.unwrap() {
        CreateOutcome::Created { id } => id,
        o => panic!("erwartet Created, bekommen {o:?}"),
    }
}

async fn titles(pool: &PgPool, q: &str, c: Option<ArticleCategory>) -> Vec<String> {
    let mut t: Vec<_> = repo::news(pool, q, c, 1).await.unwrap().items.into_iter().map(|a| a.title).collect();
    t.sort();
    t
}

#[sqlx::test]
async fn ak1_rubrik_wird_gespeichert_und_geaendert(pool: PgPool) {
    let id = create(&pool, &input("Mara Voss im Gespräch", "Text", "interview")).await;
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().category, ArticleCategory::Interview);

    let out = repo::update(&pool, id, &input("Mara Voss im Gespräch", "Text", "mix")).await.unwrap();
    assert_eq!(out, UpdateOutcome::Updated { id });
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().category, ArticleCategory::Mix);
}

#[sqlx::test]
async fn ak2_ohne_rubrik_ist_meldung(pool: PgPool) {
    let id = create(&pool, &input("Alt", "Text", "")).await;
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().category, ArticleCategory::News);
}

#[sqlx::test]
async fn ak3_filter_nach_rubrik(pool: PgPool) {
    create(&pool, &input("Interview A", "Text", "interview")).await;
    create(&pool, &input("Release B", "Text", "release")).await;
    create(&pool, &input("Interview C", "Text", "interview")).await;

    assert_eq!(titles(&pool, "", Some(ArticleCategory::Interview)).await, ["Interview A", "Interview C"]);
    assert_eq!(titles(&pool, "", None).await.len(), 3);
    assert!(titles(&pool, "", Some(ArticleCategory::Mix)).await.is_empty());
}

#[sqlx::test]
async fn ak4_suche_in_titel_und_text_ohne_gross_klein(pool: PgPool) {
    create(&pool, &input("Acid Lab Vol. 3", "Party in Leipzig", "event")).await;
    create(&pool, &input("Neues Label", "Drei Platten mit ACID-Sound", "release")).await;
    create(&pool, &input("Techno-Geschichte", "Detroit und Berlin", "news")).await;

    assert_eq!(titles(&pool, "acid", None).await, ["Acid Lab Vol. 3", "Neues Label"]);
    assert_eq!(titles(&pool, "  ACID ", Some(ArticleCategory::Release)).await, ["Neues Label"]);
    assert!(titles(&pool, "Ibiza", None).await.is_empty());
}

#[sqlx::test]
async fn ak5_sonderzeichen_sind_normaler_text(pool: PgPool) {
    create(&pool, &input("100% Vinyl", "Text", "news")).await;
    create(&pool, &input("Anderes", "Text", "news")).await;
    assert_eq!(titles(&pool, "%", None).await, ["100% Vinyl"]);
    assert!(titles(&pool, "_", None).await.is_empty());
}

#[sqlx::test]
async fn ak6_nur_veroeffentlichte_artikel(pool: PgPool) {
    create(&pool, &input("Öffentlich", "Text", "news")).await;
    create(&pool, &ArticleInput { status: "draft".into(), published_on: "".into(), ..input("Entwurf", "Text", "news") }).await;
    create(&pool, &ArticleInput { published_on: "2099-01-01".into(), ..input("Geplant", "Text", "news") }).await;
    assert_eq!(titles(&pool, "", None).await, ["Öffentlich"]);
}

#[sqlx::test]
async fn ak7_datenbank_lehnt_unbekannte_rubrik_ab(pool: PgPool) {
    let res = sqlx::query("INSERT INTO articles (title, body, category) VALUES ('X', 'Y', 'klatsch')").execute(&pool).await;
    assert!(res.is_err());
}
