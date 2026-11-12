use leptos::html;
use leptos::prelude::*;

/// Kopfzeile im Magazin-Stil: Name groß in der Mitte.
/// Links: Magazin und Shop (ANF-005). Rechts: Verwaltung – auf kleinen
/// Bildschirmen als Menü, sonst als einzelne Links.
#[component]
pub fn Navbar() -> impl IntoView {
    let menu = NodeRef::<html::Details>::new();
    // Menü nach einem Klick schließen (bei Navigation ohne Seitenwechsel)
    let close = move |_| {
        if let Some(d) = menu.get() {
            let _ = d.remove_attribute("open");
        }
    };

    view! {
        <header class="sticky top-0 z-30 border-b border-base-300 bg-base-100/95 backdrop-blur">
            <div class="container mx-auto grid h-16 grid-cols-[1fr_auto_1fr] items-center px-4 md:h-20">
                <nav class="flex gap-6 text-xs font-semibold uppercase tracking-widest">
                    <a href="/" class="hidden hover:underline underline-offset-4 sm:inline">"Magazin"</a>
                    <a href="/shop" class="hover:underline underline-offset-4">"Shop"</a>
                </nav>
                <a
                    href="/"
                    class="font-serif text-2xl font-black uppercase tracking-tight sm:text-3xl md:text-4xl"
                >
                    "Saintwhoo"
                </a>
                <div class="flex justify-end gap-2">
                    // Handy und Tablet: Menü (funktioniert auch ohne JavaScript)
                    <details node_ref=menu class="dropdown dropdown-end md:hidden">
                        <summary class="btn btn-ghost btn-sm text-xs uppercase tracking-widest">"Redaktion"</summary>
                        <ul on:click=close class="menu dropdown-content z-40 mt-2 w-52 rounded-box bg-base-100 p-2 shadow-lg">
                            <li><a href="/artikel">"Artikel"</a></li>
                            <li><a href="/artikel/neu">"Neuer Artikel"</a></li>
                            <li><a href="/produkte">"Produkte"</a></li>
                            <li><a href="/produkte/neu">"Neues Produkt"</a></li>
                        </ul>
                    </details>
                    // Desktop: einzelne Links
                    <a href="/artikel" class="btn btn-ghost btn-sm hidden text-xs uppercase tracking-widest md:inline-flex">
                        "Redaktion"
                    </a>
                    <a href="/produkte" class="btn btn-ghost btn-sm hidden text-xs uppercase tracking-widest md:inline-flex">
                        "Produkte"
                    </a>
                    <a href="/artikel/neu" class="btn btn-neutral btn-sm hidden lg:inline-flex">
                        "Neuer Artikel"
                    </a>
                </div>
            </div>
        </header>
    }
}
