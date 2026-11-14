# ANF-007 – Radio: Livestream, Sendeplan und Archiv

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Quelle: Designvorschlag „Techno App“ (Version 1), Bildschirm „Radio“

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (14.11.2026) |
| **Rolle** | Redakteur:in, Besucher:in |

**Beschreibung**
Besucher:innen hören den Livestream und sehen, was gerade läuft und was heute noch kommt. Aufgezeichnete Sets können sie im Archiv nachhören. Ein Tipp auf Play startet die Wiedergabe. Sie läuft beim Wechsel auf eine andere Seite weiter.

Die Redaktion pflegt den Sendeplan und das Archiv auf einer Verwaltungsseite.

**User Stories**

- Als Besucher:in möchte ich mit einem Tipp den Livestream starten und dabei weiter in der App stöbern, damit die Musik nicht abbricht.
- Als Besucher:in möchte ich sehen, welche Sendung gerade läuft und was heute noch kommt.
- Als Besucher:in möchte ich vergangene Sets nachhören.
- Als Redakteur:in möchte ich Sendungen und Mixe anlegen und löschen.

---

## Daten

### Sendung (`radio_shows`)

| Feld | Pflicht | Regeln |
|---|---|---|
| Titel | ja | 1–120 Zeichen, getrimmt |
| DJ | ja | 1–120 Zeichen, getrimmt |
| Datum | ja | gültiges Datum |
| Beginn | ja | `HH:MM`, Ortszeit Zürich |
| Ende | ja | `HH:MM`. Früher als Beginn = Sendung endet am nächsten Tag. Gleich wie Beginn ist nicht erlaubt. Höchstens 12 Stunden. |

### Mix im Archiv (`radio_mixes`)

| Feld | Pflicht | Regeln |
|---|---|---|
| Titel | ja | 1–120 Zeichen |
| DJ | ja | 1–120 Zeichen |
| Dauer | ja | ganze Minuten, 1–600 |
| Adresse der Aufnahme | ja | `https://…` oder `/…`, keine Leerzeichen, höchstens 500 Zeichen (gleiche Regel wie Bild-Adressen) |

### Livestream

Die Adresse des Streams steht in der Umgebungsvariable `RADIO_STREAM_URL` (z. B. in `.env`). Ist sie leer, zeigt die Seite „Der Livestream ist zurzeit nicht verfügbar.“

**Fehlertexte:** „Bitte einen Titel eingeben.“ · „Bitte einen DJ angeben.“ · „Bitte ein Datum angeben.“ · „Bitte eine Uhrzeit angeben.“ · „Bitte eine gültige Uhrzeit angeben (HH:MM).“ · „Beginn und Ende dürfen nicht gleich sein.“ · „Eine Sendung darf höchstens 12 Stunden dauern.“ · „Bitte eine Dauer zwischen 1 und 600 Minuten angeben.“ · „Bitte die Adresse der Aufnahme angeben.“ · „Bitte eine gültige Adresse angeben (https://… oder /…).“

---

## Seiten

| Adresse | Inhalt |
|---|---|
| `/radio` | Umschalter *Live* / *Archiv*. Live: Karte „Jetzt auf Sendung“ (Titel, DJ, Zeit, Pegel-Balken als Deko, Play-Knopf), darunter „Sendeplan heute“ mit Badge „JETZT“ bei der laufenden Sendung. |
| `/radio?ansicht=archiv` | Liste der Mixe (neueste zuerst, höchstens 50) mit Play-Knopf, DJ und Dauer. |
| `/radio/verwaltung` | Formulare „Sendung“ und „Mix“, Listen der kommenden Sendungen und aller Mixe, Löschen mit Bestätigungsdialog. |

**Sendeplan heute:** alle Sendungen, die am heutigen Tag (Ortszeit) laufen. Das schließt Sendungen über Mitternacht ein.

**Player:** Eine Leiste unten (auf dem Handy über der Navigation) zeigt den laufenden Titel mit Wiedergabe-Steuerung des Browsers und einem Knopf zum Schließen. Die Leiste gehört zum Seitenrahmen, nicht zur Seite. Deshalb läuft die Wiedergabe beim Navigieren weiter.

Die Startseite zeigt die Live-Karte in kurzer Form (ohne Pegel-Balken).

---

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | Sendungen gestern 23–01 Uhr, heute 02–04 und 04–06 Uhr, morgen | der Sendeplan für heute geladen wird | erscheinen die Sendung von gestern Nacht und die zwei von heute, nach Beginn sortiert |
| AK-2 | eine Sendung 02–04 Uhr | es ist 03:00 bzw. 04:00 | ist sie „JETZT“ auf Sendung bzw. nicht mehr |
| AK-3 | Titel leer oder Ende = Beginn | eine Sendung gespeichert wird | erscheinen die Meldungen, nichts wird gespeichert |
| AK-4 | Sendungen vorbei, laufend, kommend | die Verwaltung geöffnet wird | erscheinen nur die laufende und die kommende |
| AK-5 | eine Sendung | gelöscht wird | ist sie weg; erneutes Löschen meldet „nicht gefunden“ |
| AK-6 | drei Mixe, nacheinander angelegt | das Archiv geöffnet wird | steht der neueste oben; nach dem Löschen sind es zwei |
| AK-7 | Adresse `mix.mp3` | ein Mix gespeichert wird | erscheint „Bitte eine gültige Adresse angeben (https://… oder /…).“ |
| AK-8 | ungültige Werte direkt in der DB | gespeichert wird | lehnt die DB ab (Ende vor Beginn, > 12 Std., Dauer 0/601, Adresse `ftp://`/`//…`) |
| AK-9 | der Livestream läuft | auf eine andere Seite navigiert wird | läuft die Wiedergabe weiter (Browser-Test, manuell) |
| AK-10 | `RADIO_STREAM_URL` ist leer | `/radio` geöffnet wird | erscheint „Der Livestream ist zurzeit nicht verfügbar.“, kein Play-Knopf |

AK-1 bis AK-8: `tests/anf_007.rs` und Unit-Tests in `src/radio/model.rs`. AK-9 und AK-10: im Browser prüfen.

## Annahmen (bitte bestätigen)

- **Ein Stream** für das ganze Radio, Adresse per Umgebungsvariable.
- **Aufnahmen per Adresse** (wie Bilder), kein Upload. Dateien unter `public/audio/` oder auf einem anderen Server. Nur Aufnahmen mit Recht zur Veröffentlichung.
- **Verwaltung vereinfacht:** Sendungen und Mixe können angelegt und gelöscht werden. **Bearbeiten fehlt noch** (Abweichung vom CRUD-Standard ANF-002, siehe task.md).
- **„Teilen“ und „Lautstärke“** aus dem Entwurf übernimmt die Wiedergabe-Steuerung des Browsers. Ein eigener Teilen-Knopf fehlt.
- Nach einem Neuladen der Seite stoppt die Wiedergabe.

## Nicht Teil von ANF-007

- Eigener Streaming-Server, Upload von Aufnahmen
- Wochen-Sendeplan, Wiederholungen
- Live-Anzeige des aktuellen Tracks, Chat, Likes
