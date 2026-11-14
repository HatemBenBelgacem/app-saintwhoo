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

Danach läuft die App unter <http://127.0.0.1:3010>. Die Magazin-Startseite ist öffentlich, die Verwaltung liegt unter `/artikel`, und `/status` zeigt, ob die Datenbankverbindung klappt.

## Mit Docker starten (ANF-010)

Nur Docker nötig – kein Rust, Node oder lokales PostgreSQL.

```bash
cp .env.example .env              # POSTGRES_PASSWORD und ICECAST_*-Passwörter setzen
docker compose up --build         # App → http://localhost:3000 (APP_PORT), Icecast → :8000
docker compose logs -f app
docker compose down               # stoppen, Daten bleiben im Volume pgdata
docker compose down -v            # stoppen und Daten löschen
```

Der Build dauert einige Minuten (Rust + WASM). Die Datenbank im Container ist von außen nicht erreichbar; für `psql` den auskommentierten `ports`-Eintrag beim Dienst `db` nutzen. `curl localhost:3000/health` zeigt, ob App und Datenbank laufen.

## Livestream mit Icecast (Docker)

Für den Radio-Livestream (ANF-007) gibt es einen Icecast-Container in [`docker/icecast/`](../docker/icecast/).

```bash
# in .env: ICECAST_SOURCE_PASSWORD und ICECAST_ADMIN_PASSWORD setzen,
#          RADIO_STREAM_URL=http://localhost:8000/live
docker compose up -d icecast      # Status-Seite → http://localhost:8000
docker compose logs -f icecast
docker compose down
```

Senden mit z. B. BUTT oder Mixxx: Server `localhost`, Port `8000`, Typ Icecast, Benutzer `source`, Passwort `ICECAST_SOURCE_PASSWORD`, Mountpoint `/live`, Format MP3 oder Ogg. Die Verwaltung liegt unter `http://localhost:8000/admin/` (Benutzer `admin`).

Ohne gesetzte Passwörter (oder mit dem Platzhalter `bitte-aendern`) startet der Container nicht. Läuft die App später über HTTPS, muss auch der Stream über HTTPS ausgeliefert werden (Reverse-Proxy vor Icecast), sonst blockiert der Browser ihn.

## Tech-Stack

| Bereich | Technologie |
|---|---|
| Sprache | Rust (Edition 2024) |
| Web-Framework | Leptos 0.8 (SSR + Hydration) |
| HTTP-Server | Axum 0.8 |
| Datenbank | PostgreSQL + sqlx 0.9 |
| CSS | Tailwind CSS 4 + DaisyUI 5 |
| Build | cargo-leptos |
