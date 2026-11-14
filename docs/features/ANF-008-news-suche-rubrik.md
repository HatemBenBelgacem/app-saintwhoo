# ANF-008 – News: Rubrik, Suche und Filter

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Baut auf [ANF-001](ANF-001-magazin-artikel-erfassen.md) und [ANF-003](ANF-003-magazin-startseite.md) auf · Quelle: Designvorschlag „Techno App“, Bildschirm „News“

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (14.11.2026) |
| **Rolle** | Redakteur:in, Besucher:in |

**Beschreibung**
Jeder Artikel bekommt eine **Rubrik**. Die neue Seite **News** listet alle öffentlichen Artikel. Besucher:innen können nach DJ, Label oder Stichwort suchen und nach Rubrik filtern.

**User Stories**

- Als Besucher:in möchte ich News nach Rubrik filtern und durchsuchen, damit ich schnell finde, was mich interessiert.
- Als Redakteur:in möchte ich jedem Artikel eine Rubrik geben.

---

## Rubrik

| Wert (DB) | Anzeige | Chip |
|---|---|---|
| `news` (Standard) | Meldung | Meldungen |
| `interview` | Interview | Interviews |
| `release` | Release | Releases |
| `event` | Event | Events |
| `mix` | Mix | Mixe |

- Auswahlfeld „Rubrik“ im Artikel-Formular (Erstellen und Bearbeiten).
- Fehlt der Wert (ältere Anfragen), gilt *Meldung*. Ein unbekannter Wert ergibt „Bitte eine gültige Rubrik wählen.“
- Bestehende Artikel bekommen bei der Migration *Meldung*.

## Seite `/news?q=…&rubrik=…&seite=…`

- Kopf „News“ mit Untertitel, Suchfeld (Beschriftung „Suche“), Chips *Alle* + fünf Rubriken.
- Liste: Bild 84 × 84 (oder Platzhalter), Rubrik in Akzentfarbe, Titel (Link zur Leseansicht `/beitrag/<id>`), Datum · Lesezeit. Handy 1 Spalte, ab Tablet 2.
- Nur öffentliche Artikel (veröffentlicht, Datum heute oder früher), neueste zuerst, 20 pro Seite.
- **Suche:** in Titel und Text, Groß-/Kleinschreibung egal, Leerzeichen außen werden ignoriert. Zeichen wie `%` und `_` gelten als normaler Text.
- Suche, Rubrik und Seite stehen in der Adresse und funktionieren ohne JavaScript.
- Keine Artikel: „Noch keine News veröffentlicht.“ Kein Treffer: „Keine News gefunden. Versuch es mit einem anderen Suchbegriff oder einer anderen Rubrik.“
- Die Startseite zeigt die 4 neuesten News im gleichen Stil.

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | ein Artikel mit Rubrik *Interview* | gespeichert und auf *Mix* geändert wird | ist jeweils die gewählte Rubrik gespeichert |
| AK-2 | eine Anfrage ohne Rubrik | gespeichert wird | hat der Artikel die Rubrik *Meldung* |
| AK-3 | Artikel in verschiedenen Rubriken | nach *Interviews* gefiltert wird | erscheinen nur Interviews |
| AK-4 | „acid“ in einem Titel und „ACID“ in einem Text | nach „acid“ gesucht wird | erscheinen beide; mit Rubrik-Filter nur der passende |
| AK-5 | ein Titel „100% Vinyl“ | nach `%` bzw. `_` gesucht wird | erscheint nur dieser Artikel bzw. keiner |
| AK-6 | Entwurf, geplanter und veröffentlichter Artikel | `/news` geöffnet wird | erscheint nur der veröffentlichte |
| AK-7 | eine unbekannte Rubrik direkt in der DB | gespeichert wird | lehnt die DB ab |

Tests: `tests/anf_008.rs`, Unit-Tests in `src/articles/model.rs` und `src/pages/news.rs`.

## Nicht Teil von ANF-008

- **„Folge deinen DJs“** aus dem Entwurf (persönliche Reihenfolge) – braucht Benutzerkonten
- Relative Zeitangaben („vor 2 Std.“) – Artikel haben nur ein Datum
- Volltextsuche mit Wortstämmen, Suche nach Label als eigenes Feld
