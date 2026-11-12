//! ANF-005: Öffentliche Produktseite – `/shop/<id>`
//!
//! Entwürfe und unbekannte IDs zeigen die 404-Seite (Route mit `SsrMode::Async`).

use leptos::prelude::*;
use leptos_meta::Title;

use crate::articles::model::paragraphs;
use crate::components::magazine::Kicker;
use crate::components::product::{AvailabilityBadge, ProductCover};
use crate::pages::product_detail::{product_from_params, with_product};
use crate::products::api::get_shop_product;
use crate::products::model::{Product, format_chf};

#[component]
pub fn ShopProductPage() -> impl IntoView {
    with_product(product_from_params(get_shop_product), |p| view! { <ShopProduct product=p /> })
}

#[component]
fn ShopProduct(product: Product) -> impl IntoView {
    let paras = paragraphs(&product.description);
    let short = (!product.short_description.is_empty()).then(|| product.short_description.clone());

    view! {
        <Title text=format!("{} – Shop – Saintwhoo", product.name) />
        <div class="py-6 md:py-10">
            <a href="/shop" class="text-xs font-semibold uppercase tracking-widest text-base-content/60 hover:text-base-content">
                "← Zurück zum Shop"
            </a>

            <article class="mt-6 grid gap-8 md:grid-cols-2 md:gap-12">
                <figure>
                    <ProductCover
                        id=product.id
                        name=product.name.clone()
                        image=product.image.clone()
                        class="aspect-square"
                        letter_size="text-[10rem] md:text-[14rem]"
                        show_credit=false
                        eager=true
                    />
                    {product.image.clone().map(|i| view! {
                        <figcaption class="mt-2 text-right text-xs text-base-content/60">{i.credit}</figcaption>
                    })}
                </figure>

                <div>
                    <Kicker>{format!("Art.-Nr. {}", product.sku)}</Kicker>
                    <h1 class="mt-3 font-serif text-4xl font-bold leading-[1.1] break-words hyphens-auto md:text-5xl">
                        {product.name.clone()}
                    </h1>
                    {short.map(|s| view! { <p class="mt-4 text-lg text-base-content/80">{s}</p> })}

                    <div class="mt-6 flex flex-wrap items-center gap-4">
                        <span class="text-3xl font-bold tabular-nums">{format_chf(product.price_cents)}</span>
                        <AvailabilityBadge stock=product.stock large=true />
                    </div>
                    <p class="mt-1 text-xs text-base-content/60">"inkl. MWST"</p>

                    <div class="mt-8 space-y-4 leading-relaxed">
                        {paras
                            .into_iter()
                            .map(|p| view! { <p class="whitespace-pre-line break-words">{p}</p> })
                            .collect_view()}
                    </div>
                </div>
            </article>
        </div>
    }
    .into_any()
}
