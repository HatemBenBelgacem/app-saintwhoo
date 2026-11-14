# System-Design

> Stand: 12.11.2026

## 1. Überblick

Saintwhoo ist **eine** Rust-Anwendung. Derselbe Code wird zweimal gebaut:

1. **Server-Binary** (Feature `ssr`): Axum-Server, rendert HTML, führt Server-Funktionen aus, spricht mit PostgreSQL.
2. **WASM-Bundle** (Feature `hydrate`): läuft im Browser, macht das vom Server gerenderte HTML interaktiv.

```
 Browser                          Server (Axum + Leptos SSR)            PostgreSQL
┌─────────────────┐  1. GET /    ┌──────────────────────────┐
│                 │ ───────────► │ Route → Komponente       │
│  HTML + CSS     │ ◄─────────── │ rendert HTML (SSR)       │
│                 │  2. pkg/*.js │                          │
│  WASM hydriert  │ ───────────► │ statische Dateien        │
│                 │              │ (target/site)            │
│  Interaktion    │ 3. POST      │                          │   SQL    ┌──────────┐
│  ruft #[server] │ ───────────► │ #[server] fn             │ ───────► │ Tabellen │
│  Funktion auf   │ ◄─────────── │ + PgPool aus Kontext     │ ◄─────── │          │
└─────────────────┘   JSON       └──────────────────────────┘          └──────────┘
```

## 2. Komponenten

| Teil | Datei(en) | Aufgabe |
|---|---|---|
| Server-Start | `src/main.rs` | `.env` laden, Logging, Pool bauen, Migrationen, Router starten |
| App-Zustand | `src/state.rs` | `AppState { leptos_options, pool }` für Axum |
| Datenbank | `src/db.rs` | Pool erstellen, Migrationen, `pool()` für Server-Funktionen |
| HTML-Gerüst | `src/app.rs` → `shell()` | `<html>`, Head, Hydration-Skripte |
| Routing | `src/app.rs` → `App` | Leptos-Router, Navbar, Seiten |
| Seiten | `src/pages/*` | Je Route eine Komponente, Server-Funktionen daneben |
| UI-Bausteine | `src/components/*` | Wiederverwendbare Komponenten |
| Styling | `style/tailwind.css` | Tailwind + DaisyUI, gescannt wird `src/**/*.rs` |

## 3. Datenfluss

1. **Erster Aufruf:** Axum ruft die Leptos-Route auf. Komponenten mit `Resource` rufen Server-Funktionen direkt (ohne HTTP) auf. Das fertige HTML geht an den Browser.
2. **Hydration:** Der Browser lädt `pkg/app-saintwhoo.js` + `.wasm`. Leptos übernimmt das bestehende DOM, die Resource-Daten werden mitgeliefert (kein zweiter DB-Aufruf).
3. **Danach:** Server-Funktionen werden als `POST /api/<name>` aufgerufen. Der Pool kommt über `provide_context` (in `leptos_routes_with_context`) in jede Anfrage.

## 4. Datenbank

- PostgreSQL, Zugriff über `sqlx::PgPool` (max. 10 Verbindungen).
- Verbindung über `DATABASE_URL` (aus `.env`).
- Migrationen in `migrations/`, werden beim Serverstart automatisch ausgeführt (`sqlx::migrate!`).

### Aktuelles Schema

```sql
app_meta (
  key        TEXT PRIMARY KEY,
  value      TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

```sql
-- ANF-001
articles (
  id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  title        TEXT NOT NULL,          -- 1–200 Zeichen, getrimmt
  body         TEXT NOT NULL,          -- nicht nur Leerzeichen
  published_on DATE,                   -- Pflicht bei status = 'published'
  status       TEXT NOT NULL DEFAULT 'draft',  -- 'draft' | 'published'
  created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT now(), -- ANF-002, bei jedem UPDATE gesetzt
  image_url    TEXT,  -- ANF-004: https://… oder /…
  image_credit TEXT   -- ANF-004: Pflicht, wenn image_url gesetzt
)
```

```sql
-- ANF-005
products (
  id                BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name              TEXT NOT NULL,               -- 1–120 Zeichen, getrimmt
  sku               TEXT NOT NULL UNIQUE,        -- Artikelnummer, ^[A-Z0-9-]{1,40}$
  short_description TEXT NOT NULL DEFAULT '',    -- max. 200 Zeichen
  description       TEXT NOT NULL DEFAULT '',
  price_cents       INTEGER NOT NULL,            -- Rappen, 0 … 9'999'999
  stock             INTEGER NOT NULL DEFAULT 0,  -- >= 0
  status            TEXT NOT NULL DEFAULT 'draft', -- 'draft' | 'active'
  image_url         TEXT,  -- Teil C: wie articles.image_url
  image_credit      TEXT,  -- Teil C: Pflicht, wenn image_url gesetzt
  created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at        TIMESTAMPTZ NOT NULL DEFAULT now()
)
```

```sql
-- Ergänzungen 14.11.2026
articles.category  TEXT NOT NULL DEFAULT 'news'   -- ANF-008: news | interview | release | event | mix
products.category  TEXT NOT NULL DEFAULT 'other'  -- ANF-009: hoodie | shirt | vinyl | accessory | other
products.brand     TEXT NOT NULL DEFAULT ''       -- ANF-009: max. 80 Zeichen

