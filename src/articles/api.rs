//! Server-Funktionen für Artikel.
//!
//! Validierungsfehler und „nicht gefunden“ kommen als normales Ergebnis
//! (`…Outcome`) zurück, technische Fehler (z. B. DB nicht erreichbar)
//! als `ServerFnError`.

use leptos::prelude::*;

use super::model::{
    Article, ArticleCategory, ArticlePage, ArticleStatus, ArticleTeaser, CreateOutcome, DeleteOutcome,
    NewsPage, UpdateOutcome,
};

#[cfg(feature = "ssr")]
fn db_error(e: sqlx::Error) -> ServerFnError {
    tracing::error!(error = %e, "Datenbankfehler");
    ServerFnError::new("Die Daten konnten nicht verarbeitet werden. Bitte später erneut versuchen.")
}

/// ANF-001: Neuen Magazin-Artikel speichern.
/// Die Argumentnamen entsprechen den `name`-Attributen im Formular.
#[server]
pub async fn create_article(
    title: String,
    body: String,
    published_on: String,
    status: String,
    // ANF-004: optional, damit ältere Clients ohne Bildfelder weiter funktionieren
    image_url: Option<String>,
    image_credit: Option<String>,
    // ANF-008: optional, damit Anfragen ohne Rubrik weiter funktionieren
    category: Option<String>,
) -> Result<CreateOutcome, ServerFnError> {
    use super::model::ArticleInput;
    let pool = crate::db::pool()?;
    let input = ArticleInput {
        title,
        body,
        published_on,
        status,
        image_url: image_url.unwrap_or_default(),
        image_credit: image_credit.unwrap_or_default(),
        category: category.unwrap_or_default(),
    };
    super::repo::create(&pool, &input).await.map_err(db_error)
}

/// ANF-002: Liste (eine Seite, optional nach Status gefiltert).
#[server]
pub async fn list_articles(
    page: u32,
    status: Option<ArticleStatus>,
) -> Result<ArticlePage, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::list(&pool, page, status).await.map_err(db_error)
}

/// ANF-002: Einzelner Artikel (`None` = gibt es nicht).
#[server]
pub async fn get_article(id: i64) -> Result<Option<Article>, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::find(&pool, id).await.map_err(db_error)
}

/// ANF-002: Artikel ändern. Bei Erfolg Weiterleitung zur Detailseite.
#[allow(clippy::too_many_arguments)]
#[server]
pub async fn update_article(
    id: i64,
    title: String,
    body: String,
    published_on: String,
    status: String,
    image_url: Option<String>,
    image_credit: Option<String>,
    // ANF-008: optional, damit Anfragen ohne Rubrik weiter funktionieren
    category: Option<String>,
) -> Result<UpdateOutcome, ServerFnError> {
    use super::model::ArticleInput;
    let pool = crate::db::pool()?;
    let input = ArticleInput {
        title,
        body,
        published_on,
        status,
        image_url: image_url.unwrap_or_default(),
        image_credit: image_credit.unwrap_or_default(),
        category: category.unwrap_or_default(),
    };
    let outcome = super::repo::update(&pool, id, &input).await.map_err(db_error)?;
    if let UpdateOutcome::Updated { id } = outcome {
        leptos_axum::redirect(&format!("/artikel/{id}?meldung=gespeichert"));
    }
    Ok(outcome)
}

/// ANF-002: Artikel endgültig löschen. Bei Erfolg Weiterleitung zur Liste.
#[server]
pub async fn delete_article(id: i64) -> Result<DeleteOutcome, ServerFnError> {
    let pool = crate::db::pool()?;
    let outcome = super::repo::delete(&pool, id).await.map_err(db_error)?;
    if outcome == DeleteOutcome::Deleted {
        leptos_axum::redirect("/artikel?meldung=geloescht");
    }
    Ok(outcome)
}

/// ANF-003: Öffentliche Artikel für die Startseite (neueste zuerst).
#[server]
pub async fn front_page_articles() -> Result<Vec<ArticleTeaser>, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::front_page(&pool, super::model::FRONT_TOTAL).await.map_err(db_error)
}

/// ANF-003: Öffentlicher Artikel für die Leseansicht (`None` = 404).
#[server]
pub async fn get_published_article(id: i64) -> Result<Option<Article>, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::find_published(&pool, id).await.map_err(db_error)
}

/// ANF-008: News-Seite mit Suche und Rubrik-Filter.
#[server]
pub async fn news_articles(
    query: String,
    category: Option<ArticleCategory>,
    page: u32,
) -> Result<NewsPage, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::news(&pool, &query, category, page).await.map_err(db_error)
}

/// Startseite (Designvorschlag): die neuesten `limit` News.
#[server]
pub async fn latest_news(limit: i64) -> Result<Vec<ArticleTeaser>, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::front_page(&pool, limit.clamp(1, 20)).await.map_err(db_error)
}
