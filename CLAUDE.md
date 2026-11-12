# CLAUDE.md

Hinweise für Claude Code in diesem Projekt.

## Projekt

**Saintwhoo** – Fullstack-Web-App in Rust.

- **Frontend + Backend:** Leptos 0.8 (SSR + Hydration), ein Crate für beides
- **Server:** Axum 0.8 über `leptos_axum`
- **Datenbank:** PostgreSQL über `sqlx` 0.9 (Migrationen in `migrations/`)
- **Styling:** Tailwind CSS v4 + DaisyUI 5 (wird von `cargo-leptos` gebaut)

Dokumentation: siehe [docs/index.md](docs/index.md).
Anforderungen: [docs/Anforderung.md](docs/Anforderung.md) (Übersicht) + [docs/features/](docs/features/) (je Anforderung eine Datei) · Architektur: [docs/system-design.md](docs/system-design.md) · Aufgaben: [docs/task.md](docs/task.md)

## Befehle

```bash
npm install                      # Tailwind + DaisyUI (einmalig)
cp .env.example .env             # DATABASE_URL setzen
sqlx database create             # DB anlegen
cargo leptos watch               # Dev-Server mit Hot-Reload → http://127.0.0.1:3000
cargo leptos build --release     # Produktions-Build
cargo check --features ssr                                         # Server prüfen
cargo check --lib --features hydrate --target wasm32-unknown-unknown  # Client prüfen
cargo test --features ssr        # Unit- + DB-Tests (DATABASE_URL muss gesetzt sein, sqlx::test legt Test-DBs an)
cargo clippy --features ssr --all-targets -- -D warnings
sqlx migrate add <name>          # neue Migration
```

## Struktur

```
src/
  main.rs          Server-Start (nur ssr): Pool, Migrationen, Axum-Router
  lib.rs           Module + hydrate()-Einstieg für WASM
  app.rs           shell() (HTML-Gerüst) und <App/> mit Router
  state.rs         AppState (LeptosOptions + PgPool), nur ssr
  db.rs            connect(), migrate(), pool() für Server-Funktionen, nur ssr
  articles/        Fachbereich Artikel: model.rs (Server+Browser), api.rs (#[server]), repo.rs (SQL, ssr)
  products/        Fachbereich Shop-Produkte (ANF-005), gleicher Aufbau wie articles/
  components/      Wiederverwendbare UI-Komponenten (article.rs: Felder, Badge, Lösch-Dialog, Flash; product.rs: dasselbe für Produkte)
  pages/           Eine Datei pro Seite/Route
style/tailwind.css Tailwind-Einstieg mit @plugin "daisyui"
migrations/        sqlx-Migrationen (laufen beim Serverstart)
tests/             Integrationstests gegen PostgreSQL (eine Datei pro ANF)
public/            Statische Dateien
docs/              Projekt-Dokumentation
  features/        Anforderungen ANF-xxx, eine Datei pro Feature
```

## Regeln

- **Zwei Targets:** Code läuft auf dem Server (`ssr`) *und* im Browser (`hydrate`, WASM). Server-only-Code (sqlx, axum, tokio, env) immer hinter `#[cfg(feature = "ssr")]` oder in `#[server]`-Funktionen. Nach Änderungen **beide** `cargo check`-Befehle laufen lassen.
- **Datenzugriff** nur über `#[server]`-Funktionen; Pool mit `crate::db::pool()?` holen.
- **SQL:** `sqlx::query_as` mit Bind-Parametern (`$1`), nie String-Formatierung. Die `query!`-Makros brauchen beim Kompilieren eine DB – nur nutzen, wenn `DATABASE_URL` gesetzt ist oder `cargo sqlx prepare` gelaufen ist.
- **Schemaänderungen** immer als neue Migration, bestehende Migrationen nie ändern.
- **Styling:** DaisyUI-Komponenten (`btn`, `card`, `alert`, …) zuerst, Tailwind-Utilities für Layout. Keine eigenen CSS-Dateien ohne Grund. Klassen als ganze Strings schreiben (kein `format!("btn-{x}")`), sonst findet Tailwind sie nicht.
- **Neue Seite:** Datei in `src/pages/`, in `pages/mod.rs` eintragen, `<Route>` in `app.rs` ergänzen.
- **Neuer Datensatz-Typ:** dem CRUD-Muster in `docs/system-design.md` folgen (Vorlage: `src/articles/`, `src/pages/article_*.rs`).
- **Leptos-Typtiefe:** Bei „queries overflow the depth limit“ an Komponentengrenzen `.into_any()` verwenden, statt das `recursion_limit` zu erhöhen.
- **sqlx 0.9** akzeptiert nur feste SQL-Strings (`&'static str`). Kein `format!` für SQL.
- **Bilder** nur mit Bildnachweis und nur aus freien Quellen (z. B. Wikimedia Commons). Lizenz in `public/images/artikel/CREDITS.json` eintragen.
- **`public/`:** `cargo leptos watch` übernimmt neue Dateien nicht immer zuverlässig. Im Zweifel den Watcher neu starten.
- `wasm-bindgen` ist exakt gepinnt – Version nur zusammen mit der wasm-bindgen-CLI ändern.
- Sprache in UI und Doku: Deutsch.
- Vor der Umsetzung eines Features die passende Datei in `docs/features/` lesen; die Abnahmekriterien (AK-x) sind die Testfälle.
- Nach erledigten Aufgaben `docs/task.md` aktualisieren.
