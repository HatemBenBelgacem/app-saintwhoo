//! ANF-005: Produktliste (Verwaltung) – `/produkte?status=active&seite=2`

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

use crate::articles::model::format_timestamp;
use crate::components::article::server_error_text;
use crate::components::product::{
    DeleteProductButton, ProductFlash, ProductStatusBadge, ProductsVersion,
};
use crate::products::api::list_products;
use crate::products::model::{ProductPage, ProductStatus, ProductSummary, format_chf, msg};

/// Adresse der Liste mit Filter und Seite (Seite 1 wird weggelassen).
fn list_href(page: u32, status: Option<ProductStatus>) -> String {
    let mut params = Vec::new();
    if let Some(s) = status {
        params.push(format!("status={}", s.as_str()));
    }
    if page > 1 {
        params.push(format!("seite={page}"));
    }
    if params.is_empty() { "/produkte".into() } else { format!("/produkte?{}", params.join("&")) }
}

#[component]
pub fn ProductListPage() -> impl IntoView {
    let query = use_query_map();
    let version = expect_context::<ProductsVersion>().0;

    let page = move || query.with(|q| q.get("seite").and_then(|s| s.parse::<u32>().ok()).unwrap_or(1));
    let status = move || query.with(|q| q.get("status").and_then(|s| ProductStatus::parse(&s)));

    let data = Resource::new(
        move || (page(), status(), version.get()),
        |(page, status, _)| list_products(page, status),
    );

    view! {
        <Title text="Produkte – Saintwhoo" />
        <div class="flex flex-wrap items-center justify-between gap-4 mb-6">
            <h1 class="text-3xl font-bold">"Produkte"</h1>
            <div class="flex gap-2">
                <a href="/shop" class="btn btn-ghost">"Zum Shop"</a>
                <a href="/produkte/neu" class="btn btn-primary">"Neues Produkt"</a>
            </div>
        </div>

        <ProductFlash />

        <div role="tablist" class="tabs tabs-box mb-4 w-fit">
            {[None, Some(ProductStatus::Draft), Some(ProductStatus::Active)]
                .into_iter()
                .map(|s| {
                    let label = s.map(ProductStatus::label).unwrap_or("Alle");
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
                        Ok(p) => view! { <ProductList data=p filter /> }.into_any(),
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
fn ProductList(data: ProductPage, filter: Option<ProductStatus>) -> impl IntoView {
    if data.items.is_empty() {
        let (text, show_button) = match filter {
            None => (msg::LIST_EMPTY, true),
            Some(_) => (msg::LIST_EMPTY_FILTER, false),
        };
        return view! {
            <div class="card bg-base-100 shadow">
                <div class="card-body items-center text-center">
                    <p>{text}</p>
                    <Show when=move || show_button>
                        <a href="/produkte/neu" class="btn btn-primary">"Neues Produkt"</a>
                    </Show>
                </div>
            </div>
        }
        .into_any();
    }

    let items = data.items.clone();
    view! {
        // Desktop: Tabelle
        <div class="hidden md:block overflow-x-auto bg-base-100 rounded-box shadow">
            <table class="table">
                <thead>
                    <tr>
                        <th>"Name"</th>
                        <th>"Artikelnummer"</th>
                        <th class="text-right">"Preis"</th>
                        <th class="text-right">"Lagerbestand"</th>
                        <th>"Status"</th>
                        <th>"Zuletzt geändert"</th>
                        <th class="text-right">"Aktionen"</th>
                    </tr>
                </thead>
                <tbody>
                    {data.items.into_iter().map(|p| view! { <ProductRow product=p /> }).collect_view()}
                </tbody>
            </table>
        </div>

        // Handy: Karten
        <div class="md:hidden flex flex-col gap-3">
            {items.into_iter().map(|p| view! { <ProductCard product=p /> }).collect_view()}
        </div>

        <Pagination page=data.page total_pages=data.total_pages total=data.total filter />
    }
    .into_any()
}

#[component]
fn ProductRow(product: ProductSummary) -> impl IntoView {
    let id = product.id;
    view! {
        <tr class="hover:bg-base-200">
            <td>
                <a href=format!("/produkte/{id}") class="link link-hover font-medium">{product.name.clone()}</a>
            </td>
            <td class="font-mono text-sm">{product.sku}</td>
            <td class="text-right tabular-nums whitespace-nowrap">{format_chf(product.price_cents)}</td>
            <td class="text-right tabular-nums">{product.stock}</td>
            <td><ProductStatusBadge status=product.status /></td>
            <td class="text-sm text-base-content/70">{format_timestamp(product.updated_at)}</td>
            <td>
                <div class="flex justify-end gap-2">
                    <a href=format!("/produkte/{id}/bearbeiten") class="btn btn-sm">"Bearbeiten"</a>
                    <DeleteProductButton id name=product.name key="t" small=true />
                </div>
            </td>
        </tr>
    }
    .into_any()
}

#[component]
fn ProductCard(product: ProductSummary) -> impl IntoView {
    let id = product.id;
    view! {
        <div class="card bg-base-100 shadow">
            <div class="card-body p-4 gap-2">
                <a href=format!("/produkte/{id}") class="card-title link link-hover break-words">{product.name.clone()}</a>
                <div class="flex flex-wrap items-center gap-2 text-sm">
                    <ProductStatusBadge status=product.status />
                    <span class="font-mono">{product.sku}</span>
                    <span class="font-semibold tabular-nums">{format_chf(product.price_cents)}</span>
                    <span>"Bestand: " {product.stock}</span>
                </div>
                <p class="text-xs text-base-content/70">
                    "Zuletzt geändert: " {format_timestamp(product.updated_at)}
                </p>
                <div class="card-actions justify-end">
                    <a href=format!("/produkte/{id}/bearbeiten") class="btn btn-sm">"Bearbeiten"</a>
                    <DeleteProductButton id name=product.name key="k" small=true />
                </div>
            </div>
        </div>
    }
    .into_any()
}

#[component]
fn Pagination(page: u32, total_pages: u32, total: i64, filter: Option<ProductStatus>) -> impl IntoView {
    view! {
        <div class="mt-4 flex flex-wrap items-center justify-between gap-2">
            <span class="text-sm text-base-content/70">{total} " Produkte"</span>
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
