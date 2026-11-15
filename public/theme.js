// Saintwhoo – Hell/Dunkel-Umschalter (Designvorschlag „Techno App“).
//
// Wird im <head> synchron geladen, damit das gespeicherte Theme schon vor dem
// ersten Zeichnen gilt (kein Aufblitzen). Standard ist dunkel – auch bei hellem
// System. Die Wahl liegt nur im Browser (localStorage). Die Leptos-Komponente
// `ThemeToggle` (src/components/theme.rs) ruft `window.saintwhooTheme` auf.
(function () {
  "use strict";

  var KEY = "saintwhoo-theme";
  var THEMES = { dunkel: "saintwhoo", hell: "saintwhoo-hell" };
  var BAR = { dunkel: "#0B0B0C", hell: "#F4F4EF" };

  function stored() {
    try {
      return localStorage.getItem(KEY) === "hell" ? "hell" : "dunkel";
    } catch (_) {
      return "dunkel";
    }
  }

  function apply(name) {
    document.documentElement.setAttribute("data-theme", THEMES[name]);
    var meta = document.querySelector('meta[name="theme-color"]');
    if (meta) meta.setAttribute("content", BAR[name]);
  }

  // Steht im <head> direkt nach <meta name="theme-color">
  var current = stored();
  if (current === "hell") apply(current);

  window.saintwhooTheme = {
    // "dunkel" oder "hell"
    get: function () {
      return current;
    },
    // Wechselt das Theme und gibt das neue zurück
    toggle: function () {
      current = current === "hell" ? "dunkel" : "hell";
      apply(current);
      try {
        localStorage.setItem(KEY, current);
      } catch (_) {
        // privates Fenster o. Ä.: gilt dann nur bis zum Neuladen
      }
      return current;
    },
  };
})();
