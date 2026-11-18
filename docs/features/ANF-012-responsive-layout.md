# ANF-012 – Responsive Layout: alle Seiten auf Handy, Tablet und Desktop

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Bezug: NFA-02 (Responsive Design), ANF-009 (Design „Techno App“: Leiste unten, Pillen oben, Shop-Raster), ANF-011 (PWA, Safe-Area), ANF-007 (Radio-Player)

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | spezifiziert (18.11.2026), noch nicht umgesetzt |
| **Rolle** | Besucher:in, Redaktion (Verwaltung) |

**Beschreibung**
Saintwhoo wird vor allem unterwegs auf dem Handy genutzt (ANF-009, ANF-011). Die App soll deshalb auf jeder Bildschirmbreite gut lesbar und bedienbar sein: vom kleinen Handy bis zum großen Desktop-Monitor. Kein Inhalt wird abgeschnitten, die Seite rutscht nicht seitlich weg, und Knöpfe lassen sich mit dem Daumen treffen.

Die Grundlagen stehen schon im Code (siehe „Ist-Zustand“). Diese Anforderung sorgt dafür, dass **alle** Seiten das erfüllen, auch die Verwaltung (Redaktion, Shop-Verwaltung) und die Formulare. Sie ändert kein Verhalten und keine Abläufe, nur Aufbau und Abstände.

**User Stories**

- Als Besucher:in möchte ich jede Seite auf meinem Handy ohne seitliches Scrollen lesen, damit ich Artikel und Events bequem unterwegs sehe.
- Als Besucher:in möchte ich auf Tablet und Laptop ein Layout mit mehr Platz, damit ich auf einen Blick mehr sehe.
- Als Redakteur:in möchte ich Listen und Formulare auch am Handy bedienen, damit ich unterwegs einen Artikel oder ein Event korrigieren kann.

---

## Ist-Zustand (Code-Prüfung, 18.11.2026)

Diese Teile sind schon umgesetzt:

- **Navigation:** Leiste unten auf dem Handy, Bereiche als Pillen in der Kopfzeile ab 768 px (`src/components/navbar.rs`).
- **Verwaltungslisten** (`/artikel`, `/events`, `/produkte`): Tabelle ab 768 px, auf dem Handy Karten mit Knopf „Bearbeiten“.
- **Filter-Chips** (News, Eventliste, Shop) liegen in einer Zeile und scrollen waagerecht.
- **Abstand unten** für Leiste und Radio-Player berücksichtigt `env(safe-area-inset-bottom)`.
- **Raster:** Startseite und Shop mit 2, 3 oder 4 Spalten (ANF-009).

Diese Teile sind **noch nicht im Browser geprüft** und könnten Probleme haben:

- Kopfzeile bei 320–390 px: Logo, Offline-Badge, „App installieren“ und Redaktions-Menü müssen in den Platz passen.
- Tabellen zwischen 768 und 1023 px mit fünf Spalten.
- Formulare (`max-w-2xl`) bei schmaler Breite. Eingabefelder mit weniger als 16 px Schrift lassen iOS beim Tippen zoomen.
- Querformat auf dem Handy und 200 % Zoom.
- Es gibt keine automatische Prüfung der Breiten. Der Ordner `end2end/` fehlt noch (siehe [task.md](../task.md)).

---

## Breakpoints

Es gelten nur die Standard-Breakpoints von Tailwind. Eigene Werte gibt es nicht.

| Bereich | Breite | Präfix | Typisches Gerät |
|---|---|---|---|
| Klein | 320 – 389 px | (ohne) | kleines Handy, Test für Zoom und Umbruch |
| Handy | 390 – 767 px | (ohne) | iPhone, Android |
| Tablet | 768 – 1023 px | `md:` | iPad hochkant |
| Laptop | 1024 – 1279 px | `lg:` | Laptop |
| Desktop | ab 1280 px | `xl:` | Desktop-Monitor |

**Regel:** Mobile first. Die Klasse ohne Präfix gilt für das Handy, `md:`, `lg:` und `xl:` erweitern sie. Die Umschaltung zwischen Leiste unten und Pillen oben bleibt bei `md:` (wie in ANF-009).

## Felder / Daten

Keine neuen Daten, keine Migration, keine Umgebungsvariablen.

## Ablauf

1. Besucher:in öffnet eine Seite auf einem Gerät.
2. Der Browser wählt die Breite. Das Layout passt sich per CSS an, ohne JavaScript.
3. Unter 768 px: Leiste unten, Karten, Inhalte untereinander.
4. Ab 768 px: Pillen oben, Tabellen und mehrspaltige Raster.

---

## Abnahmekriterien

