# Aufgaben

> Legende: `[x]` erledigt · `[ ]` offen · `[~]` in Arbeit

## Phase 0 – Setup

- [x] Leptos 0.8 Fullstack (SSR + Hydration) mit Axum aufsetzen
- [x] PostgreSQL-Anbindung mit sqlx (Pool, Migrationen beim Start)
- [x] Tailwind CSS 4 + DaisyUI 5 einbinden (über cargo-leptos)
- [x] Startseite mit DB-Statusprüfung, Navbar, 404-Seite
- [x] Dokumentation: CLAUDE.md, index.md, Anforderung.md, system-design.md, task.md
- [x] `.env` anlegen und Datenbank erstellen (`sqlx database create`)
- [x] Ersten Lauf mit `cargo leptos watch` prüfen
- [ ] Ersten Git-Commit erstellen

## Phase 1 – Anforderungen

- [ ] Ziel und Zielgruppe in `Anforderung.md` festlegen
- [ ] Funktionale Anforderungen (ANF-xxx) sammeln und priorisieren
  - [x] ANF-001 Magazin-Artikel erfassen – spezifiziert ([features/ANF-001](features/ANF-001-magazin-artikel-erfassen.md))
  - [x] ANF-001 umsetzen (Migration `articles`, Server-Funktion, Formular, Tests für AK-1 bis AK-9)
  - [ ] ANF-001 Annahmen bestätigen (Datum ohne Uhrzeit, Zukunftsdatum erlaubt, Klartext)
  - [ ] AK-7 als End-to-End-Test (Playwright)
  - [x] ANF-002 CRUD für alle Datensätze – spezifiziert ([features/ANF-002](features/ANF-002-crud-datensaetze.md))
  - [ ] ANF-002 Annahmen bestätigen (Hard Delete, last write wins, alle dürfen alles)
  - [x] ANF-002 umsetzen (Migration `updated_at`, Liste/Detail/Bearbeiten/Löschen, Tests für AK-1 bis AK-14)
  - [ ] ANF-002 AK-10 und AK-14 im Browser prüfen bzw. als Playwright-Test
  - [x] ANF-003 Magazin-Startseite – spezifiziert und umgesetzt ([features/ANF-003](features/ANF-003-magazin-startseite.md))
  - [ ] ANF-003 Annahmen bestätigen (13 Artikel, Systemschriften, DB-Datum)
  - [x] ANF-004 Artikelbild mit Bildnachweis ([features/ANF-004](features/ANF-004-artikelbild.md))
  - [ ] ANF-004 Annahmen bestätigen (Bild per Adresse, Commons-Testbilder)
  - [x] ANF-005 Shop: Produkte erfassen und anzeigen – spezifiziert ([features/ANF-005](features/ANF-005-shop-produkte.md))
  - [ ] ANF-005 Annahmen bestätigen (CHF inkl. MWST, Bezeichnung „Produkt“, Ausverkaufte sichtbar)
  - [x] ANF-005 umsetzen (Migration `products`, Verwaltung `/produkte`, Shop `/shop`, Tests für AK-1 bis AK-18)
  - [x] ANF-005 Teil C: Produktbild mit Bildnachweis + 5 Testprodukte mit Commons-Fotos
  - [ ] ANF-005 Löschdialog „Abbrechen“ im Browser prüfen bzw. als Playwright-Test
  - [x] ANF-006 Eventliste – spezifiziert ([features/ANF-006](features/ANF-006-eventliste.md))
  - [ ] ANF-006 Annahmen bestätigen (keine Endzeit, Tickets nur verlinkt, Beschreibung Pflicht)
  - [x] ANF-006 umsetzen (Migration `events`, Verwaltung `/events`, Eventliste `/eventliste`, Tests für AK-1 bis AK-21) – erweitert um Datum, Ort, Lineup, Tickets
  - [ ] ANF-006 AK-16 und AK-21 im Browser prüfen bzw. als Playwright-Test

## Designvorschlag „Techno App“ (14.11.2026)

Quelle: Claude-Design-Canvas „Techno App Designvorschlag“ (Version 1).

