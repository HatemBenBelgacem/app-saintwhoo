# ANF-006 – Eventliste

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Baut auf [ANF-002](ANF-002-crud-datensaetze.md) (CRUD-Standard) und [ANF-004](ANF-004-artikelbild.md) (Bild mit Bildnachweis) auf

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (14.11.2026), erweitert nach Designvorschlag „Techno App“ |
| **Rolle** | Redakteur:in, Besucher:in |

**Beschreibung**
Die Redaktion kann Events mit Titel, Beschreibung und Bild erfassen und pflegen. Besucher:innen sehen alle Events in einer öffentlichen Eventliste und können jedes Event im Detail ansehen.

Die Anforderung besteht aus zwei Teilen:

- **Teil A – Events verwalten:** Erfassen, Anzeigen, Bearbeiten und Löschen nach dem CRUD-Standard aus ANF-002.
- **Teil B – Eventliste:** Öffentliche Übersicht und Eventseite für Besucher:innen.

> **Begriff:** In Code und Datenbank heißt der Datensatz `event` (Tabelle `events`). In der Oberfläche steht „Event“.

**User Stories**

- Als Redakteur:in möchte ich Events mit Titel, Beschreibung und Bild erfassen, damit ich sie auf der Website ankündigen kann.
- Als Besucher:in möchte ich alle Events übersichtlich sehen und Details zu einem Event aufrufen, damit ich weiß, was angeboten wird.

---

## Felder

| Feld | Typ | Pflicht | Regeln |
|---|---|---|---|
| Titel | Text, einzeilig | ja | 1–150 Zeichen. Leerzeichen am Anfang und Ende werden entfernt. Nur Leerzeichen gilt als leer. |
| Beschreibung | Text, mehrzeilig | ja | 1–10 000 Zeichen. Zeilenumbrüche bleiben erhalten. Leerzeilen trennen Absätze. Klartext (kein HTML/Markdown). |
| Bild-Adresse | Text, einzeilig | nein | Gleiche Regeln wie [ANF-004](ANF-004-artikelbild.md): leer = kein Bild, sonst beginnt sie mit `https://` oder `/`, keine Leerzeichen, höchstens 500 Zeichen. |
| Bildnachweis | Text, einzeilig | nur wenn eine Bild-Adresse gesetzt ist | Höchstens 200 Zeichen. Beispiel: „Foto: Name / Wikimedia Commons (CC BY 2.0)“. Ohne Bild-Adresse wird kein Nachweis gespeichert. |
| Datum | Datum | ja | Gültiges Datum (`YYYY-MM-DD`). *Erweiterung 14.11.2026* |
| Beginn | Uhrzeit | ja | `HH:MM`, Ortszeit Zürich. Datum + Beginn werden ohne Zeitzone gespeichert. *Erweiterung* |
| Ort | Text, einzeilig | ja | Club oder Location, 1–120 Zeichen. *Erweiterung* |
| Stadt | Text, einzeilig | ja | 1–80 Zeichen. *Erweiterung* |
| Lineup | Text, einzeilig | nein | Namen mit Komma getrennt, höchstens 300 Zeichen. *Erweiterung* |
| Ticket-Adresse | Text, einzeilig | nein | Beginnt mit `https://`, keine Leerzeichen, höchstens 500 Zeichen. *Erweiterung* |
| Tickets | Auswahl | ja | *Tickets verfügbar* (Standard), *Wenige Tickets*, *Ausverkauft*. *Erweiterung* |

Zusätzlich setzt das System automatisch: ID, `created_at`, `updated_at` (siehe ANF-002, R-8).

**Bildanzeige:** Hat ein Event ein Bild, füllt es die Bildfläche (zugeschnitten), Alternativtext ist der Titel. Ohne Bild erscheint die farbige Fläche mit dem Anfangsbuchstaben (gleiches Verfahren wie ANF-003/ANF-004).

---

## Teil A – Events verwalten

