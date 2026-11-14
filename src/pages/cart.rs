//! Warenkorb – `/warenkorb`
//!
//! Zeigt die ausgewählten Produkte mit Menge und Summe. Bestellen ist noch
//! nicht Teil der Anforderungen; die Seite sagt das offen.

use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::cart::{CartItem, logic, use_cart};
use crate::components::ui::PageHeader;
use crate::products::model::format_chf;

#[component]
pub fn CartPage() -> impl IntoView {
    let cart = use_cart();
    let empty = Memo::new(move |_| cart.items.with(|i| i.is_empty()));

    view! {
        <Title text="Warenkorb – Saintwhoo" />
        <div class="mx-auto max-w-2xl py-6 md:py-10">
            <PageHeader title="Warenkorb" />

            <Show
                when=move || !empty.get()
                fallback=|| view! {
                    <div class="mt-8 rounded-box border border-base-300 bg-base-200 p-8 text-center">
                        <p class="font-display text-xl">"Dein Warenkorb ist leer."</p>
                        <a href="/shop" class="btn btn-primary mt-5 min-h-12">"Zum Shop"</a>
                    </div>
                }
            >
                <ul class="mt-6 flex flex-col gap-3">
                    <For each=move || cart.items.get() key=|i| (i.id, i.quantity) let:item>
                        <CartRow item />
                    </For>
                </ul>

                <div class="mt-6 flex items-center justify-between rounded-2xl border border-base-300 bg-base-200 p-4">
                    <span class="text-muted">{move || format!("Summe ({})", logic::count_text(cart.count()))}</span>
                    <span class="text-xl font-semibold tabular-nums">{move || format_chf(cart.total_cents())}</span>
                </div>
                <p class="mt-1 text-right text-xs text-muted">"inkl. MWST"</p>

                <div role="note" class="alert mt-6 border-base-300 bg-base-200">
                    <span>"Bestellen ist noch nicht möglich. Die Kasse folgt in einer späteren Version."</span>
                </div>
                <a href="/shop" class="btn btn-ghost mt-4">"← Weiter einkaufen"</a>
            </Show>
        </div>
    }
    .into_any()
}

#[component]
fn CartRow(item: CartItem) -> impl IntoView {
    let cart = use_cart();
    let id = item.id;
    let qty = item.quantity;
    let set = move |n: i32| cart.items.update(|items| logic::set_quantity(items, id, n));
    let name = item.name.clone();
    view! {
        <li class="flex flex-wrap items-center gap-3 rounded-2xl border border-base-300 bg-base-200 p-4">
            <div class="min-w-0 flex-1">
                <a href=format!("/shop/{id}") class="font-semibold hover:underline">{item.name.clone()}</a>
                <p class="text-sm text-muted tabular-nums">{format_chf(item.price_cents)} " pro Stück"</p>
            </div>
            <div class="join" role="group" aria-label=format!("Menge {name}")>
                <button type="button" class="btn join-item min-h-11" aria-label="Eins weniger" on:click=move |_| set(qty - 1)>"−"</button>
                <span class="join-item flex min-w-11 items-center justify-center border-y border-base-300 px-3 tabular-nums" aria-live="polite">{qty}</span>
                <button type="button" class="btn join-item min-h-11" aria-label="Eins mehr" disabled=qty >= item.max on:click=move |_| set(qty + 1)>"+"</button>
            </div>
            <span class="w-28 text-right font-semibold tabular-nums">{format_chf(item.price_cents * qty)}</span>
            <button type="button" class="btn btn-ghost btn-sm min-h-11" on:click=move |_| set(0)>"Entfernen"</button>
        </li>
    }
    .into_any()
}
