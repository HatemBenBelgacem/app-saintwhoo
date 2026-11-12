//! ANF-002 – CRUD für Artikel, Integrationstests gegen PostgreSQL.
//!
//! Ausführen: `cargo test --features ssr`
//! UI-Kriterien (AK-7 Vorausfüllen, AK-10 Abbrechen, AK-14 Link) werden
//! zusätzlich manuell / per HTTP geprüft, siehe docs/features/ANF-002.
#![cfg(feature = "ssr")]

use app_saintwhoo::articles::model::{
    ArticleInput, ArticleStatus, CreateOutcome, DeleteOutcome, UpdateOutcome, msg,
};
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

async fn create(pool: &PgPool, title: &str, status: &str) -> i64 {
    let date = if status == "published" { "2026-11-12" } else { "" };
    match repo::create(pool, &input(title, "Text", date, status)).await.unwrap() {
        CreateOutcome::Created { id } => id,
        other => panic!("erwartet Created, bekommen: {other:?}"),
    }
}

async fn count(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM articles").fetch_one(pool).await.unwrap()
}

// ---------- Read ----------

#[sqlx::test]
async fn ak1_liste_neueste_zuerst(pool: PgPool) {
    for t in ["Erster", "Zweiter", "Dritter"] {
        create(&pool, t, "draft").await;
    }
    let page = repo::list(&pool, 1, None).await.unwrap();
    let titles: Vec<_> = page.items.iter().map(|a| a.title.as_str()).collect();
    assert_eq!(titles, ["Dritter", "Zweiter", "Erster"]);
    assert_eq!(page.total, 3);
}

#[sqlx::test]
async fn ak2_leere_liste(pool: PgPool) {
    let page = repo::list(&pool, 1, None).await.unwrap();
    assert!(page.items.is_empty());
    assert_eq!((page.total, page.page, page.total_pages), (0, 1, 1));
}

#[sqlx::test]
async fn ak3_seiten_mit_je_20_eintraegen(pool: PgPool) {
    for i in 1..=25 {
        create(&pool, &format!("Artikel {i}"), "draft").await;
    }
    let p1 = repo::list(&pool, 1, None).await.unwrap();
    let p2 = repo::list(&pool, 2, None).await.unwrap();
    assert_eq!((p1.items.len(), p2.items.len()), (20, 5));
    assert_eq!(p1.total_pages, 2);
    assert_eq!(p1.items[0].title, "Artikel 25");
    assert_eq!(p2.items[4].title, "Artikel 1");

    // Zu große Seitenzahl → letzte Seite
    let p9 = repo::list(&pool, 9, None).await.unwrap();
    assert_eq!((p9.page, p9.items.len()), (2, 5));
}

#[sqlx::test]
async fn ak4_filter_nach_status(pool: PgPool) {
    create(&pool, "Entwurf A", "draft").await;
    create(&pool, "Online", "published").await;
    create(&pool, "Entwurf B", "draft").await;

    let drafts = repo::list(&pool, 1, Some(ArticleStatus::Draft)).await.unwrap();
    assert_eq!(drafts.total, 2);
    assert!(drafts.items.iter().all(|a| a.status == ArticleStatus::Draft));

    let published = repo::list(&pool, 1, Some(ArticleStatus::Published)).await.unwrap();
    assert_eq!(published.total, 1);
    assert_eq!(published.items[0].title, "Online");
}

#[sqlx::test]
async fn ak5_detail_mit_allen_feldern(pool: PgPool) {
    let body = "Absatz 1\n\nAbsatz 2 – mit Umlauten: äöü";
    let id = match repo::create(&pool, &input("Detail", body, "2026-12-01", "published")).await.unwrap() {
        CreateOutcome::Created { id } => id,
        o => panic!("{o:?}"),
    };
    let a = repo::find(&pool, id).await.unwrap().unwrap();
    assert_eq!(a.title, "Detail");
    assert_eq!(a.body, body);
    assert_eq!(a.published_on, NaiveDate::from_ymd_opt(2026, 12, 1));
    assert_eq!(a.status, ArticleStatus::Published);
    assert_eq!(a.created_at, a.updated_at, "neu erstellt: beide Zeitstempel gleich");
}

#[sqlx::test]
async fn ak6_unbekannte_id(pool: PgPool) {
    assert_eq!(repo::find(&pool, 999_999).await.unwrap(), None);
}

// ---------- Update ----------

