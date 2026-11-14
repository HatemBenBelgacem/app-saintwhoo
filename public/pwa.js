// Saintwhoo – PWA-Helfer im Browser (ANF-011).
//
// Wird im <head> geladen (vor der Hydration), damit kein `beforeinstallprompt`
// verloren geht. Die Leptos-Komponenten in src/components/pwa.rs lesen den
// Zustand über `window.saintwhooPwa` und hören auf das Ereignis „saintwhoo-pwa“.
// Ohne Service-Worker-Unterstützung oder ohne JavaScript läuft die Seite wie bisher.
(function () {
  "use strict";

  var installEvent = null; // aufgehobenes beforeinstallprompt
  var waiting = null; // neuer Service Worker, der auf „Neu laden“ wartet
  var reloading = false;
  var IOS_KEY = "saintwhoo-ios-hinweis";

  function changed() {
    window.dispatchEvent(new Event("saintwhoo-pwa"));
  }

  function standalone() {
    return (
      (window.matchMedia && window.matchMedia("(display-mode: standalone)").matches) ||
      window.navigator.standalone === true
    );
  }

  // --- Service Worker -----------------------------------------------------
  if ("serviceWorker" in navigator) {
    var hadController = !!navigator.serviceWorker.controller;

    window.addEventListener("load", function () {
      navigator.serviceWorker
        .register("/sw.js", { scope: "/" })
        .then(function (reg) {
          function track(sw) {
            if (!sw) return;
            sw.addEventListener("statechange", function () {
              if (sw.state === "installed" && navigator.serviceWorker.controller) {
                waiting = sw;
                changed();
              }
            });
          }
          if (reg.waiting && navigator.serviceWorker.controller) {
            waiting = reg.waiting;
            changed();
          }
          track(reg.installing);
          reg.addEventListener("updatefound", function () {
            track(reg.installing);
          });
        })
        .catch(function () {
          /* z. B. kein HTTPS – Seite funktioniert trotzdem */
        });
    });

    // Neuer Worker hat übernommen → einmal neu laden (nicht beim allerersten Start)
    navigator.serviceWorker.addEventListener("controllerchange", function () {
      if (!hadController || reloading) return;
      reloading = true;
      window.location.reload();
    });
  }

  // --- Installation -------------------------------------------------------
  window.addEventListener("beforeinstallprompt", function (e) {
    e.preventDefault(); // kein Pop-up beim ersten Besuch; Knopf „App installieren“
    installEvent = e;
    changed();
  });
  window.addEventListener("appinstalled", function () {
    installEvent = null;
    changed();
  });

  // --- Offline: Links als echte Navigation laden ----------------------------
  // Der Client-Router würde sonst Daten vom Server holen und scheitern. Als echte
  // Navigation liefert der Service Worker die Seite aus dem Cache oder /offline.
  document.addEventListener(
    "click",
    function (e) {
      if (navigator.onLine || e.defaultPrevented || e.button !== 0) return;
      if (e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
      var a = e.target && e.target.closest && e.target.closest("a[href]");
      if (!a || a.target || a.hasAttribute("download")) return;
      var url = new URL(a.href, window.location.href);
      if (url.origin !== window.location.origin) return;
      if (url.hash && url.pathname === window.location.pathname && url.search === window.location.search) return;
      e.preventDefault();
      window.location.assign(url.href);
    },
    true,
  );

  // --- Schnittstelle für die Leptos-Komponenten -----------------------------
  window.saintwhooPwa = {
    online: function () {
      return navigator.onLine !== false;
    },
    canInstall: function () {
      return !!installEvent && !standalone();
    },
    install: function () {
      if (!installEvent) return;
      var e = installEvent;
      e.prompt();
      e.userChoice.finally(function () {
        installEvent = null;
        changed();
      });
    },
    updateReady: function () {
      return !!waiting;
    },
    applyUpdate: function () {
      if (!waiting) return;
      waiting.postMessage({ type: "SKIP_WAITING" });
    },
    // iOS/Safari kennt kein beforeinstallprompt → einmaliger Hinweis
    iosHint: function () {
      var ua = navigator.userAgent || "";
      var ios = /iphone|ipad|ipod/i.test(ua) || (/macintosh/i.test(ua) && navigator.maxTouchPoints > 1);
      if (!ios || standalone()) return false;
      try {
        return localStorage.getItem(IOS_KEY) !== "zu";
      } catch (_) {
        return false;
      }
    },
    dismissIosHint: function () {
      try {
        localStorage.setItem(IOS_KEY, "zu");
      } catch (_) {}
      changed();
    },
    // Pfade der Seiten, die offline im Cache liegen (für /offline)
    cachedPages: function () {
      if (!("caches" in window)) return Promise.resolve([]);
      return caches.keys().then(function (keys) {
        var shell = keys.filter(function (k) {
          return /^saintwhoo-v/.test(k);
        });
        return Promise.all(
          shell.map(function (k) {
            return caches.open(k).then(function (c) {
              return c.keys();
            });
          }),
        ).then(function (lists) {
          var seen = {};
          var out = [];
          lists.forEach(function (reqs) {
            reqs.forEach(function (r) {
              var u = new URL(r.url);
              var p = u.pathname;
              if (/\.[a-z0-9]+$/i.test(p) || p.indexOf("/pkg/") === 0 || p === "/offline") return;
              if (!seen[p]) {
                seen[p] = true;
                out.push(p);
              }
            });
          });
          return out.sort();
        });
      });
    },
    // Media Session: Sperrbildschirm zeigt Titel und steuert Play/Pause
    media: function (title, artist, album) {
      if (!("mediaSession" in navigator)) return;
      try {
        navigator.mediaSession.metadata = new MediaMetadata({
          title: title,
          artist: artist,
          album: album || "",
          artwork: [
            { src: "/icons/icon-192.png", sizes: "192x192", type: "image/png" },
            { src: "/icons/icon-512.png", sizes: "512x512", type: "image/png" },
          ],
        });
        var audio = function () {
          return document.querySelector("audio");
        };
        navigator.mediaSession.setActionHandler("play", function () {
          var a = audio();
          if (a) a.play();
        });
        navigator.mediaSession.setActionHandler("pause", function () {
          var a = audio();
          if (a) a.pause();
        });
      } catch (_) {}
    },
    clearMedia: function () {
      if ("mediaSession" in navigator) navigator.mediaSession.metadata = null;
    },
  };
})();
