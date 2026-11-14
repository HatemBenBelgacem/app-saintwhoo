//! Events (ANF-006).
//!
//! - `model`: Typen und Prüfregeln – läuft auf Server **und** im Browser
//! - `api`:   Server-Funktionen (`#[server]`)
//! - `repo`:  Datenbankzugriff (nur `ssr`)

pub mod api;
pub mod model;
#[cfg(feature = "ssr")]
pub mod repo;
