//! Axum-Router der App (nur ssr). Liegt in der Bibliothek, damit die
//! Integrationstests (`tests/anf_011_pwa.rs`) denselben Router prüfen wie `main.rs`.

use axum::Router;
use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};

use crate::app::{App, shell};
use crate::state::AppState;

/// Baut den kompletten Router: Healthcheck, Leptos-Seiten, statische Dateien.
pub fn router(state: AppState) -> Router {
    let routes = generate_route_list(App);
    Router::new()
        // Healthcheck für Docker (ANF-010): nur `SELECT 1`, kein HTML
        .route("/health", axum::routing::get(health))
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let state = state.clone();
                move || provide_context(state.pool.clone())
            },
            {
                let options = state.leptos_options.clone();
                move || shell(options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(middleware::from_fn(pwa_headers))
        .with_state(state)
}

/// ANF-011: `sw.js` nie im HTTP-Cache halten, damit Updates sofort erkannt werden.
/// Der Content-Type des Manifests (`application/manifest+json`) kommt von `ServeDir`.
async fn pwa_headers(req: Request, next: Next) -> Response {
    let is_sw = req.uri().path() == "/sw.js";
    let mut res = next.run(req).await;
    if is_sw {
        res.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    }
    res
}

/// `GET /health`: 200 „ok“, wenn die Datenbank antwortet, sonst 503.
async fn health(axum::extract::State(pool): axum::extract::State<sqlx::PgPool>) -> (StatusCode, &'static str) {
    match sqlx::query("SELECT 1").execute(&pool).await {
        Ok(_) => (StatusCode::OK, "ok"),
        Err(e) => {
            tracing::warn!("Healthcheck: Datenbank nicht erreichbar: {e}");
            (StatusCode::SERVICE_UNAVAILABLE, "Datenbank nicht erreichbar")
        }
    }
}
