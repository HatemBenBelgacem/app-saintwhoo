//! UI-Bausteine für Produkte und Shop (ANF-005).

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::components::article::{Field, server_error_text};
use crate::components::magazine::CoverImage;
use crate::products::api::DeleteProduct;
use crate::products::model::{
    Availability, DeleteOutcome, FieldErrors, Product, ProductCategory, ProductImage, ProductStatus, msg,
    price_input_value,
};

/// Zähler, der nach Änderungen hochgezählt wird, damit Listen neu laden.
#[derive(Clone, Copy)]
pub struct ProductsVersion(pub RwSignal<u32>);

/// Die Formularfelder eines Produkts. Ohne `initial` leer (Erstellen,
/// Status *Entwurf*, Bestand 0), mit `initial` vorausgefüllt (Bearbeiten).
#[component]
pub fn ProductFields(#[prop(optional)] initial: Option<Product>, errors: Memo<FieldErrors>) -> impl IntoView {
    let category = initial.as_ref().map(|p| p.category).unwrap_or_default();
    let brand = initial.as_ref().map(|p| p.brand.clone()).unwrap_or_default();
    let (image_url, image_credit) = match initial.as_ref().and_then(|p| p.image.clone()) {
        Some(i) => (i.url, i.credit),
        None => (String::new(), String::new()),
    };
    let (name, sku, short_description, description, price, stock, status) = match initial {
        Some(p) => (
            p.name,
            p.sku,
            p.short_description,
            p.description,
            price_input_value(p.price_cents),
            p.stock.to_string(),
            p.status,
        ),
        None => (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            "0".to_string(),
            ProductStatus::Draft,
        ),
    };
    let err = move |f: fn(&FieldErrors) -> Option<String>| Signal::derive(move || f(&errors.get()));

    view! {
        <div class="grid gap-4 sm:grid-cols-3">
            <div class="sm:col-span-2">
                <Field label="Name" error=err(|e| e.name.clone())>
                    <input
                        type="text"
                        name="name"
                        value=name
                        class="input w-full"
                        class:input-error=move || errors.get().name.is_some()
                        placeholder="Name des Produkts"
                        aria-required="true"
                    />
                </Field>
            </div>
            <Field label="Artikelnummer" error=err(|e| e.sku.clone())>
                <input
                    type="text"
                    name="sku"
                    value=sku
                    class="input w-full uppercase"
                    class:input-error=move || errors.get().sku.is_some()
                    placeholder="z. B. TS-001"
                    autocapitalize="characters"
                    aria-required="true"
                />
            </Field>
        </div>

        <div class="grid gap-4 sm:grid-cols-2">
            <Field label="Kategorie" error=err(|e| e.category.clone())>
                <select
                    name="category"
                    class="select w-full"
                    class:select-error=move || errors.get().category.is_some()
                >
                    {ProductCategory::ALL
                        .into_iter()
                        .map(|c| view! { <option value=c.as_str() selected=c == category>{c.label()}</option> })
                        .collect_view()}
                </select>
            </Field>
            <Field label="Marke / Künstler:in" hint="optional" error=err(|e| e.brand.clone())>
                <input
                    type="text"
                    name="brand"
                    value=brand
                    class="input w-full"
                    class:input-error=move || errors.get().brand.is_some()
                    placeholder="z. B. Saintwhoo oder Mara Voss"
                />
            </Field>
        </div>

        <Field label="Kurzbeschreibung" hint="optional, erscheint im Shop" error=err(|e| e.short_description.clone())>
            <input
                type="text"
                name="short_description"
                value=short_description
                class="input w-full"
                class:input-error=move || errors.get().short_description.is_some()
                placeholder="Ein Satz zum Produkt"
            />
        </Field>

        <Field label="Beschreibung" hint="optional" error=Signal::derive(|| None::<String>)>
            <textarea name="description" rows="6" class="textarea w-full" placeholder="Ausführliche Beschreibung">
                {description}
            </textarea>
        </Field>

        <div class="grid gap-4 sm:grid-cols-3">
            <Field label="Preis (CHF)" error=err(|e| e.price.clone())>
                <input
                    type="text"
                    inputmode="decimal"
                    name="price"
                    value=price
                    class="input w-full"
                    class:input-error=move || errors.get().price.is_some()
                    placeholder="19.90"
                    aria-required="true"
                />
            </Field>
            <Field label="Lagerbestand" error=err(|e| e.stock.clone())>
                <input
                    type="number"
                    inputmode="numeric"
                    min="0"
                    step="1"
                    name="stock"
                    value=stock
                    class="input w-full"
                    class:input-error=move || errors.get().stock.is_some()
                    aria-required="true"
                />
            </Field>
            <Field label="Status" error=err(|e| e.status.clone())>
                <select
                    name="status"
                    class="select w-full"
                    class:select-error=move || errors.get().status.is_some()
                >
                    {ProductStatus::ALL
                        .into_iter()
                        .map(|s| view! { <option value=s.as_str() selected=s == status>{s.label()}</option> })
                        .collect_view()}
                </select>
            </Field>
        </div>

        // Teil C: Bild
        <div class="grid gap-4 sm:grid-cols-2">
            <Field label="Bild-Adresse" hint="optional" error=err(|e| e.image_url.clone())>
                <input
                    type="text"
                    inputmode="url"
                    name="image_url"
                    value=image_url
                    class="input w-full"
                    class:input-error=move || errors.get().image_url.is_some()
                    placeholder="https://… oder /images/produkte/…"
                />
            </Field>
            <Field label="Bildnachweis" hint="Pflicht mit Bild" error=err(|e| e.image_credit.clone())>
                <input
                    type="text"
                    name="image_credit"
                    value=image_credit
                    class="input w-full"
                    class:input-error=move || errors.get().image_credit.is_some()
                    placeholder="Foto: Name / Quelle (Lizenz)"
                />
            </Field>
        </div>
    }
}