Breiten sind CSS-Pixel im Browser. Die Prüfung läuft automatisch mit Playwright (siehe unten).

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | Breite 320 px | jede öffentliche Seite und jede Verwaltungsseite geladen wird | gibt es kein seitliches Scrollen der Seite (`scrollWidth` ≤ `clientWidth`). Erlaubt sind nur Filter-Chips und Tabellen, die in ihrem eigenen Bereich scrollen |
| AK-2 | Breiten 320, 390, 768, 1024 und 1440 px | Startseite, News, Beitrag, Eventliste, Eventdetail, Radio, Shop, Produkt und Warenkorb geöffnet werden | ist alles lesbar, nichts ist abgeschnitten, Überschriften brechen sauber um |
| AK-3 | Breite 320 px | die Kopfzeile geöffnet wird | passen Logo, Offline-Badge, „App installieren“ und Redaktions-Menü. Jeder Knopf ist ganz sichtbar und anklickbar; Elemente brechen um, statt sich zu überlappen |
| AK-4 | Breite unter 768 px | eine Seite geöffnet wird | ist die Leiste unten sichtbar und die Pillen oben ausgeblendet. Ab 768 px ist es umgekehrt (ANF-009 AK-2 und AK-3 gelten weiter) |
| AK-5 | Handy (390 px), Radio-Player aktiv | das Seitenende erreicht wird | verdeckt weder die schwebende Player-Leiste noch die Leiste unten den letzten Inhalt |
| AK-6 | jede Schaltfläche und jeder Link (Knöpfe, Navigation, Chips, Menü-Einträge, Karten-Knöpfe) | Höhe und Breite gemessen werden | sind mindestens 44 × 44 px (`min-h-11`, siehe CLAUDE.md) |
| AK-7 | Verwaltungsliste (`/artikel`, `/events`, `/produkte`) | Breite 768 px | erscheint die Tabelle. Die Spalte „Aktionen“ ist per Scrollen im Tabellenbereich erreichbar |
| AK-8 | Verwaltungsliste | Breite unter 768 px | erscheinen Karten statt Tabelle. „Bearbeiten“ ist auf der Karte erreichbar |
| AK-9 | Formular zum Anlegen oder Bearbeiten (Artikel, Event, Produkt) | Breite 320 px | sind alle Felder einspaltig und volle Breite. Die Knöpfe „Speichern“ und „Abbrechen“ sind sichtbar, auch wenn sie untereinander stehen |
| AK-10 | jedes Eingabefeld | die Schrift geprüft wird | ist die Schrift mindestens 16 px groß, damit iOS beim Tippen nicht zoomt |
| AK-11 | Bilder (Artikel, Produkt, Event) | Breite 320 px | laufen Bilder nicht über den Rand und verzerren sich nicht (Seitenverhältnis bleibt) |
| AK-12 | Handy quer (844 × 390 px) | eine Seite geöffnet wird | ist sie bedienbar, und die Leiste unten verdeckt keine Knöpfe im Inhalt |
| AK-13 | Browser-Zoom 200 % bzw. Breite 320 CSS-px (WCAG-Reflow) | eine Seite geöffnet wird | bricht der Text um. Es gibt kein seitliches Scrollen für Fließtext |
| AK-14 | JavaScript ausgeschaltet | eine Seite bei 390 px und bei 1024 px geöffnet wird | ist das Layout gleich. Die Umschaltung läuft über CSS (NFA-01 bleibt erfüllt) |
| AK-15 | Breite 1440 px | Startseite und Shop geöffnet werden | bleibt der Text in einer maximalen Breite. Ab 1024 px ist die Startseite zweispaltig (ANF-009 Teil C), der Shop zeigt 4 Spalten |

**Prüfung:**

- **Automatisch (Playwright in `end2end/`):** AK-1, AK-2, AK-4, AK-6, AK-7, AK-8, AK-9, AK-10 (Schriftgröße per Computed Style), AK-11, AK-13, AK-14, AK-15. Breiten werden mit `setViewportSize` gesetzt.
- **Automatisch mit Screenshot und Sichtprüfung:** AK-3, AK-5, AK-12.
- **Manuell auf echten Geräten:** Das Zoomen von iOS beim Tippen (AK-10) und das Verhalten im Querformat (AK-12) auf einem iPhone und einem Android-Gerät.
- Screenshots für die Sichtprüfung mit `deviceScaleFactor: 2`.

## Annahmen

- **Zielbrowser:** jeweils die zwei neuesten Versionen von Chrome, Edge, Firefox und Safari (iOS und macOS). Tailwind 4 setzt neuere Browser voraus.
- **Zielgeräte für die Prüfung:** iPhone 390 px, Android 412 px, iPad 768 px, Laptop 1280 px. *Bitte bestätigen.*
- Die Verwaltung muss am Handy bedienbar sein, muss dort aber nicht alles so zeigen wie am Laptop. Wenn nötig, blendet die Tabelle Spalten aus; Funktionen fallen nicht weg.
- Die Umschaltung zwischen Karten und Tabelle bleibt rein per CSS (`hidden md:block` bzw. `md:hidden`), nicht per JavaScript.
- Layout nur mit Tailwind- und DaisyUI-Klassen. Keine neuen CSS-Dateien (CLAUDE.md).
- Design aus ANF-009 bleibt: keine neuen Farben, Schriften oder Komponenten.

## Nicht Teil von ANF-012

- Neues Design oder ein helles Theme (NFA-07 bleibt dunkel)
- Eigene Layouts für Tablets oder Smart-TVs
- Drucklayout
- Neue Funktionen oder neue Seiten
- Bildschirme unter 320 px
- Native App (siehe ANF-011)
