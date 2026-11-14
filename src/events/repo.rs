//! Datenbankzugriff für Events (nur Server).

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use sqlx::PgPool;

use super::model::{
    CreateOutcome, DeleteOutcome, Event, EventCard, EventImage, EventInput, EventListing, EventPage,
    EventRange, EventSummary, PAGE_SIZE, TEASER_MAX_CHARS, TicketStatus, UpdateOutcome, total_pages,
    validate,
};
use crate::articles::model::teaser;

const SELECT_EVENT: &str = "SELECT id, title, description, starts_at, venue, city, lineup, ticket_url,
        ticket_status, image_url, image_credit, created_at, updated_at
     FROM events WHERE id = $1";

#[derive(sqlx::FromRow)]
struct EventRow {
    id: i64,
    title: String,
    description: String,
    starts_at: NaiveDateTime,
    venue: String,
    city: String,
    lineup: String,
    ticket_url: Option<String>,
    ticket_status: String,
    image_url: Option<String>,
    image_credit: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<EventRow> for Event {
    fn from(r: EventRow) -> Self {
        Event {
            id: r.id,
            title: r.title,
            description: r.description,
            starts_at: r.starts_at,
            venue: r.venue,
            city: r.city,
            lineup: r.lineup,
            ticket_url: r.ticket_url,
            // Die DB erlaubt per CHECK nur gültige Werte
            ticket_status: TicketStatus::parse(&r.ticket_status).unwrap_or_default(),
            image: EventImage::from_columns(r.image_url, r.image_credit),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

/// Heutiges Datum in Schweizer Ortszeit (laut Datenbank-Uhr).
pub async fn local_today(pool: &PgPool) -> Result<NaiveDate, sqlx::Error> {
    sqlx::query_scalar("SELECT (now() AT TIME ZONE 'Europe/Zurich')::date").fetch_one(pool).await
}

/// Aktuelle Ortszeit (Europe/Zurich) laut Datenbank-Uhr.
pub async fn local_now(pool: &PgPool) -> Result<NaiveDateTime, sqlx::Error> {
    sqlx::query_scalar("SELECT (now() AT TIME ZONE 'Europe/Zurich')::timestamp").fetch_one(pool).await
}

// ---------- Create ----------

pub async fn create(pool: &PgPool, input: &EventInput) -> Result<CreateOutcome, sqlx::Error> {
    let e = match validate(input) {
        Ok(e) => e,
        Err(errors) => return Ok(CreateOutcome::Invalid(errors)),
    };
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO events
             (title, description, starts_at, venue, city, lineup, ticket_url, ticket_status, image_url, image_credit)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING id",
    )
    .bind(&e.title)
    .bind(&e.description)
    .bind(e.starts_at)
    .bind(&e.venue)
    .bind(&e.city)
    .bind(&e.lineup)
    .bind(&e.ticket_url)
    .bind(e.ticket_status.as_str())
    .bind(e.image.as_ref().map(|i| &i.url))
    .bind(e.image.as_ref().map(|i| &i.credit))
    .fetch_one(pool)
    .await?;
    Ok(CreateOutcome::Created { id })
}

// ---------- Read ----------

pub async fn find(pool: &PgPool, id: i64) -> Result<Option<Event>, sqlx::Error> {
    let row: Option<EventRow> = sqlx::query_as(SELECT_EVENT).bind(id).fetch_optional(pool).await?;
    Ok(row.map(Into::into))
}

/// Eine Seite der Verwaltungsliste, zuletzt geändert zuerst.
pub async fn list(pool: &PgPool, page: u32) -> Result<EventPage, sqlx::Error> {
    let total: i64 = sqlx::query_scalar("SELECT count(*) FROM events").fetch_one(pool).await?;
    let total_pages = total_pages(total);
    let page = page.clamp(1, total_pages);
    let offset = i64::from(page - 1) * PAGE_SIZE;

    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        title: String,
        starts_at: NaiveDateTime,
        city: String,
        image_url: Option<String>,
        image_credit: Option<String>,
        updated_at: DateTime<Utc>,
    }
    let rows: Vec<Row> = sqlx::query_as(
        "SELECT id, title, starts_at, city, image_url, image_credit, updated_at
         FROM events
         ORDER BY updated_at DESC, id DESC
         LIMIT $1 OFFSET $2",
    )
    .bind(PAGE_SIZE)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(EventPage {
        items: rows
            .into_iter()
            .map(|r| EventSummary {
                id: r.id,
                title: r.title,
                starts_at: r.starts_at,
                city: r.city,
                image: EventImage::from_columns(r.image_url, r.image_credit),
                updated_at: r.updated_at,
            })
            .collect(),
        page,
        total_pages,
        total,
    })
}

/// Öffentliche Eventliste: Events ab `today` im gewählten Zeitraum,
/// nach Beginn sortiert (nächstes zuerst).
pub async fn upcoming(pool: &PgPool, range: EventRange, today: NaiveDate) -> Result<EventListing, sqlx::Error> {
    let (from, to) = range.bounds(today);
    let rows: Vec<EventRow> = sqlx::query_as(
        "SELECT id, title, description, starts_at, venue, city, lineup, ticket_url,
                ticket_status, image_url, image_credit, created_at, updated_at
         FROM events
         WHERE starts_at >= $1 AND ($2::timestamp IS NULL OR starts_at < $2)
         ORDER BY starts_at ASC, id ASC",
    )
    .bind(from)
    .bind(to)
    .fetch_all(pool)
    .await?;

    Ok(EventListing {
        today,
        items: rows
            .into_iter()
            .map(|r| {
                let e: Event = r.into();
                EventCard {
                    id: e.id,
                    teaser: teaser(&e.description, TEASER_MAX_CHARS),
                    title: e.title,
                    starts_at: e.starts_at,
                    venue: e.venue,
                    city: e.city,
                    lineup: e.lineup,
                    ticket_url: e.ticket_url,
                    ticket_status: e.ticket_status,
                    image: e.image,
                }
            })
            .collect(),
    })
}

// ---------- Update ----------

pub async fn update(pool: &PgPool, id: i64, input: &EventInput) -> Result<UpdateOutcome, sqlx::Error> {
    let e = match validate(input) {
        Ok(e) => e,
        Err(errors) => return Ok(UpdateOutcome::Invalid(errors)),
    };
    let updated: Option<i64> = sqlx::query_scalar(
        "UPDATE events
         SET title = $1, description = $2, starts_at = $3, venue = $4, city = $5, lineup = $6,
             ticket_url = $7, ticket_status = $8, image_url = $9, image_credit = $10, updated_at = now()
         WHERE id = $11
         RETURNING id",
    )
    .bind(&e.title)
    .bind(&e.description)
    .bind(e.starts_at)
    .bind(&e.venue)
    .bind(&e.city)
    .bind(&e.lineup)
    .bind(&e.ticket_url)
    .bind(e.ticket_status.as_str())
    .bind(e.image.as_ref().map(|i| &i.url))
    .bind(e.image.as_ref().map(|i| &i.credit))
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(match updated {
        Some(id) => UpdateOutcome::Updated { id },
        None => UpdateOutcome::NotFound,
    })
}

// ---------- Delete ----------

/// Löscht endgültig (Hard Delete).
pub async fn delete(pool: &PgPool, id: i64) -> Result<DeleteOutcome, sqlx::Error> {
    let res = sqlx::query("DELETE FROM events WHERE id = $1").bind(id).execute(pool).await?;
    Ok(if res.rows_affected() == 0 { DeleteOutcome::NotFound } else { DeleteOutcome::Deleted })
}
