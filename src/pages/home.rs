//! ANF-003: Magazin-Startseite – `/`

use leptos::prelude::*;

use crate::articles::api::front_page_articles;
use crate::articles::model::{ArticleTeaser, FRONT_GRID, format_date_long, msg};
use crate::components::article::server_error_text;
use crate::components::magazine::{CoverImage, Kicker, SectionTitle, reading_time_text};

fn href(a: &ArticleTeaser) -> String {
    format!("/beitrag/{}", a.id)
}

fn meta_line(a: &ArticleTeaser) -> String {
    format!("{} · {}", format_date_long(a.published_on), reading_time_text(a.reading_minutes))
}

#[component]
pub fn HomePage() -> impl IntoView {
    let articles = Resource::new(|| (), |_| front_page_articles());

    view! {
        <Suspense fallback=|| view! {
            <div class="flex justify-center py-24"><span class="loading loading-dots loading-lg"></span></div>
        }>
            {move || Suspend::new(async move {
                match articles.await {
                    Ok(list) if list.is_empty() => view! { <Empty /> }.into_any(),
                    Ok(list) => view! { <FrontPage articles=list /> }.into_any(),
                    Err(e) => view! {
                        <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
                    }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
fn Empty() -> impl IntoView {
    view! {
        <div class="py-24 text-center">
            <p class="font-serif text-3xl">{msg::FRONT_EMPTY}</p>
            <a href="/artikel/neu" class="btn btn-neutral mt-6">"Ersten Artikel schreiben"</a>
        </div>
    }
}

#[component]
fn FrontPage(articles: Vec<ArticleTeaser>) -> impl IntoView {
    let mut rest = articles.into_iter();
    let lead = rest.next().expect("nicht leer");
    let grid: Vec<_> = rest.by_ref().take(FRONT_GRID).collect();
    let list: Vec<_> = rest.collect();

    view! {
        <div class="py-6 md:py-10">
            <Lead article=lead />

            <Show when={
                let empty = grid.is_empty();
                move || !empty
            }>
                <section class="mt-16">
                    <SectionTitle>"Neueste Artikel"</SectionTitle>
                    <div class="grid gap-x-8 gap-y-12 sm:grid-cols-2 lg:grid-cols-3">
                        {grid.clone().into_iter().map(|a| view! { <Card article=a /> }).collect_view()}
                    </div>
                </section>
            </Show>

            <Show when={
                let empty = list.is_empty();
                move || !empty
            }>
                <section class="mt-16">
                    <SectionTitle>"Weitere Artikel"</SectionTitle>
                    <ol class="grid gap-x-12 md:grid-cols-2">
                        {list
                            .clone()
                            .into_iter()
                            .enumerate()
                            .map(|(i, a)| view! { <ListItem article=a number=i + 1 /> })
                            .collect_view()}
                    </ol>
                </section>
            </Show>
        </div>
    }
    .into_any()
}

/// Aufmacher: großes Bild, daneben (Desktop) bzw. darunter (Handy) der Text.
#[component]
fn Lead(article: ArticleTeaser) -> impl IntoView {
    let link = href(&article);
    let meta = meta_line(&article);
    view! {
        <article class="group grid items-center gap-6 lg:grid-cols-5 lg:gap-10">
            <a href=link.clone() class="block overflow-hidden lg:col-span-3" tabindex="-1" aria-hidden="true">
                <div class="transition-transform duration-700 group-hover:scale-105">
                    <CoverImage
                        id=article.id
                        title=article.title.clone()
                        image=article.image.clone()
                        class="aspect-[4/3]"
                        letter_size="text-[12rem] md:text-[18rem]"
                        eager=true
                    />
                </div>
            </a>
            <div class="lg:col-span-2">
                <Kicker accent=true>"Titelgeschichte"</Kicker>
                <h1 class="mt-3 font-serif text-4xl font-bold leading-[1.1] break-words hyphens-auto md:text-5xl">
                    <a href=link.clone() class="decoration-2 underline-offset-[6px] group-hover:underline">
                        {article.title}
                    </a>
                </h1>
                <p class="mt-5 text-lg leading-relaxed text-base-content/80">{article.teaser}</p>
                <p class="mt-5 text-xs uppercase tracking-widest text-base-content/60">{meta}</p>
                <a href=link class="mt-6 inline-block border-b-2 border-base-content pb-0.5 text-xs font-bold uppercase tracking-[0.2em]">
                    "Weiterlesen"
                </a>
            </div>
        </article>
    }
    .into_any()
}

#[component]
fn Card(article: ArticleTeaser) -> impl IntoView {
    let link = href(&article);
    view! {
        <article class="group">
            <a href=link.clone() class="block overflow-hidden" tabindex="-1" aria-hidden="true">
                <div class="transition-transform duration-500 group-hover:scale-105">
                    <CoverImage id=article.id title=article.title.clone() image=article.image.clone() class="aspect-[3/2]" />
                </div>
            </a>
            <div class="mt-4">
                <Kicker>{format_date_long(article.published_on)}</Kicker>
            </div>
            <h3 class="mt-2 font-serif text-2xl font-bold leading-snug break-words hyphens-auto">
                <a href=link class="decoration-2 underline-offset-4 group-hover:underline">{article.title}</a>
            </h3>
            // Anriss ist serverseitig schon auf 160 Zeichen gekürzt (kein line-clamp,
            // sonst entsteht ein doppeltes „…“)
            <p class="mt-2 text-base-content/70">{article.teaser}</p>
            <p class="mt-3 text-xs uppercase tracking-widest text-base-content/60">
                {reading_time_text(article.reading_minutes)}
            </p>
        </article>
    }
    .into_any()
}

#[component]
fn ListItem(article: ArticleTeaser, number: usize) -> impl IntoView {
    let link = href(&article);
    let meta = meta_line(&article);
    view! {
        <li class="group flex gap-5 border-b border-base-300 py-5">
            <span class="w-8 shrink-0 font-serif text-3xl font-bold text-base-content/30">{number}</span>
            <div class="min-w-0 flex-1">
                <h3 class="font-serif text-xl font-bold leading-snug break-words hyphens-auto">
                    <a href=link.clone() class="decoration-2 underline-offset-4 group-hover:underline">{article.title.clone()}</a>
                </h3>
                <p class="mt-2 text-xs uppercase tracking-widest text-base-content/60">{meta}</p>
            </div>
            <a href=link class="block w-24 shrink-0 overflow-hidden" tabindex="-1" aria-hidden="true">
                <CoverImage
                    id=article.id
                    title=article.title
                    image=article.image
                    class="aspect-square"
                    letter_size="text-6xl"
                    show_credit=false
                />
            </a>
        </li>
    }
    .into_any()
}
