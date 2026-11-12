# Saintwhoo – Dokumentation

Einstiegspunkt für alle Projektdokumente.

| Dokument | Inhalt |
|---|---|
| [Anforderung.md](Anforderung.md) | Was die App können muss – Übersicht (fachlich und technisch) |
| [features/](features/) | Eine Datei pro Anforderung (ANF-xxx) mit Abnahmekriterien |
| [system-design.md](system-design.md) | Wie die App aufgebaut ist (Architektur, Datenfluss, Datenbank) |
| [task.md](task.md) | Aufgabenliste und Fortschritt |
| [../CLAUDE.md](../CLAUDE.md) | Arbeitsregeln und Befehle für Claude Code |

## Schnellstart

Voraussetzungen: Rust (stable) mit Target `wasm32-unknown-unknown`, `cargo-leptos`, `sqlx-cli`, Node.js, PostgreSQL.

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos sqlx-cli --locked   # falls noch nicht installiert
npm install
cp .env.example .env        # DATABASE_URL anpassen
sqlx database create
cargo leptos watch
```

Danach läuft die App unter <http://127.0.0.1:3000>. Die Magazin-Startseite ist öffentlich, die Verwaltung liegt unter `/artikel`, und `/status` zeigt, ob die Datenbankverbindung klappt.

## Tech-Stack

| Bereich | Technologie |
|---|---|
| Sprache | Rust (Edition 2024) |
| Web-Framework | Leptos 0.8 (SSR + Hydration) |
| HTTP-Server | Axum 0.8 |
| Datenbank | PostgreSQL + sqlx 0.9 |
| CSS | Tailwind CSS 4 + DaisyUI 5 |
| Build | cargo-leptos |
