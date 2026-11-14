# ANF-010 – Docker-Container: App und Datenbank mit einem Befehl starten

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Bezug: [task.md](../task.md) Phase 3 „Dockerfile + docker-compose (App + PostgreSQL)“ · Offene Frage „Wo wird deployt?“

| | |
|---|---|
| **Priorität** | Soll |
| **Status** | spezifiziert (14.11.2026) |
| **Rolle** | Entwickler:in, Betrieb |

**Beschreibung**
Saintwhoo soll als Docker-Image gebaut und mit `docker compose` zusammen mit PostgreSQL gestartet werden können. Wer das Projekt klont, braucht dafür nur Docker – kein Rust, kein Node, kein lokales PostgreSQL. Das Image ist die Grundlage für ein späteres Deployment (VPS oder Cloud).

Die Entwicklung mit `cargo leptos watch` bleibt wie bisher. Docker ersetzt sie nicht.

**User Stories**

- Als Entwickler:in möchte ich mit `docker compose up` die ganze App samt Datenbank starten, damit ich nichts lokal installieren muss.
- Als Betrieb möchte ich ein schlankes Image ohne Geheimnisse, damit ich es sicher in eine Registry laden und auf einem Server starten kann.
- Als Betrieb möchte ich, dass Daten einen Neustart überleben und Migrationen von selbst laufen.

---

## Teil A – Image (`Dockerfile`)

Mehrstufiger Build (Multi-Stage):

| Stufe | Basis | Aufgabe |
|---|---|---|
| **builder** | offizielles Rust-Image (Version wie lokal, `edition = "2024"` → Rust ≥ 1.85) | Node + `npm ci` (Tailwind, DaisyUI), Ziel `wasm32-unknown-unknown`, `cargo-leptos` und `wasm-bindgen-cli` **in exakt der Version aus `Cargo.toml` (`=0.2.130`)** installieren, dann `cargo leptos build --release` |
| **runtime** | schlankes Debian (`debian:bookworm-slim` o. Ä.) | nur Server-Binary + `target/site` (CSS, WASM, JS, Bilder aus `public/`) |

Regeln:

- Das Runtime-Image enthält **kein** Rust, Node, Quellcode, `node_modules` oder `.env`.
- Die App läuft als **eigener Benutzer ohne Root-Rechte**.
- Die Migrationen sind über `sqlx::migrate!` ins Binary eingebaut. Der Ordner `migrations/` muss im Image nicht liegen.
- Abhängigkeiten werden in einer eigenen Schicht gebaut (Cache), damit ein geänderter Quelltext nicht alles neu baut. *(Soll)*
- Zielgröße des Runtime-Images: **unter 150 MB**. *(Soll)*
- Port **3000** wird freigegeben (`EXPOSE 3000`).

Dazu eine **`.dockerignore`**: `target/`, `node_modules/`, `.env`, `.git/`, `end2end/`, `docs/`.

## Teil B – Konfiguration über Umgebungsvariablen

Alles, was sich je Umgebung ändert, kommt aus der Umgebung – nie fest ins Image (NFA-05).

| Variable | Pflicht | Wert im Container | Zweck |
|---|---|---|---|
| `DATABASE_URL` | ja | `postgres://saintwhoo:…@db:5432/saintwhoo` | Verbindung zu PostgreSQL |
| `LEPTOS_SITE_ADDR` | ja | `0.0.0.0:3000` | Lauscht auf allen Schnittstellen. **Wichtig:** `127.0.0.1` aus `Cargo.toml` ist im Container von außen nicht erreichbar. |
| `LEPTOS_SITE_ROOT` | ja | `site` (Pfad zum kopierten `target/site`) | Statische Dateien |
| `LEPTOS_ENV` | ja | `PROD` | Kein Hot-Reload |
| `LEPTOS_OUTPUT_NAME` | ja | `app-saintwhoo` | Name der WASM/JS-Dateien |
| `RADIO_STREAM_URL` | nein | Stream-Adresse (ANF-007) | leer = „Livestream nicht verfügbar“ |
| `RUST_LOG` | nein | `info` | Log-Stufe |

Fehlt `DATABASE_URL`, bricht die App beim Start mit einer klaren Meldung ab (verhält sich schon heute so).

## Teil C – `docker-compose.yml`

Zwei Dienste:

| Dienst | Inhalt |
|---|---|
| `db` | `postgres:17` (oder neuer), Benutzer/Passwort/DB aus `.env`, **benanntes Volume** `pgdata` für die Daten, Healthcheck mit `pg_isready` |
| `app` | wird aus dem `Dockerfile` gebaut, Port `3000:3000`, `depends_on: db` mit `condition: service_healthy`, `restart: unless-stopped` |

