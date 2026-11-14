//! Bausteine für die öffentliche Magazin-Ansicht (ANF-003).

use leptos::prelude::*;

use crate::articles::model::ArticleImage;

/// Farbverläufe für Platzhalter-Bilder. Ganze Klassen-Strings, damit
/// Tailwind sie findet. Die Farbe hängt von der Artikel-ID ab.
const COVERS: [&str; 6] = [
    "bg-linear-to-br from-neutral-900 via-neutral-700 to-neutral-500",
    "bg-linear-to-br from-stone-900 via-amber-900 to-orange-700",
    "bg-linear-to-br from-slate-950 via-slate-700 to-sky-800",
    "bg-linear-to-br from-zinc-950 via-rose-950 to-rose-800",
    "bg-linear-to-br from-emerald-950 via-teal-900 to-stone-700",
    "bg-linear-to-br from-indigo-950 via-violet-900 to-fuchsia-900",
];

/// Bildfläche. Mit `image` das echte Bild (zugeschnitten, Bildnachweis
/// klein unten rechts), sonst Farbverlauf + Anfangsbuchstabe des Titels.
/// `class` bestimmt Seitenverhältnis/Größe (z. B. `aspect-[3/2]`).
#[component]
pub fn CoverImage(
    id: i64,
    title: String,
    #[prop(default = None)] image: Option<ArticleImage>,
    #[prop(optional)] class: &'static str,
    /// Schriftgröße des Buchstabens (ganze Tailwind-Klasse)
    #[prop(default = "text-8xl")]
    letter_size: &'static str,
    /// Bildnachweis im Bild anzeigen (sonst nur als Tooltip)
    #[prop(default = true)]
    show_credit: bool,
    /// Sofort laden (für Bilder ganz oben), sonst erst beim Scrollen
    #[prop(optional)]
    eager: bool,
    /// Beschriftung der Platzhalter-Fläche für Screenreader (ANF-005: Produkte)
    #[prop(default = "Bild zum Artikel")]
    label: &'static str,
) -> impl IntoView {
    if let Some(img) = image {
        let credit = img.credit.clone();
        return view! {
            <div class=format!("relative w-full overflow-hidden bg-base-300 {class}") title=img.credit>
                <img
                    src=img.url
                    alt=title
                    loading=if eager { "eager" } else { "lazy" }
                    decoding="async"
                    class="absolute inset-0 h-full w-full object-cover object-[50%_30%]"
                />
                <Show when=move || show_credit>
                    <span class="absolute bottom-0 right-0 max-w-full truncate bg-black/40 px-2 py-0.5 text-[0.6rem] text-white/90">
                        {credit.clone()}
                    </span>
                </Show>
            </div>
        }
        .into_any();
    }

    let gradient = COVERS[id.rem_euclid(COVERS.len() as i64) as usize];
    let letter = title.chars().find(|c| c.is_alphanumeric()).map(|c| c.to_uppercase().to_string()).unwrap_or_default();
    view! {
        <div
            class=format!("relative w-full overflow-hidden {gradient} {class}")
            role="img"
            aria-label=format!("{label}: {title}")
        >
            <span class=format!(
                "absolute -bottom-[0.18em] left-[0.08em] font-display font-black leading-none text-white/15 select-none {letter_size}",
            )>{letter}</span>
            <span class="absolute inset-0 bg-linear-to-t from-black/30 to-transparent"></span>
        </div>
    }
    .into_any()
}

/// Kleine Großbuchstaben-Zeile über Titeln.
#[component]
pub fn Kicker(children: Children, #[prop(optional)] accent: bool) -> impl IntoView {
    view! {
        <p
            class="text-[0.7rem] font-bold uppercase tracking-[0.2em]"
            class:text-primary=accent
            class:text-base-content=move || !accent
        >
            {children()}
        </p>
    }
}

/// Abschnittsüberschrift mit Linie.
#[component]
pub fn SectionTitle(children: Children) -> impl IntoView {
    view! {
        <h2 class="mb-6 font-display text-2xl font-bold">
            {children()}
        </h2>
    }
}

pub fn reading_time_text(minutes: u32) -> String {
    format!("{minutes} Min. Lesezeit")
}
