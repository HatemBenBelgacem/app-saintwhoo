# ANF-002 – CRUD für alle Datensätze

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Baut auf [ANF-001](ANF-001-magazin-artikel-erfassen.md) auf

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (13.11.2026) |
| **Rolle** | Redakteur:in |

**Beschreibung**
Alle fachlichen Datensätze lassen sich vollständig verwalten: **erstellen** (Create), **anzeigen** (Read), **bearbeiten** (Update) und **löschen** (Delete).
Die Anforderung besteht aus zwei Teilen:

- **Teil A – CRUD-Standard:** Regeln, die für jeden Datensatz-Typ gelten, auch für zukünftige.
- **Teil B – Artikel:** Anwendung des Standards auf Magazin-Artikel. Das ist heute der einzige fachliche Datensatz-Typ.

Technische Tabellen wie `app_meta` oder `_sqlx_migrations` gehören nicht dazu.

**User Story**
Als Redakteur:in möchte ich alle Datensätze auflisten, ansehen, ändern und löschen können, damit ich meine Inhalte vollständig selbst pflegen kann.

---

## Teil A – CRUD-Standard (gilt für alle Datensätze)

### A1. Seiten und Adressen

Für jeden Datensatz-Typ `<typ>` (Mehrzahl, deutsch, z. B. `artikel`) gibt es:

| Operation | Seite | Adresse |
|---|---|---|
| Liste (Read) | Übersicht aller Datensätze | `/<typ>` |
| Detail (Read) | Ein Datensatz mit allen Feldern | `/<typ>/<id>` |
| Erstellen (Create) | Leeres Formular | `/<typ>/neu` |
| Bearbeiten (Update) | Formular mit den aktuellen Werten | `/<typ>/<id>/bearbeiten` |
| Löschen (Delete) | Knopf mit Bestätigungsdialog auf Detail- und Listenseite | – |

### A2. Regeln

| Nr. | Regel |
|---|---|
| R-1 | **Gleiche Prüfregeln:** Erstellen und Bearbeiten verwenden dieselben Regeln und Fehlertexte. Der Server prüft immer, die Datenbank sichert zusätzlich ab. |
| R-2 | **Liste:** neueste zuerst. Ab mehr als 20 Einträgen wird die Liste auf Seiten mit je 20 Einträgen verteilt. |
| R-3 | **Leere Liste:** Es erscheint ein Hinweis mit einem Knopf zum Erstellen. |
| R-4 | **Nicht gefunden:** Eine Adresse mit unbekannter oder gelöschter ID zeigt die 404-Seite. |
| R-5 | **Löschen nur mit Bestätigung:** Erst nach einem Klick auf „Löschen“ im Dialog wird gelöscht. „Abbrechen“ ändert nichts. |
| R-6 | **Endgültiges Löschen:** Ein gelöschter Datensatz ist dauerhaft entfernt (kein Papierkorb). |
| R-7 | **Rückmeldung:** Nach Erstellen, Speichern und Löschen erscheint eine Erfolgsmeldung. Bei technischen Fehlern erscheint eine verständliche Fehlermeldung, und die Eingaben bleiben erhalten. |
| R-8 | **Zeitstempel:** Jeder Datensatz hat `created_at` (Erstellung) und `updated_at` (letzte Änderung). Beide werden vom System gesetzt und auf der Detailseite angezeigt. |
| R-9 | **Navigation:** Jede CRUD-Seite hat einen Weg zurück zur Liste. Die Liste ist in der Navigationsleiste verlinkt. |
| R-10 | **Neue Datensatz-Typen** bekommen alle Operationen aus A1 und halten sich an die Regeln R-1 bis R-9. |

---

## Teil B – Artikel

### B1. Felder

