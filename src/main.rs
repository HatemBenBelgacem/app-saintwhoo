#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use app_saintwhoo::app::{App, shell};
    use app_saintwhoo::state::AppState;
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};

    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Datenbank verbinden und Migrationen ausführen
    let pool = app_saintwhoo::db::connect()
        .await
        .expect("Verbindung zu PostgreSQL fehlgeschlagen (DATABASE_URL prüfen)");
    app_saintwhoo::db::migrate(&pool)
        .await
        .expect("Migrationen fehlgeschlagen");

    let conf = get_configuration(None).expect("Leptos-Konfiguration fehlt");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let state = AppState {
        leptos_options,
        pool,
    };

    let app = Router::new()
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
        .with_state(state);

    tracing::info!("Server läuft auf http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Kein Client-seitiger main – Hydration startet in lib.rs (hydrate()).
}
