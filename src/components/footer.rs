use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-16 border-t border-base-300">
            <div class="container mx-auto flex flex-col items-center gap-4 px-4 py-10 sm:flex-row sm:justify-between">
                <a href="/" class="inline-flex min-h-11 items-center font-display text-2xl font-bold tracking-tight">"Saintwhoo"</a>
                <nav aria-label="Fußzeile" class="flex flex-wrap justify-center gap-x-6 gap-y-2 text-sm text-muted">
                    <a href="/news" class="inline-flex min-h-11 items-center hover:text-base-content">"News"</a>
                    <a href="/eventliste" class="inline-flex min-h-11 items-center hover:text-base-content">"Events"</a>
                    <a href="/radio" class="inline-flex min-h-11 items-center hover:text-base-content">"Radio"</a>
                    <a href="/shop" class="inline-flex min-h-11 items-center hover:text-base-content">"Shop"</a>
                    <a href="/status" class="inline-flex min-h-11 items-center hover:text-base-content">"Systemstatus"</a>
                </nav>
                <p class="text-sm text-muted">"© Saintwhoo"</p>
            </div>
        </footer>
    }
}
