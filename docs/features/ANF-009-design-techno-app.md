# ANF-009 – Design „Techno App“: Look, Navigation, Startseite, Shop

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Quelle: Designvorschlag „Techno App“ (Version 1, Claude-Design-Canvas, 14.11.2026) · Ergänzt [ANF-003](ANF-003-magazin-startseite.md) und [ANF-005](ANF-005-shop-produkte.md)

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | umgesetzt (14.11.2026) |
| **Rolle** | Besucher:in, Shop-Verwalter:in |

**Beschreibung**
Saintwhoo wird zur App für Techno-Fans: News über DJs, Events, Radio und Merch. Die Oberfläche ist dunkel, schnell und auf Musik ausgerichtet. Diese Anforderung beschreibt alles aus dem Entwurf, was nicht zu Events (ANF-006), Radio (ANF-007) oder News (ANF-008) gehört.

---

## Teil A – Look

| Token | Wert | DaisyUI |
|---|---|---|
| Hintergrund | `#0B0B0C` | `base-100` |
| Fläche | `#161618` | `base-200` |
| Linie | `#2C2C31` | `base-300` |
| Gedämpft | `#9A9AA3` | Tailwind `text-muted` |
| Text | `#F3F3EF` | `base-content`, `neutral` |
| Akzent (Acid-Gelb) | `#C6FF1A` | `primary`, `accent` |

- Eigenes DaisyUI-Theme `saintwhoo` in `style/tailwind.css`. **Dunkel ist Standard**, auch bei hellem System.
- **Helles Theme** `saintwhoo-hell` *(ergänzt 15.11.2026, Designvorschlag Version 1)*: Hintergrund `#F4F4EF`, Fläche `#FFFFFF`, Linie `#DCDCD5`, gedämpft `#5C5C64`, Text `#0B0B0C`. Akzentflächen (`primary`) sind schwarz mit Acid-Gelb als Schrift.
- **Umschalter** oben rechts in der Kopfzeile (runder Knopf, 44 px, `aria-label` „Hellen/Dunklen Modus aktivieren“). `public/theme.js` setzt `data-theme` vor dem ersten Zeichnen und merkt sich die Wahl im `localStorage` (`saintwhoo-theme`).
- Schriften: **Space Grotesk 700** für Titel und Zahlen (`font-display`), **Inter 400–600** für Text (`font-sans`). Beide von Google Fonts.
- Runde Karten (20 px), Pillen-Knöpfe, keine Schatten. Touch-Ziele mindestens 44 px.

## Teil B – Navigation

- **Handy:** Leiste unten mit fünf Symbolen: Start, News, Events, Radio, Shop. Der aktive Bereich ist in Akzentfarbe und hat `aria-current="page"`. Detailseiten zählen zu ihrem Bereich (z. B. `/beitrag/…` → News).
- **Ab Tablet:** dieselben Bereiche als Pillen in der Kopfzeile, die Leiste unten entfällt.
- **Redaktion:** Menü rechts in der Kopfzeile (Artikel, Events, Radio, Produkte und „Neu …“). Es funktioniert ohne JavaScript.

## Teil C – Startseite `/`

Ersetzt das Magazin-Layout aus ANF-003 (Aufmacher, Raster, Liste). Die Regeln „nur öffentliche Artikel“ und die Leseansicht `/beitrag/<id>` bleiben.

- **Livestream:** Karte „Jetzt auf Sendung“ mit Play-Knopf (ANF-007), Link „Sendeplan und Archiv“
- **Neueste News:** 4 Artikel (ANF-008), Link „Alle“
- **Nächste Events:** 3 Events mit Datumsblock (ANF-006), Link „Alle“
- **Merch:** 4 neueste aktive Produkte mit „+“-Knopf, Link „Alle“
- Handy: alles untereinander (Live, News, Events, Merch). Desktop: links Live + Events, rechts News, darunter Merch.
- Jeder Block lädt für sich. Ein Fehler in einem Block lässt die anderen stehen.

## Teil D – Shop (Erweiterung ANF-005)

