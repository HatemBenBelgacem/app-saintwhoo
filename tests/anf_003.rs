//! ANF-003 – Magazin-Startseite, Integrationstests gegen PostgreSQL.
//!
//! Ausführen: `cargo test --features ssr`
#![cfg(feature = "ssr")]

use app_saintwhoo::articles::model::{FRONT_TOTAL, TEASER_MAX_CHARS};
use app_saintwhoo::articles::repo;
use sqlx::PgPool;

/// Legt einen Artikel an. `days` = Veröffentlichungsdatum relativ zu heute
/// (0 = heute, -1 = gestern, 1 = morgen); `None` bei Entwürfen ohne Datum.
async fn insert(pool: &PgPool, title: &str, body: &str, status: &str, days: Option<i32>) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO articles (title, body, status, published_on)
         VALUES ($1, $2, $3, CASE WHEN $4::int IS NULL THEN NULL ELSE CURRENT_DATE + $4::int END)
         RETURNING id",
    )
    .bind(title)
    .bind(body)
    .bind(status)
    .bind(days)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[sqlx::test]
async fn ak1_nur_oeffentliche_artikel(pool: PgPool) {
    insert(&pool, "Entwurf", "Text", "draft", None).await;
    insert(&pool, "Entwurf mit Datum", "Text", "draft", Some(-1)).await;
    insert(&pool, "Heute", "Text", "published", Some(0)).await;
    insert(&pool, "Morgen", "Text", "published", Some(1)).await;

    let list = repo::front_page(&pool, FRONT_TOTAL).await.unwrap();
    let titles: Vec<_> = list.iter().map(|a| a.title.as_str()).collect();
    assert_eq!(titles, ["Heute"]);
}

#[sqlx::test]
async fn ak2_neuestes_datum_ist_aufmacher(pool: PgPool) {
    insert(&pool, "Vor einer Woche", "Text", "published", Some(-7)).await;
    insert(&pool, "Gestern", "Text", "published", Some(-1)).await;
    insert(&pool, "Vorgestern (später erstellt)", "Text", "published", Some(-2)).await;

    let list = repo::front_page(&pool, FRONT_TOTAL).await.unwrap();
    let titles: Vec<_> = list.iter().map(|a| a.title.as_str()).collect();
    assert_eq!(titles, ["Gestern", "Vorgestern (später erstellt)", "Vor einer Woche"]);
}

#[sqlx::test]
async fn ak2_gleiches_datum_zuletzt_erstellt_zuerst(pool: PgPool) {
    insert(&pool, "Früher erstellt", "Text", "published", Some(0)).await;
    insert(&pool, "Später erstellt", "Text", "published", Some(0)).await;
    let list = repo::front_page(&pool, FRONT_TOTAL).await.unwrap();
    assert_eq!(list[0].title, "Später erstellt");
}

#[sqlx::test]
async fn ak3_hoechstens_13_artikel(pool: PgPool) {
    for i in 0..15 {
        insert(&pool, &format!("Artikel {i}"), "Text", "published", Some(-i)).await;
    }
    let list = repo::front_page(&pool, FRONT_TOTAL).await.unwrap();
    assert_eq!(list.len(), 13);
    assert_eq!(list[0].title, "Artikel 0");
    assert_eq!(list[12].title, "Artikel 12");
}

#[sqlx::test]
async fn ak4_ak5_anriss_und_lesezeit(pool: PgPool) {
    let body = "Wort ".repeat(450); // 2250 Zeichen, 450 Wörter
    insert(&pool, "Lang", &body, "published", Some(0)).await;
    let a = &repo::front_page(&pool, FRONT_TOTAL).await.unwrap()[0];
    assert!(a.teaser.chars().count() <= TEASER_MAX_CHARS);
    assert!(a.teaser.ends_with('…'));
    assert_eq!(a.reading_minutes, 3);
}

#[sqlx::test]
async fn ak6_keine_oeffentlichen_artikel(pool: PgPool) {
    insert(&pool, "Nur Entwurf", "Text", "draft", None).await;
    assert!(repo::front_page(&pool, FRONT_TOTAL).await.unwrap().is_empty());
}

#[sqlx::test]
async fn ak7_ak8_leseansicht_nur_fuer_oeffentliche(pool: PgPool) {
    let public = insert(&pool, "Öffentlich", "Absatz 1\n\nAbsatz 2", "published", Some(-1)).await;
    let draft = insert(&pool, "Entwurf", "Text", "draft", Some(-1)).await;
    let planned = insert(&pool, "Geplant", "Text", "published", Some(3)).await;

    let a = repo::find_published(&pool, public).await.unwrap().unwrap();
    assert_eq!(a.body, "Absatz 1\n\nAbsatz 2");
    assert_eq!(repo::find_published(&pool, draft).await.unwrap(), None);
    assert_eq!(repo::find_published(&pool, planned).await.unwrap(), None);
    assert_eq!(repo::find_published(&pool, 987_654).await.unwrap(), None);
}
