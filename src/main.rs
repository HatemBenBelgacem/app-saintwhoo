#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use app_saintwhoo::state::AppState;
    use leptos::prelude::*;

    // `app-saintwhoo healthcheck`: für den Docker-HEALTHCHECK (kein curl im Image)
    if std::env::args().nth(1).as_deref() == Some("healthcheck") {
        std::process::exit(healthcheck());
    }

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
    let addr = env_port().map_or(leptos_options.site_addr, |port| {
        std::net::SocketAddr::from(([0, 0, 0, 0], port))
    });

    let state = AppState {
        leptos_options,
        pool,
    };

    let app = app_saintwhoo::server::router(state);

    tracing::info!("Server läuft auf http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    tracing::info!("Server beendet");
}

/// Fragt `GET /health` beim eigenen Server ab. Rückgabe 0 = gesund, 1 = nicht gesund.
/// Port aus `PORT`, sonst `LEPTOS_SITE_ADDR` (Standard 3000), immer über 127.0.0.1.
#[cfg(feature = "ssr")]
fn healthcheck() -> i32 {
    use std::io::{Read, Write};
    use std::net::{SocketAddr, TcpStream};
    use std::time::Duration;

    let port = env_port().unwrap_or_else(|| {
        std::env::var("LEPTOS_SITE_ADDR")
            .ok()
            .and_then(|a| a.parse::<SocketAddr>().ok())
            .map_or(3000, |a| a.port())
    });
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let check = || -> std::io::Result<bool> {
        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(3))?;
        stream.set_read_timeout(Some(Duration::from_secs(3)))?;
        stream.write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")?;
        let mut head = [0u8; 12];
        stream.read_exact(&mut head)?;
        Ok(head.starts_with(b"HTTP/1.1 200"))
    };
    match check() {
        Ok(true) => 0,
        Ok(false) => {
            eprintln!("healthcheck: /health antwortet nicht mit 200");
            1
        }
        Err(e) => {
            eprintln!("healthcheck: {e}");
            1
        }
    }
}

/// Port aus der Umgebungsvariable `PORT` (setzen Plattformen wie Railway).
/// Ist sie gesetzt, hört der Server auf allen Schnittstellen (`0.0.0.0`).
#[cfg(feature = "ssr")]
fn env_port() -> Option<u16> {
    std::env::var("PORT").ok()?.trim().parse().ok()
}

/// Wartet auf Ctrl+C oder SIGTERM (`docker stop`), damit laufende Anfragen
/// noch fertig beantwortet werden.
#[cfg(feature = "ssr")]
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Ctrl+C-Handler fehlgeschlagen");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("SIGTERM-Handler fehlgeschlagen")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("Signal empfangen, Server fährt herunter …");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // Kein Client-seitiger main – Hydration startet in lib.rs (hydrate()).
}
