//! Server-Funktionen für das Radio (ANF-007).

use leptos::prelude::*;

use super::model::{DeleteOutcome, LiveInfo, Mix, MixOutcome, RadioOverview, ShowOutcome};

#[cfg(feature = "ssr")]
fn db_error(e: sqlx::Error) -> ServerFnError {
    tracing::error!(error = %e, "Datenbankfehler");
    ServerFnError::new("Die Daten konnten nicht verarbeitet werden. Bitte später erneut versuchen.")
}

/// Adresse des Livestreams aus `RADIO_STREAM_URL` (leer = nicht eingerichtet).
#[cfg(feature = "ssr")]
fn stream_url() -> Option<String> {
    std::env::var("RADIO_STREAM_URL").ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

/// Live-Ansicht: Stream, aktuelle Uhrzeit (Ortszeit) und Sendeplan für heute.
#[server]
pub async fn radio_live() -> Result<LiveInfo, ServerFnError> {
    let pool = crate::db::pool()?;
    let now = crate::events::repo::local_now(&pool).await.map_err(db_error)?;
    let today = super::repo::shows_on(&pool, now.date()).await.map_err(db_error)?;
    Ok(LiveInfo { stream_url: stream_url(), now, today })
}

/// Archiv: aufgezeichnete Sets, neueste zuerst.
#[server]
pub async fn radio_archive() -> Result<Vec<Mix>, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::mixes(&pool).await.map_err(db_error)
}

/// Verwaltung: kommende Sendungen und Archiv.
#[server]
pub async fn radio_admin() -> Result<RadioOverview, ServerFnError> {
    let pool = crate::db::pool()?;
    let now = crate::events::repo::local_now(&pool).await.map_err(db_error)?;
    super::repo::admin(&pool, now).await.map_err(db_error)
}

#[server]
pub async fn create_show(
    title: String,
    host: String,
    date: String,
    start: String,
    end: String,
) -> Result<ShowOutcome, ServerFnError> {
    use super::model::ShowInput;
    let pool = crate::db::pool()?;
    let input = ShowInput { title, host, date, start, end };
    let outcome = super::repo::create_show(&pool, &input).await.map_err(db_error)?;
    if let ShowOutcome::Created { .. } = outcome {
        leptos_axum::redirect("/radio/verwaltung?meldung=sendung-gespeichert");
    }
    Ok(outcome)
}

#[server]
pub async fn delete_show(id: i64) -> Result<DeleteOutcome, ServerFnError> {
    let pool = crate::db::pool()?;
    let outcome = super::repo::delete_show(&pool, id).await.map_err(db_error)?;
    if outcome == DeleteOutcome::Deleted {
        leptos_axum::redirect("/radio/verwaltung?meldung=sendung-geloescht");
    }
    Ok(outcome)
}

#[server]
pub async fn create_mix(
    title: String,
    host: String,
    duration_minutes: String,
    audio_url: String,
) -> Result<MixOutcome, ServerFnError> {
    use super::model::MixInput;
    let pool = crate::db::pool()?;
    let input = MixInput { title, host, duration_minutes, audio_url };
    let outcome = super::repo::create_mix(&pool, &input).await.map_err(db_error)?;
    if let MixOutcome::Created { .. } = outcome {
        leptos_axum::redirect("/radio/verwaltung?meldung=mix-gespeichert");
    }
    Ok(outcome)
}

#[server]
pub async fn delete_mix(id: i64) -> Result<DeleteOutcome, ServerFnError> {
    let pool = crate::db::pool()?;
    let outcome = super::repo::delete_mix(&pool, id).await.map_err(db_error)?;
    if outcome == DeleteOutcome::Deleted {
        leptos_axum::redirect("/radio/verwaltung?meldung=mix-geloescht");
    }
    Ok(outcome)
}
