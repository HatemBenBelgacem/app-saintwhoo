# ANF-011 – Progressive Web App (PWA): Saintwhoo installieren und offline öffnen

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Bezug: ANF-009 (Design „Techno App“, Leiste unten), ANF-007 (Radio-Player), ANF-010 (Docker/Deployment)

| | |
|---|---|
| **Priorität** | Soll |
| **Status** | umgesetzt (14.11.2026) |
| **Rolle** | Besucher:in |

**Beschreibung**
Saintwhoo ist als „App für Techno-Fans“ gedacht und vor allem fürs Handy gestaltet (ANF-009). Besucher:innen sollen die Seite deshalb wie eine App auf dem Startbildschirm installieren können: eigenes Icon, Start ohne Browserleiste, dunkler Startbildschirm. Ist das Netz weg (U-Bahn, Club-Keller), öffnet die App trotzdem und zeigt zuletzt gesehene Seiten oder eine klare Offline-Seite statt der Fehlerseite des Browsers.

Es gibt **keinen** Eintrag in einem App-Store und keine native App. Alles läuft weiter über dieselbe Leptos-App (SSR + Hydration).

**User Stories**

- Als Besucher:in möchte ich Saintwhoo auf meinem Handy installieren, damit ich die App mit einem Tipp vom Startbildschirm öffne.
- Als Besucher:in möchte ich die App auch ohne Netz öffnen können, damit ich zuletzt gelesene News und die Events sehe.
- Als Besucher:in möchte ich das Radio über den Sperrbildschirm pausieren und fortsetzen, damit ich das Handy nicht entsperren muss. *(Kann)*

---

## Teil A – Web App Manifest

Datei `public/manifest.webmanifest`, im `<head>` von `shell()` eingebunden (`<link rel="manifest" …>`).

| Feld | Wert |
|---|---|
| `name` | `Saintwhoo` |
| `short_name` | `Saintwhoo` |
| `description` | `News, Events, Radio und Merch für Techno-Fans` |
| `lang` | `de` |
| `start_url` | `/?quelle=pwa` |
| `scope` | `/` |
| `id` | `/` |
| `display` | `standalone` |
| `orientation` | nicht gesetzt (Hoch- und Querformat erlaubt) |
| `background_color` | `#0B0B0C` (wie `theme-color` und Theme `saintwhoo`) |
| `theme_color` | `#0B0B0C` |
| `icons` | siehe unten |
| `shortcuts` *(Kann)* | „Radio“ → `/radio`, „Events“ → `/eventliste`, „Shop“ → `/shop` |

**Icons** (in `public/icons/`, selbst gestaltet – Schriftzug/Logo „S“ in Acid-Gelb auf dunklem Grund, keine fremden Marken):

| Datei | Größe | Zweck |
|---|---|---|
| `icon-192.png` | 192 × 192 | `any` |
| `icon-512.png` | 512 × 512 | `any` |
| `icon-maskable-512.png` | 512 × 512 | `maskable` (Motiv in der inneren Sicherheitszone, 80 %) |
| `apple-touch-icon.png` | 180 × 180 | iOS, über `<link rel="apple-touch-icon">` |
| `favicon.svg` *(Soll)* | Vektor | Browser-Tab |

Ergänzend im `<head>` für iOS/Safari: `apple-mobile-web-app-capable`, `apple-mobile-web-app-status-bar-style = black-translucent`, `apple-mobile-web-app-title = Saintwhoo`. Die Leiste unten (ANF-009) berücksichtigt bereits `viewport-fit=cover`; im Standalone-Modus muss sie `env(safe-area-inset-bottom)` einhalten.

## Teil B – Service Worker und Offline

Datei `public/sw.js` (reines JavaScript, wird nicht von Rust erzeugt), Scope `/`. Registrierung im Browser nach der Hydration (in `hydrate()` bzw. einem kleinen Inline-Skript), **nur** wenn `navigator.serviceWorker` vorhanden ist.

**Caching-Regeln**

| Anfrage | Strategie | Begründung |
|---|---|---|
| Seiten (HTML, `mode: navigate`) | **Network first**, Fallback: Cache, dann `/offline` | Inhalte sollen aktuell sein, offline aber erreichbar |
| `/pkg/*` (CSS, JS, WASM) | **Stale-while-revalidate** | Dateinamen enthalten keinen Hash (`hash-files` ist aus); siehe Annahmen |
| `/icons/*`, `/manifest.webmanifest`, `/offline` | **Precache** beim `install` | App-Hülle muss offline sofort da sein |
| `/images/*` (Artikel-, Produkt-, Eventbilder) | **Cache first**, max. 60 Einträge, ältere fliegen raus | Bilder ändern sich selten |
| Server-Funktionen (`/api/*`), `POST`/`PUT`/`DELETE` | **nie** cachen, immer Netz | Daten und Formulare dürfen nicht veraltet sein |
| Radio-Livestream (`RADIO_STREAM_URL`), Audio-Dateien, Range-Anfragen | **nie** anfassen (kein `respondWith`) | Endlos-Stream bzw. große Dateien gehören nicht in den Cache |
| `/health`, `/status` | nie cachen | Betriebsprüfung muss echt sein |
| Fremde Herkünfte (z. B. Google Fonts) | nicht cachen *(Soll: Fonts cache first)* | |