-- ANF-006
events (
  id, title, description,
  starts_at     TIMESTAMP NOT NULL,   -- Ortszeit Zürich, ohne Zeitzone
  venue, city, lineup,
  ticket_url    TEXT,                 -- https://…
  ticket_status TEXT NOT NULL DEFAULT 'available', -- available | few | sold_out
  image_url, image_credit, created_at, updated_at
)

-- ANF-007
radio_shows (id, title, host, starts_at TIMESTAMP, ends_at TIMESTAMP, created_at)  -- max. 12 Std.
radio_mixes (id, title, host, duration_minutes INTEGER, audio_url TEXT, created_at)
```

Die Geschäftsregeln stehen zusätzlich als `CHECK`-Constraints in der DB.

**Zeitzone:** Event- und Sendezeiten sind Ortszeit (Europe/Zurich) und werden als `TIMESTAMP` ohne Zeitzone gespeichert. „Heute“ und „jetzt“ ermittelt die Datenbank mit `now() AT TIME ZONE 'Europe/Zurich'`. So braucht das WASM-Bundle kein `chrono-tz`.

### Modul-Aufbau pro Fachbereich (Beispiel `src/articles/`)

| Datei | Läuft auf | Inhalt |
|---|---|---|
| `model.rs` | Server + Browser | Typen, `validate()`, Fehlertexte |
| `api.rs` | Server (Aufruf vom Browser) | `#[server]`-Funktionen |
| `repo.rs` | nur Server (`ssr`) | SQL mit sqlx |

### Routen

| Adresse | Seite | SSR-Modus |
|---|---|---|
| `/` | Startseite: Live, News, Events, Merch (ANF-009) | Standard |
| `/news` | News mit Suche und Rubrik (`?q=`, `?rubrik=`, `?seite=`) – ANF-008 | Standard |
| `/eventliste` | Eventliste (`?zeitraum=`) – ANF-006 | Standard |
| `/eventliste/:id` | Eventseite | Async (für 404) |
| `/events`, `/events/neu`, `/events/:id`, `/events/:id/bearbeiten` | Eventverwaltung (CRUD) | Async bei `:id` |
| `/radio` | Radio Live + Sendeplan, `?ansicht=archiv` – ANF-007 | Standard |
| `/radio/verwaltung` | Sendungen und Mixe anlegen/löschen | Standard |
| `/warenkorb` | Warenkorb (nur im Browser) – ANF-009 | Standard |
| `/beitrag/:id` | Leseansicht (nur öffentliche Artikel) | Async (für 404) |
| `/status` | Systemstatus (DB-Verbindung) | Standard |
| `/health` | Healthcheck für Docker: `200 ok` oder `503` (Axum-Route, kein Leptos) – ANF-010 | – |
| `/artikel` | Liste (`?status=`, `?seite=`, `?meldung=`) | Standard |
| `/artikel/neu` | Erstellen | Standard |
| `/artikel/:id` | Detail | Async (für 404) |
| `/artikel/:id/bearbeiten` | Bearbeiten | Async (für 404) |
| `/shop` | Shop-Übersicht (`?kategorie=`, `?sortierung=`, `?seite=`) – ANF-005/009 | Standard |
| `/shop/:id` | Produktseite (nur aktive Produkte) | Async (für 404) |
| `/produkte` | Produktliste (`?status=`, `?seite=`, `?meldung=`) | Standard |
| `/produkte/neu` | Produkt erstellen | Standard |
| `/produkte/:id` | Produkt-Detail | Async (für 404) |
| `/produkte/:id/bearbeiten` | Produkt bearbeiten | Async (für 404) |

### CRUD-Muster (ANF-002, R-10)

Jeder neue Datensatz-Typ folgt dem Artikel-Muster:

1. Migration mit `id`, fachlichen Spalten, `created_at`, `updated_at` und `CHECK`-Constraints
2. `src/<typ>/model.rs`: Struct, `…Input`, `validate()`, `…Outcome`-Enums (`Created`/`Updated`/`Invalid`/`NotFound`/`Deleted`), Fehlertexte
3. `src/<typ>/repo.rs`: `create`, `find`, `list` (Seiten + Filter), `update`, `delete`
4. `src/<typ>/api.rs`: Server-Funktionen. Nach Update und Delete wird mit `leptos_axum::redirect` weitergeleitet.
5. Seiten Liste, Detail, Neu, Bearbeiten und Komponenten für Felder und Lösch-Dialog
6. `tests/anf_xxx.rs` mit einem Test pro Abnahmekriterium

