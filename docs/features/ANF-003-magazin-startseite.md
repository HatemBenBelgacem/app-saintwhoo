# ANF-003 – Magazin-Startseite

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Baut auf [ANF-001](ANF-001-magazin-artikel-erfassen.md) und [ANF-002](ANF-002-crud-datensaetze.md) auf

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (13.11.2026) |
| **Rolle** | Besucher:in |

**Beschreibung**
Wer die Website öffnet, sieht auf der Startseite die veröffentlichten Artikel im Stil eines Lifestyle-Magazins: ein großer Aufmacher, darunter ein Raster aus Artikel-Karten. Mit einem Klick auf einen Artikel öffnet sich eine Leseansicht.

Die Gestaltung lehnt sich an bekannte Magazin-Websites an (z. B. gq-magazin.de): großer Aufmacher, Serifen-Überschriften, kleine Großbuchstaben-Zeilen über den Titeln, viel Weißraum. Logo, Schriften, Farben und Inhalte fremder Marken werden **nicht** übernommen.

**User Story**
Als Besucher:in möchte ich auf der Startseite die neuesten Artikel attraktiv präsentiert sehen, damit ich schnell etwas Interessantes zum Lesen finde.

## Sichtbarkeit

Ein Artikel ist **öffentlich**, wenn
- sein Status *Veröffentlicht* ist **und**
- sein Veröffentlichungsdatum heute oder in der Vergangenheit liegt.

Entwürfe und Artikel mit Datum in der Zukunft (geplant) sind für Besucher:innen nicht sichtbar, weder auf der Startseite noch über die direkte Adresse.

## Aufbau der Startseite `/`

| Bereich | Inhalt |
|---|---|
| **Kopfzeile** | Name „Saintwhoo“ groß in der Mitte, Link „Redaktion“ zur Artikelverwaltung |
| **Aufmacher** | Der neueste öffentliche Artikel: große Bildfläche, Zeile „Titelgeschichte“, großer Titel, Anriss, Datum und Lesezeit |
| **Neueste Artikel** | Die nächsten bis zu 6 Artikel als Karten: Bildfläche, Datum, Titel, Anriss, Lesezeit |
| **Weitere Artikel** | Die nächsten bis zu 6 Artikel als kompakte nummerierte Liste mit kleinem Bild |
| **Fußzeile** | Copyright, Link zur Redaktion und zum Systemstatus |

- **Reihenfolge:** Veröffentlichungsdatum absteigend, bei gleichem Datum der zuletzt erstellte zuerst.
- **Anriss:** Anfang des Textes, höchstens 160 Zeichen, an einer Wortgrenze gekürzt, mit „…“.
- **Lesezeit:** Wörter ÷ 200, aufgerundet, mindestens „1 Min. Lesezeit“.
- **Bildfläche:** Solange Artikel keine Bilder haben, wird eine farbige Fläche mit dem Anfangsbuchstaben des Titels gezeigt. Die Farbe hängt vom Artikel ab und bleibt immer gleich.
- **Layout:** Handy 1 Spalte, Tablet 2 Spalten, Desktop 3 Spalten. Der Aufmacher steht auf dem Desktop neben dem Bild, auf dem Handy darunter.

## Leseansicht `/beitrag/<id>`