- Cache-Name mit Version, z. B. `saintwhoo-v1`. Beim `activate` werden alte Caches gelöscht.
- Neuer Service Worker: `skipWaiting` erst, wenn die Person in einem Hinweis „Neue Version verfügbar – Neu laden“ tippt *(Soll)*; sonst beim nächsten Start.
- `sw.js` selbst wird mit `Cache-Control: no-cache` ausgeliefert, damit Updates sofort erkannt werden.

**Offline-Seite** `/offline` (neue Route, `src/pages/offline.rs`): dunkles Theme, Hinweis „Du bist offline“, Knopf „Erneut versuchen“, Links zu den im Cache vorhandenen Bereichen. Sie braucht keine Datenbank.

**Offline-Hinweis in der App** *(Soll)*: Ist `navigator.onLine` falsch, zeigt die Kopfzeile ein kleines Badge „Offline“. Aktionen, die den Server brauchen (Formulare in der Redaktion, Warenkorb-Kasse sobald es sie gibt), sind dann deaktiviert oder melden „Nur online möglich“.

## Teil C – Installation

- Browser mit `beforeinstallprompt` (Chrome, Edge, Android): Im Menü bzw. auf der Startseite erscheint ein Knopf **„App installieren“**. Er ist ausgeblendet, wenn die App schon installiert ist (`display-mode: standalone`) oder der Browser das Ereignis nicht sendet. *(Soll)*
- iOS/Safari: kein automatischer Dialog. Ein einmaliger, schließbarer Hinweis erklärt „Teilen → Zum Home-Bildschirm“. Geschlossen = gemerkt in `localStorage`. *(Kann)*
- Kein aufdringliches Pop-up beim ersten Besuch.

## Teil D – Radio im Hintergrund *(Kann)*

- Media Session API im Player (`src/components/player.rs`): Titel der Sendung/des Mixes, „Saintwhoo Radio“ als Künstler, App-Icon als Cover. Play/Pause auf dem Sperrbildschirm steuern das `<audio>`-Element.
- Der Player läuft im Standalone-Modus beim Seitenwechsel weiter (wie heute, ANF-007 AK-9).

## Teil E – Betrieb

- PWA setzt **HTTPS** voraus (Ausnahme: `localhost`). Railway liefert HTTPS; beim Docker-Betrieb übernimmt das ein Reverse-Proxy (ANF-010).
- `manifest.webmanifest` wird mit `Content-Type: application/manifest+json` ausgeliefert.
- Keine neuen Umgebungsvariablen nötig.

---

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | App läuft über HTTPS (oder `localhost`) | Chrome DevTools → Application → Manifest geöffnet wird | wird das Manifest ohne Fehler erkannt, Name „Saintwhoo“, Icons 192/512 und ein maskierbares Icon sind vorhanden |
| AK-2 | Android-Handy mit Chrome | „App installieren“ gewählt wird | liegt ein Saintwhoo-Icon auf dem Startbildschirm; beim Öffnen startet die App ohne Browserleiste mit dunklem Startbildschirm |
| AK-3 | iPhone mit Safari | „Zum Home-Bildschirm“ gewählt wird | erscheint das Apple-Touch-Icon und die App startet im Vollbild; die Leiste unten wird nicht vom Home-Indikator verdeckt |
| AK-4 | App wurde einmal online geöffnet | `GET /` | ist ein Service Worker mit Scope `/` aktiv |
| AK-5 | Startseite und `/eventliste` wurden online besucht, dann Netz aus | diese Seiten erneut geöffnet werden | werden sie aus dem Cache angezeigt, mit Styles und Bildern |
| AK-6 | Netz aus | eine nie besuchte Seite (z. B. `/radio`) geöffnet wird | erscheint die Offline-Seite „Du bist offline“ mit „Erneut versuchen“, nicht die Fehlerseite des Browsers |
| AK-7 | Netz wieder an, Offline-Seite sichtbar | „Erneut versuchen“ | wird die gewünschte Seite normal geladen |
| AK-8 | online, ein Artikel wird bearbeitet | gespeichert und die Liste neu geladen wird | zeigt sie die neue Fassung (Server-Funktionen und Seiten werden nicht veraltet aus dem Cache geliefert) |
| AK-9 | Radio-Livestream läuft | DevTools → Application → Cache Storage | liegt der Stream **nicht** im Cache; die Wiedergabe startet so schnell wie ohne Service Worker |
| AK-10 | neue Version wurde deployt | die installierte App geöffnet wird | wird spätestens beim zweiten Start die neue Version (CSS/WASM) geladen; alte Caches sind gelöscht |
| AK-11 | Lighthouse (Mobil) gegen die deployte App | Prüfung läuft | sind die PWA-Prüfungen „installierbar“ und „offline-fähig“ bestanden *(Soll)* |
| AK-12 | Browser ohne Service-Worker-Unterstützung bzw. JavaScript aus | Seite wird geöffnet | funktioniert die Seite wie bisher (SSR, NFA-01), keine Fehlermeldung |
| AK-13 | Netz aus | App ist geöffnet | zeigt die Kopfzeile „Offline“ *(Soll)* |
| AK-14 | Radio läuft, Handy gesperrt | Pause auf dem Sperrbildschirm | pausiert der Stream; Titel der Sendung wird angezeigt *(Kann)* |

