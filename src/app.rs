use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    SsrMode, StaticSegment,
    components::{Route, Router, Routes},
    path,
};

use crate::components::{
    article::ArticlesVersion, footer::Footer, navbar::Navbar, product::ProductsVersion,
};
use crate::pages::{
    article_detail::ArticleDetailPage, article_edit::ArticleEditPage,
    article_list::ArticleListPage, article_new::ArticleNewPage, beitrag::BeitragPage,
    home::HomePage, not_found::NotFound, product_detail::ProductDetailPage,
    product_edit::ProductEditPage, product_list::ProductListPage, product_new::ProductNewPage,
    shop::ShopPage, shop_product::ShopProductPage, status::StatusPage,
};

/// HTML-Grundgerüst, das der Server ausliefert.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="de" data-theme="light" style="color-scheme: light">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="min-h-screen bg-base-100 antialiased">
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

    view! {
        <Stylesheet id="leptos" href="/pkg/app-saintwhoo.css" />
        <Title text="Saintwhoo – Magazin" />

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
                    <Route path=path!("/produkte") view=ProductListPage />
                    <Route path=path!("/produkte/neu") view=ProductNewPage />
                    <Route path=path!("/produkte/:id") view=ProductDetailPage ssr=SsrMode::Async />
                    <Route path=path!("/produkte/:id/bearbeiten") view=ProductEditPage ssr=SsrMode::Async />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
