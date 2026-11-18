//! ANF-008: News-Eintrag (Liste auf `/news` und Startseite).

use leptos::prelude::*;

use crate::articles::model::{ArticleTeaser, format_date_long};
use crate::components::magazine::{CoverImage, reading_time_text};

/// Zeile wie im Design: Bild 84 × 84, Rubrik, Titel, Datum + Lesezeit.
#[component]
pub fn NewsRow(article: ArticleTeaser, #[prop(default = "h3")] heading: &'static str) -> impl IntoView {
    let link = format!("/beitrag/{}", article.id);
    let meta = format!("{} · {}", format_date_long(article.published_on), reading_time_text(article.reading_minutes));
    let title = view! { <a href=link.clone() class="inline-flex min-h-11 items-center wrap-anywhere hover:underline">{article.title.clone()}</a> };
    let title = match heading {
        "h2" => view! { <h2 class="font-semibold leading-snug break-words wrap-anywhere hyphens-auto">{title}</h2> }.into_any(),
        _ => view! { <h3 class="font-semibold leading-snug break-words wrap-anywhere hyphens-auto">{title}</h3> }.into_any(),
    };
    view! {
        <article class="flex gap-3.5 rounded-2xl border border-base-300 bg-base-200 p-3.5">
            <a href=link tabindex="-1" aria-hidden="true" class="block size-21 shrink-0 overflow-hidden rounded-xl">
                <CoverImage
                    id=article.id
                    title=article.title.clone()
                    image=article.image.clone()
                    class="size-21"
                    letter_size="text-6xl"
                    show_credit=false
                />
            </a>
            <div class="flex min-w-0 flex-col gap-1.5">
                <span class="text-xs font-semibold uppercase tracking-wider text-primary">{article.category.label()}</span>
                {title}
                <span class="text-[13px] text-muted">{meta}</span>
            </div>
        </article>
    }
    .into_any()
}
