//! ANF-011 – PWA, Integrationstests gegen den echten Router.
//!
//! Ausführen: `cargo test --features ssr`
//! Die Datenbank ist hier absichtlich **nicht** erreichbar (`connect_lazy` auf
//! einen geschlossenen Port): Offline-Seite und App-Hülle dürfen sie nicht brauchen.
//! Statische Dateien kommen direkt aus `public/`.
#![cfg(feature = "ssr")]

use app_saintwhoo::state::AppState;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use axum::response::Response;
use leptos::prelude::LeptosOptions;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

fn app() -> axum::Router {
    let leptos_options = LeptosOptions::builder().output_name("app-saintwhoo").site_root("public").build();
    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(200))
        .connect_lazy("postgres://niemand@127.0.0.1:1/keine_db")
        .unwrap();
    app_saintwhoo::server::router(AppState { leptos_options, pool })
}

async fn get(path: &str) -> Response {
    app().oneshot(Request::get(path).body(Body::empty()).unwrap()).await.unwrap()
}

async fn text(res: Response) -> String {
    String::from_utf8(to_bytes(res.into_body(), 4 * 1024 * 1024).await.unwrap().to_vec()).unwrap()
}

fn header_of(res: &Response, name: header::HeaderName) -> &str {
    res.headers().get(name).and_then(|v| v.to_str().ok()).unwrap_or("")
}

#[tokio::test]
async fn ak1_manifest_mit_name_icons_und_content_type() {
    let res = get("/manifest.webmanifest").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(header_of(&res, header::CONTENT_TYPE), "application/manifest+json");

    let m: serde_json::Value = serde_json::from_str(&text(res).await).expect("Manifest ist gültiges JSON");
    assert_eq!(m["name"], "Saintwhoo");
    assert_eq!(m["short_name"], "Saintwhoo");
    assert_eq!(m["lang"], "de");
    assert_eq!(m["start_url"], "/?quelle=pwa");
    assert_eq!(m["scope"], "/");
    assert_eq!(m["display"], "standalone");
    assert_eq!(m["background_color"], "#0B0B0C");
    assert_eq!(m["theme_color"], "#0B0B0C");

    let icons = m["icons"].as_array().unwrap();
    let has = |size: &str, purpose: &str| {
        icons.iter().any(|i| i["sizes"] == size && i["purpose"] == purpose && i["type"] == "image/png")
    };
    assert!(has("192x192", "any"), "Icon 192 fehlt");
    assert!(has("512x512", "any"), "Icon 512 fehlt");
    assert!(has("512x512", "maskable"), "maskierbares Icon fehlt");

    // Jedes Icon im Manifest wird auch ausgeliefert
    for icon in icons {
        let src = icon["src"].as_str().unwrap();
        assert_eq!(get(src).await.status(), StatusCode::OK, "{src} fehlt");
    }
}

#[tokio::test]
async fn ak1_icons_haben_die_richtige_groesse() {
    for (path, size) in [
        ("public/icons/icon-192.png", 192),
        ("public/icons/icon-512.png", 512),
        ("public/icons/icon-maskable-512.png", 512),
        ("public/icons/apple-touch-icon.png", 180),
    ] {
        let png = std::fs::read(path).unwrap();
        assert_eq!(&png[1..4], b"PNG", "{path} ist kein PNG");
        // IHDR: Breite und Höhe ab Byte 16, big endian
        let w = u32::from_be_bytes(png[16..20].try_into().unwrap());
        let h = u32::from_be_bytes(png[20..24].try_into().unwrap());
        assert_eq!((w, h), (size, size), "{path}");
    }
}

#[tokio::test]
async fn ak4_service_worker_wird_mit_no_cache_ausgeliefert() {
    let res = get("/sw.js").await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(header_of(&res, header::CACHE_CONTROL), "no-cache");
    assert!(header_of(&res, header::CONTENT_TYPE).contains("javascript"));
    let sw = text(res).await;
    // Caching-Regeln: Server-Funktionen und Range-Anfragen nie anfassen
    assert!(sw.contains("\"/api/\""));
    assert!(sw.contains("range"));
    assert!(sw.contains("\"/offline\""));
}

#[tokio::test]
async fn ak4_seite_bindet_manifest_icons_und_registrierung_ein() {
    let res = get("/offline").await;
    let html = text(res).await;
    assert!(html.contains(r#"<link rel="manifest" href="/manifest.webmanifest""#));
    assert!(html.contains(r#"<link rel="apple-touch-icon" href="/icons/apple-touch-icon.png""#));
    assert!(html.contains(r#"name="apple-mobile-web-app-status-bar-style" content="black-translucent""#));
    assert!(html.contains(r#"<script src="/pwa.js">"#));

    let js = get("/pwa.js").await;
    assert_eq!(js.status(), StatusCode::OK);
    let js = text(js).await;
    assert!(js.contains(r#""serviceWorker" in navigator"#), "Registrierung nur mit Service-Worker-Unterstützung (AK-12)");
    assert!(js.contains(r#"register("/sw.js", { scope: "/" })"#));
}

#[tokio::test]
async fn ak6_offline_seite_ohne_datenbank() {
    let res = get("/offline?ziel=%2Fradio").await;
    assert_eq!(res.status(), StatusCode::OK);
    let html = text(res).await;
    assert!(html.contains("Du bist offline"));
    assert!(html.contains("Erneut versuchen"));
    // AK-7: „Erneut versuchen“ führt zur gewünschten Seite
    assert!(html.contains(r#"href="/radio""#));
}

#[tokio::test]
async fn ak6_offline_seite_leitet_nicht_nach_aussen() {
    let html = text(get("/offline?ziel=%2F%2Fboese.example").await).await;
    assert!(!html.contains("boese.example"));
}
