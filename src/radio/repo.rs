//! Datenbankzugriff für das Radio (nur Server).

use chrono::{NaiveDate, NaiveDateTime};
use sqlx::PgPool;

use super::model::{
    ARCHIVE_LIMIT, DeleteOutcome, Mix, MixInput, MixOutcome, RadioOverview, Show, ShowInput, ShowOutcome,
    validate_mix, validate_show,
};
use crate::datetime::{add_days, start_of};

#[derive(sqlx::FromRow)]
struct ShowRow {
    id: i64,
    title: String,
    host: String,
    starts_at: NaiveDateTime,
    ends_at: NaiveDateTime,
}

impl From<ShowRow> for Show {
    fn from(r: ShowRow) -> Self {
        Show { id: r.id, title: r.title, host: r.host, starts_at: r.starts_at, ends_at: r.ends_at }
    }
}

#[derive(sqlx::FromRow)]
struct MixRow {
    id: i64,
    title: String,
    host: String,
    duration_minutes: i32,
    audio_url: String,
}

impl From<MixRow> for Mix {
    fn from(r: MixRow) -> Self {
        Mix { id: r.id, title: r.title, host: r.host, duration_minutes: r.duration_minutes, audio_url: r.audio_url }
    }
}

// ---------- Sendungen ----------

pub async fn create_show(pool: &PgPool, input: &ShowInput) -> Result<ShowOutcome, sqlx::Error> {
    let s = match validate_show(input) {
        Ok(s) => s,
        Err(errors) => return Ok(ShowOutcome::Invalid(errors)),
    };
    let id = sqlx::query_scalar(
        "INSERT INTO radio_shows (title, host, starts_at, ends_at) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(&s.title)
    .bind(&s.host)
    .bind(s.starts_at)
    .bind(s.ends_at)
    .fetch_one(pool)
    .await?;
    Ok(ShowOutcome::Created { id })
}

/// Sendungen, die am Tag `day` laufen (auch solche über Mitternacht),
/// nach Beginn sortiert.
pub async fn shows_on(pool: &PgPool, day: NaiveDate) -> Result<Vec<Show>, sqlx::Error> {
    let rows: Vec<ShowRow> = sqlx::query_as(
        "SELECT id, title, host, starts_at, ends_at
         FROM radio_shows
         WHERE starts_at < $2 AND ends_at > $1
         ORDER BY starts_at, id",
    )
    .bind(start_of(day))
    .bind(start_of(add_days(day, 1)))
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

/// Sendungen, die noch nicht vorbei sind (Verwaltung).
pub async fn upcoming_shows(pool: &PgPool, now: NaiveDateTime) -> Result<Vec<Show>, sqlx::Error> {
    let rows: Vec<ShowRow> = sqlx::query_as(
        "SELECT id, title, host, starts_at, ends_at FROM radio_shows WHERE ends_at > $1 ORDER BY starts_at, id",
    )
    .bind(now)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn delete_show(pool: &PgPool, id: i64) -> Result<DeleteOutcome, sqlx::Error> {
    let res = sqlx::query("DELETE FROM radio_shows WHERE id = $1").bind(id).execute(pool).await?;
    Ok(if res.rows_affected() == 0 { DeleteOutcome::NotFound } else { DeleteOutcome::Deleted })
}

// ---------- Mixe ----------

pub async fn create_mix(pool: &PgPool, input: &MixInput) -> Result<MixOutcome, sqlx::Error> {
    let m = match validate_mix(input) {
        Ok(m) => m,
        Err(errors) => return Ok(MixOutcome::Invalid(errors)),
    };
    let id = sqlx::query_scalar(
        "INSERT INTO radio_mixes (title, host, duration_minutes, audio_url) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(&m.title)
    .bind(&m.host)
    .bind(m.duration_minutes)
    .bind(&m.audio_url)
    .fetch_one(pool)
    .await?;
    Ok(MixOutcome::Created { id })
}

/// Archiv: neueste zuerst, höchstens `ARCHIVE_LIMIT`.
pub async fn mixes(pool: &PgPool) -> Result<Vec<Mix>, sqlx::Error> {
    let rows: Vec<MixRow> = sqlx::query_as(
        "SELECT id, title, host, duration_minutes, audio_url
         FROM radio_mixes ORDER BY created_at DESC, id DESC LIMIT $1",
    )
    .bind(ARCHIVE_LIMIT)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn delete_mix(pool: &PgPool, id: i64) -> Result<DeleteOutcome, sqlx::Error> {
    let res = sqlx::query("DELETE FROM radio_mixes WHERE id = $1").bind(id).execute(pool).await?;
    Ok(if res.rows_affected() == 0 { DeleteOutcome::NotFound } else { DeleteOutcome::Deleted })
}

/// Alles für die Verwaltungsseite.
pub async fn admin(pool: &PgPool, now: NaiveDateTime) -> Result<RadioOverview, sqlx::Error> {
    Ok(RadioOverview { shows: upcoming_shows(pool, now).await?, mixes: mixes(pool).await? })
}
