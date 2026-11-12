//! Server-Funktionen für Produkte (ANF-005).
//!
//! Validierungsfehler und „nicht gefunden“ kommen als normales Ergebnis
//! (`…Outcome`) zurück, technische Fehler als `ServerFnError`.

use leptos::prelude::*;

use super::model::{
    CreateOutcome, DeleteOutcome, Product, ProductPage, ProductStatus, ShopPage, ShopSort,
    UpdateOutcome,
};

#[cfg(feature = "ssr")]
fn db_error(e: sqlx::Error) -> ServerFnError {
    tracing::error!(error = %e, "Datenbankfehler");
    ServerFnError::new("Die Daten konnten nicht verarbeitet werden. Bitte später erneut versuchen.")
}

/// Neues Produkt speichern. Bei Erfolg Weiterleitung zur Detailseite.
/// Die Argumentnamen entsprechen den `name`-Attributen im Formular.
#[allow(clippy::too_many_arguments)]
#[server]
pub async fn create_product(
    name: String,
    sku: String,
    short_description: String,
    description: String,
    price: String,
    stock: String,
    status: String,
    // Teil C: optional, damit Anfragen ohne Bildfelder weiter funktionieren
    image_url: Option<String>,
    image_credit: Option<String>,
) -> Result<CreateOutcome, ServerFnError> {
    use super::model::ProductInput;
    let pool = crate::db::pool()?;
    let input = ProductInput {
        name,
        sku,
        short_description,
        description,
        price,
        stock,
        status,
        image_url: image_url.unwrap_or_default(),
        image_credit: image_credit.unwrap_or_default(),
    };
    let outcome = super::repo::create(&pool, &input).await.map_err(db_error)?;
    if let CreateOutcome::Created { id } = outcome {
        leptos_axum::redirect(&format!("/produkte/{id}?meldung=gespeichert"));
    }
    Ok(outcome)
}

/// Verwaltungsliste (eine Seite, optional nach Status gefiltert).
#[server]
pub async fn list_products(
    page: u32,
    status: Option<ProductStatus>,
) -> Result<ProductPage, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::list(&pool, page, status).await.map_err(db_error)
}

/// Einzelnes Produkt für die Verwaltung (`None` = gibt es nicht).
#[server]
pub async fn get_product(id: i64) -> Result<Option<Product>, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::find(&pool, id).await.map_err(db_error)
}

/// Produkt ändern. Bei Erfolg Weiterleitung zur Detailseite.
#[allow(clippy::too_many_arguments)]
#[server]
pub async fn update_product(
    id: i64,
    name: String,
    sku: String,
    short_description: String,
    description: String,
    price: String,
    stock: String,
    status: String,
    // Teil C: optional, damit Anfragen ohne Bildfelder weiter funktionieren
    image_url: Option<String>,
    image_credit: Option<String>,
) -> Result<UpdateOutcome, ServerFnError> {
    use super::model::ProductInput;
    let pool = crate::db::pool()?;
    let input = ProductInput {
        name,
        sku,
        short_description,
        description,
        price,
        stock,
        status,
        image_url: image_url.unwrap_or_default(),
        image_credit: image_credit.unwrap_or_default(),
    };
    let outcome = super::repo::update(&pool, id, &input).await.map_err(db_error)?;
    if let UpdateOutcome::Updated { id } = outcome {
        leptos_axum::redirect(&format!("/produkte/{id}?meldung=gespeichert"));
    }
    Ok(outcome)
}

/// Produkt endgültig löschen. Bei Erfolg Weiterleitung zur Liste.
#[server]
pub async fn delete_product(id: i64) -> Result<DeleteOutcome, ServerFnError> {
    let pool = crate::db::pool()?;
    let outcome = super::repo::delete(&pool, id).await.map_err(db_error)?;
    if outcome == DeleteOutcome::Deleted {
        leptos_axum::redirect("/produkte?meldung=geloescht");
    }
    Ok(outcome)
}

/// Shop: aktive Produkte, eine Seite, sortiert.
#[server]
pub async fn shop_products(page: u32, sort: ShopSort) -> Result<ShopPage, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::shop(&pool, page, sort).await.map_err(db_error)
}

/// Shop: ein aktives Produkt (`None` = 404).
#[server]
pub async fn get_shop_product(id: i64) -> Result<Option<Product>, ServerFnError> {
    let pool = crate::db::pool()?;
    super::repo::find_active(&pool, id).await.map_err(db_error)
}
