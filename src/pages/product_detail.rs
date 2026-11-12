//! ANF-005: Detailseite (Verwaltung) – `/produkte/<id>`

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::articles::model::format_timestamp;
use crate::components::article::server_error_text;
use crate::components::product::{
    AvailabilityBadge, DeleteProductButton, ProductCover, ProductFlash, ProductStatusBadge,
};
use crate::pages::not_found::NotFound;
use crate::products::model::{Product, ProductStatus, format_chf};

type ProductResource = Resource<Result<Option<Product>, ServerFnError>>;

/// Liest `:id` aus der Adresse und lädt das Produkt mit `load`. Eine
/// ungültige ID verhält sich wie eine unbekannte (404).
pub fn product_from_params<F, Fut>(load: F) -> ProductResource
where
    F: Fn(i64) -> Fut + Copy + Send + Sync + 'static,
    Fut: std::future::Future<Output = Result<Option<Product>, ServerFnError>> + Send + 'static,
{
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|s| s.parse::<i64>().ok()));
    Resource::new(id, move |id| async move {
        match id {
            Some(id) => load(id).await,
            None => Ok(None),
        }
    })
}

/// Zeigt Laden / Fehler / 404 und ruft sonst `view` mit dem Produkt auf.
pub fn with_product<V: IntoView + 'static>(product: ProductResource, view: fn(Product) -> V) -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <span class="loading loading-dots loading-lg"></span> }>
            {move || Suspend::new(async move {
                match product.await {
                    Ok(Some(p)) => view(p).into_any(),
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
pub fn ProductDetailPage() -> impl IntoView {
    use crate::products::api::get_product;
    with_product(product_from_params(get_product), |p| view! { <ProductDetail product=p /> })
}

#[component]
fn ProductDetail(product: Product) -> impl IntoView {
    let id = product.id;
    let active = product.status == ProductStatus::Active;
    let description = (!product.description.is_empty()).then(|| product.description.clone());
    let short = (!product.short_description.is_empty()).then(|| product.short_description.clone());

    view! {
        <Title text=format!("{} – Produkte – Saintwhoo", product.name) />
        <div class="max-w-3xl mx-auto">
            <a href="/produkte" class="btn btn-ghost btn-sm mb-4">"← Zurück zur Liste"</a>

            <ProductFlash />

            <article class="card bg-base-100 shadow overflow-hidden">
                {product.image.clone().map(|img| {
                    let credit = img.credit.clone();
                    view! {
                        <figure class="flex-col">
                            <ProductCover
                                id=id
                                name=product.name.clone()
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
                        <ProductStatusBadge status=product.status />
                        <AvailabilityBadge stock=product.stock />
                        <span class="font-mono text-sm text-base-content/70">{product.sku.clone()}</span>
                    </div>

                    <h1 class="text-3xl font-bold break-words">{product.name.clone()}</h1>
                    {short.map(|s| view! { <p class="text-lg text-base-content/80">{s}</p> })}

                    <dl class="grid grid-cols-[auto_1fr] gap-x-6 gap-y-1">
                        <dt class="text-base-content/70">"Preis"</dt>
                        <dd class="font-semibold tabular-nums">{format_chf(product.price_cents)}</dd>
                        <dt class="text-base-content/70">"Lagerbestand"</dt>
                        <dd class="tabular-nums">{product.stock}</dd>
                    </dl>

                    {match description {
                        Some(d) => view! {
                            <div class="whitespace-pre-wrap break-words leading-relaxed mt-2">{d}</div>
                        }.into_any(),
                        None => view! {
                            <p class="italic text-base-content/60 mt-2">"Keine Beschreibung"</p>
                        }.into_any(),
                    }}

                    <dl class="mt-4 grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-sm text-base-content/70">
                        <dt>"Erstellt am"</dt>
                        <dd>{format_timestamp(product.created_at)}</dd>
                        <dt>"Zuletzt geändert"</dt>
                        <dd>{format_timestamp(product.updated_at)}</dd>
                    </dl>

                    <div class="card-actions justify-end mt-4">
                        <Show when=move || active>
                            <a href=format!("/shop/{id}") class="btn btn-ghost">"Im Shop ansehen"</a>
                        </Show>
                        <a href=format!("/produkte/{id}/bearbeiten") class="btn btn-primary">"Bearbeiten"</a>
                        <DeleteProductButton id name=product.name />
                    </div>
                </div>
            </article>
        </div>
    }
}
