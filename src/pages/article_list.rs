//! ANF-002: Artikelliste – `/artikel?status=draft&seite=2`

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::articles::api::list_articles;
use crate::articles::model::{ArticlePage, ArticleStatus, ArticleSummary, format_date, format_timestamp, msg};
use crate::components::article::{ArticlesVersion, DeleteArticleButton, Flash, StatusBadge, server_error_text};

/// Adresse der Liste mit Filter und Seite (Seite 1 wird weggelassen).
fn list_href(page: u32, status: Option<ArticleStatus>) -> String {
    let mut params = Vec::new();
    if let Some(s) = status {
        params.push(format!("status={}", s.as_str()));
    }
    if page > 1 {
        params.push(format!("seite={page}"));
    }
    if params.is_empty() {
        "/artikel".into()
    } else {
        format!("/artikel?{}", params.join("&"))
    }
}

#[component]
pub fn ArticleListPage() -> impl IntoView {
    let query = use_query_map();
    let version = expect_context::<ArticlesVersion>().0;

    let page = move || query.with(|q| q.get("seite").and_then(|s| s.parse::<u32>().ok()).unwrap_or(1));
    let status = move || query.with(|q| q.get("status").and_then(|s| ArticleStatus::parse(&s)));

    let data = Resource::new(
        move || (page(), status(), version.get()),
        |(page, status, _)| list_articles(page, status),
    );

    view! {
        <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
            <h1 class="text-3xl font-bold">"Artikel"</h1>
            <a href="/artikel/neu" class="btn btn-primary">"Neuer Artikel"</a>
        </div>

        <Flash />

        <div role="tablist" class="tabs tabs-box mb-4 w-fit">
            {[None, Some(ArticleStatus::Draft), Some(ArticleStatus::Published)]
                .into_iter()
                .map(|s| {
                    let label = s.map(ArticleStatus::label).unwrap_or("Alle");
                    view! {
                        <a role="tab" href=list_href(1, s) class="tab" class:tab-active=move || status() == s>
                            {label}
                        </a>
                    }
                })
                .collect_view()}
        </div>

        <Suspense fallback=|| view! { <span class="loading loading-dots loading-lg"></span> }>
            {move || {
                let filter = status();
                Suspend::new(async move {
                    match data.await {
                        Ok(p) => view! { <ArticleList data=p filter /> }.into_any(),
                        Err(e) => view! {
                            <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
                        }.into_any(),
                    }
                })
            }}
        </Suspense>
    }
}

#[component]
fn ArticleList(data: ArticlePage, filter: Option<ArticleStatus>) -> impl IntoView {
    if data.items.is_empty() {
        return if filter.is_none() {
            view! {
                <div class="card border border-base-300 bg-base-200">
                    <div class="card-body items-center text-center">
                        <p>{msg::LIST_EMPTY}</p>
                        <a href="/artikel/neu" class="btn btn-primary">"Neuer Artikel"</a>
                    </div>
                </div>
            }
            .into_any()
        } else {
            view! {
                <div class="card border border-base-300 bg-base-200">
                    <div class="card-body items-center text-center">
                        <p>{msg::LIST_EMPTY_FILTER}</p>
                    </div>
                </div>
            }
            .into_any()
        };
    }

    let items = data.items.clone();
    view! {
        // Desktop: Tabelle
        <div class="hidden md:block overflow-x-auto rounded-box border border-base-300 bg-base-200">
            <table class="table">
                <thead>
                    <tr>
                        <th>"Titel"</th>
                        <th>"Status"</th>
                        <th>"Veröffentlichung"</th>
                        <th>"Zuletzt geändert"</th>
                        <th class="text-right">"Aktionen"</th>
                    </tr>
                </thead>
                <tbody>
                    {data.items.into_iter().map(|a| view! { <ArticleRow article=a /> }).collect_view()}
                </tbody>
            </table>
        </div>

        // Handy: Karten
        <div class="md:hidden flex flex-col gap-3">
            {items.into_iter().map(|a| view! { <ArticleCard article=a /> }).collect_view()}
        </div>

        <Pagination page=data.page total_pages=data.total_pages total=data.total filter />
    }
    .into_any()
}

fn date_text(a: &ArticleSummary) -> String {
    a.published_on.map(format_date).unwrap_or_else(|| "–".into())
}

#[component]
fn ArticleRow(article: ArticleSummary) -> impl IntoView {
    let id = article.id;
    view! {
        <tr class="hover:bg-base-300/40">
            <td>
                <a href=format!("/artikel/{id}") class="link link-hover font-medium">{article.title.clone()}</a>
            </td>
            <td><StatusBadge status=article.status /></td>
            <td>{date_text(&article)}</td>
            <td class="text-sm text-base-content/70">{format_timestamp(article.updated_at)}</td>
            <td>
                <div class="flex justify-end gap-2">
                    <a href=format!("/artikel/{id}/bearbeiten") class="btn btn-sm">"Bearbeiten"</a>
                    <DeleteArticleButton id title=article.title key="t" small=true />
                </div>
            </td>
        </tr>
    }
    .into_any()
}

#[component]
fn ArticleCard(article: ArticleSummary) -> impl IntoView {
    let id = article.id;
    view! {
        <div class="card border border-base-300 bg-base-200">
            <div class="card-body p-4 gap-2">
                <a href=format!("/artikel/{id}") class="card-title link link-hover wrap-anywhere min-h-11">{article.title.clone()}</a>
                <div class="flex flex-wrap items-center gap-2 text-sm">
                    <StatusBadge status=article.status />
                    <span>{date_text(&article)}</span>
                </div>
                <p class="text-xs text-base-content/70">
                    "Zuletzt geändert: " {format_timestamp(article.updated_at)}
                </p>
                <div class="card-actions justify-end">
                    <a href=format!("/artikel/{id}/bearbeiten") class="btn btn-sm">"Bearbeiten"</a>
                    <DeleteArticleButton id title=article.title key="k" small=true />
                </div>
            </div>
        </div>
    }
    .into_any()
}

#[component]
fn Pagination(page: u32, total_pages: u32, total: i64, filter: Option<ArticleStatus>) -> impl IntoView {
    view! {
        <div class="mt-4 flex flex-wrap items-center justify-between gap-2">
            <span class="text-sm text-base-content/70">{total} " Artikel"</span>
            <Show when=move || { total_pages > 1 }>
                <nav class="join" aria-label="Seiten">
                    {(1..=total_pages)
                        .map(|n| view! {
                            <a
                                href=list_href(n, filter)
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
        </div>
    }
}
