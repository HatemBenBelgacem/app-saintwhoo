//! Datenbankzugriff (nur Server / `ssr`).

use leptos::prelude::*;
use sqlx::{PgPool, postgres::PgPoolOptions};

/// Baut den Connection-Pool aus `DATABASE_URL`.
pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL ist nicht gesetzt");
    PgPoolOptions::new().max_connections(10).connect(&url).await
}

/// Führt alle Migrationen aus `./migrations` aus.
pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

/// Holt den Pool in einer Server-Funktion aus dem Leptos-Kontext.
pub fn pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("PgPool fehlt im Kontext"))
}