AK-4, AK-6 (Route `/offline` antwortet 200 ohne DB) und Teil E (Content-Types) lassen sich als Integrationstest in `tests/anf_011_pwa.rs` prüfen. AK-5 bis AK-10 als Playwright-Test (`context.setOffline(true)`) in `end2end/`. AK-2, AK-3 und AK-14 nur manuell auf echten Geräten.

## Annahmen

- Nur Lesen offline. Formulare offline zwischenspeichern und später senden (Background Sync) ist **nicht** Teil dieser Anforderung.
- `cargo-leptos` erzeugt `/pkg/app-saintwhoo.{js,wasm,css}` ohne Hash im Namen. Darum stale-while-revalidate plus Cache-Version im Service Worker. Alternative: `hash-files = true` in `Cargo.toml` einschalten, dann ist für `/pkg/*` *cache first* möglich. Entscheidung bei der Umsetzung.
- Die Cache-Version in `sw.js` wird bei jedem Release erhöht (von Hand oder per Build-Schritt).
- Der Warenkorb bleibt wie heute im Browser; ihn über Neuladen zu speichern ist eine eigene Aufgabe (task.md).
- Icons werden selbst erstellt (kein Bildnachweis nötig). Falls doch eine fremde Grafik verwendet wird, gilt die Bildregel aus CLAUDE.md.
- Google Fonts bleiben extern; offline fällt die Schrift auf die Systemschrift zurück, wenn sie nicht gecacht ist.

## Entscheidungen bei der Umsetzung (14.11.2026)

- **`/pkg/*` network first statt stale-while-revalidate**, `hash-files` bleibt aus. Grund: Seiten kommen network first frisch vom Server; CSS/JS/WASM ohne Hash müssen genau zu diesem HTML passen, sonst scheitert die Hydration nach einem Deploy. Offline kommen sie aus dem Cache. AK-10 ist damit sofort erfüllt.
- **Offline-Fallback per Umleitung** auf `/offline?ziel=<Pfad>` (statt die Offline-Seite unter der gewünschten Adresse auszuliefern). So passt die Hydration zur Route; „Erneut versuchen“ lädt `ziel` (nur eigene Pfade).
- Beim `install` werden zusätzlich die `/pkg`-Dateien gecacht, die im HTML von `/offline` stehen. So ist die Offline-Seite sofort gestylt.
- Die Browser-Logik liegt in `public/pwa.js` (`window.saintwhooPwa`): Registrierung, `beforeinstallprompt`, Update-Hinweis, iOS-Hinweis, Media Session, Liste der gecachten Seiten. Die Leptos-Komponenten in `src/components/pwa.rs` rufen sie über `wasm-bindgen` auf. Ohne das Skript gelten die Startwerte.
- Offline werden Link-Klicks als echte Navigation geladen (statt über den Client-Router). So liefert der Service Worker die Seite aus dem Cache oder `/offline`.
- Google Fonts werden cache first gecacht (Soll).
- Offline zeigt das Redaktions-Menü „Nur online möglich“. Die Formulare selbst sind noch nicht deaktiviert (offen in task.md).
- `shell()` und `App` enden mit `.into_any()`, weil der Router jetzt in der Bibliothek liegt (`src/server.rs`). Sonst wird ein Symbolname über 1 MB lang, und der macOS-Linker bricht beim Bauen der cdylib ab.

## Nicht Teil von ANF-011

- Push-Benachrichtigungen (z. B. „Liveset startet“) – braucht Benutzerkonten und einen Push-Dienst, eigene Anforderung
- Background Sync / Offline-Bearbeitung in der Redaktion
- Einträge in App-Stores (Google Play über TWA, Apple App Store)
- Herunterladen von Radio-Mixes zum Offline-Hören
- Eigene native Apps
