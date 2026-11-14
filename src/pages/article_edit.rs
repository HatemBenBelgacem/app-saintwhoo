//! ANF-002: Bearbeiten – `/artikel/<id>/bearbeiten`

use leptos::prelude::*;

use crate::articles::api::UpdateArticle;
use crate::articles::model::{Article, FieldErrors, UpdateOutcome, msg};
use crate::components::article::{ArticleFields, server_error_text};
use crate::pages::article_detail::{article_from_params, with_article};

#[component]
pub fn ArticleEditPage() -> impl IntoView {
    with_article(article_from_params(), |a| view! { <ArticleEditForm article=a /> })
}

#[component]
fn ArticleEditForm(article: Article) -> impl IntoView {
    let action = ServerAction::<UpdateArticle>::new();
    let id = article.id;

    let errors = Memo::new(move |_| match action.value().get() {
        Some(Ok(UpdateOutcome::Invalid(e))) => e,
        _ => FieldErrors::default(),
    });

    view! {
        <div class="max-w-2xl mx-auto">
            <h1 class="text-3xl font-bold mb-6">"Artikel bearbeiten"</h1>

            {move || match action.value().get() {
                Some(Ok(UpdateOutcome::Invalid(_))) => Some(view! {
                    <div role="alert" class="alert alert-warning mb-4">
                        <span>"Bitte die markierten Felder korrigieren."</span>
                    </div>
                }.into_any()),
                Some(Ok(UpdateOutcome::NotFound)) => Some(view! {
                    <div role="alert" class="alert alert-error mb-4">
                        <span>{msg::NOT_FOUND}</span>
                        <a href="/artikel" class="btn btn-sm">"Zur Liste"</a>
                    </div>
                }.into_any()),
                Some(Err(e)) => Some(view! {
                    <div role="alert" class="alert alert-error mb-4">
                        <span>{server_error_text(&e)}</span>
                    </div>
                }.into_any()),
                _ => None,
            }}

            <div class="card border border-base-300 bg-base-200">
                <div class="card-body">
                    <ActionForm action attr:novalidate=true>
                        <input type="hidden" name="id" value=id />
                        <fieldset class="fieldset" disabled=move || action.pending().get()>
                            <ArticleFields initial=article errors />
                            <div class="mt-4 flex justify-end gap-2">
                                <a href=format!("/artikel/{id}") class="btn btn-ghost">"Abbrechen"</a>
                                <button type="submit" class="btn btn-primary">
                                    <Show when=move || action.pending().get()>
                                        <span class="loading loading-spinner loading-sm"></span>
                                    </Show>
                                    "Speichern"
                                </button>
                            </div>
                        </fieldset>
                    </ActionForm>
                </div>
            </div>
        </div>
    }
}
