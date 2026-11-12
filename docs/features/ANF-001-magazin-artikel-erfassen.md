# ANF-001 – Magazin-Artikel erfassen

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md)

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (12.11.2026) |
| **Rolle** | Redakteur:in |

**Beschreibung**
Ein:e Redakteur:in kann einen neuen Magazin-Artikel erfassen. Ein Artikel besteht aus Titel, Text, Veröffentlichungsdatum und Status. Der Artikel wird dauerhaft in der Datenbank gespeichert.

**User Story**
Als Redakteur:in möchte ich einen Magazin-Artikel mit Titel, Text, Veröffentlichungsdatum und Status erfassen, damit ich Inhalte vorbereiten und veröffentlichen kann.

## Felder

| Feld | Typ | Pflicht | Regeln |
|---|---|---|---|
| Titel | Text, einzeilig | ja | 1–200 Zeichen. Leerzeichen am Anfang und Ende werden entfernt. Nur Leerzeichen gilt als leer. |
| Text | Text, mehrzeilig | ja | Mindestens 1 Zeichen (ohne Leerzeichen). Zeilenumbrüche bleiben erhalten. |
| Veröffentlichungsdatum | Datum | nur bei Status *Veröffentlicht* | Gültiges Kalenderdatum. Bei *Entwurf* optional. |
| Status | Auswahl | ja | Genau einer der Werte **Entwurf** oder **Veröffentlicht**. Standardwert: **Entwurf**. |

Zusätzlich speichert das System automatisch eine eindeutige ID und den Erstellungszeitpunkt (nicht sichtbar im Formular).

## Ablauf

1. Die Redakteurin oder der Redakteur öffnet die Seite „Neuer Artikel“.
2. Das Formular zeigt die vier Felder. Der Status ist auf *Entwurf* vorbelegt.
3. Die Felder werden ausgefüllt, dann wird auf „Speichern“ geklickt.
4. Das System prüft die Eingaben (siehe Regeln).
   - **Gültig:** Der Artikel wird gespeichert. Es erscheint eine Erfolgsmeldung.
   - **Ungültig:** Nichts wird gespeichert. Bei jedem fehlerhaften Feld steht eine verständliche Fehlermeldung. Die bisherigen Eingaben bleiben erhalten.

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | Titel, Text und Status *Entwurf*, kein Datum | gespeichert wird | wird der Artikel als Entwurf ohne Datum gespeichert |
| AK-2 | Titel, Text, Status *Veröffentlicht* und ein Datum | gespeichert wird | wird der Artikel als veröffentlicht mit diesem Datum gespeichert |
| AK-3 | Status *Veröffentlicht*, aber kein Datum | gespeichert wird | erscheint „Bitte ein Veröffentlichungsdatum angeben.“ und nichts wird gespeichert |
| AK-4 | Titel leer oder nur Leerzeichen | gespeichert wird | erscheint „Bitte einen Titel eingeben.“ und nichts wird gespeichert |
| AK-5 | Titel mit mehr als 200 Zeichen | gespeichert wird | erscheint „Der Titel darf höchstens 200 Zeichen lang sein.“ |
| AK-6 | Text leer oder nur Leerzeichen | gespeichert wird | erscheint „Bitte einen Text eingeben.“ und nichts wird gespeichert |
| AK-7 | ein Validierungsfehler | die Fehlermeldung erscheint | sind alle bisherigen Eingaben noch im Formular |
| AK-8 | ein gespeicherter Artikel | die Datenbank abgefragt wird | sind Titel, Text, Datum und Status unverändert gespeichert (inkl. Umlaute und Zeilenumbrüche) |
| AK-9 | die gleichen Regeln | eine Anfrage direkt an den Server geht (ohne Formular) | prüft auch der Server alle Regeln und lehnt ungültige Daten ab |

## Annahmen (bitte bestätigen)

- Das Veröffentlichungsdatum ist ein **Datum ohne Uhrzeit**.
- Ein Datum **in der Zukunft** ist erlaubt (geplante Veröffentlichung).
- Der Text ist **Klartext**. Formatierung (z. B. Markdown oder Rich-Text) ist nicht Teil von ANF-001.

## Nicht Teil von ANF-001

- Artikel anzeigen, auflisten, bearbeiten oder löschen → siehe [ANF-002](ANF-002-crud-datensaetze.md)
- Bilder, Kategorien, Tags, Autor:in
- Anmeldung und Berechtigungen (wer Redakteur:in ist)

## Umsetzung

| Teil | Ort |
|---|---|
| Seite „Neuer Artikel“ | `/artikel/neu` → [src/pages/article_new.rs](../../src/pages/article_new.rs) |
| Regeln + Fehlertexte | [src/articles/model.rs](../../src/articles/model.rs) (`validate`, `msg`) |
| Server-Funktion | [src/articles/api.rs](../../src/articles/api.rs) (`create_article`) |
| Datenbank | [src/articles/repo.rs](../../src/articles/repo.rs), Tabelle `articles` ([Migration](../../migrations/20261112120000_create_articles.sql)) |

Die Regeln werden doppelt geprüft: im Server vor dem Speichern (`validate`) und als `CHECK`-Constraints in PostgreSQL.

**Tests** (`cargo test --features ssr`):

| AK | Unit-Test (`model.rs`) | Integrationstest mit DB (`tests/anf_001.rs`) |
|---|---|---|
| AK-1 | ✔ | ✔ |
| AK-2 | ✔ | ✔ |
| AK-3 | ✔ | ✔ |
| AK-4 | ✔ | ✔ |
| AK-5 | ✔ | ✔ (via AK-9) |
| AK-6 | ✔ | ✔ |
| AK-7 | – | manuell: Formular wird nur nach Erfolg geleert |
| AK-8 | – | ✔ (Umlaute, Emoji, Zeilenumbrüche) |
| AK-9 | – | ✔ (Server-Funktion + DB-Constraints) |
