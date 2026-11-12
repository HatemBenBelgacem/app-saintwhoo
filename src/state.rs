use axum::extract::FromRef;
use leptos::prelude::LeptosOptions;
use sqlx::PgPool;

/// Gemeinsamer Zustand für den Axum-Server.
#[derive(Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
}
