//! ANF-004 – Artikelbild mit Bildnachweis, Integrationstests gegen PostgreSQL.
#![cfg(feature = "ssr")]

use app_saintwhoo::articles::model::{
    ArticleImage, ArticleInput, CreateOutcome, FRONT_TOTAL, UpdateOutcome, msg,
};
use app_saintwhoo::articles::repo;
use sqlx::PgPool;

fn input(url: &str, credit: &str) -> ArticleInput {
    ArticleInput {
        title: "Mit Bild".into(),
        body: "Text".into(),
        published_on: "2026-01-01".into(),
        status: "published".into(),
        image_url: url.into(),
        image_credit: credit.into(),
    }
}

async fn create(pool: &PgPool, i: &ArticleInput) -> i64 {
    match repo::create(pool, i).await.unwrap() {
        CreateOutcome::Created { id } => id,
        o => panic!("erwartet Created, bekommen {o:?}"),
    }
}

async fn count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM articles").fetch_one(pool).await.unwrap()
}

#[sqlx::test]
async fn ak1_bild_wird_gespeichert_und_ueberall_geliefert(pool: PgPool) {
    let id = create(&pool, &input("/images/artikel/a.jpg", "Foto: Test (CC BY 2.0)")).await;
    let expected = Some(ArticleImage { url: "/images/artikel/a.jpg".into(), credit: "Foto: Test (CC BY 2.0)".into() });

    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().image, expected);
    assert_eq!(repo::find_published(&pool, id).await.unwrap().unwrap().image, expected);
    assert_eq!(repo::front_page(&pool, FRONT_TOTAL).await.unwrap()[0].image, expected);
}

#[sqlx::test]
async fn ak2_ak3_ungueltig_speichert_nichts(pool: PgPool) {
    match repo::create(&pool, &input("/a.jpg", "")).await.unwrap() {
        CreateOutcome::Invalid(e) => assert_eq!(e.image_credit.as_deref(), Some(msg::IMAGE_CREDIT_MISSING)),
        o => panic!("{o:?}"),
    }
    match repo::create(&pool, &input("bild.jpg", "Foto")).await.unwrap() {
        CreateOutcome::Invalid(e) => assert_eq!(e.image_url.as_deref(), Some(msg::IMAGE_URL_INVALID)),
        o => panic!("{o:?}"),
    }
    assert_eq!(count(&pool).await, 0);
}

#[sqlx::test]
async fn ak5_nachweis_ohne_bild_wird_nicht_gespeichert(pool: PgPool) {
    let id = create(&pool, &input("", "Foto: niemand")).await;
    let row: (Option<String>, Option<String>) =
        sqlx::query_as("SELECT image_url, image_credit FROM articles WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(row, (None, None));
}

#[sqlx::test]
async fn ak6_bild_entfernen_beim_bearbeiten(pool: PgPool) {
    let id = create(&pool, &input("/a.jpg", "Foto")).await;
    let out = repo::update(&pool, id, &input("", "")).await.unwrap();
    assert_eq!(out, UpdateOutcome::Updated { id });
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap().image, None);
}

#[sqlx::test]
async fn ak8_datenbank_lehnt_ungueltige_bilddaten_ab(pool: PgPool) {
    let bad: [(Option<&str>, Option<&str>); 5] = [
        (Some("/a.jpg"), None),            // Bild ohne Nachweis
        (Some("/a.jpg"), Some("   ")),     // leerer Nachweis
        (None, Some("Foto")),              // Nachweis ohne Bild
        (Some("ftp://x/a.jpg"), Some("F")), // falsches Schema
        (Some("/mit leer.jpg"), Some("F")), // Leerzeichen
    ];
    for (url, credit) in bad {
        let res = sqlx::query(
            "INSERT INTO articles (title, body, status, image_url, image_credit)
             VALUES ('T', 'B', 'draft', $1, $2)",
        )
        .bind(url)
        .bind(credit)
        .execute(&pool)
        .await;
        assert!(res.is_err(), "DB hätte ablehnen müssen: {url:?} / {credit:?}");
    }
    assert_eq!(count(&pool).await, 0);
}
