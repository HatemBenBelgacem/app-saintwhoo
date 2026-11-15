//! Hell/Dunkel-Umschalter oben rechts (Designvorschlag „Techno App“).
//!
//! Die Logik im Browser steckt in `public/theme.js` (`window.saintwhooTheme`):
//! Es setzt `data-theme` am `<html>` schon vor dem ersten Zeichnen und merkt
//! sich die Wahl im `localStorage`. Hier nur der Knopf. Auf dem Server und vor
//! der Hydration gilt „dunkel“ – so passt das SSR-HTML zur Hydration.

use leptos::prelude::*;

#[cfg(feature = "hydrate")]
mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(catch, js_namespace = saintwhooTheme, js_name = get)]
        fn get_js() -> Result<String, JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooTheme, js_name = toggle)]
        fn toggle_js() -> Result<String, JsValue>;
    }

    /// Ist gerade das helle Theme aktiv? Fehlt das Skript: dunkel.
    pub fn hell() -> bool {
        get_js().is_ok_and(|t| t == "hell")
    }

    /// Wechselt das Theme und gibt zurück, ob jetzt hell aktiv ist.
    pub fn toggle() -> bool {
        toggle_js().is_ok_and(|t| t == "hell")
    }
}

/// Runder Knopf (44 px) mit halb gefülltem Kreis.
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let hell = RwSignal::new(false);

    // Nach der Hydration den echten Zustand aus theme.js übernehmen
    #[cfg(feature = "hydrate")]
    Effect::new(move |_| hell.set(js::hell()));

    let on_click = move |_| {
        #[cfg(feature = "hydrate")]
        hell.set(js::toggle());
    };

    let label = move || if hell.get() { "Dunklen Modus aktivieren" } else { "Hellen Modus aktivieren" };

    view! {
        <button
            type="button"
            on:click=on_click
            aria-label=label
            title=label
            class="btn btn-circle btn-outline size-11 min-h-11 border-base-300 bg-base-200"
        >
            <svg
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                aria-hidden="true"
            >
                <circle cx="12" cy="12" r="9" />
                <path d="M12 3a9 9 0 0 1 0 18z" fill="currentColor" />
            </svg>
        </button>
    }
    .into_any()
}
