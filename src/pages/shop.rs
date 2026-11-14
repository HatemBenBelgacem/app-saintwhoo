//! ANF-005: Shop-Übersicht – `/shop?kategorie=hoodies&sortierung=preis-auf&seite=2`
//!
//! Layout nach dem Designvorschlag „Techno App“: Kategorie-Chips, Karten im
//! Raster mit „+“-Knopf, Warenkorb oben und als Leiste unten.
//! Filter, Sortierung und Seiten sind normale Links: teilbar und ohne JavaScript nutzbar.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

use crate::components::article::server_error_text;
use crate::components::cart::{AddToCartButton, CartBar, CartButton};
use crate::components::product::ProductCover;
use crate::components::ui::{Chip, PageHeader};
use crate::products::api::shop_products;
use crate::products::model::{Availability, ProductCategory, ShopItem, ShopPage, ShopSort, format_chf, msg};

/// Adresse des Shops; Standardwerte werden weggelassen.
fn shop_href(page: u32, sort: ShopSort, category: Option<ProductCategory>) -> String {
    let mut params = Vec::new();
    if let Some(c) = category {
        params.push(format!("kategorie={}", c.slug()));
    }
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
    let category = move || query.with(|q| q.get("kategorie").and_then(|s| ProductCategory::from_slug(&s)));

    let data = Resource::new(move || (page(), sort(), category()), |(p, s, c)| shop_products(p, s, c));
    let chips = std::iter::once(None).chain(ProductCategory::ALL.into_iter().map(Some));

    view! {
        <Title text="Shop – Saintwhoo" />
        <div class="py-6 md:py-10">
            <PageHeader title="Shop" subtitle="Merch von DJs und Labels">
                <CartButton />
            </PageHeader>

            <nav aria-label="Kategorie" class="-mx-4 mt-5 flex gap-2 overflow-x-auto px-4 pb-1">
                {chips
                    .map(|c| {
                        let label = c.map(ProductCategory::label).unwrap_or("Alle");
                        view! {
                            <Chip
                                href=Signal::derive(move || shop_href(1, sort(), c))
                                active=Signal::derive(move || category() == c)
                                label
                            />
                        }
                    })
                    .collect_view()}
            </nav>

            <nav aria-label="Sortierung" class="mt-3 flex flex-wrap items-center gap-1">
                <span class="mr-1 text-sm text-muted">"Sortieren:"</span>
                {ShopSort::ALL
                    .into_iter()
                    .map(|s| view! {
                        <a
                            href=move || shop_href(1, s, category())
                            class="btn btn-ghost btn-sm font-normal"
                            class=("text-primary", move || sort() == s)
                            class=("font-semibold", move || sort() == s)
                            aria-current=move || (sort() == s).then_some("true")
                        >
                            {s.label()}
                        </a>
                    })
                    .collect_view()}
            </nav>

            <Suspense fallback=|| view! {
                <div class="flex justify-center py-24"><span class="loading loading-dots loading-lg"></span></div>
            }>
                {move || {
                    let (sort, category) = (sort(), category());
                    Suspend::new(async move {
                        match data.await {
                            Ok(p) if p.items.is_empty() => {
                                let text = if category.is_some() { msg::SHOP_EMPTY_CATEGORY } else { msg::SHOP_EMPTY };
                                view! { <p class="py-24 text-center font-display text-2xl">{text}</p> }.into_any()
                            }
                            Ok(p) => view! { <ShopGrid data=p sort category /> }.into_any(),
                            Err(e) => view! {
                                <div role="alert" class="alert alert-error mt-6">{server_error_text(&e)}</div>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>

            <CartBar />
        </div>
    }
    .into_any()
}

#[component]
fn ShopGrid(data: ShopPage, sort: ShopSort, category: Option<ProductCategory>) -> impl IntoView {
    let total_pages = data.total_pages;
    let current = data.page;
    view! {
        <p class="mt-6 mb-3 text-sm text-muted">{format!("{} Produkte", data.total)}</p>
        <ul class="grid grid-cols-2 gap-3 md:grid-cols-3 lg:grid-cols-4">
            {data.items.into_iter().map(|p| view! { <ShopCard product=p /> }).collect_view()}
        </ul>

        <Show when=move || { total_pages > 1 }>
            <nav class="join mt-10 flex justify-center" aria-label="Seiten">
                {(1..=total_pages)
                    .map(|n| view! {
                        <a
                            href=shop_href(n, sort, category)
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
    let sold_out = Availability::from_stock(product.stock) == Availability::SoldOut;
    let low = match Availability::from_stock(product.stock) {
        Availability::Low(_) => Some(Availability::from_stock(product.stock).label()),
        _ => None,
    };
    let brand = (!product.brand.is_empty()).then(|| product.brand.clone());
    view! {
        <li class="group flex flex-col gap-2.5 rounded-2xl border border-base-300 bg-base-200 p-2.5">
            <a href=link.clone() class="relative block overflow-hidden rounded-xl" tabindex="-1" aria-hidden="true">
                <div class="transition-transform duration-500 group-hover:scale-105">
                    <ProductCover id=product.id name=product.name.clone() image=product.image.clone() class="aspect-square" letter_size="text-7xl" show_credit=false />
                </div>
                <Show when=move || sold_out>
                    <span class="absolute inset-0 flex items-center justify-center bg-black/40">
                        <span class="rounded-md bg-base-content px-2 py-1 text-xs font-bold text-base-100">"Ausverkauft"</span>
                    </span>
                </Show>
            </a>
            <div class="flex min-w-0 flex-col gap-0.5">
                <h2 class="text-[15px] font-semibold leading-snug break-words">
                    <a href=link class="hover:underline">{product.name.clone()}</a>
                </h2>
                {brand.map(|b| view! { <span class="text-[13px] text-muted">{b}</span> })}
                {low.map(|l| view! { <span class="text-[13px] text-warning">{l}</span> })}
            </div>
            <div class="mt-auto flex min-h-11 items-center justify-between gap-2">
                <span class="text-[15px] font-semibold whitespace-nowrap tabular-nums" class:text-muted=sold_out>{format_chf(product.price_cents)}</span>
                <AddToCartButton id=product.id name=product.name price_cents=product.price_cents stock=product.stock />
            </div>
        </li>
    }
    .into_any()
}