| Feld | Pflicht | Regeln |
|---|---|---|
| Kategorie | ja | *Hoodies*, *T-Shirts*, *Vinyl*, *Accessoires*, *Sonstiges* (Standard). Fehlermeldung: „Bitte eine gültige Kategorie wählen.“ |
| Marke / Künstler:in | nein | höchstens 80 Zeichen, erscheint unter dem Produktnamen. Fehlermeldung: „Die Marke darf höchstens 80 Zeichen lang sein.“ |

- Shop-Kopf mit Knopf **Warenkorb** und Anzahl.
- **Kategorie-Chips** *Alle* + fünf Kategorien (`?kategorie=hoodies|t-shirts|vinyl|accessoires|sonstiges`), dazu die Sortierung aus ANF-005.
- Karten im Raster (Handy 2, Tablet 3, Desktop 4 Spalten): Bild, Name, Marke, „Nur noch n verfügbar“, Preis, runder „+“-Knopf. Ausverkauft: Bild mit Overlay „Ausverkauft“, kein „+“.
- Leere Kategorie: „In dieser Kategorie gibt es zurzeit keine Produkte.“
- Preise bleiben in **CHF** (ANF-005). Der Entwurf zeigt €.

## Teil E – Warenkorb `/warenkorb`

- „+“ auf Karten und „In den Warenkorb“ auf der Produktseite legen ein Stück dazu. Mehr als den Lagerbestand lässt der Warenkorb nicht zu.
- Leiste unten im Shop: Anzahl, Summe, „Zum Warenkorb“ (nur wenn etwas drin ist).
- Seite Warenkorb: Produkte mit Menge (− / +), Zeilensumme, Entfernen, Gesamtsumme inkl. MWST.
- Der Warenkorb lebt im Browser und bleibt beim Navigieren erhalten. **Nach dem Neuladen ist er leer.**
- **Bestellen ist nicht möglich.** Die Seite sagt das offen: „Bestellen ist noch nicht möglich. Die Kasse folgt in einer späteren Version.“

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | eine beliebige Seite | sie geöffnet wird | ist der Hintergrund dunkel (`#0B0B0C`), Titel in Space Grotesk, Akzent Acid-Gelb – auch wenn das System hell eingestellt ist |
| AK-2 | Handy-Breite (390 px) | `/beitrag/<id>` geöffnet wird | zeigt die Leiste unten „News“ als aktiv; nichts ragt seitlich heraus |
| AK-3 | Desktop | eine Seite geöffnet wird | sind die Bereiche oben als Pillen sichtbar, die Leiste unten nicht |
| AK-4 | Produkte in mehreren Kategorien | im Shop *Hoodies* gewählt wird | erscheinen nur Hoodies |
| AK-5 | ein Produkt mit Kategorie und Marke | gespeichert wird | sind beide gespeichert; die Marke steht auf der Karte |
| AK-6 | ein Produkt mit Bestand 2 | dreimal „+“ getippt wird | sind 2 Stück im Warenkorb, der Knopf ist danach gesperrt |
| AK-7 | ein ausverkauftes Produkt | der Shop geöffnet wird | zeigt die Karte „Ausverkauft“ und keinen „+“-Knopf |
| AK-8 | Produkte im Warenkorb | Menge geändert oder entfernt wird | stimmen Anzahl und Summe |
| AK-9 | eine beliebige Seite im dunklen Modus | der Umschalter oben rechts angetippt wird | wird die Seite hell (`#F4F4EF`); nach dem Neuladen bleibt sie hell, ohne kurz dunkel aufzublitzen |

AK-4/5: `tests/anf_005.rs` läuft weiter, Kategorie/Marke in Unit-Tests `src/products/model.rs`. AK-6/8: Unit-Tests `src/components/cart.rs` (Logik). AK-1 bis 3 und 7: Screenshots bzw. Browser.

## Nicht Teil von ANF-009

- **Anmelden**, „Folge deinen DJs“, „Merken“, „Warteliste“ – brauchen Benutzerkonten
- Kasse, Bezahlung, Bestellungen
- Größenauswahl bei Kleidung
