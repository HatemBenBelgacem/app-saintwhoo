//! ANF-003: Öffentliche Leseansicht – `/beitrag/<id>`

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::articles::api::get_published_article;
use crate::articles::model::{Article, format_date_long, paragraphs, reading_minutes};
use crate::components::magazine::{CoverImage, Kicker, reading_time_text};
use crate::pages::article_detail::with_article;

#[component]
pub fn BeitragPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|s| s.parse::<i64>().ok()));
    let article = Resource::new(id, |id| async move {
        match id {
            Some(id) => get_published_article(id).await,
            None => Ok(None),
        }
    });
    with_article(article, |a| view! { <Beitrag article=a /> })
}

#[component]
fn Beitrag(article: Article) -> impl IntoView {
    let date = article.published_on.map(format_date_long).unwrap_or_default();
    let meta = format!("{date} · {}", reading_time_text(reading_minutes(&article.body)));
    let paras = paragraphs(&article.body);

    view! {
        <Title text=format!("{} – Saintwhoo", article.title) />
        <article class="mx-auto max-w-3xl py-8 md:py-12">
            <a href="/" class="text-xs font-semibold uppercase tracking-widest text-base-content/60 hover:text-base-content">
                "← Startseite"
            </a>

            <header class="mt-8 text-center">
                <Kicker accent=true>{meta}</Kicker>
                <h1 class="mt-4 font-serif text-4xl font-bold leading-[1.1] break-words hyphens-auto md:text-6xl">
                    {article.title.clone()}
                </h1>
            </header>

            <figure class="mt-10 -mx-4 sm:mx-0">
                <CoverImage
                    id=article.id
                    title=article.title
                    image=article.image.clone()
                    class="aspect-[16/9]"
                    letter_size="text-[10rem] md:text-[16rem]"
                    show_credit=false
                    eager=true
                />
                {article.image.map(|i| view! {
                    <figcaption class="mt-2 px-4 text-right text-xs text-base-content/60 sm:px-0">{i.credit}</figcaption>
                })}
            </figure>

            <div class="mt-10 space-y-6 font-serif text-lg leading-relaxed md:text-xl md:leading-relaxed">
                {paras
                    .into_iter()
                    .enumerate()
                    .map(|(i, p)| {
                        view! {
                            <p
                                class="whitespace-pre-line break-words hyphens-auto"
                                class=(
                                    [
                                        "first-letter:float-left",
                                        "first-letter:mr-3",
                                        "first-letter:mt-1",
                                        "first-letter:text-7xl",
                                        "first-letter:font-black",
                                        "first-letter:leading-[0.8]",
                                    ],
                                    i == 0,
                                )
                            >
                                {p}
                            </p>
                        }
                    })
                    .collect_view()}
            </div>

            <div class="mt-16 border-t border-base-300 pt-8 text-center">
                <a href="/" class="inline-block border-b-2 border-base-content pb-0.5 text-xs font-bold uppercase tracking-[0.2em]">
                    "Weitere Artikel"
                </a>
            </div>
        </article>
    }
    .into_any()
}
