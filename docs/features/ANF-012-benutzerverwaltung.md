# ANF-012 – Benutzerverwaltung

> Übersicht aller Anforderungen: [Anforderung.md](../Anforderung.md) · Baut auf [ANF-002](ANF-002-crud-datensaetze.md) (CRUD-Standard) auf · Betrifft [ANF-010](ANF-010-docker-container.md) (Umgebungsvariablen) und [ANF-011](ANF-011-pwa.md) (Service Worker)

| | |
|---|---|
| **Priorität** | Muss |
| **Status** | spezifiziert (15.11.2026) |
| **Rolle** | Admin, Redakteur:in, Shop-Verwalter:in, Besucher:in |

**Beschreibung**
Bisher darf jede:r ohne Login Artikel, Events, Radio und Produkte verwalten (siehe Annahmen in ANF-002, ANF-005, ANF-006). Mit dieser Anforderung bekommt Saintwhoo **Benutzerkonten mit Rollen**. Nur angemeldete Benutzer:innen mit passender Rolle dürfen Inhalte verwalten. Ein Admin legt Konten an, vergibt Rollen und sperrt Konten.

Die öffentlichen Seiten (Startseite, News, Eventliste, Radio, Shop, Warenkorb) bleiben **ohne Login** nutzbar.

Die Anforderung besteht aus drei Teilen:

- **Teil A – Anmelden und Abmelden:** Login mit E-Mail und Passwort, Sitzung, Logout.
- **Teil B – Rechte:** Welche Rolle darf was; Schutz der Verwaltungsseiten und Server-Funktionen.
- **Teil C – Benutzer verwalten:** Admin verwaltet Konten nach dem CRUD-Standard aus ANF-002; jede:r pflegt das eigene Konto.

> **Begriff:** In Code und Datenbank heißt der Datensatz `user` (Tabelle `users`). In der Oberfläche steht „Benutzer“ bzw. „Konto“.

**User Stories**

- Als Admin möchte ich Konten für das Team anlegen und Rollen vergeben, damit nur berechtigte Personen Inhalte ändern können.
- Als Redakteur:in möchte ich mich anmelden und Artikel, Events und Radio pflegen, ohne dass Fremde meine Inhalte löschen können.
- Als Shop-Verwalter:in möchte ich mich anmelden und nur die Produkte pflegen.
- Als angemeldete Person möchte ich mein Passwort ändern, damit mein Konto sicher bleibt.
- Als Besucher:in möchte ich die App weiter ohne Konto nutzen.

---

## Rollen

| Rolle | Wert in DB | Darf |
|---|---|---|
| **Admin** | `admin` | alles, inklusive Benutzerverwaltung |
| **Redaktion** | `redaktion` | Artikel (ANF-001/008), Events (ANF-006), Radio (ANF-007) verwalten |
| **Shop** | `shop` | Produkte (ANF-005) verwalten |

- Ein Konto hat **eine oder mehrere** Rollen (z. B. Redaktion **und** Shop).
- `admin` schließt alle anderen Rechte ein.
- Besucher:innen haben kein Konto und brauchen keins.

## Felder

| Feld | Typ | Pflicht | Regeln |
|---|---|---|---|
| E-Mail | Text, einzeilig | ja | Dient als Login-Name. Leerzeichen am Anfang/Ende werden entfernt, wird **klein geschrieben** gespeichert. Genau ein `@`, Teil davor und danach nicht leer, Teil danach enthält einen Punkt. Höchstens 254 Zeichen. **Eindeutig** (Groß-/Kleinschreibung egal). |
| Anzeigename | Text, einzeilig | ja | 1–80 Zeichen. Wird in der Kopfzeile und in der Benutzerliste gezeigt. |
| Rollen | Mehrfachauswahl | ja | Mindestens eine von *Admin*, *Redaktion*, *Shop*. |
| Passwort | Passwort | beim Anlegen ja | 12–128 Zeichen. Darf nicht gleich der E-Mail sein. Wird **nur als Hash** gespeichert (Argon2id), nie im Klartext, nie in Logs. |
| Status | Auswahl | ja | *Aktiv* (Standard) oder *Gesperrt*. Gesperrte Konten können sich nicht anmelden. |
| Passwort ändern erzwingen | Ja/Nein | ja | Wird automatisch gesetzt, wenn ein Admin ein Passwort vergibt. Dann muss die Person nach dem Login zuerst ein eigenes Passwort setzen. |