#[sqlx::test]
async fn ak8_bearbeiten_speichert_und_setzt_updated_at(pool: PgPool) {
    let id = create(&pool, "Alt", "draft").await;
    let before = repo::find(&pool, id).await.unwrap().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));

    let out = repo::update(&pool, id, &input("  Neu  ", "Neuer Text", "2026-11-20", "published")).await.unwrap();
    assert_eq!(out, UpdateOutcome::Updated { id });

    let after = repo::find(&pool, id).await.unwrap().unwrap();
    assert_eq!(after.title, "Neu");
    assert_eq!(after.body, "Neuer Text");
    assert_eq!(after.status, ArticleStatus::Published);
    assert_eq!(after.published_on, NaiveDate::from_ymd_opt(2026, 11, 20));
    assert!(after.updated_at > before.updated_at, "updated_at muss neuer sein");
    assert_eq!(after.created_at, before.created_at, "created_at bleibt gleich");
}

#[sqlx::test]
async fn ak9_ungueltige_aenderung_aendert_nichts(pool: PgPool) {
    let id = create(&pool, "Original", "draft").await;
    let before = repo::find(&pool, id).await.unwrap().unwrap();

    let out = repo::update(&pool, id, &input("   ", "Text", "", "published")).await.unwrap();
    match out {
        UpdateOutcome::Invalid(e) => {
            assert_eq!(e.title.as_deref(), Some(msg::TITLE_EMPTY));
            assert_eq!(e.published_on.as_deref(), Some(msg::DATE_MISSING));
        }
        o => panic!("erwartet Invalid, bekommen: {o:?}"),
    }
    assert_eq!(repo::find(&pool, id).await.unwrap().unwrap(), before);
}

#[sqlx::test]
async fn veroeffentlicht_zurueck_auf_entwurf(pool: PgPool) {
    let id = create(&pool, "Online", "published").await;
    let out = repo::update(&pool, id, &input("Online", "Text", "", "draft")).await.unwrap();
    assert_eq!(out, UpdateOutcome::Updated { id });
    let a = repo::find(&pool, id).await.unwrap().unwrap();
    assert_eq!((a.status, a.published_on), (ArticleStatus::Draft, None));
}

// ---------- Delete ----------

#[sqlx::test]
async fn ak11_loeschen_entfernt_endgueltig(pool: PgPool) {
    let keep = create(&pool, "Bleibt", "draft").await;
    let id = create(&pool, "Weg", "draft").await;
    assert_eq!(repo::delete(&pool, id).await.unwrap(), DeleteOutcome::Deleted);
    assert_eq!(repo::find(&pool, id).await.unwrap(), None);
    assert!(repo::find(&pool, keep).await.unwrap().is_some());
    assert_eq!(count(&pool).await, 1);
}

/// AK-12: in einem anderen Tab gelöscht → Speichern/Löschen meldet
/// „existiert nicht mehr“ und legt nichts neu an.
#[sqlx::test]
async fn ak12_bearbeiten_oder_loeschen_nach_loeschen(pool: PgPool) {
    let id = create(&pool, "Weg", "draft").await;
    repo::delete(&pool, id).await.unwrap();

    let out = repo::update(&pool, id, &input("Wieder da?", "Text", "", "draft")).await.unwrap();
    assert_eq!(out, UpdateOutcome::NotFound);
    assert_eq!(repo::delete(&pool, id).await.unwrap(), DeleteOutcome::NotFound);
    assert_eq!(count(&pool).await, 0);
}

/// AK-13: Direkte Aufrufe mit unbekannter ID werden abgelehnt.
#[sqlx::test]
async fn ak13_unbekannte_id_bei_update_und_delete(pool: PgPool) {
    let out = repo::update(&pool, 424_242, &input("T", "B", "", "draft")).await.unwrap();
    assert_eq!(out, UpdateOutcome::NotFound);
    assert_eq!(repo::delete(&pool, 424_242).await.unwrap(), DeleteOutcome::NotFound);
    assert_eq!(count(&pool).await, 0);
}

/// R-8: `updated_at` wird per Migration ergänzt und ist bei neuen Artikeln gesetzt.
#[sqlx::test]
async fn updated_at_spalte_ist_pflicht(pool: PgPool) {
    let nullable: String = sqlx::query_scalar(
        "SELECT is_nullable FROM information_schema.columns
         WHERE table_name = 'articles' AND column_name = 'updated_at'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(nullable, "NO");
}
