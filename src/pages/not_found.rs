use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
            resp.set_status(axum::http::StatusCode::NOT_FOUND);
        }
    }

    view! {
        <div class="hero min-h-[60vh]">
            <div class="hero-content text-center">
                <div>
                    <h1 class="text-5xl font-bold">"404"</h1>
                    <p class="py-4">"Diese Seite gibt es nicht."</p>
                    <a href="/" class="btn btn-primary">"Zur Startseite"</a>
                </div>
            </div>
        </div>
    }
}