- Datum und Lesezeit, großer Titel, Bildfläche, Text in Absätzen (getrennt durch Leerzeilen) mit großem Anfangsbuchstaben (Initiale).
- Link zurück zur Startseite.
- Nicht öffentliche oder unbekannte Artikel → 404-Seite.

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | ein Entwurf, ein veröffentlichter Artikel mit Datum heute, einer mit Datum morgen | die Startseite geöffnet wird | ist nur der Artikel mit Datum heute sichtbar |
| AK-2 | mehrere öffentliche Artikel | die Startseite geöffnet wird | ist der mit dem neuesten Veröffentlichungsdatum der Aufmacher |
| AK-3 | 15 öffentliche Artikel | die Startseite geöffnet wird | gibt es 1 Aufmacher, 6 Karten und 6 Listeneinträge (13 Artikel) |
| AK-4 | ein Artikel mit 500 Zeichen Text | er als Karte erscheint | ist der Anriss höchstens 160 Zeichen lang, endet an einer Wortgrenze und mit „…“ |
| AK-5 | ein Text mit 450 Wörtern | Lesezeit angezeigt wird | steht „3 Min. Lesezeit“ |
| AK-6 | keine öffentlichen Artikel | die Startseite geöffnet wird | erscheint „Noch keine Artikel veröffentlicht.“ |
| AK-7 | ein öffentlicher Artikel | auf Titel oder Bild geklickt wird | öffnet sich `/beitrag/<id>` mit dem ganzen Text in Absätzen |
| AK-8 | ein Entwurf oder geplanter Artikel | `/beitrag/<id>` direkt aufgerufen wird | erscheint die 404-Seite (Status 404) |
| AK-9 | die Startseite | auf Handy, Tablet und Desktop angesehen | passt sich das Raster an (1 / 2 / 3 Spalten), nichts ragt seitlich heraus |

## Annahmen (bitte bestätigen)

- „Heute“ richtet sich nach dem Datum des Datenbankservers.
- Die Startseite zeigt höchstens 13 Artikel. Ein „Mehr laden“ oder Archiv kommt später.
- Schriften: Systemschriften (Serif für Titel). Es werden keine externen Schriften geladen (Datenschutz, Ladezeit).
- Die DB-Statusprüfung zieht von der Startseite nach `/status`.

## Nicht Teil von ANF-003

- Bilder hochladen (bis dahin farbige Platzhalter)
- Kategorien / Rubriken, Autor:innen, „Meistgelesen“
- Suche, Archiv, Newsletter, Teilen-Knöpfe

## Umsetzung

| Teil | Ort |
|---|---|
| Startseite `/` | [src/pages/home.rs](../../src/pages/home.rs) (Aufmacher, Karten, Liste) |
| Leseansicht `/beitrag/<id>` | [src/pages/beitrag.rs](../../src/pages/beitrag.rs) (`SsrMode::Async` für echtes 404) |
| Bildfläche, Kicker, Abschnittstitel | [src/components/magazine.rs](../../src/components/magazine.rs) |
| Kopf- und Fußzeile | [src/components/navbar.rs](../../src/components/navbar.rs), [src/components/footer.rs](../../src/components/footer.rs) |
| Anriss, Lesezeit, Absätze, langes Datum | [src/articles/model.rs](../../src/articles/model.rs) (`teaser`, `reading_minutes`, `paragraphs`, `format_date_long`) |
| Abfragen „öffentlich“ | [src/articles/repo.rs](../../src/articles/repo.rs) (`front_page`, `find_published`) |
| Systemstatus `/status` | [src/pages/status.rs](../../src/pages/status.rs) |

- Titel nutzen `hyphens-auto`. Mit `lang="de"` trennt der Browser deutsche Wörter korrekt (z. B. „Webent-wicklung“).
- Die Seite wird immer in heller Ansicht angezeigt (siehe NFA-07).

**Tests** (`cargo test --features ssr`): Unit-Tests in `model.rs`, DB-Tests in [tests/anf_003.rs](../../tests/anf_003.rs). Zusätzlich wurde mit Screenshots aus Headless-Chrome geprüft: Desktop 1440 px, Handy 390 px, hell und dunkel.

| AK | Test |
|---|---|
| AK-1 | DB-Test (Entwurf, heute, morgen) + HTTP |
| AK-2 | DB-Test (inkl. gleiches Datum) |
| AK-3 | DB-Test (15 → 13) |
| AK-4 | Unit + DB-Test |
| AK-5 | Unit + DB-Test |
| AK-6 | DB-Test |
| AK-7 | DB-Test + HTTP (`/beitrag/4` → 200) |
| AK-8 | DB-Test + HTTP (`/beitrag/5` Entwurf → 404) |
| AK-9 | Screenshots 390 px / 1440 px |
