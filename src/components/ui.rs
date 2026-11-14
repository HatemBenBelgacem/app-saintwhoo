//! Kleine Bausteine aus dem Designvorschlag „Techno App“.

use leptos::prelude::*;

/// Seitenkopf: großer Titel und eine Zeile darunter.
#[component]
pub fn PageHeader(
    title: &'static str,
    #[prop(optional)] subtitle: Option<&'static str>,
    /// Rechts neben dem Titel (z. B. Warenkorb-Knopf)
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <header class="flex flex-wrap items-end justify-between gap-4">
            <div class="flex flex-col gap-1.5">
                <h1 class="font-display text-[2rem] font-bold leading-tight tracking-tight md:text-5xl">{title}</h1>
                {subtitle.map(|s| view! { <p class="text-[15px] text-muted md:text-base">{s}</p> })}
            </div>
            {children.map(|c| c())}
        </header>
    }
    .into_any()
}

/// Filter-Chip als Link (teilbar, funktioniert ohne JavaScript).
#[component]
pub fn Chip(#[prop(into)] href: Signal<String>, #[prop(into)] active: Signal<bool>, label: &'static str) -> impl IntoView {
    view! {
        <a
            href=move || href.get()
            class="inline-flex min-h-11 flex-none items-center rounded-full px-4 text-[15px] transition-colors"
            class=("bg-primary", move || active.get())
            class=("text-primary-content", move || active.get())
            class=("font-semibold", move || active.get())
            class=("border", move || !active.get())
            class=("border-base-300", move || !active.get())
            class=("font-medium", move || !active.get())
            class=("hover:border-base-content", move || !active.get())
            aria-current=move || active.get().then_some("true")
        >
            {label}
        </a>
    }
    .into_any()
}

/// Kleines Badge „LIVE“ / „JETZT“ / „HEUTE“ in Akzentfarbe.
#[component]
pub fn AccentTag(label: &'static str) -> impl IntoView {
    view! {
        <span class="rounded-md bg-primary px-2 py-1 text-xs font-bold tracking-wider text-primary-content">{label}</span>
    }
}
