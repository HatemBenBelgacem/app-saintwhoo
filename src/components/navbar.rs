//! Navigation nach dem Designvorschlag „Techno App“:
//! - Kopfzeile mit Logo, Hauptbereichen (ab Tablet), Redaktions-Menü und Hell/Dunkel-Umschalter
//! - Leiste unten mit fünf Symbolen (nur Handy)

use leptos::html;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::pwa::{InstallButton, OfflineBadge, use_pwa};
use crate::components::theme::ThemeToggle;

/// Ein Hauptbereich der App.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Start,
    News,
    Events,
    Radio,
    Shop,
}

impl Section {
    const ALL: [Section; 5] = [Section::Start, Section::News, Section::Events, Section::Radio, Section::Shop];

    fn label(self) -> &'static str {
        match self {
            Section::Start => "Start",
            Section::News => "News",
            Section::Events => "Events",
            Section::Radio => "Radio",
            Section::Shop => "Shop",
        }
    }

    fn href(self) -> &'static str {
        match self {
            Section::Start => "/",
            Section::News => "/news",
            Section::Events => "/eventliste",
            Section::Radio => "/radio",
            Section::Shop => "/shop",
        }
    }

    /// Gehört die Adresse zu diesem Bereich? (Detailseiten zählen mit.)
    fn matches(self, path: &str) -> bool {
        let under = |p: &str| path == p || path.starts_with(&format!("{p}/"));
        match self {
            Section::Start => path == "/",
            Section::News => under("/news") || under("/beitrag"),
            Section::Events => under("/eventliste"),
            Section::Radio => path == "/radio",
            Section::Shop => under("/shop"),
        }
    }
}

/// Symbol (24 px, Strich) – Pfade aus dem Designvorschlag.
#[component]
fn SectionIcon(section: Section) -> impl IntoView {
    let paths = match section {
        Section::Start => view! { <path d="M3 11l9-7 9 7v9a1 1 0 0 1-1 1h-5v-6H9v6H4a1 1 0 0 1-1-1z" /> }.into_any(),
        Section::News => view! {
            <path d="M4 5h13v14H6a2 2 0 0 1-2-2z" />
            <path d="M17 9h3v8a2 2 0 0 1-2 2" />
            <path d="M7 9h6M7 13h6" />
        }
        .into_any(),
        Section::Events => view! {
            <rect x="3" y="5" width="18" height="16" rx="2" />
            <path d="M3 10h18M8 3v4M16 3v4" />
        }
        .into_any(),
        Section::Radio => view! {
            <circle cx="12" cy="12" r="2" />
            <path d="M8.5 8.5a5 5 0 0 0 0 7M15.5 8.5a5 5 0 0 1 0 7M5.6 5.6a9.5 9.5 0 0 0 0 12.8M18.4 5.6a9.5 9.5 0 0 1 0 12.8" />
        }
        .into_any(),
        Section::Shop => view! {
            <path d="M5 8h14l-1 12H6z" />
            <path d="M9 8V6a3 3 0 0 1 6 0v2" />
        }
        .into_any(),
    };
    view! {
        <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            {paths}
        </svg>
    }
}

/// Kopfzeile: Logo links, Bereiche in der Mitte (ab Tablet), Redaktion und Theme rechts.
#[component]
pub fn Navbar() -> impl IntoView {
    let location = use_location();

    view! {
        <header class="sticky top-0 z-30 border-b border-base-300 bg-base-100/95 pt-[env(safe-area-inset-top)] backdrop-blur">
            <div class="container mx-auto flex h-16 items-center justify-between gap-4 px-4">
                <div class="flex items-center gap-2">
                    <a href="/" class="font-display text-2xl font-bold tracking-tight">"Saintwhoo"</a>
                    <OfflineBadge />
                </div>

                <nav aria-label="Hauptmenü" class="hidden gap-1 md:flex">
                    {Section::ALL
                        .into_iter()
                        .map(|s| {
                            let active = move || location.pathname.with(|p| s.matches(p));
                            view! {
                                <a
                                    href=s.href()
                                    class="inline-flex min-h-11 items-center rounded-full px-4 text-[15px] font-medium transition-colors"
                                    class=("bg-primary", active)
                                    class=("text-primary-content", active)
                                    class=("hover:bg-base-200", move || !active())
                                    aria-current=move || active().then_some("page")
                                >
                                    {s.label()}
                                </a>
                            }
                        })
                        .collect_view()}
                </nav>

                <div class="flex items-center gap-2">
                    <InstallButton />
                    <EditorMenu />
                    <ThemeToggle />
                </div>
            </div>
        </header>
    }
    .into_any()
}

/// Redaktions-Menü (funktioniert auch ohne JavaScript).
#[component]
fn EditorMenu() -> impl IntoView {
    let pwa = use_pwa();
    let menu = NodeRef::<html::Details>::new();
    // Menü nach einem Klick schließen (bei Navigation ohne Seitenwechsel)
    let close = move |_| {
        if let Some(d) = menu.get() {
            let _ = d.remove_attribute("open");
        }
    };
    view! {
        <details node_ref=menu class="dropdown dropdown-end">
            <summary class="btn btn-outline btn-sm min-h-11 border-base-300 px-4 font-medium">"Redaktion"</summary>
            <ul on:click=close class="menu dropdown-content z-40 mt-2 w-56 rounded-box border border-base-300 bg-base-200 p-2">
                <Show when=move || !pwa.online.get()>
                    <li class="px-3 py-2 text-sm text-warning">"Nur online möglich"</li>
                </Show>
                <li class="menu-title">"Inhalte pflegen"</li>
                <li><a href="/artikel">"Artikel"</a></li>
                <li><a href="/events">"Events"</a></li>
                <li><a href="/radio/verwaltung">"Radio"</a></li>
                <li><a href="/produkte">"Produkte"</a></li>
                <li class="menu-title">"Neu"</li>
                <li><a href="/artikel/neu">"Neuer Artikel"</a></li>
                <li><a href="/events/neu">"Neues Event"</a></li>
                <li><a href="/produkte/neu">"Neues Produkt"</a></li>
            </ul>
        </details>
    }
    .into_any()
}

/// Leiste unten auf dem Handy. Bleibt am unteren Rand stehen.
#[component]
pub fn BottomNav() -> impl IntoView {
    let location = use_location();
    view! {
        <nav
            aria-label="Hauptmenü"
            class="fixed inset-x-0 bottom-0 z-30 flex justify-around border-t border-base-300 bg-base-100 px-2 pt-2 pb-[max(1rem,env(safe-area-inset-bottom))] md:hidden"
        >
            {Section::ALL
                .into_iter()
                .map(|s| {
                    let active = move || location.pathname.with(|p| s.matches(p));
                    view! {
                        <a
                            href=s.href()
                            class="flex min-h-14 flex-1 flex-col items-center justify-center gap-1 text-xs font-semibold"
                            class=("text-primary", active)
                            class=("text-muted", move || !active())
                            aria-current=move || active().then_some("page")
                        >
                            <SectionIcon section=s />
                            <span>{s.label()}</span>
                        </a>
                    }
                })
                .collect_view()}
        </nav>
    }
    .into_any()
}