Zusätzlich setzt das System automatisch: ID, `created_at`, `updated_at` (ANF-002, R-8), `last_login_at`.

---

## Teil A – Anmelden und Abmelden

### A1. Anmelden – `/anmelden?ziel=…`

- Formular mit **E-Mail**, **Passwort** und Knopf **„Anmelden“**. Funktioniert auch ohne JavaScript (normales Formular, SSR).
- **Erfolg:** Die Sitzung startet. Weiterleitung zu `ziel` (nur Pfade dieser App, die mit `/` beginnen und nicht mit `//`), sonst zur Startseite. Meldung „Willkommen, <Anzeigename>.“
- **Fehler:** Immer dieselbe Meldung **„E-Mail oder Passwort ist falsch.“** – egal ob die E-Mail unbekannt, das Passwort falsch oder das Konto gesperrt ist. Die E-Mail bleibt im Feld, das Passwort nicht.
- **Schutz vor Durchprobieren:** Nach **5 Fehlversuchen** für dieselbe E-Mail innerhalb von 15 Minuten wird diese E-Mail **15 Minuten** lang abgewiesen: „Zu viele Versuche. Bitte in 15 Minuten erneut versuchen.“
- Ist *Passwort ändern erzwingen* gesetzt, führt der Login direkt zu `/konto/passwort` mit „Bitte zuerst ein eigenes Passwort festlegen.“ Bis dahin leiten alle Verwaltungsseiten dorthin um.
- Ist man schon angemeldet, leitet `/anmelden` zur Startseite weiter.

### A2. Sitzung

- Nach dem Login setzt der Server ein **Sitzungs-Cookie** `saintwhoo_session`: `HttpOnly`, `SameSite=Lax`, `Path=/`, im Betrieb (HTTPS) zusätzlich `Secure`.
- Das Cookie enthält nur ein **zufälliges Token** (mind. 32 Byte). In der Datenbank steht nur dessen **Hash** (Tabelle `sessions`).
- Die Sitzung gilt **14 Tage ab der letzten Aktivität**, höchstens 30 Tage ab dem Login. Abgelaufene Sitzungen werden abgewiesen und regelmäßig gelöscht.
- Rollen und Status werden **bei jeder Anfrage aus der Datenbank** gelesen. Wird ein Konto gesperrt oder eine Rolle entzogen, gilt das sofort.
- Bei Passwortänderung, Sperren und Löschen werden **alle Sitzungen** des Kontos beendet (bei eigener Passwortänderung: alle außer der aktuellen).

### A3. Abmelden

- Knopf **„Abmelden“** im Konto-Menü. Er schickt ein Formular (`POST /abmelden`), kein Link (`GET`).
- Die Sitzung wird in der Datenbank gelöscht und das Cookie entfernt. Weiterleitung zur Startseite mit „Du bist abgemeldet.“

### A4. Kopfzeile

- **Nicht angemeldet:** Link **„Anmelden“** statt des Redaktions-Menüs.
- **Angemeldet:** Menü mit dem Anzeigenamen. Darin nur die Bereiche, für die die Rollen reichen (Artikel/Events/Radio bei Redaktion, Produkte bei Shop, **Benutzer** bei Admin), dazu **„Mein Konto“** und **„Abmelden“**.
- Touch-Ziele min. 44 px (`min-h-11`), Design wie ANF-009.

---

## Teil B – Rechte

### B1. Rechte-Tabelle

| Bereich | Adressen | Besucher:in | Redaktion | Shop | Admin |
|---|---|---|---|---|---|
| Öffentliche Seiten | `/`, `/news`, `/beitrag/*`, `/eventliste*`, `/radio`, `/shop*`, `/warenkorb`, `/offline` | ✓ | ✓ | ✓ | ✓ |
| Artikel verwalten | `/artikel*` | – | ✓ | – | ✓ |
| Events verwalten | `/events*` | – | ✓ | – | ✓ |
| Radio verwalten | `/radio/verwaltung` | – | ✓ | – | ✓ |
| Produkte verwalten | `/produkte*` | – | – | ✓ | ✓ |
| Benutzer verwalten | `/benutzer*` | – | – | – | ✓ |
| Eigenes Konto | `/konto*` | – | ✓ | ✓ | ✓ |
| Systemstatus | `/status` | – | – | – | ✓ |

