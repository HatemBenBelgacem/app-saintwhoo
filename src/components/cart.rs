//! Warenkorb (Designvorschlag „Techno App“, Shop).
//!
//! Der Warenkorb lebt im Browser im Kontext von `App` und bleibt beim
//! Navigieren erhalten. Nach einem Neuladen der Seite ist er leer
//! (Bestellen/Kasse ist noch nicht Teil der Anforderungen).

use leptos::prelude::*;

use crate::products::model::format_chf;

/// Ein Produkt im Warenkorb.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CartItem {
    pub id: i64,
    pub name: String,
    pub price_cents: i32,
    pub quantity: i32,
    /// Lagerbestand beim Hinzufügen – mehr geht nicht in den Warenkorb.
    pub max: i32,
}

/// Reine Logik, damit sie ohne Browser testbar ist.
pub mod logic {
    use super::CartItem;

    /// Legt ein Stück dazu. Gibt `false` zurück, wenn der Bestand erreicht ist.
    pub fn add(items: &mut Vec<CartItem>, id: i64, name: &str, price_cents: i32, stock: i32) -> bool {
        if stock <= 0 {
            return false;
        }
        match items.iter_mut().find(|i| i.id == id) {
            Some(i) if i.quantity >= i.max => false,
            Some(i) => {
                i.quantity += 1;
                true
            }
            None => {
                items.push(CartItem { id, name: name.into(), price_cents, quantity: 1, max: stock });
                true
            }
        }
    }

    /// Menge ändern (1 … Bestand). 0 entfernt das Produkt.
    pub fn set_quantity(items: &mut Vec<CartItem>, id: i64, quantity: i32) {
        if quantity <= 0 {
            items.retain(|i| i.id != id);
        } else if let Some(i) = items.iter_mut().find(|i| i.id == id) {
            i.quantity = quantity.min(i.max);
        }
    }

    pub fn count(items: &[CartItem]) -> i32 {
        items.iter().map(|i| i.quantity).sum()
    }

    pub fn total_cents(items: &[CartItem]) -> i32 {
        items.iter().map(|i| i.price_cents * i.quantity).sum()
    }

    /// `1 Artikel`, `3 Artikel`
    pub fn count_text(n: i32) -> String {
        format!("{n} Artikel")
    }
}

#[derive(Clone, Copy)]
pub struct Cart {
    pub items: RwSignal<Vec<CartItem>>,
}

impl Cart {
    pub fn new() -> Self {
        Self { items: RwSignal::new(Vec::new()) }
    }

    pub fn count(&self) -> i32 {
        self.items.with(|i| logic::count(i))
    }

    pub fn total_cents(&self) -> i32 {
        self.items.with(|i| logic::total_cents(i))
    }

    pub fn quantity_of(&self, id: i64) -> i32 {
        self.items.with(|items| items.iter().find(|i| i.id == id).map(|i| i.quantity).unwrap_or(0))
    }
}

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}

pub fn use_cart() -> Cart {
    expect_context::<Cart>()
}

/// Runder „+“-Knopf (Karte) bzw. breiter Knopf (Produktseite).
#[component]
pub fn AddToCartButton(
    id: i64,
    name: String,
    price_cents: i32,
    stock: i32,
    #[prop(optional)] wide: bool,
) -> impl IntoView {
    let cart = use_cart();
    if stock <= 0 {
        // Auf Karten zeigt das Bild „Ausverkauft“ – dort kein zweiter Hinweis
        return match wide {
            true => view! { <button type="button" class="btn min-h-12 px-6" disabled=true>"Ausverkauft"</button> }.into_any(),
            false => ().into_any(),
        };
    }
    let in_cart = Memo::new(move |_| cart.quantity_of(id));
    let full = Memo::new(move |_| in_cart.get() >= stock);
    let label = format!("{name} in den Warenkorb legen");
    let n = name.clone();
    let add = move |_| {
        cart.items.update(|items| {
            logic::add(items, id, &n, price_cents, stock);
        });
    };

    if wide {
        view! {
            <button type="button" class="btn btn-primary min-h-12 px-6" on:click=add disabled=move || full.get()>
                {move || if full.get() { "Ganzer Bestand im Warenkorb".to_string() } else { "In den Warenkorb".to_string() }}
            </button>
            <Show when=move || { in_cart.get() > 0 }>
                <a href="/warenkorb" class="link text-sm" role="status">
                    {move || format!("{} im Warenkorb – ansehen", in_cart.get())}
                </a>
            </Show>
        }
        .into_any()
    } else {
        view! {
            <button
                type="button"
                class="btn btn-primary btn-circle size-11"
                aria-label=label
                on:click=add
                disabled=move || full.get()
            >
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" aria-hidden="true">
                    <path d="M12 5v14M5 12h14" />
                </svg>
            </button>
        }
        .into_any()
    }
}

/// Knopf „Warenkorb“ mit Anzahl (Shop-Kopf).
#[component]
pub fn CartButton() -> impl IntoView {
    let cart = use_cart();
    let count = Memo::new(move |_| cart.count());
    view! {
        <a
            href="/warenkorb"
            class="btn btn-outline min-h-11 gap-2 border-base-300 font-medium"
            aria-label=move || format!("Warenkorb, {}", logic::count_text(count.get()))
        >
            "Warenkorb"
            <span aria-hidden="true" class="badge badge-primary badge-sm min-w-6 font-bold">{move || count.get()}</span>
        </a>
    }
    .into_any()
}

/// Leiste unten im Shop: Anzahl, Summe, Link zum Warenkorb.
#[component]
pub fn CartBar() -> impl IntoView {
    let cart = use_cart();
    let count = Memo::new(move |_| cart.count());
    view! {
        <Show when=move || { count.get() > 0 }>
            <div class="sticky bottom-24 z-20 mt-6 flex items-center justify-between gap-3 rounded-2xl border border-base-300 bg-base-200 p-3 pl-4 md:bottom-4">
                <div class="flex flex-col" role="status">
                    <span class="text-sm text-muted">{move || logic::count_text(count.get())}</span>
                    <span class="text-lg font-semibold tabular-nums">{move || format_chf(cart.total_cents())}</span>
                </div>
                <a href="/warenkorb" class="btn btn-primary min-h-12 px-5">"Zum Warenkorb"</a>
            </div>
        </Show>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::logic::*;

    #[test]
    fn hinzufuegen_bis_zum_bestand() {
        let mut items = Vec::new();
        assert!(add(&mut items, 1, "Hoodie", 5900, 2));
        assert!(add(&mut items, 1, "Hoodie", 5900, 2));
        assert!(!add(&mut items, 1, "Hoodie", 5900, 2), "Bestand 2 erreicht");
        assert!(add(&mut items, 2, "Tote Bag", 1900, 10));
        assert_eq!(count(&items), 3);
        assert_eq!(total_cents(&items), 2 * 5900 + 1900);
    }

    #[test]
    fn ausverkauft_nicht_moeglich() {
        let mut items = Vec::new();
        assert!(!add(&mut items, 1, "Slipmat", 2400, 0));
        assert!(items.is_empty());
    }

    #[test]
    fn menge_aendern_und_entfernen() {
        let mut items = Vec::new();
        add(&mut items, 1, "Hoodie", 5900, 3);
        set_quantity(&mut items, 1, 10);
        assert_eq!(items[0].quantity, 3, "höchstens Bestand");
        set_quantity(&mut items, 1, 0);
        assert!(items.is_empty());
    }
}
