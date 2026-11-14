//! ANF-011: PWA-Bedienelemente – Offline-Badge, „App installieren“,
//! Hinweis „Neue Version verfügbar“, iOS-Hinweis, Media Session.
//!
//! Die Logik im Browser steckt in `public/pwa.js` (`window.saintwhooPwa`).
//! Hier nur die Brücke dorthin. Auf dem Server und vor der Hydration gelten
//! die Startwerte (online, nichts zu installieren) – so passt das SSR-HTML
//! zur Hydration, und ohne JavaScript ändert sich nichts (AK-12).

use leptos::prelude::*;

/// Brücke zu `window.saintwhooPwa`. Fehlt das Skript, liefern alle Aufrufe
/// die Startwerte (`catch` fängt den JS-Fehler ab).
#[cfg(feature = "hydrate")]
mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = online)]
        fn online_js() -> Result<bool, JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = canInstall)]
        fn can_install_js() -> Result<bool, JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = install)]
        fn install_js() -> Result<(), JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = updateReady)]
        fn update_ready_js() -> Result<bool, JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = applyUpdate)]
        fn apply_update_js() -> Result<(), JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = iosHint)]
        fn ios_hint_js() -> Result<bool, JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = dismissIosHint)]
        fn dismiss_ios_hint_js() -> Result<(), JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = cachedPages)]
        fn cached_pages_js() -> Result<js_sys::Promise, JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = media)]
        fn media_js(title: &str, artist: &str, album: &str) -> Result<(), JsValue>;
        #[wasm_bindgen(catch, js_namespace = saintwhooPwa, js_name = clearMedia)]
        fn clear_media_js() -> Result<(), JsValue>;
    }

    pub fn online() -> bool {
        online_js().unwrap_or(true)
    }
    pub fn can_install() -> bool {
        can_install_js().unwrap_or(false)
    }
    pub fn install() {
        let _ = install_js();
    }
    pub fn update_ready() -> bool {
        update_ready_js().unwrap_or(false)
    }
    pub fn apply_update() {
        let _ = apply_update_js();
    }
    pub fn ios_hint() -> bool {
        ios_hint_js().unwrap_or(false)
    }
    pub fn dismiss_ios_hint() {
        let _ = dismiss_ios_hint_js();
    }
    pub fn media(title: &str, artist: &str, album: &str) {
        let _ = media_js(title, artist, album);
    }
    pub fn clear_media() {
        let _ = clear_media_js();
    }
    pub async fn cached_pages() -> Vec<String> {
        let Ok(promise) = cached_pages_js() else { return Vec::new() };
        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(v) => js_sys::Array::from(&v).iter().filter_map(|p| p.as_string()).collect(),
            Err(_) => Vec::new(),
        }
    }
}

/// Zustand der PWA im Kontext von `App`.
#[derive(Clone, Copy)]
pub struct Pwa {
    pub online: RwSignal<bool>,
    pub can_install: RwSignal<bool>,
    pub update_ready: RwSignal<bool>,
    pub ios_hint: RwSignal<bool>,
}

impl Pwa {
    /// Legt den Zustand an und hält ihn im Browser aktuell.
    pub fn provide() {
        let pwa = Pwa {
            online: RwSignal::new(true),
            can_install: RwSignal::new(false),
            update_ready: RwSignal::new(false),
            ios_hint: RwSignal::new(false),
        };
        provide_context(pwa);

        // Effekte laufen nur im Browser, nach der Hydration
        Effect::new(move |_| {
            #[cfg(feature = "hydrate")]
            {
                let refresh = move || {
                    pwa.online.set(js::online());
                    pwa.can_install.set(js::can_install());
                    pwa.update_ready.set(js::update_ready());
                    pwa.ios_hint.set(js::ios_hint());
                };
                refresh();
                for ev in ["saintwhoo-pwa", "online", "offline"] {
                    // Listener leben so lange wie die App
                    let _ = window_event_listener_untyped(ev, move |_| refresh());
                }
            }
        });
    }
}

