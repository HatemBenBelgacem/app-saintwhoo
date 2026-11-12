//! Shop-Produkte (ANF-005).
//!
//! - `model`: Typen, Prüfregeln, Preis-Parser – läuft auf Server **und** im Browser
//! - `api`:   Server-Funktionen (`#[server]`)
//! - `repo`:  Datenbankzugriff (nur `ssr`)

pub mod api;
pub mod model;
#[cfg(feature = "ssr")]
pub mod repo;
