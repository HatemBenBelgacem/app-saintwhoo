//! Magazin-Artikel (ANF-001).
//!
//! - `model`: Typen und Validierung – läuft auf Server **und** im Browser
//! - `api`:   Server-Funktionen (`#[server]`)
//! - `repo`:  Datenbankzugriff (nur `ssr`)

pub mod api;
pub mod model;
#[cfg(feature = "ssr")]
pub mod repo;