pub fn use_pwa() -> Pwa {
    expect_context::<Pwa>()
}

/// Kleines Badge „Offline“ in der Kopfzeile (AK-13).
#[component]
pub fn OfflineBadge() -> impl IntoView {
    let pwa = use_pwa();
    view! {
        <Show when=move || !pwa.online.get()>
            <span role="status" class="badge badge-warning badge-sm font-semibold">"Offline"</span>
        </Show>
    }
    .into_any()
}

/// Knopf „App installieren“ – nur sichtbar, wenn der Browser `beforeinstallprompt`
/// gesendet hat und die App nicht schon installiert läuft.
#[component]
pub fn InstallButton() -> impl IntoView {
    let pwa = use_pwa();
    let install = move |_| {
        #[cfg(feature = "hydrate")]
        js::install();
    };
    view! {
        <Show when=move || pwa.can_install.get()>
            <button type="button" on:click=install class="btn btn-primary btn-sm min-h-11 px-3 sm:px-4" aria-label="App installieren">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                    <path d="M12 4v11M7 10l5 5 5-5M5 20h14" />
                </svg>
                <span class="sm:hidden">"App"</span>
                <span class="hidden sm:inline">"App installieren"</span>
            </button>
        </Show>
    }
    .into_any()
}

/// Hinweis unten, wenn ein neuer Service Worker bereitsteht.
#[component]
pub fn UpdateToast() -> impl IntoView {
    let pwa = use_pwa();
    let reload = move |_| {
        #[cfg(feature = "hydrate")]
        js::apply_update();
    };
    view! {
        <Show when=move || pwa.update_ready.get()>
            <div class="toast toast-center z-40 mb-24 md:mb-4">
                <div role="status" class="alert border border-base-300 bg-base-200">
                    <span>"Neue Version verfügbar"</span>
                    <button type="button" on:click=reload class="btn btn-primary btn-sm min-h-11">"Neu laden"</button>
                </div>
            </div>
        </Show>
    }
    .into_any()
}

/// iOS/Safari: einmaliger, schließbarer Hinweis „Teilen → Zum Home-Bildschirm“.
#[component]
pub fn IosInstallHint() -> impl IntoView {
    let pwa = use_pwa();
    let close = move |_| {
        pwa.ios_hint.set(false);
        #[cfg(feature = "hydrate")]
        js::dismiss_ios_hint();
    };
    view! {
        <Show when=move || pwa.ios_hint.get()>
            <div role="note" class="mb-6 flex items-start gap-3 rounded-box border border-base-300 bg-base-200 p-4">
                <p class="flex-1 text-sm">
                    <span class="font-semibold">"Saintwhoo als App: "</span>
                    <span class="text-muted">"In Safari auf „Teilen“ tippen und „Zum Home-Bildschirm“ wählen."</span>
                </p>
                <button type="button" on:click=close class="btn btn-ghost btn-circle size-11" aria-label="Hinweis schließen">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
                        <path d="M6 6l12 12M18 6L6 18" />
                    </svg>
                </button>
            </div>
        </Show>
    }
    .into_any()
}

/// Media Session: Titel auf dem Sperrbildschirm, Play/Pause steuern das `<audio>`.
pub fn set_media_session(_title: Option<(String, String)>) {
    #[cfg(feature = "hydrate")]
    match _title {
        Some((title, subtitle)) => js::media(&title, "Saintwhoo Radio", &subtitle),
        None => js::clear_media(),
    }
}

/// Pfade der Seiten, die offline im Cache liegen (leer auf dem Server).
pub async fn cached_pages() -> Vec<String> {
    #[cfg(feature = "hydrate")]
    {
        js::cached_pages().await
    }
    #[cfg(not(feature = "hydrate"))]
    {
        Vec::new()
    }
}