### Konventionen

- Tabellennamen: Plural, `snake_case` (`users`, `orders`)
- Primärschlüssel: `id UUID` oder `BIGINT GENERATED ALWAYS AS IDENTITY`
- Zeitstempel: `TIMESTAMPTZ`, Spalten `created_at` / `updated_at`
- Fremdschlüssel mit `ON DELETE`-Regel explizit angeben

## 5. Styling

- Tailwind CSS 4 wird von `cargo-leptos` gebaut (`tailwind-input-file`), DaisyUI wird als Plugin aus `node_modules` geladen.
- Theme: eigenes dunkles DaisyUI-Theme `saintwhoo` (14.11.2026, Designvorschlag „Techno App“, ANF-009). `data-theme="saintwhoo"` und `color-scheme: dark` am `<html>`. Farben stehen als Tokens in `style/tailwind.css`.
- Schriften: Inter (`font-sans`) und Space Grotesk (`font-display`) über Google Fonts im `shell()`.
- Seitenrahmen in `App`: Kopfzeile (`Navbar`), Leiste unten auf dem Handy (`BottomNav`), Player-Leiste (`PlayerBar`). Player und Warenkorb liegen als Kontext in `App`, damit sie beim Navigieren erhalten bleiben.
- Große Komponenten enden mit `.into_any()`. Sonst werden die Typnamen so lang, dass der macOS-Linker abbricht (`ld: Assertion failed … makeSymbolStringInPlace`).
- Ergebnis: `/pkg/app-saintwhoo.css`, eingebunden in `App` über `<Stylesheet>`.

## 6. Konfiguration

| Variable | Wo | Bedeutung |
|---|---|---|
| `DATABASE_URL` | `.env` | PostgreSQL-Verbindung |
| `RUST_LOG` | `.env` | Log-Level (tracing) |
| `RADIO_STREAM_URL` | `.env` | Adresse des Radio-Livestreams (ANF-007). Leer = „Livestream nicht verfügbar“ |
| `LEPTOS_SITE_ADDR` | Umgebung | Überschreibt `site-addr` (Standard `127.0.0.1:3000`, im Container `0.0.0.0:3000`) |
| `LEPTOS_SITE_ROOT`, `LEPTOS_OUTPUT_NAME`, `LEPTOS_ENV` | Umgebung | Im Docker-Image fest gesetzt (`Dockerfile`), weil `Cargo.toml` dort fehlt |
| `POSTGRES_USER`, `POSTGRES_PASSWORD`, `POSTGRES_DB`, `APP_PORT` | `.env` | Nur für `docker compose` (ANF-010); die App im Container baut daraus ihre `DATABASE_URL` |
| `[package.metadata.leptos]` | `Cargo.toml` | Build-Einstellungen von cargo-leptos |

## 7. Deployment mit Docker (ANF-010)

`cargo leptos build --release` erzeugt:
- `target/release/app-saintwhoo` (Server)
- `target/site/` (CSS, JS, WASM, statische Dateien)

Das [`Dockerfile`](../Dockerfile) macht genau das in zwei Stufen:

| Stufe | Basis | Inhalt |
|---|---|---|
| `builder` | `rust:1.96-bookworm` + Node 24 | `npm ci`, `cargo-leptos` 0.3.10, `cargo leptos build --release`. cargo-leptos lädt wasm-bindgen (passend zu `Cargo.lock`), wasm-opt und Tailwind selbst. Cache-Mounts für Registry und `target/`. |
| `runtime` | `debian:bookworm-slim` | nur Binary + `site/`, Benutzer `saintwhoo` (ohne Root), `curl` für den Healthcheck auf `/health` |

Die Migrationen stecken über `sqlx::migrate!` im Binary und laufen beim Start. Bei SIGTERM (`docker stop`) beendet der Server laufende Anfragen und fährt dann herunter (`with_graceful_shutdown`).

[`docker-compose.yml`](../docker-compose.yml): `db` (PostgreSQL 17, Volume `pgdata`, Healthcheck `pg_isready`, Port nicht nach außen), `app` (startet erst, wenn `db` gesund ist) und `icecast` (ANF-007). HTTPS übernimmt später ein Reverse-Proxy davor.

## 8. Offene Entscheidungen

- Authentifizierung (Sessions mit Cookies vs. JWT)
- Fehlerbehandlung: eigener Fehlertyp statt `ServerFnError::new(String)`
- Zeitzone für Anzeige (aktuell UTC; Europe/Zurich würde `chrono-tz` im WASM-Bundle brauchen)
- Deployment-Ziel
