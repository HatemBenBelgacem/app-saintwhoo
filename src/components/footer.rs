use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-20 border-t border-base-300">
            <div class="container mx-auto flex flex-col items-center gap-4 px-4 py-10 sm:flex-row sm:justify-between">
                <a href="/" class="font-serif text-2xl font-black uppercase tracking-tight">"Saintwhoo"</a>
                <nav class="flex gap-6 text-xs font-semibold uppercase tracking-widest text-base-content/70">
                    <a href="/artikel" class="hover:text-base-content">"Redaktion"</a>
                    <a href="/status" class="hover:text-base-content">"Systemstatus"</a>
                </nav>
                <p class="text-xs text-base-content/60">"© Saintwhoo Magazin"</p>
            </div>
        </footer>
    }
}
