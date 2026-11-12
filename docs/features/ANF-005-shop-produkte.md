# ANF-005 – Shop: Produkte erfassen und im Shop anzeigen

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Baut auf [ANF-002](ANF-002-crud-datensaetze.md) (CRUD-Standard) auf

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (12.11.2026) |
| **Rolle** | Shop-Verwalter:in, Besucher:in |

**Beschreibung**
Shop-Verwalter:innen können Shop-Artikel erfassen und pflegen. Besucher:innen sehen alle aktiven Shop-Artikel in einem öffentlichen Shop und können jeden Artikel im Detail ansehen.

Die Anforderung besteht aus drei Teilen:

- **Teil A – Produkte verwalten:** Erfassen, Anzeigen, Bearbeiten und Löschen nach dem CRUD-Standard aus ANF-002.
- **Teil B – Shop-Ansicht:** Öffentliche Übersicht und Produktseite für Besucher:innen.
- **Teil C – Produktbild:** Foto mit Bildnachweis, gleiche Regeln wie beim Artikelbild ([ANF-004](ANF-004-artikelbild.md)). Ergänzt am 13.11.2026.

> **Begriff:** Im Projekt bezeichnet „Artikel“ bereits Magazin-Artikel (ANF-001 bis ANF-003, Adresse `/artikel`). Shop-Artikel heißen deshalb in Code, Datenbank und Adressen **Produkt** (`products`, `/produkte`). In der Oberfläche darf trotzdem „Artikel“ stehen, wenn das fachlich gewünscht ist (siehe Annahmen).

**User Stories**

- Als Shop-Verwalter:in möchte ich Produkte mit Name, Beschreibung, Preis und Lagerbestand erfassen, damit ich mein Sortiment im Shop anbieten kann.
- Als Besucher:in möchte ich alle verfügbaren Produkte übersichtlich sehen und Details zu einem Produkt aufrufen, damit ich mich über das Angebot informieren kann.

---

## Felder

| Feld | Typ | Pflicht | Regeln |
|---|---|---|---|
| Name | Text, einzeilig | ja | 1–120 Zeichen. Leerzeichen am Anfang und Ende werden entfernt. Nur Leerzeichen gilt als leer. |
| Artikelnummer | Text, einzeilig | ja | 1–40 Zeichen, nur Buchstaben, Ziffern und `-`. Wird in Großbuchstaben gespeichert. **Eindeutig** über alle Produkte. |
| Kurzbeschreibung | Text, einzeilig | nein | Höchstens 200 Zeichen. Erscheint in der Shop-Übersicht. |
| Beschreibung | Text, mehrzeilig | nein | Zeilenumbrüche bleiben erhalten. Leerzeilen trennen Absätze. |
| Preis | Betrag in CHF | ja | 0.00 bis 99 999.99, höchstens 2 Nachkommastellen. Eingabe mit Punkt oder Komma. Wird intern in Rappen (ganze Zahl) gespeichert. |
| Lagerbestand | ganze Zahl | ja | 0 oder größer. Standardwert: **0**. |
| Status | Auswahl | ja | **Entwurf** (nicht im Shop) oder **Aktiv** (im Shop sichtbar). Standardwert: **Entwurf**. |

Zusätzlich setzt das System automatisch: ID, `created_at`, `updated_at` (siehe ANF-002, R-8).

**Anzeige von Werten**

- **Preis:** immer mit 2 Nachkommastellen und Tausender-Apostroph, z. B. „CHF 1'249.00“.
- **Verfügbarkeit** (abgeleitet aus dem Lagerbestand):
  - Bestand 0 → Badge **„Ausverkauft“** (rot)
  - Bestand 1–5 → Badge **„Nur noch <n> verfügbar“** (gelb)
  - Bestand > 5 → Badge **„An Lager“** (grün)

---

## Teil A – Produkte verwalten

