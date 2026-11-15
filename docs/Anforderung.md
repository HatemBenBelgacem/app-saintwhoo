# Anforderungen

> Stand: 14.11.2026 · Status: **Entwurf** – Richtung festgelegt durch den Designvorschlag „Techno App“ (ANF-009), weitere Anforderungen offen.

## 1. Ziel

Saintwhoo ist eine App für Techno-Fans: News über DJs, Releases und Events, ein Eventkalender, Radio-Livesets zum Streamen und ein Merch-Shop. Dunkel, schnell und ganz auf die Musik ausgerichtet. *(Aus dem Designvorschlag „Techno App“, Version 1. „Saintwhoo“ ist ein Arbeitstitel.)*

## 2. Zielgruppe / Rollen

| Rolle | Beschreibung |
|---|---|
| Redakteur:in | Erfasst und veröffentlicht Artikel (ANF-001, ANF-008), Events (ANF-006) und pflegt das Radio (ANF-007) |
| Shop-Verwalter:in | Erfasst und pflegt Produkte für den Shop (ANF-005) |
| Besucher:in | Liest News (ANF-003, ANF-008), sieht Events (ANF-006), hört Radio (ANF-007), stöbert im Shop und legt Produkte in den Warenkorb (ANF-005, ANF-009) |
| Benutzer (angemeldet) | _offen_ |
| Admin | _offen_ |

## 3. Funktionale Anforderungen

Format: `ANF-<Nr>` (dreistellig) · Priorität **M**uss / **S**oll / **K**ann

| ID | Anforderung | Priorität | Status |
|---|---|---|---|
| [ANF-001](features/ANF-001-magazin-artikel-erfassen.md) | Magazin-Artikel erfassen | M | umgesetzt |
| [ANF-002](features/ANF-002-crud-datensaetze.md) | CRUD für alle Datensätze (Liste, Detail, Bearbeiten, Löschen) | M | umgesetzt |
| [ANF-003](features/ANF-003-magazin-startseite.md) | Magazin-Startseite und Leseansicht | M | umgesetzt |
| [ANF-004](features/ANF-004-artikelbild.md) | Artikelbild mit Bildnachweis | M | umgesetzt |
| [ANF-005](features/ANF-005-shop-produkte.md) | Shop: Produkte erfassen und im Shop anzeigen | M | umgesetzt |
| [ANF-006](features/ANF-006-eventliste.md) | Eventliste: Events mit Datum, Ort, Lineup, Tickets und Bild | M | umgesetzt |
| [ANF-007](features/ANF-007-radio.md) | Radio: Livestream, Sendeplan und Archiv | M | umgesetzt |
| [ANF-008](features/ANF-008-news-suche-rubrik.md) | News: Rubrik, Suche und Filter | M | umgesetzt |
| [ANF-009](features/ANF-009-design-techno-app.md) | Design „Techno App“: Look, Navigation, Startseite, Shop-Filter, Warenkorb | M | umgesetzt |
| [ANF-010](features/ANF-010-docker-container.md) | Docker-Container: App und PostgreSQL mit `docker compose` starten | S | umgesetzt |
| [ANF-011](features/ANF-011-pwa.md) | PWA: App installieren, offline öffnen, Radio auf dem Sperrbildschirm | S | umgesetzt |

Jede Anforderung hat eine eigene Datei in [features/](features/).

### User Stories (optional)

- Als _Rolle_ möchte ich _Funktion_, damit _Nutzen_.

## 4. Nicht-funktionale Anforderungen

| ID | Anforderung | Priorität |
|---|---|---|
| NFA-01 | Server-Side Rendering: Seiten sind ohne JavaScript lesbar, danach interaktiv (Hydration) | M |
| NFA-02 | Responsive Design (Mobil bis Desktop) mit Tailwind/DaisyUI | M |
| NFA-03 | Daten werden dauerhaft in PostgreSQL gespeichert; Schema nur über Migrationen | M |
| NFA-04 | SQL nur mit Bind-Parametern (Schutz vor SQL-Injection) | M |
| NFA-05 | Geheimnisse (DB-Passwort usw.) nur in `.env` / Umgebung, nie im Git | M |
| NFA-06 | Seitenaufbau < 1 s im lokalen Netz | S |
| NFA-07 | Standard ist die dunkle Ansicht (DaisyUI-Theme `saintwhoo`, ANF-009), auch bei hellem System. Oben rechts lässt sich auf Hell (`saintwhoo-hell`) umschalten; die Wahl bleibt im Browser gespeichert *(geändert 15.11.2026, vorher „immer dunkel“)* | M |
| NFA-08 | Barrierearm: semantisches HTML, Kontraste, Tastaturbedienung | S |

## 5. Technische Rahmenbedingungen

- Rust-Fullstack mit Leptos 0.8 und Axum
- PostgreSQL als einzige Datenbank
- Tailwind CSS 4 + DaisyUI 5 für das UI
- Build und Dev-Server mit `cargo-leptos`

## 6. Abgrenzung (nicht im Umfang)

- _offen_

## 7. Offene Fragen

- Was ist der fachliche Kern der App? (bisher: Magazin mit Artikeln)
- ANF-001: Annahmen zu Datum und Textformat bestätigen
- ANF-002: Annahmen zu Löschen, gleichzeitigem Bearbeiten und Rechten bestätigen
- ANF-005: Annahmen zu Währung, Bezeichnung „Produkt“/„Artikel“ und ausverkauften Produkten bestätigen
- ANF-006: Datum, Ort und Tickets sind ergänzt. Ist die Beschreibung weiterhin Pflicht?
- ANF-007: Woher kommt der Livestream (`RADIO_STREAM_URL`)? Mit welchem Recht werden Aufnahmen veröffentlicht?
- Braucht es Benutzerkonten / Login? Der Designvorschlag setzt sie voraus für „Anmelden“, „Folge deinen DJs“, „Merken“ und „Warteliste“.
- Soll der Shop echte Bestellungen annehmen (Kasse, Bezahlung)? Bis dahin ist der Warenkorb nur eine Merkliste im Browser.
- Wo wird deployt (VPS, Cloud)? Das Docker-Image (ANF-010) ist die Grundlage dafür.
