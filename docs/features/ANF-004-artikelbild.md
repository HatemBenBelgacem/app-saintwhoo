# ANF-004 – Artikelbild mit Bildnachweis

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Ergänzt [ANF-001](ANF-001-magazin-artikel-erfassen.md), [ANF-002](ANF-002-crud-datensaetze.md) und [ANF-003](ANF-003-magazin-startseite.md)

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (13.11.2026) |
| **Rolle** | Redakteur:in, Besucher:in |

**Beschreibung**
Ein Artikel kann ein Bild haben. Das Bild wird über seine Adresse (URL) angegeben. Zu jedem Bild gehört ein **Bildnachweis** (Fotograf:in, Lizenz, Quelle), damit Urheberrecht und Lizenzen eingehalten werden. Hat ein Artikel ein Bild, ersetzt es auf der Startseite und in der Leseansicht die farbige Platzhalter-Fläche.

**User Story**
Als Redakteur:in möchte ich einem Artikel ein Bild mit Bildnachweis geben, damit der Artikel auf der Startseite ansprechend aussieht und die Rechte am Bild korrekt genannt werden.

## Felder (neu, beim Erstellen und Bearbeiten)

| Feld | Typ | Pflicht | Regeln |
|---|---|---|---|
| Bild-Adresse | Text, einzeilig | nein | Leer = kein Bild. Sonst beginnt sie mit `https://` (externes Bild) oder `/` (Bild im Projekt, z. B. `/images/artikel/name.jpg`). Keine Leerzeichen, höchstens 500 Zeichen. |
| Bildnachweis | Text, einzeilig | nur wenn eine Bild-Adresse gesetzt ist | Höchstens 200 Zeichen. Beispiel: „Foto: Name / Wikimedia Commons (CC BY 2.0)“ |

Ohne Bild-Adresse wird kein Bildnachweis gespeichert.

## Anzeige

| Ort | Darstellung |
|---|---|
| Startseite: Aufmacher und Karten | Bild füllt die Bildfläche (zugeschnitten). Der Bildnachweis steht klein unten rechts im Bild. |
| Startseite: Liste „Weitere Artikel“ | Kleines quadratisches Bild. Der Nachweis erscheint als Tooltip. |
| Leseansicht | Großes Bild. Der Nachweis steht als Bildunterschrift darunter. |
| Detailseite (Redaktion) | Bild mit Bildnachweis über dem Text |
| Ohne Bild | Wie bisher die farbige Fläche mit Anfangsbuchstaben |

Das Bild hat als Alternativtext den Titel des Artikels (Barrierefreiheit). Bilder unterhalb des sichtbaren Bereichs werden erst beim Scrollen geladen (`loading="lazy"`).

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | Bild-Adresse und Bildnachweis | gespeichert wird | sind beide gespeichert und das Bild erscheint auf Startseite, Leseansicht und Detailseite |
| AK-2 | eine Bild-Adresse ohne Bildnachweis | gespeichert wird | erscheint „Bitte einen Bildnachweis angeben.“ und nichts wird gespeichert |
| AK-3 | eine Bild-Adresse wie `ftp://…`, `bild.jpg` oder mit Leerzeichen | gespeichert wird | erscheint „Bitte eine gültige Bild-Adresse angeben (https://… oder /…).“ |
| AK-4 | ein Bildnachweis mit mehr als 200 Zeichen | gespeichert wird | erscheint „Der Bildnachweis darf höchstens 200 Zeichen lang sein.“ |
| AK-5 | keine Bild-Adresse, aber ein Bildnachweis | gespeichert wird | wird der Artikel ohne Bild und ohne Nachweis gespeichert |
| AK-6 | ein Artikel mit Bild | bearbeitet und die Bild-Adresse geleert wird | hat der Artikel danach kein Bild mehr (Platzhalter erscheint) |
| AK-7 | ein Artikel mit Bild | die Startseite angezeigt wird | ist der Bildnachweis sichtbar (bei Aufmacher und Karten im Bild) |
| AK-8 | ungültige Bilddaten | direkt an die Datenbank geschickt werden | lehnt die Datenbank sie ab (CHECK-Constraints) |

## Annahmen (bitte bestätigen)

- Bilder werden **per Adresse** angegeben. Ein Hochladen von Dateien kommt in einer späteren Anforderung.
- Testbilder stammen von **Wikimedia Commons** (freie Lizenzen) und liegen lokal unter `public/images/artikel/`. Herkunft und Lizenz stehen in `public/images/artikel/CREDITS.json`.
- Externe Bilder (`https://…`) werden direkt vom fremden Server geladen. Die Redaktion ist für die Rechte verantwortlich.

## Nicht Teil von ANF-004

- Bilder hochladen, zuschneiden oder verkleinern
- Mehrere Bilder / Galerien pro Artikel
- Eigenes Feld für Bildbeschreibung (Alt-Text), bis dahin wird der Titel verwendet

## Umsetzung

| Teil | Ort |
|---|---|
| Migration (Spalten + 4 CHECK-Constraints) | [migrations/20261113100000_articles_image.sql](../../migrations/20261113100000_articles_image.sql) |
| Regeln + Fehlertexte | [src/articles/model.rs](../../src/articles/model.rs) (`is_valid_image_url`, `ArticleImage`, `validate`) |
| Formularfelder | [src/components/article.rs](../../src/components/article.rs) (`ArticleFields`) |
| Bildanzeige / Platzhalter | [src/components/magazine.rs](../../src/components/magazine.rs) (`CoverImage`) |
| Testbilder + Lizenzen | [public/images/artikel/](../../public/images/artikel/), [CREDITS.json](../../public/images/artikel/CREDITS.json) |

- Die Server-Funktionen nehmen `image_url` und `image_credit` als optionale Felder an. Anfragen ohne Bildfelder funktionieren also weiter.
- Das Bild ist auf das Gesicht ausgerichtet zugeschnitten (`object-[50%_30%]`). Der Aufmacher und das Bild der Leseansicht werden sofort geladen, alle anderen erst beim Scrollen.

**Tests:** Unit-Tests in `model.rs` (AK-1 bis AK-5) und DB-Tests in [tests/anf_004.rs](../../tests/anf_004.rs) (AK-1, AK-2, AK-3, AK-5, AK-6, AK-8). AK-7 wurde per Screenshot geprüft.

## Testdaten

Fünf Artikel über DJs mit Fotos von Wikimedia Commons:

| Artikel | Foto | Lizenz |
|---|---|---|
| Peggy Gou | Davide Guidone | Public Domain |
| Carl Cox | Sergey Kozak | CC BY 2.0 |
| Charlotte de Witte | Alan Overbeek | CC BY-SA 4.0 |
| Richie Hawtin | Raminta Malinauskaite | CC BY-SA 3.0 |
| Paul Kalkbrenner | pitpony.photography | CC BY-SA 3.0 |

Die Fotos wurden auf höchstens 1600 px verkleinert. Bei CC BY-SA gilt die gleiche Lizenz auch für die verkleinerte Fassung.
