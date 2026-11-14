// Saintwhoo – Service Worker (ANF-011, Teil B).
//
// Reines JavaScript, wird unverändert aus public/ ausgeliefert (Cache-Control: no-cache).
// Bei jeder Änderung an dieser Datei VERSION erhöhen: Beim `activate` werden
// alle Caches gelöscht, die nicht zur aktuellen Version gehören.

const VERSION = "v1";
const SHELL = `saintwhoo-${VERSION}`; // App-Hülle, Seiten, /pkg
const IMAGES = `saintwhoo-bilder-${VERSION}`; // /images/*, max. 60 Einträge
const FONTS = `saintwhoo-fonts-${VERSION}`; // Google Fonts
const CACHES = [SHELL, IMAGES, FONTS];
const MAX_IMAGES = 60;

// Precache: App-Hülle, die offline sofort da sein muss
const PRECACHE = [
  "/offline",
  "/manifest.webmanifest",
  "/pwa.js",
  "/icons/favicon.svg",
  "/icons/icon-192.png",
  "/icons/icon-512.png",
  "/icons/icon-maskable-512.png",
  "/icons/apple-touch-icon.png",
];

// Nie anfassen (kein respondWith): Daten, Formulare, Betriebsprüfung
const NEVER = ["/api/", "/health", "/status", "/sw.js"];

self.addEventListener("install", (event) => {
  event.waitUntil(
    (async () => {
      const cache = await caches.open(SHELL);
      await cache.addAll(PRECACHE);
      // CSS/JS/WASM der Offline-Seite mitnehmen, damit sie offline gestylt ist.
      // Die Namen stehen im HTML (so klappt es auch mit Hash im Dateinamen).
      try {
        const html = await (await cache.match("/offline")).text();
        const assets = [...new Set(html.match(/\/pkg\/[^"'\s)]+/g) || [])];
        await Promise.all(assets.map((a) => cache.add(a).catch(() => {})));
      } catch (_) {
        // nicht schlimm: /pkg wird beim nächsten Online-Besuch gecacht
      }
    })(),
  );
  // Kein skipWaiting hier: erst, wenn die Person „Neu laden“ tippt (siehe message)
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    (async () => {
      const keys = await caches.keys();
      await Promise.all(
        keys.filter((k) => k.startsWith("saintwhoo-") && !CACHES.includes(k)).map((k) => caches.delete(k)),
      );
      await self.clients.claim();
    })(),
  );
});

self.addEventListener("message", (event) => {
  if (event.data && event.data.type === "SKIP_WAITING") self.skipWaiting();
});

self.addEventListener("fetch", (event) => {
  const req = event.request;
  if (req.method !== "GET") return; // POST/PUT/DELETE: immer Netz
  if (req.headers.has("range")) return; // Audio/Video in Stücken
  if (req.destination === "audio" || req.destination === "video") return; // Radio-Stream, Mixe

  const url = new URL(req.url);

  if (url.origin !== self.location.origin) {
    if (url.hostname === "fonts.googleapis.com" || url.hostname === "fonts.gstatic.com") {
      event.respondWith(cacheFirst(req, FONTS));
    }
    return; // andere fremde Herkünfte (z. B. Livestream) nicht anfassen
  }

  if (NEVER.some((p) => url.pathname === p || url.pathname.startsWith(p))) return;
  if (url.pathname.startsWith("/audio/")) return;

  if (req.mode === "navigate") {
    event.respondWith(page(req, url));
  } else if (url.pathname.startsWith("/pkg/")) {
    // Network first statt stale-while-revalidate: Das HTML kommt frisch vom Netz,
    // CSS/JS/WASM ohne Hash müssen genau dazu passen, sonst scheitert die Hydration.
    event.respondWith(networkFirst(req, SHELL));
  } else if (url.pathname.startsWith("/images/")) {
    event.respondWith(cacheFirst(req, IMAGES, MAX_IMAGES));
  } else if (url.pathname.startsWith("/icons/") || url.pathname === "/manifest.webmanifest" || url.pathname === "/pwa.js") {
    event.respondWith(cacheFirst(req, SHELL));
  }
  // alles andere: Browser-Standard
});

/// Seiten: Network first → Cache → Offline-Seite (per Umleitung, damit die
/// Hydration zur Adresse /offline passt; `ziel` merkt sich die gewünschte Seite).
async function page(req, url) {
  try {
    const res = await fetch(req);
    if (res.ok && res.type === "basic") {
      const copy = res.clone();
      caches.open(SHELL).then((c) => c.put(req, copy));
    }
    return res;
  } catch (_) {
    const hit = (await caches.match(req)) || (await caches.match(req, { ignoreSearch: true }));
    if (hit) return hit;
    if (url.pathname === "/offline") {
      return (await caches.match("/offline")) || Response.error();
    }
    const ziel = encodeURIComponent(url.pathname + url.search);
    return Response.redirect(`/offline?ziel=${ziel}`, 302);
  }
}

async function networkFirst(req, cacheName) {
  try {
    const res = await fetch(req);
    if (res.ok) {
      const copy = res.clone();
      caches.open(cacheName).then((c) => c.put(req, copy));
    }
    return res;
  } catch (err) {
    const hit = await caches.match(req);
    if (hit) return hit;
    throw err;
  }
}

async function cacheFirst(req, cacheName, maxEntries) {
  const cache = await caches.open(cacheName);
  const hit = await cache.match(req);
  if (hit) return hit;
  const res = await fetch(req);
  // opaque (Fonts ohne CORS) ist ok, Fehlerseiten nicht
  if (res.ok || res.type === "opaque") {
    await cache.put(req, res.clone());
    if (maxEntries) trim(cache, maxEntries);
  }
  return res;
}

/// Älteste Einträge entfernen (keys() liefert in Einfügereihenfolge).
async function trim(cache, maxEntries) {
  const keys = await cache.keys();
  for (const k of keys.slice(0, Math.max(0, keys.length - maxEntries))) {
    await cache.delete(k);
  }
}
