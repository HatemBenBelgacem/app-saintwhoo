# Anforderungen

> Stand: 12.11.2026 · Status: **Entwurf** – die fachlichen Anforderungen sind noch offen und müssen ergänzt werden.

## 1. Ziel

_Was soll Saintwhoo lösen? Für wen?_ (1–3 Sätze, noch offen)

## 2. Zielgruppe / Rollen

| Rolle | Beschreibung |
|---|---|
| Redakteur:in | Erfasst und veröffentlicht Magazin-Artikel (ANF-001) und Events (ANF-006) |
| Shop-Verwalter:in | Erfasst und pflegt Produkte für den Shop (ANF-005) |
| Besucher:in | Liest veröffentlichte Artikel auf der Startseite (ANF-003), sieht Produkte im Shop (ANF-005) und Events in der Eventliste (ANF-006) |
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
| [ANF-006](features/ANF-006-eventliste.md) | Eventliste: Events mit Titel, Beschreibung und Bild | M | offen |

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
| NFA-07 | Immer helle Ansicht (DaisyUI-Theme `light`), auch bei Dunkelmodus im System | M |
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
- ANF-006: Brauchen Events ein Datum, eine Uhrzeit und einen Ort? Ist die Beschreibung Pflicht?
- Braucht es Benutzerkonten / Login?
- Wo wird deployt (Docker, VPS, Cloud)?