### B2. Verhalten bei fehlenden Rechten

- **Nicht angemeldet** und Verwaltungsseite aufgerufen → Weiterleitung zu `/anmelden?ziel=<aufgerufene Adresse>`.
- **Angemeldet, aber Rolle fehlt** → Seite „Kein Zugriff“ mit **HTTP-Status 403** und Link zur Startseite.
- **Jede `#[server]`-Funktion**, die Daten ändert oder Verwaltungsdaten liefert, prüft Anmeldung und Rolle **selbst auf dem Server**. Ausblenden von Knöpfen allein reicht nicht. Ohne Recht antwortet sie mit einem Fehler (401 bzw. 403) und ändert nichts.
- Öffentliche Server-Funktionen (Artikel/Events/Produkte lesen) bleiben ohne Login aufrufbar.

### B3. Schutz vor fremden Formularen (CSRF)

- Das Cookie hat `SameSite=Lax`.
- Zusätzlich lehnt der Server ändernde Anfragen (`POST`, `PUT`, `DELETE`) ab, wenn der `Origin`-Header gesetzt ist und nicht zur eigenen Adresse passt.

---

## Teil C – Benutzer verwalten

### C1. Admin: Benutzer – nach CRUD-Standard

Es gelten die Regeln R-1 bis R-10 aus [ANF-002 Teil A](ANF-002-crud-datensaetze.md#teil-a--crud-standard-gilt-für-alle-datensätze), mit den Abweichungen unten.

| Operation | Adresse |
|---|---|
| Liste | `/benutzer` |
| Detail | `/benutzer/<id>` |
| Erstellen | `/benutzer/neu` |
| Bearbeiten | `/benutzer/<id>/bearbeiten` |
| Löschen | Knopf mit Bestätigungsdialog auf Liste und Detailseite |

**Liste – `/benutzer`**

- Spalten: **Anzeigename** (Link zur Detailseite), **E-Mail**, **Rollen** (als Badges), **Status** (*Gesperrt* als rotes Badge), **Letzte Anmeldung** („noch nie“, wenn leer).
- Pro Zeile: **Bearbeiten** und **Löschen**. Oben: Knopf **„Neuer Benutzer“**.
- Reihenfolge: Anzeigename A–Z. Auf dem Handy Karten statt Tabelle.

**Erstellen – `/benutzer/neu`**

- Felder: E-Mail, Anzeigename, Rollen, **Startpasswort**. *Passwort ändern erzwingen* wird automatisch gesetzt.
- Nach dem Speichern: Weiterleitung zur Detailseite mit „Benutzer gespeichert.“ Das Startpasswort gibt der Admin selbst weiter (kein E-Mail-Versand, siehe Abgrenzung).

**Bearbeiten – `/benutzer/<id>/bearbeiten`**

- Felder: E-Mail, Anzeigename, Rollen, Status.
- Feld **„Neues Passwort“** (optional): leer = Passwort bleibt. Ausgefüllt = neues Passwort, *Passwort ändern erzwingen* wird gesetzt, alle Sitzungen des Kontos enden. So setzt ein Admin ein vergessenes Passwort zurück.

**Detail – `/benutzer/<id>`**

- Zeigt alle Felder außer dem Passwort, dazu Erstellt am, Zuletzt geändert, Letzte Anmeldung.

**Löschen**

- Dialog: **„Benutzer „<Anzeigename>“ wirklich löschen? Das kann nicht rückgängig gemacht werden.“**
- Inhalte (Artikel, Events, Produkte) bleiben erhalten. *(Es gibt bisher keine Autor-Verknüpfung.)*
- Nach dem Löschen: Weiterleitung zur Liste mit „Benutzer gelöscht.“

**Schutzregeln**

- Es muss immer **mindestens ein aktiver Admin** bleiben. Löschen, Sperren oder Entzug der Admin-Rolle beim letzten aktiven Admin wird abgelehnt: „Es muss mindestens ein aktiver Admin bleiben.“
- Ein Admin kann **sich selbst** weder löschen noch sperren: „Du kannst dein eigenes Konto nicht löschen oder sperren.“

### C2. Eigenes Konto – `/konto`

- Zeigt Anzeigename, E-Mail und Rollen.
- Anzeigename ändern (gleiche Regeln). E-Mail und Rollen ändert nur ein Admin.
- **Passwort ändern – `/konto/passwort`:** Felder *Aktuelles Passwort*, *Neues Passwort*, *Neues Passwort wiederholen*. Bei erzwungener Änderung entfällt *Aktuelles Passwort* nicht – es ist das Startpasswort.
- Nach Erfolg: „Passwort geändert.“, *Passwort ändern erzwingen* wird zurückgesetzt, alle anderen Sitzungen enden.

### C3. Erster Admin

- Gibt es beim Serverstart **keinen** Benutzer und sind `ADMIN_EMAIL` und `ADMIN_PASSWORD` gesetzt, legt der Server damit einen Admin an (mit *Passwort ändern erzwingen*) und schreibt ins Log: „Erster Admin <E-Mail> angelegt.“ Das Passwort erscheint nicht im Log.
- Gibt es schon Benutzer, werden die Variablen ignoriert.
- Gibt es keinen Benutzer und fehlen die Variablen, startet der Server trotzdem und warnt im Log: „Kein Benutzer vorhanden – ADMIN_EMAIL und ADMIN_PASSWORD setzen.“ Verwaltungsseiten sind dann für niemanden erreichbar.
- Die Variablen stehen in `.env.example` und beim Dienst `app` in `docker-compose.yml` (Regel ANF-010).

### C4. Fehlertexte

| Fall | Meldung |
|---|---|
| E-Mail leer | „Bitte eine E-Mail-Adresse eingeben.“ |
| E-Mail ungültig / zu lang | „Bitte eine gültige E-Mail-Adresse eingeben.“ |
| E-Mail schon vergeben | „Diese E-Mail-Adresse wird schon verwendet.“ |
| Anzeigename leer / zu lang | „Bitte einen Anzeigenamen eingeben.“ / „Der Anzeigename darf höchstens 80 Zeichen lang sein.“ |
| Keine Rolle gewählt | „Bitte mindestens eine Rolle wählen.“ |
| Passwort zu kurz / zu lang | „Das Passwort muss mindestens 12 Zeichen lang sein.“ / „Das Passwort darf höchstens 128 Zeichen lang sein.“ |
| Passwort gleich E-Mail | „Das Passwort darf nicht die E-Mail-Adresse sein.“ |
| Wiederholung passt nicht | „Die Passwörter stimmen nicht überein.“ |
| Aktuelles Passwort falsch | „Das aktuelle Passwort ist falsch.“ |
| Letzter Admin | „Es muss mindestens ein aktiver Admin bleiben.“ |
| Eigenes Konto löschen/sperren | „Du kannst dein eigenes Konto nicht löschen oder sperren.“ |

---

## Teil D – Zusammenspiel mit der PWA (ANF-011)

- `/anmelden`, `/abmelden`, `/konto*` und `/benutzer*` stehen im Service Worker in `NEVER` und werden nie gecacht. `VERSION` in `public/sw.js` erhöhen.
- Seiten mit Verwaltungs- oder Kontodaten dürfen nicht im Cache landen: Antworten mit Sitzung bekommen `Cache-Control: private, no-store`.
- Offline zeigt das Konto-Menü „Nur online möglich“ (wie heute das Redaktions-Menü).

---

## Abnahmekriterien

| Nr. | Gegeben | Wenn | Dann |
|---|---|---|---|
| AK-1 | ein aktives Konto | mit richtiger E-Mail und richtigem Passwort angemeldet wird | ist man angemeldet, die Kopfzeile zeigt den Anzeigenamen, `last_login_at` ist gesetzt |
| AK-2 | ein Konto mit E-Mail `Anna@Example.ch` | mit `anna@example.ch` angemeldet wird | klappt die Anmeldung (Groß-/Kleinschreibung egal) |
| AK-3 | falsches Passwort, unbekannte E-Mail oder gesperrtes Konto | angemeldet wird | erscheint jeweils genau „E-Mail oder Passwort ist falsch.“, es entsteht keine Sitzung |
| AK-4 | 5 Fehlversuche für eine E-Mail in 15 Minuten | ein 6. Versuch (auch mit richtigem Passwort) kommt | erscheint „Zu viele Versuche. …“, es entsteht keine Sitzung |
| AK-5 | nicht angemeldet | `/artikel/neu` aufgerufen wird | Weiterleitung zu `/anmelden?ziel=/artikel/neu`; nach dem Login landet man dort |
| AK-6 | `ziel=https://boese.example` oder `ziel=//boese.example` | angemeldet wird | landet man auf der Startseite, nicht auf der fremden Seite |
| AK-7 | angemeldet mit Rolle *Shop* | `/artikel` oder `/benutzer` aufgerufen wird | erscheint „Kein Zugriff“ mit Status 403 |
| AK-8 | angemeldet mit Rolle *Redaktion* | `/produkte/neu` aufgerufen wird | erscheint „Kein Zugriff“ (403); das Menü zeigt keinen Eintrag „Produkte“ |
| AK-9 | nicht angemeldet bzw. ohne passende Rolle | eine ändernde Server-Funktion (z. B. Artikel löschen) direkt aufgerufen wird | antwortet sie mit 401 bzw. 403, in der Datenbank ändert sich nichts |
| AK-10 | nicht angemeldet | Startseite, News, Eventliste, Radio, Shop und Warenkorb geöffnet werden | funktionieren alle wie bisher |
| AK-11 | angemeldet | „Abmelden“ geklickt wird | ist die Sitzung in der DB gelöscht, das Cookie entfernt; Verwaltungsseiten leiten wieder zum Login |
| AK-12 | das Sitzungs-Cookie | im Browser angesehen wird | ist es `HttpOnly` und `SameSite=Lax` (im Betrieb auch `Secure`); in der DB steht nur ein Hash des Tokens |
| AK-13 | eine Sitzung, 14 Tage ohne Aktivität | eine Verwaltungsseite aufgerufen wird | gilt man als nicht angemeldet |
| AK-14 | ein Redakteur ist angemeldet | ein Admin ihm die Rolle *Redaktion* entzieht | bekommt er bei der nächsten Anfrage 403, ohne sich neu anzumelden |
| AK-15 | eine Person ist auf zwei Geräten angemeldet | ein Admin das Konto sperrt | sind beide Sitzungen beendet, eine neue Anmeldung scheitert (AK-3) |
| AK-16 | Admin | einen Benutzer mit gültigen Daten anlegt | erscheint er in `/benutzer`; beim ersten Login muss er ein eigenes Passwort setzen |
| AK-17 | Admin | einen Benutzer mit einer schon vergebenen E-Mail (andere Schreibweise) anlegt | erscheint „Diese E-Mail-Adresse wird schon verwendet.“, nichts wird gespeichert |
| AK-18 | ungültige Felder (siehe C4) | gespeichert wird | erscheint beim jeweiligen Feld die Meldung, die Eingaben bleiben (außer Passwörtern), nichts wird gespeichert |
| AK-19 | genau ein aktiver Admin | er gelöscht, gesperrt oder ihm die Admin-Rolle entzogen werden soll | erscheint „Es muss mindestens ein aktiver Admin bleiben.“, nichts ändert sich |
| AK-20 | ein angemeldeter Admin | er sein eigenes Konto löschen oder sperren will | erscheint „Du kannst dein eigenes Konto nicht löschen oder sperren.“ |
| AK-21 | angemeldet, auf zwei Geräten | auf Gerät 1 das Passwort geändert wird | bleibt Gerät 1 angemeldet, Gerät 2 ist abgemeldet; das alte Passwort funktioniert nicht mehr |
| AK-22 | ein Passwort wird gesetzt | die Tabelle `users` und das Server-Log angesehen werden | steht dort nur ein Argon2id-Hash bzw. gar nichts, nie das Passwort |
| AK-23 | leere Tabelle `users`, `ADMIN_EMAIL`/`ADMIN_PASSWORD` gesetzt | der Server startet | gibt es genau einen Admin; ein zweiter Start legt keinen weiteren an |
| AK-24 | eine ändernde Anfrage mit fremdem `Origin`-Header und gültigem Cookie | beim Server ankommt | wird sie abgelehnt (403), nichts ändert sich |
| AK-25 | die installierte PWA | `/benutzer` oder `/konto` online aufgerufen und dann offline gegangen wird | sind diese Seiten nicht aus dem Cache abrufbar (Offline-Seite erscheint) |
| AK-26 | ungültige Werte | direkt in die Datenbank geschrieben werden | lehnt die Datenbank leere E-Mails/Anzeigenamen, doppelte E-Mails (ohne Groß-/Kleinschreibung) und Konten ohne Rolle ab (CHECK/UNIQUE-Constraints) |

## Annahmen (bitte bestätigen)

- **Keine Selbstregistrierung:** Konten legt nur ein Admin an. Fan-Konten für „Folge deinen DJs“, „Merken“ und „Warteliste“ (ANF-008, ANF-006, ANF-009) kommen in einer eigenen Anforderung und bauen auf dieser auf.
- **Kein E-Mail-Versand:** Passwort vergessen → ein Admin setzt ein neues Startpasswort (C1). Ein Link „Passwort vergessen“ per E-Mail braucht einen Mail-Dienst und ist eine eigene Anforderung.
- **Rollen statt Einzelrechte:** Drei feste Rollen reichen. Wer einen Artikel geschrieben hat, wird (noch) nicht gespeichert; alle mit Rolle *Redaktion* dürfen alle Artikel ändern.
- **Mindestlänge 12 Zeichen**, sonst keine Regeln für Ziffern/Sonderzeichen (Empfehlung NIST SP 800-63B).
- **Sitzung in PostgreSQL**, kein JWT: so lassen sich Sitzungen sofort beenden.
- **Zwei-Faktor-Anmeldung** ist vorerst nicht nötig.
- `/status` war bisher öffentlich und wird nur noch für Admins angezeigt; `/health` (Docker/Railway) bleibt öffentlich.

## Nicht Teil von ANF-012

- Selbstregistrierung, Fan-Konten, Profile für Besucher:innen
- „Passwort vergessen“ per E-Mail, E-Mail-Bestätigung
- Login mit Google/Apple o. Ä. (OAuth), Zwei-Faktor-Anmeldung
- Autor:in pro Artikel/Event, Änderungsprotokoll („wer hat was geändert“)
- Feinere Rechte (z. B. „nur eigene Artikel bearbeiten“)
- Bestellungen und Kundenkonten im Shop

## Umsetzung (Vorschlag)

| Teil | Ort |
|---|---|
| Migration `users` (`email TEXT NOT NULL`, `UNIQUE INDEX ON lower(email)`, `display_name`, `password_hash`, `roles TEXT[] NOT NULL CHECK (cardinality(roles) > 0 AND roles <@ ARRAY['admin','redaktion','shop'])`, `active BOOL`, `must_change_password BOOL`, `last_login_at`, Zeitstempel) | `migrations/<datum>_users.sql` |
| Migration `sessions` (`token_hash BYTEA PRIMARY KEY`, `user_id` → `users ON DELETE CASCADE`, `created_at`, `last_seen_at`, `expires_at`) und `login_attempts` (E-Mail, Zeitpunkt) | `migrations/<datum>_sessions.sql` |
| Modell, Prüfregeln, Rollen-Enum (Server+Browser) | `src/users/model.rs` |
| Server-Funktionen (Login, Logout, Konto, Benutzer-CRUD) | `src/users/api.rs` |
| SQL | `src/users/repo.rs` |
| Passwort-Hash (Crate `argon2`), Token (`rand`/`getrandom`, Hash mit `sha2`), Cookie lesen/setzen über `leptos_axum::extract` und `ResponseOptions`, Helfer `require_role(Role::Redaktion)?` für alle Server-Funktionen | `src/auth.rs` (nur ssr) |
| Origin-Prüfung als Axum-Middleware, erster Admin beim Start | `src/server.rs`, `src/main.rs` |
| Seiten | `src/pages/anmelden.rs`, `konto.rs`, `konto_passwort.rs`, `user_list.rs`, `user_detail.rs`, `user_new.rs`, `user_edit.rs`, `kein_zugriff.rs` |
| Kopfzeile mit Konto-Menü | `src/components/navbar.rs` |
| Bestehende Server-Funktionen absichern (Artikel, Events, Radio, Produkte) | `src/*/api.rs` |
| Service Worker: `NEVER` ergänzen, `VERSION` erhöhen | `public/sw.js` |
| `ADMIN_EMAIL`, `ADMIN_PASSWORD` | `.env.example`, `docker-compose.yml` |
| Tests | Unit-Tests in `model.rs`, DB- und Router-Tests in `tests/anf_012.rs` |
