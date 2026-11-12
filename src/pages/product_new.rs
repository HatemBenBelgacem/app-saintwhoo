//! ANF-005: Neues Produkt – `/produkte/neu`
//!
//! Nach dem Speichern leitet der Server zur Detailseite weiter
//! (mit JavaScript navigiert der Browser selbst, ohne antwortet der Server mit 302).

use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::article::server_error_text;
use crate::components::product::{ProductFields, ProductsVersion};
use crate::products::api::CreateProduct;
use crate::products::model::{CreateOutcome, FieldErrors};

#[component]
pub fn ProductNewPage() -> impl IntoView {
    let action = ServerAction::<CreateProduct>::new();
    let version = use_context::<ProductsVersion>();

    let errors = Memo::new(move |_| match action.value().get() {
        Some(Ok(CreateOutcome::Invalid(e))) => e,
        _ => FieldErrors::default(),
    });

    // Listen neu laden, damit das neue Produkt dort erscheint
    Effect::new(move |_| {
        if let Some(Ok(CreateOutcome::Created { .. })) = action.value().get()
            && let Some(v) = version
        {
            v.0.update(|n| *n += 1);
        }
    });

    view! {
        <Title text="Neues Produkt – Saintwhoo" />
        <div class="max-w-2xl mx-auto">
            <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
                <h1 class="text-3xl font-bold">"Neues Produkt"</h1>
                <a href="/produkte" class="btn btn-ghost btn-sm">"← Zur Liste"</a>
            </div>

            {move || match action.value().get() {
                Some(Ok(CreateOutcome::Invalid(_))) => Some(view! {
                    <div role="alert" class="alert alert-warning mb-4">
                        <span>"Bitte die markierten Felder korrigieren."</span>
                    </div>
                }.into_any()),
                Some(Err(e)) => Some(view! {
                    <div role="alert" class="alert alert-error mb-4">
                        <span>{server_error_text(&e)}</span>
                    </div>
                }.into_any()),
                _ => None,
            }}

            <div class="card bg-base-100 shadow">
                <div class="card-body">
                    // Validierung macht der Server (gleiche Regeln, gleiche Texte)
                    <ActionForm action attr:novalidate=true>
                        <fieldset class="fieldset" disabled=move || action.pending().get()>
                            <ProductFields errors />
                            <div class="mt-4 flex justify-end gap-2">
                                <a href="/produkte" class="btn btn-ghost">"Abbrechen"</a>
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
