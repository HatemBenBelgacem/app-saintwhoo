use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    SsrMode, StaticSegment,
    components::{Route, Router, Routes},
    path,
};

use crate::components::{
    article::ArticlesVersion,
    cart::Cart,
    event::EventsVersion,
    footer::Footer,
    navbar::{BottomNav, Navbar},
    player::{Player, PlayerBar},
    product::ProductsVersion,
};
use crate::pages::{
    article_detail::ArticleDetailPage, article_edit::ArticleEditPage,
    article_list::ArticleListPage, article_new::ArticleNewPage, beitrag::BeitragPage, cart::CartPage,
    event_detail::EventDetailPage, event_edit::EventEditPage, event_list::EventListPage,
    event_new::EventNewPage, eventliste::EventListePage, eventliste_event::EventListeEventPage,
    home::HomePage, news::NewsListPage, not_found::NotFound, product_detail::ProductDetailPage,
    product_edit::ProductEditPage, product_list::ProductListPage, product_new::ProductNewPage,
    radio::RadioPage, radio_admin::RadioAdminPage, shop::ShopPage, shop_product::ShopProductPage, status::StatusPage,
};

/// HTML-Grundgerüst, das der Server ausliefert.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="de" data-theme="saintwhoo" style="color-scheme: dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
                <meta name="theme-color" content="#0B0B0C" />
                // Schriften laut Designvorschlag: Inter (Text) + Space Grotesk (Titel)
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
                <link
                    rel="stylesheet"
                    href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Space+Grotesk:wght@500;700&display=swap"
                />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="min-h-screen bg-base-100 font-sans text-base-content antialiased">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(ArticlesVersion(RwSignal::new(0)));
    provide_context(ProductsVersion(RwSignal::new(0)));
    provide_context(EventsVersion(RwSignal::new(0)));
    provide_context(Player::new());
    provide_context(Cart::new());

    view! {
        <Stylesheet id="leptos" href="/pkg/app-saintwhoo.css" />
        <Title text="Saintwhoo – Techno News, Events, Radio und Merch" />

        <Router>
            <Navbar />
            <main class="container mx-auto p-4">
                <Routes fallback=NotFound>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=path!("/status") view=StatusPage />
                    <Route path=path!("/beitrag/:id") view=BeitragPage ssr=SsrMode::Async />
                    <Route path=path!("/artikel") view=ArticleListPage />
                    <Route path=path!("/artikel/neu") view=ArticleNewPage />
                    // Async: Seite erst senden, wenn der Artikel geladen ist –
                    // so kann bei unbekannter ID der Status 404 gesetzt werden
                    <Route path=path!("/artikel/:id") view=ArticleDetailPage ssr=SsrMode::Async />
                    <Route path=path!("/artikel/:id/bearbeiten") view=ArticleEditPage ssr=SsrMode::Async />
                    // ANF-005: Shop und Produktverwaltung
                    <Route path=path!("/shop") view=ShopPage />
                    <Route path=path!("/shop/:id") view=ShopProductPage ssr=SsrMode::Async />
                    <Route path=path!("/warenkorb") view=CartPage />
                    <Route path=path!("/produkte") view=ProductListPage />
                    <Route path=path!("/produkte/neu") view=ProductNewPage />
                    <Route path=path!("/produkte/:id") view=ProductDetailPage ssr=SsrMode::Async />
                    <Route path=path!("/produkte/:id/bearbeiten") view=ProductEditPage ssr=SsrMode::Async />
                    // ANF-008: News mit Suche und Rubrik
                    <Route path=path!("/news") view=NewsListPage />
                    // ANF-007: Radio
                    <Route path=path!("/radio") view=RadioPage />
                    <Route path=path!("/radio/verwaltung") view=RadioAdminPage />
                    // ANF-006: Eventliste und Eventverwaltung
                    <Route path=path!("/eventliste") view=EventListePage />
                    <Route path=path!("/eventliste/:id") view=EventListeEventPage ssr=SsrMode::Async />
                    <Route path=path!("/events") view=EventListPage />
                    <Route path=path!("/events/neu") view=EventNewPage />
                    <Route path=path!("/events/:id") view=EventDetailPage ssr=SsrMode::Async />
                    <Route path=path!("/events/:id/bearbeiten") view=EventEditPage ssr=SsrMode::Async />
                </Routes>
            </main>
            <Footer />
            <PlayerBar />
            // Platz für die Navigationsleiste unten (Handy)
            <div class="h-24 md:hidden" aria-hidden="true"></div>
            <BottomNav />
        </Router>
    }
}
