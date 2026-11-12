//! Datenbankzugriff für Artikel (nur Server).

use chrono::{DateTime, NaiveDate, Utc};
use sqlx::PgPool;

use super::model::{
    Article, ArticleImage, ArticleInput, ArticlePage, ArticleStatus, ArticleSummary, ArticleTeaser,
    CreateOutcome, DeleteOutcome, PAGE_SIZE, TEASER_MAX_CHARS, UpdateOutcome, ValidArticle,
    reading_minutes, teaser, total_pages, validate,
};

#[derive(sqlx::FromRow)]
struct ArticleRow {
    id: i64,
    title: String,
    body: String,
    published_on: Option<NaiveDate>,
    status: String,
    image_url: Option<String>,
    image_credit: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<ArticleRow> for Article {
    fn from(r: ArticleRow) -> Self {
        Article {
            id: r.id,
            title: r.title,
            body: r.body,
            published_on: r.published_on,
            // Die DB erlaubt per CHECK nur gültige Werte
            status: ArticleStatus::parse(&r.status).unwrap_or_default(),
            image: ArticleImage::from_columns(r.image_url, r.image_credit),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SummaryRow {
    id: i64,
    title: String,
    published_on: Option<NaiveDate>,
    status: String,
    updated_at: DateTime<Utc>,
}

impl From<SummaryRow> for ArticleSummary {
    fn from(r: SummaryRow) -> Self {
        ArticleSummary {
            id: r.id,
            title: r.title,
            published_on: r.published_on,
            status: ArticleStatus::parse(&r.status).unwrap_or_default(),
            updated_at: r.updated_at,
        }
    }
}

// ---------- Create (ANF-001) ----------

/// Prüft die Eingaben und speichert den Artikel, wenn alles gültig ist.
/// Die Prüfung findet also immer auf dem Server statt (ANF-001, AK-9).
pub async fn create(pool: &PgPool, input: &ArticleInput) -> Result<CreateOutcome, sqlx::Error> {
    match validate(input) {
        Ok(article) => {
            let id = insert(pool, &article).await?;
            Ok(CreateOutcome::Created { id })
        }
        Err(errors) => Ok(CreateOutcome::Invalid(errors)),
    }
}

pub async fn insert(pool: &PgPool, a: &ValidArticle) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(
        "INSERT INTO articles (title, body, published_on, status, image_url, image_credit)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id",
    )
    .bind(&a.title)
    .bind(&a.body)
    .bind(a.published_on)
    .bind(a.status.as_str())
    .bind(a.image.as_ref().map(|i| &i.url))
    .bind(a.image.as_ref().map(|i| &i.credit))
    .fetch_one(pool)
    .await
}

// ---------- Read (ANF-002) ----------

pub async fn find(pool: &PgPool, id: i64) -> Result<Option<Article>, sqlx::Error> {
    let row: Option<ArticleRow> = sqlx::query_as(
        "SELECT id, title, body, published_on, status, image_url, image_credit, created_at, updated_at
         FROM articles WHERE id = $1",
    )
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(Into::into))
}

/// Eine Seite der Liste, neueste zuerst. Eine zu große Seitenzahl wird
/// auf die letzte Seite begrenzt.
pub async fn list(
    pool: &PgPool,
    page: u32,
    status: Option<ArticleStatus>,
) -> Result<ArticlePage, sqlx::Error> {
    let status = status.map(ArticleStatus::as_str);

    let total: i64 =
        sqlx::query_scalar("SELECT count(*) FROM articles WHERE ($1::text IS NULL OR status = $1)")
            .bind(status)
            .fetch_one(pool)
            .await?;

    let total_pages = total_pages(total);
    let page = page.clamp(1, total_pages);
    let offset = i64::from(page - 1) * PAGE_SIZE;

    let rows: Vec<SummaryRow> = sqlx::query_as(
        "SELECT id, title, published_on, status, updated_at
         FROM articles
         WHERE ($1::text IS NULL OR status = $1)
         ORDER BY created_at DESC, id DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(status)
    .bind(PAGE_SIZE)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(ArticlePage {
        items: rows.into_iter().map(Into::into).collect(),
        page,
        total_pages,
        total,
    })
}

// ---------- Update (ANF-002) ----------

/// Prüft und speichert Änderungen. Gibt `NotFound` zurück, wenn es den
/// Artikel nicht (mehr) gibt – dann wird nichts angelegt (AK-12).
pub async fn update(
    pool: &PgPool,
    id: i64,
    input: &ArticleInput,
) -> Result<UpdateOutcome, sqlx::Error> {
    let a = match validate(input) {
        Ok(a) => a,
        Err(errors) => return Ok(UpdateOutcome::Invalid(errors)),
    };
    let updated: Option<i64> = sqlx::query_scalar(
        "UPDATE articles
         SET title = $1, body = $2, published_on = $3, status = $4,
             image_url = $5, image_credit = $6, updated_at = now()
         WHERE id = $7
         RETURNING id",
    )
    .bind(&a.title)
    .bind(&a.body)
    .bind(a.published_on)
    .bind(a.status.as_str())
    .bind(a.image.as_ref().map(|i| &i.url))
    .bind(a.image.as_ref().map(|i| &i.credit))
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(match updated {
        Some(id) => UpdateOutcome::Updated { id },
        None => UpdateOutcome::NotFound,
    })
}

// ---------- Delete (ANF-002) ----------

/// Löscht endgültig (Hard Delete).
pub async fn delete(pool: &PgPool, id: i64) -> Result<DeleteOutcome, sqlx::Error> {
    let res = sqlx::query("DELETE FROM articles WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(if res.rows_affected() == 0 {
        DeleteOutcome::NotFound
    } else {
        DeleteOutcome::Deleted
    })
}

// ---------- Öffentlich (ANF-003) ----------

/// Bedingung „öffentlich“: veröffentlicht und Datum heute oder früher.
/// (Als Kommentar, da sqlx nur feste SQL-Strings erlaubt – in beiden
/// Abfragen unten identisch verwenden.)
/// `status = 'published' AND published_on <= CURRENT_DATE`
pub async fn front_page(pool: &PgPool, limit: i64) -> Result<Vec<ArticleTeaser>, sqlx::Error> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        title: String,
        body: String,
        published_on: NaiveDate,
        image_url: Option<String>,
        image_credit: Option<String>,
    }
    let rows: Vec<Row> = sqlx::query_as(
        "SELECT id, title, body, published_on, image_url, image_credit
         FROM articles
         WHERE status = 'published' AND published_on <= CURRENT_DATE
         ORDER BY published_on DESC, created_at DESC, id DESC
         LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|r| ArticleTeaser {
            id: r.id,
            teaser: teaser(&r.body, TEASER_MAX_CHARS),
            reading_minutes: reading_minutes(&r.body),
            title: r.title,
            published_on: r.published_on,
            image: ArticleImage::from_columns(r.image_url, r.image_credit),
        })
        .collect())
}

/// Ein öffentlicher Artikel – `None` für Entwürfe, geplante oder unbekannte.
pub async fn find_published(pool: &PgPool, id: i64) -> Result<Option<Article>, sqlx::Error> {
    let row: Option<ArticleRow> = sqlx::query_as(
        "SELECT id, title, body, published_on, status, image_url, image_credit, created_at, updated_at
         FROM articles
         WHERE id = $1 AND status = 'published' AND published_on <= CURRENT_DATE",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Into::into))
}
