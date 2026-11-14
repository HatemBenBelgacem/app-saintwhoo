//! ANF-002: Detailseite – `/artikel/<id>`

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::articles::api::get_article;
use crate::articles::model::{Article, format_date, format_timestamp};
use crate::components::article::{DeleteArticleButton, Flash, StatusBadge, server_error_text};
use crate::components::magazine::CoverImage;
use crate::pages::not_found::NotFound;

/// Liest `:id` aus der Adresse und lädt den Artikel. Eine ungültige ID
/// verhält sich wie eine unbekannte (404).
pub fn article_from_params() -> Resource<Result<Option<Article>, ServerFnError>> {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|s| s.parse::<i64>().ok()));
    Resource::new(id, |id| async move {
        match id {
            Some(id) => get_article(id).await,
            None => Ok(None),
        }
    })
}

/// Zeigt Laden / Fehler / 404 und ruft sonst `view` mit dem Artikel auf.
pub fn with_article<V: IntoView + 'static>(
    article: Resource<Result<Option<Article>, ServerFnError>>,
    view: fn(Article) -> V,
) -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <span class="loading loading-dots loading-lg"></span> }>
            {move || Suspend::new(async move {
                match article.await {
                    Ok(Some(a)) => view(a).into_any(),
                    Ok(None) => view! { <NotFound /> }.into_any(),
                    Err(e) => view! {
                        <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
                    }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn ArticleDetailPage() -> impl IntoView {
    with_article(article_from_params(), |a| view! { <ArticleDetail article=a /> })
}

#[component]
fn ArticleDetail(article: Article) -> impl IntoView {
    let id = article.id;
    view! {
        <div class="max-w-3xl mx-auto">
            <a href="/artikel" class="btn btn-ghost btn-sm mb-4">"← Zurück zur Liste"</a>

            <Flash />

            <article class="card overflow-hidden border border-base-300 bg-base-200">
                {article.image.clone().map(|img| {
                    let credit = img.credit.clone();
                    view! {
                        <figure class="flex-col">
                            <CoverImage
                                id=id
                                title=article.title.clone()
                                image=Some(img)
                                class="aspect-[16/9]"
                                show_credit=false
                            />
                            <figcaption class="w-full px-8 pt-2 text-right text-xs text-base-content/60">{credit}</figcaption>
                        </figure>
                    }
                })}
                <div class="card-body">
                    <div class="flex flex-wrap items-center gap-2">
                        <StatusBadge status=article.status />
                        <span class="text-sm text-base-content/70">
                            {article
                                .published_on
                                .map(|d| format!("Veröffentlichung: {}", format_date(d)))
                                .unwrap_or_else(|| "Kein Veröffentlichungsdatum".into())}
                        </span>
                    </div>

                    <h1 class="text-3xl font-bold break-words">{article.title.clone()}</h1>

                    // Zeilenumbrüche bleiben durch `whitespace-pre-wrap` erhalten
                    <div class="whitespace-pre-wrap break-words leading-relaxed">{article.body}</div>

                    <dl class="mt-4 grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-sm text-base-content/70">
                        <dt>"Erstellt am"</dt>
                        <dd>{format_timestamp(article.created_at)}</dd>
                        <dt>"Zuletzt geändert"</dt>
                        <dd>{format_timestamp(article.updated_at)}</dd>
                    </dl>

                    <div class="card-actions justify-end mt-4">
                        <a href=format!("/artikel/{id}/bearbeiten") class="btn btn-primary">"Bearbeiten"</a>
                        <DeleteArticleButton id title=article.title />
                    </div>
                </div>
            </article>
        </div>
    }
}
