//! Offline-Seite – `/offline` (ANF-011, AK-6/AK-7).
//!
//! Der Service Worker leitet ohne Netz auf `/offline?ziel=<Pfad>` um, wenn die
//! gewünschte Seite nicht im Cache liegt. Braucht keine Datenbank.

use leptos::html;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::components::pwa::cached_pages;

/// Nur Pfade auf der eigenen Seite zulassen (kein `//fremd.example`).
pub fn safe_target(ziel: Option<String>) -> String {
    match ziel {
        Some(z) if z.starts_with('/') && !z.starts_with("//") && !z.starts_with("/\\") => z,
        _ => "/".to_string(),
    }
}

/// Lesbarer Name für einen Pfad aus dem Cache.
fn page_label(path: &str) -> String {
    match path {
        "/" => "Startseite".into(),
        "/news" => "News".into(),
        "/eventliste" => "Events".into(),
        "/radio" => "Radio".into(),
        "/shop" => "Shop".into(),
        "/warenkorb" => "Warenkorb".into(),
        p if p.starts_with("/beitrag/") => format!("News-Beitrag {}", &p["/beitrag/".len()..]),
        p if p.starts_with("/eventliste/") => format!("Event {}", &p["/eventliste/".len()..]),
        p if p.starts_with("/shop/") => format!("Produkt {}", &p["/shop/".len()..]),
        p => p.to_string(),
    }
}

#[component]
pub fn OfflinePage() -> impl IntoView {
    let query = use_query_map();
    let target = move || safe_target(query.with(|q| q.get("ziel")));

    // Offline liefert der Service Worker die vorab gecachte `/offline` (ohne `ziel`).
    // Die Hydration übernimmt das `href` aus diesem HTML, darum hier nachziehen.
    let retry = NodeRef::<html::A>::new();
    Effect::new(move |_| {
        if let Some(a) = retry.get() {
            let _ = a.set_attribute("href", &target());
        }
    });

    // Im Browser: welche Seiten liegen im Cache?
    let pages = RwSignal::new(Vec::<String>::new());
    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            pages.set(cached_pages().await);
        });
    });

    view! {
        <div class="mx-auto max-w-xl py-10 text-center">
            <svg class="mx-auto mb-6 text-primary" width="56" height="56" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <path d="M2 2l20 20M8.5 16.5a5 5 0 0 1 7 0M5 12.9a10 10 0 0 1 5.2-2.8M19 12.9a10 10 0 0 0-2.3-1.6M2 8.8a15 15 0 0 1 4.2-2.6M22 8.8A15 15 0 0 0 11 4.9" />
                <circle cx="12" cy="20" r="0.5" fill="currentColor" />
            </svg>
            <h1 class="font-display text-4xl font-bold">"Du bist offline"</h1>
            <p class="mt-3 text-muted">"Diese Seite ist ohne Netz nicht verfügbar. Zuletzt besuchte Seiten kannst du trotzdem öffnen."</p>
            // rel=external: echte Navigation, damit der Server (oder der Cache) die Seite liefert
            <a node_ref=retry href=target rel="external" class="btn btn-primary mt-6 min-h-11 px-6">"Erneut versuchen"</a>

            <Show when=move || pages.with(|p| !p.is_empty())>
                <section class="mt-10 rounded-box border border-base-300 bg-base-200 p-4 text-left">
                    <h2 class="mb-2 font-display text-lg font-bold">"Offline verfügbar"</h2>
                    <ul class="menu w-full p-0">
                        <For each=move || pages.get() key=|p| p.clone() let:p>
                            <li><a href=p.clone() rel="external" class="min-h-11">{page_label(&p)}</a></li>
                        </For>
                    </ul>
                </section>
            </Show>
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ziel_nur_eigene_pfade() {
        assert_eq!(safe_target(Some("/radio".into())), "/radio");
        assert_eq!(safe_target(Some("/news?rubrik=club".into())), "/news?rubrik=club");
        assert_eq!(safe_target(Some("//boese.example".into())), "/");
        assert_eq!(safe_target(Some("https://boese.example".into())), "/");
        assert_eq!(safe_target(Some("/\\boese.example".into())), "/");
        assert_eq!(safe_target(None), "/");
    }
}