- Passwörter stehen in `.env` (nicht im Git). Neu: **`.env.example`** mit allen Variablen und Platzhaltern, damit klar ist, was gesetzt werden muss. *(Die Datei fehlt heute, obwohl `CLAUDE.md` sie nennt.)*
- Der Datenbank-Port wird **nicht** nach außen freigegeben (nur im Compose-Netz erreichbar). Für die lokale Entwicklung darf ein auskommentierter Eintrag `5432:5432` dabeistehen.
- Migrationen laufen beim Start der App (wie bisher). Kein eigener Migrations-Dienst.

## Teil D – Betrieb

- **Healthcheck der App:** `GET /status` antwortet mit HTTP 200. *(Soll: eigener schlanker Endpunkt `/health`, der nur `SELECT 1` prüft und kein HTML rendert.)*
- **Sauberes Beenden:** Bei `docker compose stop` (SIGTERM) beendet sich die App innerhalb von 10 Sekunden, laufende Anfragen werden fertig beantwortet (Axum `with_graceful_shutdown`). *(Soll)*
- **Logs** gehen auf stdout/stderr, damit `docker compose logs app` sie zeigt.
- Doku: Abschnitt „Mit Docker starten“ in `docs/index.md` bzw. `CLAUDE.md` (Befehle unten).

```bash
cp .env.example .env            # Passwörter setzen
docker compose up --build       # bauen und starten → http://localhost:3000
docker compose logs -f app      # Logs
docker compose down             # stoppen (Daten bleiben im Volume)
docker compose down -v          # stoppen und Daten löschen
```

---

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | frisch geklontes Repo, nur Docker installiert, `.env` aus `.env.example` | `docker compose up --build` läuft | ist die App unter `http://localhost:3000` erreichbar, die Startseite zeigt Live, News, Events und Merch |
| AK-2 | leere Datenbank | die App zum ersten Mal startet | laufen alle Migrationen; `/status` meldet „Verbunden“ |
| AK-3 | App läuft | eine Seite im Browser geöffnet wird | werden CSS und WASM geladen (dunkles Theme, Hydration funktioniert, z. B. „+“ im Shop legt in den Warenkorb) |
| AK-4 | ein Artikel wurde angelegt | `docker compose down` und wieder `up` | ist der Artikel noch da |
| AK-5 | Datenbank startet langsamer als die App | `docker compose up` | wartet die App auf den Healthcheck der DB und startet danach ohne Fehler |
| AK-6 | das gebaute Image | `docker run --rm --entrypoint id <image>` bzw. Inspektion | läuft die App nicht als `root`; im Image liegen weder `.env` noch Quellcode noch `node_modules` |
| AK-7 | `DATABASE_URL` fehlt | der Container startet | beendet er sich mit der Meldung „DATABASE_URL ist nicht gesetzt“ |
| AK-8 | `RADIO_STREAM_URL` gesetzt | `/radio` geöffnet wird | wird der Stream angeboten; ohne Variable erscheint „Der Livestream ist zurzeit nicht verfügbar.“ |
| AK-9 | App läuft | `docker compose stop app` | ist der Container nach höchstens 10 s beendet, im Log steht kein Absturz *(Soll)* |
| AK-10 | das Runtime-Image | `docker image ls` | ist es kleiner als 150 MB *(Soll)* |

AK-1 bis AK-9: manuell bzw. als Skript in der CI (`docker compose up -d`, `curl -f http://localhost:3000/status`). Ein Docker-Build in der CI (Phase 3) prüft AK-6 und AK-10 bei jeder Änderung.

## Annahmen

- Ein Image für alle Umgebungen; Unterschiede nur über Umgebungsvariablen.
- Plattform `linux/amd64`. Auf Apple Silicon wird lokal `linux/arm64` gebaut; beide sollen funktionieren, ein Multi-Arch-Build ist aber nicht Pflicht.
- Bilder liegen weiterhin in `public/images/` und werden ins Image kopiert. Hochgeladene Bilder gibt es noch nicht (siehe task.md); kommen sie, braucht es ein zweites Volume.
- HTTPS übernimmt später ein Reverse-Proxy (z. B. Caddy, Traefik) vor dem Container, nicht die App selbst.

## Nicht Teil von ANF-010

- Deployment auf einen konkreten Server oder in eine Cloud, Registry-Push
- Reverse-Proxy und TLS-Zertifikate
- Backups der Datenbank
- Kubernetes, Skalierung auf mehrere App-Instanzen
- Hot-Reload im Container (Entwicklung bleibt bei `cargo leptos watch`)