Es gelten alle Regeln R-1 bis R-10 aus [ANF-002 Teil A](ANF-002-crud-datensaetze.md#teil-a--crud-standard-gilt-für-alle-datensätze).

| Operation | Adresse |
|---|---|
| Liste | `/events` |
| Detail | `/events/<id>` |
| Erstellen | `/events/neu` |
| Bearbeiten | `/events/<id>/bearbeiten` |
| Löschen | Knopf mit Bestätigungsdialog auf Liste und Detailseite |

### A1. Liste – `/events`

- Spalten: **Bild** (kleines Vorschaubild oder Platzhalter), **Titel** (Link zur Detailseite), **Beginn** (mit Stadt), **Zuletzt geändert**.
- Pro Zeile: **Bearbeiten** und **Löschen**.
- Oben: Knopf **„Neues Event“**.
- Reihenfolge: zuletzt geändert zuerst.
- Auf dem Handy werden die Zeilen als Karten statt als Tabelle angezeigt.
- Leere Liste: „Noch keine Events vorhanden.“ mit Knopf „Neues Event“.

### A2. Erstellen und Bearbeiten

1. Die Redakteurin oder der Redakteur öffnet „Neues Event“ (bzw. „Bearbeiten“).
2. Das Formular zeigt alle Felder. Neu: alle Felder leer. Bearbeiten: gespeicherte Werte.
3. Nach „Speichern“ prüft der Server alle Regeln.
   - **Gültig:** Das Event wird gespeichert. Weiterleitung zur Detailseite mit „Event gespeichert.“
   - **Ungültig:** Nichts wird gespeichert. Bei jedem fehlerhaften Feld steht die Fehlermeldung. Die Eingaben bleiben erhalten.

**Fehlertexte**

| Fall | Meldung |
|---|---|
| Titel leer | „Bitte einen Titel eingeben.“ |
| Titel zu lang | „Der Titel darf höchstens 150 Zeichen lang sein.“ |
| Beschreibung leer | „Bitte eine Beschreibung eingeben.“ |
| Beschreibung zu lang | „Die Beschreibung darf höchstens 10 000 Zeichen lang sein.“ |
| Bild-Adresse ungültig | „Bitte eine gültige Bild-Adresse angeben (https://… oder /…).“ |
| Bild-Adresse ohne Bildnachweis | „Bitte einen Bildnachweis angeben.“ |
| Bildnachweis zu lang | „Der Bildnachweis darf höchstens 200 Zeichen lang sein.“ |
| Datum fehlt / ungültig | „Bitte ein Datum angeben.“ / „Bitte ein gültiges Datum angeben.“ |
| Beginn fehlt / ungültig | „Bitte eine Uhrzeit angeben.“ / „Bitte eine gültige Uhrzeit angeben (HH:MM).“ |
| Ort leer / zu lang | „Bitte einen Ort angeben.“ / „Der Ort darf höchstens 120 Zeichen lang sein.“ |
| Stadt leer / zu lang | „Bitte eine Stadt angeben.“ / „Die Stadt darf höchstens 80 Zeichen lang sein.“ |
| Lineup zu lang | „Das Lineup darf höchstens 300 Zeichen lang sein.“ |
| Ticket-Adresse ungültig | „Bitte eine gültige Ticket-Adresse angeben (https://…).“ |

### A3. Detail – `/events/<id>`

- Zeigt Bild mit Bildnachweis, Titel, Beschreibung in Absätzen, Erstellt am und Zuletzt geändert.
- Link **„In der Eventliste ansehen“** zu `/eventliste/<id>`.
- Knöpfe: **Bearbeiten**, **Löschen**, **Zurück zur Liste**.

### A4. Löschen

- Dialog: **„Event „<Titel>“ wirklich löschen? Das kann nicht rückgängig gemacht werden.“** mit *Abbrechen* und *Löschen* (rot).
- Nach dem Löschen: Weiterleitung zur Liste mit „Event gelöscht.“

---

## Teil B – Eventliste (öffentlich)

### B1. Übersicht – `/eventliste?zeitraum=…`

Layout nach dem Designvorschlag „Techno App“ (geändert am 14.11.2026):

- Zeigt nur **kommende Events**: Beginn heute (ab 00:00) oder später. Vergangene Events sind ausgeblendet.
- **Reihenfolge:** nach Beginn, das nächste zuerst.
- **Zeitraum-Filter** als Chips: *Alle*, *Heute*, *Diese Woche* (bis Sonntag), *Wochenende* (Freitag 00:00 bis Montag 00:00). Der Filter steht in der Adresse (`?zeitraum=heute|woche|wochenende`).
- **Highlight:** Das nächste Event erscheint groß: Bild, Badge „HEUTE“ (wenn heute), „Wenige Tickets“, Datum und Uhrzeit, Titel, Ort · Stadt, Lineup, Anfang der Beschreibung, Knopf **„Tickets sichern“**.
- **Demnächst:** Die weiteren Events als Zeilen mit Datumsblock (Wochentag, Tag, Monat), Titel, Ort · Stadt · Uhrzeit, Lineup und Knopf **„Tickets“**.
- Ticket-Knopf: öffnet die Ticket-Adresse in einem neuen Fenster. Ist das Event ausverkauft, steht dort ein ausgegrauter Knopf „Ausverkauft“. Ohne Ticket-Adresse gibt es keinen Knopf.
- **Layout:** Handy 1 Spalte; ab Tablet Highlight mit Bild links; „Demnächst“ in 2 bzw. 3 Spalten.
- Leere Liste: „Zurzeit sind keine Events geplant.“; leerer Zeitraum: „In diesem Zeitraum sind keine Events geplant.“
- Die Eventliste ist in der Navigation verlinkt („Events“).

### B2. Eventseite – `/eventliste/<id>`

- Großes Bild mit Bildnachweis als Bildunterschrift, Datum und Uhrzeit, Titel, Ort · Stadt, Lineup, Ticket-Knopf, Beschreibung in Absätzen.
- Link **„Zurück zur Eventliste“**.
- Unbekannte IDs → **404-Seite** (HTTP-Status 404).

---

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | Titel, Beschreibung, Bild-Adresse und Bildnachweis | gespeichert wird | ist das Event gespeichert und erscheint in `/events` und `/eventliste` mit Bild |
| AK-2 | Titel und Beschreibung, keine Bild-Adresse | gespeichert wird | ist das Event ohne Bild gespeichert, es erscheint der Platzhalter mit Anfangsbuchstaben |
| AK-3 | Titel leer oder nur Leerzeichen | gespeichert wird | erscheint „Bitte einen Titel eingeben.“ und nichts wird gespeichert |
| AK-4 | Titel mit 151 Zeichen | gespeichert wird | erscheint „Der Titel darf höchstens 150 Zeichen lang sein.“ |
| AK-5 | Beschreibung leer | gespeichert wird | erscheint „Bitte eine Beschreibung eingeben.“ und nichts wird gespeichert |
| AK-6 | Bild-Adresse ohne Bildnachweis | gespeichert wird | erscheint „Bitte einen Bildnachweis angeben.“ und nichts wird gespeichert |
| AK-7 | Bild-Adresse wie `ftp://…`, `bild.jpg` oder mit Leerzeichen | gespeichert wird | erscheint „Bitte eine gültige Bild-Adresse angeben (https://… oder /…).“ |
| AK-8 | keine Bild-Adresse, aber ein Bildnachweis | gespeichert wird | wird das Event ohne Bild und ohne Nachweis gespeichert |
| AK-9 | ein Event wird bearbeitet | Titel geändert und gespeichert wird | ist der neue Titel gespeichert, `updated_at` ist neuer, `created_at` unverändert |
| AK-10 | ein Event mit Bild | bearbeitet und die Bild-Adresse geleert wird | hat das Event danach kein Bild mehr (Platzhalter erscheint) |
| AK-11 | der Löschdialog ist offen | „Löschen“ geklickt wird | ist das Event entfernt, die Liste zeigt „Event gelöscht.“ |
| AK-12 | Events gestern, heute und später | `/eventliste` geöffnet wird | fehlt das Event von gestern; die anderen stehen nach Beginn sortiert, das nächste zuerst *(geändert 14.11.2026)* |
| AK-13 | keine Events | `/eventliste` geöffnet wird | erscheint „Zurzeit sind keine Events geplant.“ |
| AK-14 | eine Beschreibung mit zwei Absätzen | die Eventseite geöffnet wird | erscheinen zwei getrennte Absätze |
| AK-15 | eine unbekannte ID | `/eventliste/<id>` aufgerufen wird | erscheint die 404-Seite (Status 404) |
| AK-16 | die Eventliste | auf Handy, Tablet und Desktop angesehen | passt sich das Raster an (1 / 2 / 3 Spalten), nichts ragt seitlich heraus |
| AK-17 | ungültige Werte | Erstellen oder Bearbeiten direkt an den Server bzw. die Datenbank geschickt wird | prüft der Server alle Regeln; die Datenbank lehnt leere Titel/Beschreibungen/Orte, ungültige Ticket- und Bilddaten zusätzlich ab (CHECK-Constraints) |
| AK-18 | Datum, Beginn, Ort oder Stadt fehlen oder sind ungültig | gespeichert wird | erscheint beim jeweiligen Feld die Meldung, nichts wird gespeichert |
| AK-19 | Ticket-Adresse `http://…` oder ohne `https://` | gespeichert wird | erscheint „Bitte eine gültige Ticket-Adresse angeben (https://…).“ |
| AK-20 | Events am Mittwoch, Freitag, Sonntag und nächsten Montag; heute ist Mittwoch | Zeitraum *Heute* / *Diese Woche* / *Wochenende* gewählt wird | erscheint nur Mittwoch / Mittwoch bis Sonntag / Freitag und Sonntag |
| AK-21 | ein Event mit Status *Ausverkauft* | die Eventliste geöffnet wird | zeigt es statt des Ticket-Links einen ausgegrauten Knopf „Ausverkauft“ |

## Annahmen (bitte bestätigen)

- **Datum, Uhrzeit, Ort, Lineup und Tickets** wurden mit dem Designvorschlag „Techno App“ (14.11.2026) ergänzt. Zeiten gelten als Ortszeit Zürich.
- **Keine Endzeit:** Ein Event verschwindet am Tag nach seinem Beginn aus der Liste (auch wenn es über Mitternacht läuft).
- **Tickets** werden nicht selbst verkauft, nur verlinkt. Der Ticketstatus wird von Hand gepflegt.
- **Kein Status Entwurf/Veröffentlicht:** Jedes gespeicherte Event ist sofort in der Eventliste sichtbar.
- **Beschreibung ist Pflicht.**
- **Bilder per Adresse** wie in ANF-004. Testbilder nur aus freien Quellen (z. B. Wikimedia Commons) unter `public/images/events/`, Lizenz in `public/images/events/CREDITS.json`.
- **Keine Seitenaufteilung** in der Eventliste, solange es wenige Events gibt.
- **Rechte:** Wie in ANF-002 darf ohne Login jede:r Events verwalten.
- **Löschen ist endgültig** (wie ANF-002).

## Nicht Teil von ANF-006

- Preis, eigener Ticketverkauf, Anmeldung
- **„Merken“ und „Warteliste“** aus dem Designvorschlag – brauchen Benutzerkonten (siehe Anforderung.md, offene Fragen)
- Kalenderansicht, Export (iCal), Suche
- Bild-Upload, mehrere Bilder pro Event
- Login, Berechtigungen

## Umsetzung (Vorschlag)

Nach dem CRUD-Muster in [system-design.md](../system-design.md), Vorlage `src/articles/`:

| Teil | Ort |
|---|---|
| Migration `events` (`title TEXT NOT NULL`, `description TEXT NOT NULL`, `image_url TEXT NULL`, `image_credit TEXT NULL`, CHECK-Constraints wie bei `articles`) | `migrations/<datum>_events.sql` |
| Modell, Prüfregeln (Bildregeln aus `src/articles/model.rs` wiederverwenden) | `src/events/model.rs` |
| Server-Funktionen | `src/events/api.rs` |
| SQL | `src/events/repo.rs` |
| Verwaltung | `src/pages/event_list.rs`, `event_detail.rs`, `event_new.rs`, `event_edit.rs` |
| Öffentliche Eventliste | `src/pages/eventliste.rs` (`/eventliste`), `src/pages/eventliste_event.rs` (`/eventliste/<id>`, `SsrMode::Async` für echtes 404) |
| Komponenten (Felder, Eventkarte; Bild über `CoverImage`) | `src/components/event.rs` |
| Tests | Unit-Tests in `model.rs`, DB-Tests in `tests/anf_006.rs` |
| Datum/Uhrzeit auf Deutsch, Zeiträume | `src/datetime.rs` |
