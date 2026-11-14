//! ANF-007 – Radio, Integrationstests gegen PostgreSQL.
//!
//! Ausführen: `cargo test --features ssr`
//! Prüfregeln stehen als Unit-Tests in `src/radio/model.rs`.
#![cfg(feature = "ssr")]

use app_saintwhoo::radio::model::{DeleteOutcome, MixInput, MixOutcome, ShowInput, ShowOutcome, msg};
use app_saintwhoo::radio::repo;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

fn show(title: &str, date: &str, start: &str, end: &str) -> ShowInput {
    ShowInput { title: title.into(), host: "Mara Voss".into(), date: date.into(), start: start.into(), end: end.into() }
}

fn mix(title: &str) -> MixInput {
    MixInput { title: title.into(), host: "Lea Stern".into(), duration_minutes: "90".into(), audio_url: "/audio/a.mp3".into() }
}

fn day(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

fn at(s: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").unwrap()
}

async fn add_show(pool: &PgPool, i: &ShowInput) -> i64 {
    match repo::create_show(pool, i).await.unwrap() {
        ShowOutcome::Created { id } => id,
        o => panic!("erwartet Created, bekommen {o:?}"),
    }
}

#[sqlx::test]
async fn ak1_sendeplan_heute_mit_sendung_ueber_mitternacht(pool: PgPool) {
    add_show(&pool, &show("Gestern spät", "2026-11-13", "23:00", "01:00")).await;
    add_show(&pool, &show("Morgenlicht", "2026-11-14", "04:00", "06:00")).await;
    add_show(&pool, &show("Nachtschicht", "2026-11-14", "02:00", "04:00")).await;
    add_show(&pool, &show("Morgen", "2026-11-15", "02:00", "04:00")).await;
    add_show(&pool, &show("Vorgestern", "2026-11-12", "20:00", "22:00")).await;

    let titles: Vec<_> = repo::shows_on(&pool, day("2026-11-14")).await.unwrap().into_iter().map(|s| s.title).collect();
    assert_eq!(titles, ["Gestern spät", "Nachtschicht", "Morgenlicht"]);
}

#[sqlx::test]
async fn ak2_jetzt_auf_sendung(pool: PgPool) {
    add_show(&pool, &show("Nachtschicht", "2026-11-14", "02:00", "04:00")).await;
    let shows = repo::shows_on(&pool, day("2026-11-14")).await.unwrap();
    assert!(shows[0].is_on_air(at("2026-11-14 03:00")));
    assert!(!shows[0].is_on_air(at("2026-11-14 04:00")));
}

#[sqlx::test]
async fn ak3_ungueltige_sendung_nicht_gespeichert(pool: PgPool) {
    let out = repo::create_show(&pool, &show("", "2026-11-14", "02:00", "02:00")).await.unwrap();
    match out {
        ShowOutcome::Invalid(e) => {
            assert_eq!(e.title.as_deref(), Some(msg::TITLE_EMPTY));
            assert_eq!(e.end.as_deref(), Some(msg::SAME_TIME));
        }
        o => panic!("erwartet Invalid, bekommen {o:?}"),
    }
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM radio_shows").fetch_one(&pool).await.unwrap();
    assert_eq!(n, 0);
}

#[sqlx::test]
async fn ak4_verwaltung_zeigt_nur_kommende_sendungen(pool: PgPool) {
    add_show(&pool, &show("Vorbei", "2026-11-14", "00:00", "02:00")).await;
    add_show(&pool, &show("Läuft", "2026-11-14", "02:00", "04:00")).await;
    add_show(&pool, &show("Kommt", "2026-11-14", "04:00", "06:00")).await;
    let titles: Vec<_> =
        repo::upcoming_shows(&pool, at("2026-11-14 03:00")).await.unwrap().into_iter().map(|s| s.title).collect();
    assert_eq!(titles, ["Läuft", "Kommt"]);
}

#[sqlx::test]
async fn ak5_sendung_loeschen(pool: PgPool) {
    let id = add_show(&pool, &show("Weg", "2026-11-14", "02:00", "04:00")).await;
    assert_eq!(repo::delete_show(&pool, id).await.unwrap(), DeleteOutcome::Deleted);
    assert_eq!(repo::delete_show(&pool, id).await.unwrap(), DeleteOutcome::NotFound);
}

#[sqlx::test]
async fn ak6_archiv_neueste_zuerst_und_loeschen(pool: PgPool) {
    for t in ["Erster", "Zweiter", "Dritter"] {
        assert!(matches!(repo::create_mix(&pool, &mix(t)).await.unwrap(), MixOutcome::Created { .. }));
    }
    let list = repo::mixes(&pool).await.unwrap();
    let titles: Vec<_> = list.iter().map(|m| m.title.as_str()).collect();
    assert_eq!(titles, ["Dritter", "Zweiter", "Erster"]);
    assert_eq!(list[0].duration_minutes, 90);

    assert_eq!(repo::delete_mix(&pool, list[0].id).await.unwrap(), DeleteOutcome::Deleted);
    assert_eq!(repo::mixes(&pool).await.unwrap().len(), 2);
}

#[sqlx::test]
async fn ak7_ungueltiger_mix_nicht_gespeichert(pool: PgPool) {
    let out = repo::create_mix(&pool, &MixInput { audio_url: "mix.mp3".into(), ..mix("X") }).await.unwrap();
    assert!(matches!(out, MixOutcome::Invalid(e) if e.audio_url.as_deref() == Some(msg::AUDIO_URL_INVALID)));
    assert!(repo::mixes(&pool).await.unwrap().is_empty());
}

#[sqlx::test]
async fn ak8_datenbank_lehnt_ungueltige_werte_ab(pool: PgPool) {
    let bad_show = sqlx::query(
        "INSERT INTO radio_shows (title, host, starts_at, ends_at) VALUES ('X', 'Y', '2026-11-14 04:00', '2026-11-14 02:00')",
    )
    .execute(&pool)
    .await;
    assert!(bad_show.is_err(), "Ende vor Beginn");

    let too_long = sqlx::query(
        "INSERT INTO radio_shows (title, host, starts_at, ends_at) VALUES ('X', 'Y', '2026-11-14 00:00', '2026-11-14 13:00')",
    )
    .execute(&pool)
    .await;
    assert!(too_long.is_err(), "länger als 12 Stunden");

    for (minutes, url) in [(0, "/a.mp3"), (601, "/a.mp3"), (60, "ftp://x"), (60, "//evil/x")] {
        let res = sqlx::query("INSERT INTO radio_mixes (title, host, duration_minutes, audio_url) VALUES ('X', 'Y', $1, $2)")
            .bind(minutes)
            .bind(url)
            .execute(&pool)
            .await;
        assert!(res.is_err(), "{minutes} {url}");
    }
}
