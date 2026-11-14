//! Server-Funktionen für Events (ANF-006).
//!
//! Validierungsfehler und „nicht gefunden“ kommen als normales Ergebnis
//! (`…Outcome`) zurück, technische Fehler als `ServerFnError`.

use leptos::prelude::*;

use super::model::{CreateOutcome, DeleteOutcome, Event, EventListing, EventPage, EventRange, UpdateOutcome};

#[cfg(feature = "ssr")]
fn db_error(e: sqlx::Error) -> ServerFnError {
    tracing::error!(error = %e, "Datenbankfehler");
    ServerFnError::new("Die Daten konnten nicht verarbeitet werden. Bitte später erneut versuchen.")
}

/// Neues Event speichern. Bei Erfolg Weiterleitung zur Detailseite.
/// Die Argumentnamen entsprechen den `name`-Attributen im Formular.
#[allow(clippy::too_many_arguments)]
#[server]
pub async fn create_event(
    title: String,
    description: String,
    date: String,
    time: String,
    venue: String,
    city: String,
    lineup: Option<String>,
    ticket_url: Option<String>,
    ticket_status: Option<String>,
    image_url: Option<String>,
    image_credit: Option<String>,
) -> Result<CreateOutcome, ServerFnError> {
    use super::model::EventInput;
    let pool = crate::db::pool()?;
    let input = EventInput {
        title,
        description,
        date,
        time,
        venue,
        city,
        lineup: lineup.unwrap_or_default(),
        ticket_url: ticket_url.unwrap_or_default(),
        ticket_status: ticket_status.unwrap_or_default(),
        image_url: image_url.unwrap_or_default(),
        image_credit: image_credit.unwrap_or_default(),
    };
    let outcome = super::repo::create(&pool, &input).await.map_err(db_error)?;
    if let CreateOutcome::Created { id } = outcome {
        leptos_axum::redirect(&format!("/events/{id}?meldung=gespeichert"));
    }
    Ok(outcome)
}

/// Verwaltungsliste (eine Seite).
#[server]
pub async fn list_events(page: u32) -> Result<EventPage, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::list(&pool, page).await.map_err(db_error)
}

/// Einzelnes Event (`None` = gibt es nicht). Für Verwaltung und Eventseite.
#[server]
pub async fn get_event(id: i64) -> Result<Option<Event>, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::find(&pool, id).await.map_err(db_error)
}

/// Event ändern. Bei Erfolg Weiterleitung zur Detailseite.
#[allow(clippy::too_many_arguments)]
#[server]
pub async fn update_event(
    id: i64,
    title: String,
    description: String,
    date: String,
    time: String,
    venue: String,
    city: String,
    lineup: Option<String>,
    ticket_url: Option<String>,
    ticket_status: Option<String>,
    image_url: Option<String>,
    image_credit: Option<String>,
) -> Result<UpdateOutcome, ServerFnError> {
    use super::model::EventInput;
    let pool = crate::db::pool()?;
    let input = EventInput {
        title,
        description,
        date,
        time,
        venue,
        city,
        lineup: lineup.unwrap_or_default(),
        ticket_url: ticket_url.unwrap_or_default(),
        ticket_status: ticket_status.unwrap_or_default(),
        image_url: image_url.unwrap_or_default(),
        image_credit: image_credit.unwrap_or_default(),
    };
    let outcome = super::repo::update(&pool, id, &input).await.map_err(db_error)?;
    if let UpdateOutcome::Updated { id } = outcome {
        leptos_axum::redirect(&format!("/events/{id}?meldung=gespeichert"));
    }
    Ok(outcome)
}

/// Event endgültig löschen. Bei Erfolg Weiterleitung zur Liste.
#[server]
pub async fn delete_event(id: i64) -> Result<DeleteOutcome, ServerFnError> {
    let pool = crate::db::pool()?;
    let outcome = super::repo::delete(&pool, id).await.map_err(db_error)?;
    if outcome == DeleteOutcome::Deleted {
        leptos_axum::redirect("/events?meldung=geloescht");
    }
    Ok(outcome)
}

/// Öffentliche Eventliste: kommende Events im Zeitraum (Ortszeit Zürich).
#[server]
pub async fn upcoming_events(range: EventRange) -> Result<EventListing, ServerFnError> {
    let pool = crate::db::pool()?;
    let today = super::repo::local_today(&pool).await.map_err(db_error)?;
    super::repo::upcoming(&pool, range, today).await.map_err(db_error)
}