Wie in [ANF-001](ANF-001-magazin-artikel-erfassen.md#felder) (Titel, Text, Veröffentlichungsdatum, Status), dazu **neu**:

| Feld | Typ | Sichtbar | Regeln |
|---|---|---|---|
| Zuletzt geändert (`updated_at`) | Zeitpunkt | Detailseite | Wird beim Erstellen und bei jedem Speichern vom System gesetzt. |

### B2. Liste – `/artikel`

- Spalten: **Titel** (Link zur Detailseite), **Status** (als Badge: *Entwurf* grau, *Veröffentlicht* grün), **Veröffentlichungsdatum** (leer = „–“), **Zuletzt geändert**.
- Pro Zeile: Aktionen **Bearbeiten** und **Löschen**.
- Oben: Knopf **„Neuer Artikel“**.
- Filter nach Status: *Alle* (Standard), *Entwurf*, *Veröffentlicht*.
- Auf dem Handy werden die Zeilen als Karten statt als Tabelle angezeigt.

### B3. Detail – `/artikel/<id>`

- Zeigt Titel, Status, Veröffentlichungsdatum, den ganzen Text (mit Zeilenumbrüchen), Erstellt am und Zuletzt geändert.
- Knöpfe: **Bearbeiten**, **Löschen**, **Zurück zur Liste**.

### B4. Bearbeiten – `/artikel/<id>/bearbeiten`

- Gleiches Formular wie in ANF-001, aber mit den gespeicherten Werten vorausgefüllt.
- Die Prüfregeln und Fehlertexte sind dieselben wie in ANF-001 (AK-3 bis AK-6).
- Nach dem Speichern: Weiterleitung zur Detailseite und Meldung „Änderungen gespeichert.“
- „Abbrechen“ führt ohne Speichern zur Detailseite zurück.

### B5. Löschen

- Dialog: **„Artikel „<Titel>“ wirklich löschen? Das kann nicht rückgängig gemacht werden.“** mit den Knöpfen *Abbrechen* und *Löschen* (rot).
- Nach dem Löschen: Weiterleitung zur Liste und Meldung „Artikel gelöscht.“

### B6. Erstellen (Ergänzung zu ANF-001)

- Die Erfolgsmeldung nach dem Erstellen enthält einen Link zum neuen Artikel.

---

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | 3 gespeicherte Artikel | `/artikel` geöffnet wird | sind alle 3 aufgelistet, der zuletzt erstellte zuerst |
| AK-2 | keine Artikel | `/artikel` geöffnet wird | erscheint „Noch keine Artikel vorhanden.“ mit Knopf „Neuer Artikel“ |
| AK-3 | 25 Artikel | `/artikel` geöffnet wird | sind 20 Artikel auf Seite 1 und 5 auf Seite 2 |
| AK-4 | Artikel mit beiden Status | der Filter „Entwurf“ gewählt wird | sind nur Entwürfe sichtbar |
| AK-5 | ein gespeicherter Artikel | seine Detailseite geöffnet wird | sind alle Felder inkl. Zeitstempel sichtbar, die Zeilenumbrüche im Text bleiben erhalten |
| AK-6 | eine ID, die es nicht gibt | `/artikel/<id>` oder `/artikel/<id>/bearbeiten` geöffnet wird | erscheint die 404-Seite |
| AK-7 | ein gespeicherter Artikel | die Bearbeiten-Seite geöffnet wird | sind alle Felder mit den gespeicherten Werten vorausgefüllt |
| AK-8 | ein Artikel wird bearbeitet | Titel und Status geändert und gespeichert werden | sind die neuen Werte gespeichert, `updated_at` ist neuer als vorher, `created_at` ist unverändert |
| AK-9 | ein Artikel wird bearbeitet | ein ungültiger Wert eingegeben wird (z. B. leerer Titel) | erscheint derselbe Fehlertext wie in ANF-001, und der gespeicherte Artikel bleibt unverändert |
| AK-10 | der Löschdialog ist offen | „Abbrechen“ geklickt wird | bleibt der Artikel bestehen |
| AK-11 | der Löschdialog ist offen | „Löschen“ geklickt wird | ist der Artikel aus der Datenbank entfernt, die Liste wird angezeigt mit „Artikel gelöscht.“ |
| AK-12 | ein Artikel wurde gerade gelöscht oder bearbeitet (z. B. in einem anderen Tab) | im ersten Tab gespeichert oder gelöscht wird | erscheint „Dieser Artikel existiert nicht mehr.“, und es wird nichts neu angelegt |
| AK-13 | die gleichen Regeln | Update oder Delete direkt an den Server geschickt werden (ohne Formular) | prüft der Server alle Regeln und antwortet bei unbekannter ID mit einem Fehler |
| AK-14 | ein neuer Artikel wurde erstellt | die Erfolgsmeldung erscheint | enthält sie einen Link, der zur Detailseite des Artikels führt |

## Annahmen (bitte bestätigen)

- **Löschen ist endgültig** (Hard Delete). Es gibt keinen Papierkorb und kein Wiederherstellen.
- **Gleichzeitiges Bearbeiten:** Wer zuletzt speichert, gewinnt („last write wins“). Es gibt keine Warnung, wenn jemand anderes inzwischen gespeichert hat.
- **Alle Redakteur:innen dürfen alles:** jeden Artikel sehen, bearbeiten und löschen. Berechtigungen kommen erst mit einem Login.
- **Veröffentlichte Artikel** dürfen bearbeitet und auch wieder auf *Entwurf* gesetzt werden.
- Seitenlänge der Liste: **20 Einträge**.

## Nicht Teil von ANF-002

- Öffentliche Leseansicht für Besucher:innen (Magazin-Frontend)
- Volltextsuche, Sortieren nach Spalten
- Versionsverlauf / Änderungsprotokoll, Papierkorb
- Mehrere Datensätze auf einmal bearbeiten oder löschen
- Anmeldung und Berechtigungen

## Umsetzung

| Teil | Ort |
|---|---|
| Liste `/artikel` | [src/pages/article_list.rs](../../src/pages/article_list.rs) |
| Detail `/artikel/<id>` | [src/pages/article_detail.rs](../../src/pages/article_detail.rs) |
| Bearbeiten `/artikel/<id>/bearbeiten` | [src/pages/article_edit.rs](../../src/pages/article_edit.rs) |
| Erstellen `/artikel/neu` (mit Link, B6) | [src/pages/article_new.rs](../../src/pages/article_new.rs) |
| Formularfelder, Badge, Lösch-Dialog, Meldungen | [src/components/article.rs](../../src/components/article.rs) |
| Server-Funktionen | [src/articles/api.rs](../../src/articles/api.rs): `list_articles`, `get_article`, `update_article`, `delete_article` |
| SQL | [src/articles/repo.rs](../../src/articles/repo.rs) |
| Migration `updated_at` | [migrations/20261113090000_articles_updated_at.sql](../../migrations/20261113090000_articles_updated_at.sql) |

**Technische Entscheidungen**

- **Weiterleitung nach Speichern/Löschen** über `leptos_axum::redirect`. Mit JavaScript navigiert der Browser selbst, ohne JavaScript antwortet der Server mit `302`. Die Erfolgsmeldung steht in der Adresse (`?meldung=gespeichert` bzw. `?meldung=geloescht`).
- **Lösch-Dialog** mit DaisyUI `modal-toggle` (Checkbox). Er funktioniert auch ohne JavaScript.
- **Detail- und Bearbeiten-Seite** werden mit `SsrMode::Async` gerendert. So liefert eine unbekannte ID echt den HTTP-Status `404`.
- **Filter und Seiten** sind normale Links (`?status=draft&seite=2`), also teilbar und ohne JavaScript nutzbar.
- **Zeitstempel** werden in UTC angezeigt, damit Server und Browser dieselbe Ausgabe erzeugen.
- **„Nicht gefunden“** kommt als Ergebnis `NotFound` zurück (keine Exception). Die Oberfläche zeigt dann „Dieser Artikel existiert nicht mehr.“

**Tests** (`cargo test --features ssr`, Datei [tests/anf_002.rs](../../tests/anf_002.rs)) und HTTP-Prüfung am laufenden Server:

| AK | DB-Test | HTTP geprüft | Bemerkung |
|---|---|---|---|
| AK-1 | ✔ | ✔ | |
| AK-2 | ✔ | ✔ | |
| AK-3 | ✔ | – | inkl. zu großer Seitenzahl → letzte Seite |
| AK-4 | ✔ | ✔ | |
| AK-5 | ✔ | ✔ | |
| AK-6 | ✔ | ✔ | Status 404 auch für `/artikel/abc` |
| AK-7 | – | ✔ | Werte stehen im Server-HTML |
| AK-8 | ✔ | ✔ | mit und ohne JavaScript |
| AK-9 | ✔ | ✔ | |
| AK-10 | – | – | **manuell** im Browser prüfen |
| AK-11 | ✔ | ✔ | |
| AK-12 | ✔ | ✔ | |
| AK-13 | ✔ | ✔ | |
| AK-14 | – | – | **manuell** im Browser prüfen |