/// Status als Badge: Entwurf grau, Aktiv grün.
#[component]
pub fn ProductStatusBadge(status: ProductStatus) -> impl IntoView {
    let class = match status {
        ProductStatus::Draft => "badge badge-ghost",
        ProductStatus::Active => "badge badge-success",
    };
    view! { <span class=class>{status.label()}</span> }
}

/// Verfügbarkeit als Badge: ausverkauft rot, wenige gelb, an Lager grün.
#[component]
pub fn AvailabilityBadge(stock: i32, #[prop(optional)] large: bool) -> impl IntoView {
    let a = Availability::from_stock(stock);
    let class = match (a, large) {
        (Availability::SoldOut, false) => "badge badge-error",
        (Availability::Low(_), false) => "badge badge-warning",
        (Availability::InStock, false) => "badge badge-success",
        (Availability::SoldOut, true) => "badge badge-error badge-lg",
        (Availability::Low(_), true) => "badge badge-warning badge-lg",
        (Availability::InStock, true) => "badge badge-success badge-lg",
    };
    view! { <span class=class>{a.label()}</span> }
}

/// Produktbild – mit `image` das Foto (Bildnachweis klein unten rechts),
/// sonst die Platzhalter-Fläche mit Anfangsbuchstabe.
#[component]
pub fn ProductCover(
    id: i64,
    name: String,
    #[prop(default = None)] image: Option<ProductImage>,
    #[prop(optional)] class: &'static str,
    #[prop(default = "text-8xl")] letter_size: &'static str,
    #[prop(default = true)] show_credit: bool,
    #[prop(optional)] eager: bool,
) -> impl IntoView {
    view! { <CoverImage id title=name image class letter_size show_credit eager label="Bild zum Produkt" /> }
}

/// Erfolgsmeldung nach einer Weiterleitung (`?meldung=gespeichert|geloescht`).
#[component]
pub fn ProductFlash() -> impl IntoView {
    let query = use_query_map();
    move || {
        let text = query.with(|q| match q.get("meldung").as_deref() {
            Some("gespeichert") => Some(msg::SAVED),
            Some("geloescht") => Some(msg::DELETED),
            _ => None,
        });
        text.map(|t| {
            view! {
                <div role="status" class="alert alert-success mb-4">
                    <span>{t}</span>
                </div>
            }
        })
    }
}

/// Löschen-Knopf mit Bestätigungsdialog. Funktioniert auch ohne JavaScript
/// (DaisyUI `modal-toggle`). `key` macht die ID eindeutig, wenn dasselbe
/// Produkt mehrmals auf einer Seite vorkommt (Tabelle + Karten).
#[component]
pub fn DeleteProductButton(
    id: i64,
    name: String,
    #[prop(optional)] key: &'static str,
    #[prop(optional)] small: bool,
) -> impl IntoView {
    let action = ServerAction::<DeleteProduct>::new();
    let version = use_context::<ProductsVersion>();
    Effect::new(move |_| {
        if let Some(Ok(DeleteOutcome::Deleted)) = action.value().get()
            && let Some(v) = version
        {
            v.0.update(|n| *n += 1);
        }
    });

    let modal_id = format!("produkt-loeschen-{key}{id}");
    let heading_id = format!("{modal_id}-titel");
    let (for_open, for_cancel, for_backdrop) = (modal_id.clone(), modal_id.clone(), modal_id.clone());
    let labelledby = heading_id.clone();

    view! {
        <label for=for_open class="btn btn-error btn-outline" class:btn-sm=small>
            "Löschen"
        </label>
        <input type="checkbox" id=modal_id class="modal-toggle" />
        <div class="modal" role="dialog" aria-labelledby=labelledby>
            <div class="modal-box text-left">
                <h3 id=heading_id class="text-lg font-bold">"Produkt löschen"</h3>
                <p class="py-4">
                    "Produkt „" {name} "“ wirklich löschen? Das kann nicht rückgängig gemacht werden."
                </p>
                {move || match action.value().get() {
                    Some(Ok(DeleteOutcome::NotFound)) => Some(view! {
                        <div role="alert" class="alert alert-error">{msg::NOT_FOUND}</div>
                    }.into_any()),
                    Some(Err(e)) => Some(view! {
                        <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
                    }.into_any()),
                    _ => None,
                }}
                <div class="modal-action">
                    <label for=for_cancel class="btn">"Abbrechen"</label>
                    <ActionForm action>
                        <input type="hidden" name="id" value=id />
                        <button type="submit" class="btn btn-error" disabled=move || action.pending().get()>
                            <Show when=move || action.pending().get()>
                                <span class="loading loading-spinner loading-sm"></span>
                            </Show>
                            "Löschen"
                        </button>
                    </ActionForm>
                </div>
            </div>
            <label class="modal-backdrop" for=for_backdrop>"Schließen"</label>
        </div>
    }
    .into_any()
}
