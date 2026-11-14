//! ANF-001: Seite „Neuer Artikel“ – `/artikel/neu`

use leptos::html;
use leptos::prelude::*;

use crate::articles::api::CreateArticle;
use crate::articles::model::{CreateOutcome, FieldErrors};
use crate::components::article::{ArticleFields, ArticlesVersion, server_error_text};

#[component]
pub fn ArticleNewPage() -> impl IntoView {
    let action = ServerAction::<CreateArticle>::new();
    let form_ref = NodeRef::<html::Form>::new();
    let version = use_context::<ArticlesVersion>();

    let errors = Memo::new(move |_| match action.value().get() {
        Some(Ok(CreateOutcome::Invalid(e))) => e,
        _ => FieldErrors::default(),
    });

    // Nach erfolgreichem Speichern: Formular leeren, Listen neu laden
    Effect::new(move |_| {
        if let Some(Ok(CreateOutcome::Created { .. })) = action.value().get() {
            if let Some(form) = form_ref.get() {
                form.reset();
            }
            if let Some(v) = version {
                v.0.update(|n| *n += 1);
            }
        }
    });

    view! {
        <div class="max-w-2xl mx-auto">
            <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
                <h1 class="text-3xl font-bold">"Neuer Artikel"</h1>
                <a href="/artikel" class="btn btn-ghost btn-sm">"← Zur Liste"</a>
            </div>

            {move || match action.value().get() {
                Some(Ok(CreateOutcome::Created { id })) => view! {
                    <div role="status" class="alert alert-success mb-4">
                        <span>"Artikel gespeichert."</span>
                        <a href=format!("/artikel/{id}") class="btn btn-sm">"Artikel ansehen"</a>
                    </div>
                }.into_any(),
                Some(Ok(CreateOutcome::Invalid(_))) => view! {
                    <div role="alert" class="alert alert-warning mb-4">
                        <span>"Bitte die markierten Felder korrigieren."</span>
                    </div>
                }.into_any(),
                Some(Err(e)) => view! {
                    <div role="alert" class="alert alert-error mb-4">
                        <span>{server_error_text(&e)}</span>
                    </div>
                }.into_any(),
                None => ().into_any(),
            }}

            <div class="card border border-base-300 bg-base-200">
                <div class="card-body">
                    // Validierung macht der Server (gleiche Regeln, gleiche Texte)
                    <ActionForm action node_ref=form_ref attr:novalidate=true>
                        <fieldset class="fieldset" disabled=move || action.pending().get()>
                            <ArticleFields errors />
                            <div class="mt-4 flex justify-end gap-2">
                                <a href="/artikel" class="btn btn-ghost">"Abbrechen"</a>
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