Es gelten alle Regeln R-1 bis R-10 aus [ANF-002 Teil A](ANF-002-crud-datensaetze.md#teil-a--crud-standard-gilt-für-alle-datensätze).

| Operation | Adresse |
|---|---|
| Liste | `/produkte` |
| Detail | `/produkte/<id>` |
| Erstellen | `/produkte/neu` |
| Bearbeiten | `/produkte/<id>/bearbeiten` |
| Löschen | Knopf mit Bestätigungsdialog auf Liste und Detailseite |

### A1. Liste – `/produkte`

- Spalten: **Name** (Link zur Detailseite), **Artikelnummer**, **Preis**, **Lagerbestand**, **Status** (Badge: *Entwurf* grau, *Aktiv* grün), **Zuletzt geändert**.
- Pro Zeile: **Bearbeiten** und **Löschen**.
- Oben: Knopf **„Neues Produkt“**.
- Filter nach Status: *Alle* (Standard), *Entwurf*, *Aktiv*.
- Auf dem Handy werden die Zeilen als Karten statt als Tabelle angezeigt.
- Leere Liste: „Noch keine Produkte vorhanden.“ mit Knopf „Neues Produkt“.

### A2. Erstellen und Bearbeiten

1. Die Shop-Verwalterin oder der Shop-Verwalter öffnet „Neues Produkt“ (bzw. „Bearbeiten“).
2. Das Formular zeigt alle Felder. Neu: Status *Entwurf*, Lagerbestand *0*. Bearbeiten: gespeicherte Werte.
3. Nach „Speichern“ prüft der Server alle Regeln.
   - **Gültig:** Das Produkt wird gespeichert. Weiterleitung zur Detailseite mit „Produkt gespeichert.“
   - **Ungültig:** Nichts wird gespeichert. Bei jedem fehlerhaften Feld steht die Fehlermeldung. Die Eingaben bleiben erhalten.

**Fehlertexte**

| Fall | Meldung |
|---|---|
| Name leer | „Bitte einen Namen eingeben.“ |
| Name zu lang | „Der Name darf höchstens 120 Zeichen lang sein.“ |
| Artikelnummer leer | „Bitte eine Artikelnummer eingeben.“ |
| Artikelnummer mit ungültigen Zeichen | „Die Artikelnummer darf nur Buchstaben, Ziffern und Bindestriche enthalten.“ |
| Artikelnummer zu lang | „Die Artikelnummer darf höchstens 40 Zeichen lang sein.“ |
| Artikelnummer schon vergeben | „Diese Artikelnummer ist bereits vergeben.“ |
| Kurzbeschreibung zu lang | „Die Kurzbeschreibung darf höchstens 200 Zeichen lang sein.“ |
| Preis leer oder keine Zahl | „Bitte einen gültigen Preis eingeben, z. B. 19.90.“ |
| Preis negativ, zu hoch oder mehr als 2 Nachkommastellen | „Der Preis muss zwischen 0.00 und 99'999.99 CHF liegen (höchstens 2 Nachkommastellen).“ |
| Lagerbestand negativ oder keine ganze Zahl | „Der Lagerbestand muss eine ganze Zahl ab 0 sein.“ |
| Status ungültig (nur ohne Formular möglich) | „Bitte einen gültigen Status wählen.“ |

### A3. Detail – `/produkte/<id>`

- Zeigt alle Felder, Verfügbarkeits-Badge, Erstellt am und Zuletzt geändert.
- Bei Status *Aktiv*: Link **„Im Shop ansehen“** zu `/shop/<id>`.
- Knöpfe: **Bearbeiten**, **Löschen**, **Zurück zur Liste**.

### A4. Löschen

- Dialog: **„Produkt „<Name>“ wirklich löschen? Das kann nicht rückgängig gemacht werden.“** mit *Abbrechen* und *Löschen* (rot).
- Nach dem Löschen: Weiterleitung zur Liste mit „Produkt gelöscht.“

---

## Teil B – Shop-Ansicht (öffentlich)

### B1. Shop-Übersicht – `/shop`

- Zeigt **nur Produkte mit Status *Aktiv***. Ausverkaufte Produkte werden auch gezeigt, aber mit Badge „Ausverkauft“.
- Jedes Produkt als **Karte**: Bildfläche, Name, Kurzbeschreibung (falls vorhanden), Preis, Verfügbarkeits-Badge.
- Ein Klick auf Karte oder Name öffnet die Produktseite.
- **Reihenfolge:** Standard „Neueste zuerst“ (`created_at` absteigend). Weitere Sortierung wählbar: *Preis aufsteigend*, *Preis absteigend*, *Name A–Z*.
- **Seiten:** je 24 Produkte, darunter Seitenzahlen. Sortierung und Seite stehen in der Adresse (`?sortierung=preis-auf&seite=2`) und funktionieren ohne JavaScript.
- **Layout:** Handy 1 Spalte, Tablet 2 Spalten, Desktop 3–4 Spalten.
- Leerer Shop: „Zurzeit sind keine Produkte verfügbar.“
- **Bildfläche:** Solange Produkte keine Bilder haben, eine farbige Fläche mit dem Anfangsbuchstaben des Namens (gleiches Verfahren wie in ANF-003).
- Der Shop ist in der Navigationsleiste verlinkt.

### B2. Produktseite – `/shop/<id>`

- Bildfläche, Name, Preis (groß), Verfügbarkeits-Badge, Artikelnummer, Beschreibung in Absätzen.
- Link **„Zurück zum Shop“**.
- Produkte mit Status *Entwurf* oder unbekannte IDs → **404-Seite** (HTTP-Status 404).

---

## Teil C – Produktbild

| Feld | Typ | Pflicht | Regeln |
|---|---|---|---|
| Bild-Adresse | Text, einzeilig | nein | Wie [ANF-004](ANF-004-artikelbild.md#felder-neu-beim-erstellen-und-bearbeiten): `https://…` oder `/…` (z. B. `/images/produkte/name.jpg`), keine Leerzeichen, höchstens 500 Zeichen |
| Bildnachweis | Text, einzeilig | nur mit Bild-Adresse | Höchstens 200 Zeichen, z. B. „Foto: Name / Wikimedia Commons (CC BY 2.0)“ |

Ohne Bild-Adresse wird kein Bildnachweis gespeichert. Die Fehlertexte sind dieselben wie in ANF-004.

| Ort | Darstellung |
|---|---|
| Shop-Karten | Bild füllt die quadratische Fläche (zugeschnitten), Bildnachweis klein unten rechts im Bild |
| Produktseite | Großes quadratisches Bild, Nachweis als Bildunterschrift |
| Detailseite (Verwaltung) | Bild mit Nachweis über den Angaben |
| Ohne Bild | Wie bisher die farbige Fläche mit Anfangsbuchstaben |

Alternativtext ist der Produktname.

---

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | Name, Artikelnummer, Preis „19.90“, Lagerbestand 10, Status *Entwurf* | gespeichert wird | ist das Produkt mit Preis 1990 Rappen gespeichert und erscheint in `/produkte`, aber nicht in `/shop` |
| AK-2 | Preis „19,90“ (mit Komma) | gespeichert wird | wird derselbe Preis wie mit „19.90“ gespeichert |
| AK-3 | Name leer oder nur Leerzeichen | gespeichert wird | erscheint „Bitte einen Namen eingeben.“ und nichts wird gespeichert |
| AK-4 | Preis „-5“, „abc“ oder „19.999“ | gespeichert wird | erscheint die passende Preis-Fehlermeldung und nichts wird gespeichert |
| AK-5 | Lagerbestand „-1“ oder „2.5“ | gespeichert wird | erscheint „Der Lagerbestand muss eine ganze Zahl ab 0 sein.“ |
| AK-6 | ein Produkt mit Artikelnummer „TS-001“ existiert | ein zweites Produkt mit „ts-001“ gespeichert wird | erscheint „Diese Artikelnummer ist bereits vergeben.“ und nichts wird gespeichert |
| AK-7 | ein Produkt wird bearbeitet | Preis und Status geändert und gespeichert werden | sind die neuen Werte gespeichert, `updated_at` ist neuer, `created_at` unverändert |
| AK-8 | der Löschdialog ist offen | „Löschen“ geklickt wird | ist das Produkt entfernt, die Liste zeigt „Produkt gelöscht.“ |
| AK-9 | 2 aktive Produkte und 1 Entwurf | `/shop` geöffnet wird | sind genau die 2 aktiven Produkte als Karten sichtbar |
| AK-10 | ein aktives Produkt mit Preis 124900 Rappen | es im Shop erscheint | steht „CHF 1'249.00“ |
| AK-11 | aktive Produkte mit Bestand 0, 3 und 20 | `/shop` geöffnet wird | zeigen sie „Ausverkauft“, „Nur noch 3 verfügbar“ und „An Lager“ |
| AK-12 | 3 aktive Produkte mit Preisen 30, 10, 20 CHF | Sortierung „Preis aufsteigend“ gewählt wird | ist die Reihenfolge 10, 20, 30 CHF |
| AK-13 | 30 aktive Produkte | `/shop` geöffnet wird | sind 24 Produkte auf Seite 1 und 6 auf Seite 2 |
| AK-14 | keine aktiven Produkte | `/shop` geöffnet wird | erscheint „Zurzeit sind keine Produkte verfügbar.“ |
| AK-15 | ein aktives Produkt | auf seine Karte geklickt wird | öffnet sich `/shop/<id>` mit allen öffentlichen Angaben und der Beschreibung in Absätzen |
| AK-16 | ein Produkt im Status *Entwurf* oder eine unbekannte ID | `/shop/<id>` aufgerufen wird | erscheint die 404-Seite (Status 404) |
| AK-17 | die Shop-Übersicht | auf Handy, Tablet und Desktop angesehen | passt sich das Raster an (1 / 2 / 3–4 Spalten), nichts ragt seitlich heraus |
| AK-18 | ungültige Werte | Erstellen oder Bearbeiten direkt an den Server geschickt wird (ohne Formular) | prüft der Server alle Regeln; die Datenbank verhindert zusätzlich negative Preise/Bestände und doppelte Artikelnummern |
| AK-19 | Bild-Adresse und Bildnachweis | gespeichert wird | sind beide gespeichert; das Bild erscheint im Shop, auf der Produktseite und in der Verwaltung. Wird die Adresse beim Bearbeiten geleert, erscheint wieder der Platzhalter |
| AK-20 | Bild-Adresse ohne Nachweis oder ungültige Adresse (`bild.jpg`) | gespeichert wird | erscheint der Fehlertext aus ANF-004 und nichts wird gespeichert |
| AK-21 | ungültige Bilddaten | direkt an die Datenbank geschickt werden | lehnt die Datenbank sie ab (CHECK-Constraints) |

## Annahmen (bitte bestätigen)

- **Währung:** nur CHF, Preise inkl. MWST. Keine Rabatte oder Aktionspreise.
- **Bezeichnung:** In der Verwaltung heißt es „Produkte“. Soll in der Oberfläche stattdessen „Artikel“ stehen?
- **Ausverkaufte Produkte** bleiben im Shop sichtbar (mit Badge), statt ausgeblendet zu werden.
- **Bilder** werden per Adresse angegeben (Teil C). Ohne Bild erscheint der farbige Platzhalter. Hochladen kommt mit einer eigenen Anforderung.
- **Rechte:** Wie in ANF-002 darf ohne Login jede:r Produkte verwalten. Die Verwaltung wird erst mit einer Login-Anforderung geschützt.
- **Löschen ist endgültig** (wie ANF-002). Später, mit Bestellungen, wird Löschen von bereits bestellten Produkten evtl. durch „Archivieren“ ersetzt.

## Nicht Teil von ANF-005

- Warenkorb, Bestellung, Bezahlung, Versand
- Kundenkonten, Login, Berechtigungen
- Bild-Upload, mehrere Bilder pro Produkt
- Kategorien, Filter nach Eigenschaften, Volltextsuche
- Varianten (Größe, Farbe), Rabatte, Gutscheine
- Automatisches Reduzieren des Lagerbestands (kommt mit Bestellungen)

## Umsetzung

| Teil | Ort |
|---|---|
| Migration `products` | [migrations/20261114090000_create_products.sql](../../migrations/20261114090000_create_products.sql) |
| Migration Bild (Teil C) | [migrations/20261114100000_products_image.sql](../../migrations/20261114100000_products_image.sql) |
| Testbilder + Lizenzen | [public/images/produkte/](../../public/images/produkte/), [CREDITS.json](../../public/images/produkte/CREDITS.json) |
| Modell, Prüfregeln, Preis-Parser und -Formatierung | [src/products/model.rs](../../src/products/model.rs) (`validate`, `parse_price`, `format_chf`, `Availability`, `ShopSort`) |
| Server-Funktionen | [src/products/api.rs](../../src/products/api.rs) |
| SQL | [src/products/repo.rs](../../src/products/repo.rs) (`shop`, `find_active` für den öffentlichen Teil) |
| Liste `/produkte` | [src/pages/product_list.rs](../../src/pages/product_list.rs) |
| Detail `/produkte/<id>` | [src/pages/product_detail.rs](../../src/pages/product_detail.rs) |
| Erstellen `/produkte/neu` | [src/pages/product_new.rs](../../src/pages/product_new.rs) |
| Bearbeiten `/produkte/<id>/bearbeiten` | [src/pages/product_edit.rs](../../src/pages/product_edit.rs) |
| Shop `/shop` | [src/pages/shop.rs](../../src/pages/shop.rs) |
| Produktseite `/shop/<id>` | [src/pages/shop_product.rs](../../src/pages/shop_product.rs) |
| Felder, Badges, Lösch-Dialog, Meldungen | [src/components/product.rs](../../src/components/product.rs) |
| Navigation (Shop links, „Produkte“ rechts, auf dem Handy im Menü „Redaktion“) | [src/components/navbar.rs](../../src/components/navbar.rs) |

**Technische Entscheidungen**

- **Preis in Rappen** (`INTEGER`). So gibt es keine Rundungsfehler. Die Eingabe akzeptiert Punkt, Komma und Tausender-Apostroph (`1'249.00`).
- **Artikelnummer** wird in Großbuchstaben gespeichert. „ts-001“ und „TS-001“ gelten also als gleich. Erlaubt sind nur die Buchstaben A–Z (ohne Umlaute).
- **„Bereits vergeben“** prüft der Server vor dem Speichern, damit alle Fehler auf einmal angezeigt werden. Wenn zwei Anfragen gleichzeitig kommen, greift zusätzlich der `UNIQUE`-Constraint. Auch dann erscheint dieselbe Feldmeldung.
- **Nach dem Erstellen** wird wie beim Bearbeiten zur Detailseite weitergeleitet (`?meldung=gespeichert`). Das funktioniert mit und ohne JavaScript (302).
- **Sortierung** über einen Bind-Parameter mit `ORDER BY CASE …`, weil sqlx nur feste SQL-Strings erlaubt. Bei gleichem Wert steht das neueste Produkt zuerst. „Name A–Z“ ignoriert Groß- und Kleinschreibung.
- **Bild:** gleiche Komponente wie im Magazin (`CoverImage`, für Produkte `ProductCover`). Die Regeln für das Bild kommen aus `articles::model` (`is_valid_image_url`, `ArticleImage`) und werden nicht doppelt geschrieben.

**Tests** (`cargo test --features ssr`): Unit-Tests in `src/products/model.rs`, DB-Tests in [tests/anf_005.rs](../../tests/anf_005.rs). Zusätzlich am laufenden Server per HTTP geprüft und mit Screenshots aus Headless-Chrome (1440 px und 390 px).

| AK | Test |
|---|---|
| AK-1 | DB-Test + HTTP (Formular-POST ohne JavaScript → 302 zur Detailseite) |
| AK-2 | Unit + DB-Test |
| AK-3 | Unit + DB-Test |
| AK-4 | Unit + DB-Test |
| AK-5 | Unit + DB-Test |
| AK-6 | DB-Test (auch beim Bearbeiten und zusammen mit anderen Fehlern) |
| AK-7 | DB-Test + HTTP (Formular vorausgefüllt) |
| AK-8 | DB-Test + HTTP (302 → `/produkte?meldung=geloescht`, erneut → `NotFound`) |
| AK-9 | DB-Test + HTTP |
| AK-10 | Unit + DB-Test + Screenshot |
| AK-11 | Unit + DB-Test + Screenshot |
| AK-12 | DB-Test (alle 4 Sortierungen) + HTTP |
| AK-13 | DB-Test (30 → 24 + 6, zu große Seitenzahl → letzte Seite) |
| AK-14 | DB-Test |
| AK-15 | DB-Test + Screenshot |
| AK-16 | DB-Test + HTTP (`/shop/<Entwurf>` und `/shop/999999` → 404) |
| AK-17 | Screenshots 390 px / 1440 px |
| AK-18 | DB-Test (Constraints) |
| AK-19 | Unit + DB-Test + Screenshot |
| AK-20 | Unit + DB-Test |
| AK-21 | DB-Test (Constraints) |

Nicht automatisch getestet: „Abbrechen“ im Löschdialog (reines CSS, wie bei ANF-002 AK-10). Das bitte im Browser prüfen.

## Testdaten

Fünf aktive Produkte mit Fotos von Wikimedia Commons, verkleinert auf höchstens 1600 px. Herkunft und Lizenz stehen in [CREDITS.json](../../public/images/produkte/CREDITS.json).

| Produkt | Foto | Lizenz |
|---|---|---|
| Leinen-Tasche | Tom Beatty | CC BY 2.0 |
| Keramik-Tasse | Liinbean | CC BY-SA 4.0 |
| Wollschal Engadin | smittenkittenorig | CC BY 2.0 |
| Designer-Leuchte Aurora | Jules Verne Times Two | CC BY-SA 4.0 |
| Notizbuch A5 | Pink Sherbet Photography | CC BY 2.0 |

Die Produkte selbst stehen nur in der lokalen Entwicklungs-DB, nicht in einer Migration.
