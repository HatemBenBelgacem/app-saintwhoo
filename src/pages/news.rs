//! ANF-008: News – `/news?q=acid&rubrik=interview&seite=2`
//!
//! Suche und Filter stehen in der Adresse: teilbar und ohne JavaScript nutzbar.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::Form;
use leptos_router::hooks::use_query_map;

use crate::articles::api::news_articles;
use crate::articles::model::{ArticleCategory, NewsPage, msg};
use crate::components::article::server_error_text;
use crate::components::news::NewsRow;
use crate::components::ui::{Chip, PageHeader};

/// Adresse mit Suche, Rubrik und Seite (Standardwerte weggelassen).
fn news_href(q: &str, category: Option<ArticleCategory>, page: u32) -> String {
    let mut params = Vec::new();
    if !q.trim().is_empty() {
        params.push(format!("q={}", urlencode(q.trim())));
    }
    if let Some(c) = category {
        params.push(format!("rubrik={}", c.as_str()));
    }
    if page > 1 {
        params.push(format!("seite={page}"));
    }
    if params.is_empty() { "/news".into() } else { format!("/news?{}", params.join("&")) }
}

/// Einfache Prozent-Kodierung für den Suchbegriff.
fn urlencode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            b' ' => "+".into(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[component]
pub fn NewsListPage() -> impl IntoView {
    let query = use_query_map();
    let q = move || query.with(|m| m.get("q").unwrap_or_default());
    let category = move || query.with(|m| m.get("rubrik").and_then(|s| ArticleCategory::parse(&s)));
    let page = move || query.with(|m| m.get("seite").and_then(|s| s.parse::<u32>().ok()).unwrap_or(1));

    let data = Resource::new(move || (q(), category(), page()), |(q, c, p)| news_articles(q, c, p));

    let chips = std::iter::once(None).chain(ArticleCategory::ALL.into_iter().map(Some));

    view! {
        <Title text="News – Saintwhoo" />
        <div class="mx-auto max-w-5xl py-6 md:py-10">
            <PageHeader title="News" subtitle="Neues über DJs, Releases und Events" />

            <Form method="GET" action="/news" attr:class="mt-5 flex flex-col gap-2" attr:role="search">
                <label for="suche" class="text-sm font-medium text-muted">"Suche"</label>
                <div class="flex gap-2">
                    <input
                        id="suche"
                        type="search"
                        name="q"
                        prop:value=q
                        placeholder="DJ, Label oder Stichwort"
                        class="input min-h-12 w-full rounded-2xl text-base"
                    />
                    {move || category().map(|c| view! { <input type="hidden" name="rubrik" value=c.as_str() /> })}
                    <button type="submit" class="btn btn-outline min-h-12 border-base-300">"Suchen"</button>
                </div>
            </Form>

            <nav aria-label="Rubrik" class="-mx-4 mt-4 flex gap-2 overflow-x-auto px-4 pb-1">
                {chips
                    .map(|c| {
                        let label = c.map(ArticleCategory::plural).unwrap_or("Alle");
                        view! {
                            <Chip
                                href=Signal::derive(move || news_href(&q(), c, 1))
                                active=Signal::derive(move || category() == c)
                                label
                            />
                        }
                    })
                    .collect_view()}
            </nav>

            <Suspense fallback=|| view! {
                <div class="flex justify-center py-24"><span class="loading loading-dots loading-lg"></span></div>
            }>
                {move || {
                    let (q, c) = (q(), category());
                    Suspend::new(async move {
                        match data.await {
                            Ok(p) => view! { <Results data=p q c /> }.into_any(),
                            Err(e) => view! {
                                <div role="alert" class="alert alert-error mt-6">{server_error_text(&e)}</div>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
    .into_any()
}

#[component]
fn Results(data: NewsPage, q: String, c: Option<ArticleCategory>) -> impl IntoView {
    if data.items.is_empty() {
        let text = if q.trim().is_empty() && c.is_none() { msg::NEWS_EMPTY } else { msg::NEWS_NO_MATCH };
        return view! { <p class="py-20 text-center font-display text-xl">{text}</p> }.into_any();
    }
    let (page, total_pages, total) = (data.page, data.total_pages, data.total);
    let count = if total == 1 { "1 Meldung".to_string() } else { format!("{total} Meldungen") };
    view! {
        <p class="mt-6 mb-3 text-sm text-muted" role="status">{count}</p>
        <ul class="grid gap-3 md:grid-cols-2">
            {data.items.into_iter().map(|a| view! { <li><NewsRow article=a heading="h2" /></li> }).collect_view()}
        </ul>
        <Show when=move || { total_pages > 1 }>
            <nav class="join mt-8 flex justify-center" aria-label="Seiten">
                {(1..=total_pages)
                    .map(|n| view! {
                        <a
                            href=news_href(&q, c, n)
                            class="join-item btn btn-sm"
                            class:btn-active=n == page
                            aria-current=(n == page).then_some("page")
                        >
                            {n}
                        </a>
                    })
                    .collect_view()}
            </nav>
        </Show>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adressen() {
        assert_eq!(news_href("", None, 1), "/news");
        assert_eq!(news_href(" acid house ", Some(ArticleCategory::Mix), 2), "/news?q=acid+house&rubrik=mix&seite=2");
        assert_eq!(urlencode("Zürich&co"), "Z%C3%BCrich%26co");
    }
}
