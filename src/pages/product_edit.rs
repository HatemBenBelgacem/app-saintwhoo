//! ANF-005: Produkt bearbeiten – `/produkte/<id>/bearbeiten`

use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::article::server_error_text;
use crate::components::product::{ProductFields, ProductsVersion};
use crate::pages::product_detail::{product_from_params, with_product};
use crate::products::api::{UpdateProduct, get_product};
use crate::products::model::{FieldErrors, Product, UpdateOutcome, msg};

#[component]
pub fn ProductEditPage() -> impl IntoView {
    with_product(product_from_params(get_product), |p| view! { <ProductEditForm product=p /> })
}

#[component]
fn ProductEditForm(product: Product) -> impl IntoView {
    let action = ServerAction::<UpdateProduct>::new();
    let version = use_context::<ProductsVersion>();
    let id = product.id;

    let errors = Memo::new(move |_| match action.value().get() {
        Some(Ok(UpdateOutcome::Invalid(e))) => e,
        _ => FieldErrors::default(),
    });

    Effect::new(move |_| {
        if let Some(Ok(UpdateOutcome::Updated { .. })) = action.value().get()
            && let Some(v) = version
        {
            v.0.update(|n| *n += 1);
        }
    });

    view! {
        <Title text=format!("{} bearbeiten – Saintwhoo", product.name) />
        <div class="max-w-2xl mx-auto">
            <h1 class="text-3xl font-bold mb-6">"Produkt bearbeiten"</h1>

            {move || match action.value().get() {
                Some(Ok(UpdateOutcome::Invalid(_))) => Some(view! {
                    <div role="alert" class="alert alert-warning mb-4">
                        <span>"Bitte die markierten Felder korrigieren."</span>
                    </div>
                }.into_any()),
                Some(Ok(UpdateOutcome::NotFound)) => Some(view! {
                    <div role="alert" class="alert alert-error mb-4">
                        <span>{msg::NOT_FOUND}</span>
                        <a href="/produkte" class="btn btn-sm">"Zur Liste"</a>
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
                    <ActionForm action attr:novalidate=true>
                        <input type="hidden" name="id" value=id />
                        <fieldset class="fieldset" disabled=move || action.pending().get()>
                            <ProductFields initial=product errors />
                            <div class="mt-4 flex justify-end gap-2">
                                <a href=format!("/produkte/{id}") class="btn btn-ghost">"Abbrechen"</a>
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
