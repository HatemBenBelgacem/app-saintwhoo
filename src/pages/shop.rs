//! ANF-005: Shop-Übersicht – `/shop?sortierung=preis-auf&seite=2`
//!
//! Sortierung und Seiten sind normale Links: teilbar und ohne JavaScript nutzbar.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

use crate::components::article::server_error_text;
use crate::components::magazine::{Kicker, SectionTitle};
use crate::components::product::{AvailabilityBadge, ProductCover};
use crate::products::api::shop_products;
use crate::products::model::{ShopItem, ShopPage, ShopSort, format_chf, msg};

/// Adresse des Shops; Standardwerte werden weggelassen.
fn shop_href(page: u32, sort: ShopSort) -> String {
    let mut params = Vec::new();
    if sort != ShopSort::default() {
        params.push(format!("sortierung={}", sort.as_str()));
    }
    if page > 1 {
        params.push(format!("seite={page}"));
    }
    if params.is_empty() { "/shop".into() } else { format!("/shop?{}", params.join("&")) }
}

#[component]
pub fn ShopPage() -> impl IntoView {
    let query = use_query_map();
    let page = move || query.with(|q| q.get("seite").and_then(|s| s.parse::<u32>().ok()).unwrap_or(1));
    let sort = move || query.with(|q| q.get("sortierung").map(|s| ShopSort::parse(&s)).unwrap_or_default());

    let data = Resource::new(move || (page(), sort()), |(page, sort)| shop_products(page, sort));

    view! {
        <Title text="Shop – Saintwhoo" />
        <div class="py-6 md:py-10">
            <header class="mb-8 flex flex-wrap items-end justify-between gap-6">
                <div>
                    <Kicker accent=true>"Saintwhoo"</Kicker>
                    <h1 class="mt-2 font-serif text-4xl font-bold md:text-5xl">"Shop"</h1>
                </div>
                <nav aria-label="Sortierung" class="flex flex-wrap items-center gap-2">
                    <span class="text-xs font-semibold uppercase tracking-widest text-base-content/60">
                        "Sortieren:"
                    </span>
                    {ShopSort::ALL
                        .into_iter()
                        .map(|s| view! {
                            <a
                                href=shop_href(1, s)
                                class="btn btn-xs"
                                class:btn-neutral=move || sort() == s
                                class:btn-ghost=move || sort() != s
                                aria-current=move || (sort() == s).then_some("true")
                            >
                                {s.label()}
                            </a>
                        })
                        .collect_view()}
                </nav>
            </header>

            <Suspense fallback=|| view! {
                <div class="flex justify-center py-24"><span class="loading loading-dots loading-lg"></span></div>
            }>
                {move || {
                    let sort = sort();
                    Suspend::new(async move {
                        match data.await {
                            Ok(p) if p.items.is_empty() => view! {
                                <p class="py-24 text-center font-serif text-3xl">{msg::SHOP_EMPTY}</p>
                            }.into_any(),
                            Ok(p) => view! { <ShopGrid data=p sort /> }.into_any(),
                            Err(e) => view! {
                                <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn ShopGrid(data: ShopPage, sort: ShopSort) -> impl IntoView {
    let total_pages = data.total_pages;
    let current = data.page;
    view! {
        <SectionTitle>{format!("{} Produkte", data.total)}</SectionTitle>
        <ul class="grid gap-x-6 gap-y-12 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {data.items.into_iter().map(|p| view! { <ShopCard product=p /> }).collect_view()}
        </ul>

        <Show when=move || { total_pages > 1 }>
            <nav class="join mt-12 flex justify-center" aria-label="Seiten">
                {(1..=total_pages)
                    .map(|n| view! {
                        <a
                            href=shop_href(n, sort)
                            class="join-item btn btn-sm"
                            class:btn-active=n == current
                            aria-current=(n == current).then_some("page")
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

#[component]
fn ShopCard(product: ShopItem) -> impl IntoView {
    let link = format!("/shop/{}", product.id);
    let short = (!product.short_description.is_empty()).then(|| product.short_description.clone());
    view! {
        <li class="group flex flex-col">
            <a href=link.clone() class="block overflow-hidden" tabindex="-1" aria-hidden="true">
                <div class="transition-transform duration-500 group-hover:scale-105">
                    <ProductCover id=product.id name=product.name.clone() image=product.image.clone() class="aspect-square" />
                </div>
            </a>
            <h2 class="mt-4 font-serif text-xl font-bold leading-snug break-words hyphens-auto">
                <a href=link class="decoration-2 underline-offset-4 group-hover:underline">{product.name}</a>
            </h2>
            {short.map(|s| view! { <p class="mt-1 text-sm text-base-content/70">{s}</p> })}
            <div class="mt-auto flex flex-wrap items-center justify-between gap-2 pt-3">
                <span class="text-lg font-semibold tabular-nums">{format_chf(product.price_cents)}</span>
                <AvailabilityBadge stock=product.stock />
            </div>
        </li>
    }
    .into_any()
}