- [x] ANF-009 Look: dunkles DaisyUI-Theme `saintwhoo`, Inter + Space Grotesk, Karten ohne Schatten
- [x] ANF-009 Navigation: Leiste unten (Handy), Pillen oben (Desktop), Redaktions-Menü
- [x] ANF-009 Startseite neu: Live, News, Events, Merch
- [x] ANF-009 Shop: Kategorie + Marke (Migration), Kategorie-Chips, neue Karten, Warenkorb `/warenkorb`
- [x] ANF-007 Radio: Migration `radio_shows`/`radio_mixes`, `/radio`, `/radio/verwaltung`, Player läuft beim Seitenwechsel weiter, Tests AK-1 bis AK-8
- [x] ANF-008 News: Rubrik pro Artikel (Migration), `/news` mit Suche und Filter, Tests AK-1 bis AK-7
- [x] Icecast-Container für den Livestream: `docker/icecast/`, Dienst `icecast` in `docker-compose.yml`, `.env.example` (14.11.2026)
- [ ] `RADIO_STREAM_URL` in `.env` setzen (lokal `http://localhost:8000/live`; Quelle für Produktion klären, mit HTTPS über Reverse-Proxy)
- [ ] ANF-007 Bearbeiten von Sendungen und Mixe ergänzen (bisher nur Anlegen/Löschen)
- [ ] ANF-007 AK-9 (Wiedergabe läuft beim Navigieren weiter) und AK-10 im Browser prüfen
- [x] ANF-009 Helles Theme `saintwhoo-hell` + Umschalter in der Kopfzeile (`public/theme.js`, SW `VERSION` v2)
- [ ] ANF-009 AK-1 bis AK-3, AK-6 bis AK-9 im Browser prüfen bzw. als Playwright-Test (Klicks auf „+“, Warenkorb)
- [ ] Warenkorb über Neuladen hinaus speichern (localStorage oder Server)
- [ ] Produkte und Artikel mit passender Kategorie/Rubrik versehen (bestehende Daten: „Sonstiges“/„Meldung“)
- [ ] Offen, braucht Benutzerkonten: Anmelden, „Folge deinen DJs“, Events „Merken“/„Warteliste“
- [ ] Offen: Kasse/Bestellung, Größenauswahl

## ANF-011 PWA

- [x] ANF-011 PWA – spezifiziert ([features/ANF-011](features/ANF-011-pwa.md))
- [x] Icons gestalten (192, 512, maskable 512, Apple 180, favicon.svg) in `public/icons/` – geometrisches „S“, selbst gezeichnet
- [x] `public/manifest.webmanifest` (inkl. Shortcuts) + `<link>`/iOS-Meta-Tags in `shell()`
- [x] `public/sw.js` mit Caching-Regeln (Stream, Range, `/api/*` nie cachen), Registrierung in `public/pwa.js`
- [x] Route `/offline` (`src/pages/offline.rs`), Service Worker leitet offline auf `/offline?ziel=…` um
- [x] Knopf „App installieren“, Offline-Badge in der Kopfzeile, Hinweis „Neue Version verfügbar – Neu laden“, iOS-Hinweis auf der Startseite
- [x] Media Session im Radio-Player (Kann)
- [x] Router nach `src/server.rs` ausgelagert (für Tests); `sw.js` mit `Cache-Control: no-cache`
- [x] Tests: `tests/anf_011_pwa.rs` (AK-1, AK-4, AK-6/7, Content-Types)
- [x] AK-1, AK-4 bis AK-7, AK-9, AK-12 lokal mit Chrome (Playwright, Server gestoppt = offline) geprüft
- [ ] Playwright-Tests dauerhaft in `end2end/` anlegen (gibt es noch nicht)
- [ ] Redaktionsformulare offline deaktivieren (heute nur Hinweis „Nur online möglich“ im Redaktions-Menü)
- [ ] AK-2, AK-3, AK-11 (Lighthouse), AK-14 auf echten Geräten / gegen Railway prüfen
- [ ] Bei jeder Änderung an `public/sw.js`: `VERSION` erhöhen
  - [ ] Bilder hochladen (statt Adresse eintippen)
- [ ] Datenmodell in `system-design.md` entwerfen

## Phase 2 – Grundfunktionen

- [ ] Eigener Fehlertyp für Server-Funktionen
- [x] Layout (Navbar, Footer, Leiste unten) – Hell/Dunkel-Umschalter oben rechts (ANF-009)
- [ ] Authentifizierung (falls gefordert)
- [ ] Erste fachliche Tabellen + Migrationen

## Phase 3 – Qualität & Betrieb

- [ ] Tests (Unit + End-to-End mit Playwright in `end2end/`)
- [ ] CI (fmt, clippy, check für beide Targets)
- [x] ANF-010 Docker-Container – spezifiziert ([features/ANF-010](features/ANF-010-docker-container.md))
- [x] ANF-010 umsetzen: `Dockerfile`, `.dockerignore`, `docker-compose.yml` (db + app + icecast), `.env.example`, `/health`, Graceful Shutdown – AK-1 bis AK-10 geprüft
- [ ] ANF-010 AK-3 im Browser prüfen (Klick auf „+“ im Shop im Container)
- [ ] Docker-Build in die CI aufnehmen (AK-6, AK-10)
- [x] Railway-Deployment vorbereiten: `railway.toml` (Dockerfile-Build, Healthcheck `/health`), App liest `PORT`, Cache-Mounts aus dem `Dockerfile` entfernt
- [ ] Railway: Dienst auf Branch mit `railway.toml` stellen, PostgreSQL anlegen, `DATABASE_URL` setzen, Deployment prüfen
- [ ] Deployment
