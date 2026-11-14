//! Startseite – `/`
//!
//! Nach dem Designvorschlag „Techno App“ (ersetzt das Magazin-Layout aus
//! ANF-003): Livestream, neueste News und Merch. Jeder Block lädt für sich,
//! ein Fehler in einem Block lässt die anderen stehen.

use leptos::prelude::*;

use crate::articles::api::latest_news;
use crate::articles::model::msg as article_msg;
use crate::components::article::server_error_text;
use crate::components::cart::AddToCartButton;
use crate::components::event::{DateBlock, TicketBadge};
use crate::events::api::upcoming_events;
use crate::events::model::{EventCard, EventRange, msg as event_msg};
use crate::components::news::NewsRow;
use crate::components::product::ProductCover;
use crate::components::pwa::IosInstallHint;
use crate::pages::radio::LiveCard;
use crate::products::api::shop_products;
use crate::products::model::{ShopItem, ShopSort, format_chf, msg as product_msg};
use crate::radio::api::radio_live;

/// So viele News und Produkte zeigt die Startseite.
const HOME_NEWS: i64 = 4;
const HOME_MERCH: usize = 4;
const HOME_EVENTS: usize = 3;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="py-4 md:py-8">
            <h1 class="sr-only">"Saintwhoo – Techno News, Events, Radio und Merch"</h1>
            <IosInstallHint />
            // Handy: Live, News, Events untereinander (Reihenfolge wie im Entwurf).
            // Desktop: links Live + Events, rechts News über beide Zeilen.
            <div class="grid gap-8 lg:grid-cols-[minmax(0,2fr)_minmax(0,3fr)] lg:grid-rows-[auto_1fr] lg:gap-x-10">
                <div class="lg:col-start-1 lg:row-start-1"><Live /></div>
                <div class="lg:col-start-2 lg:row-span-2 lg:row-start-1"><News /></div>
                <div class="lg:col-start-1 lg:row-start-2"><NextEvents /></div>
            </div>
            <Merch />
        </div>
    }
    .into_any()
}

fn spinner() -> impl IntoView {
    view! { <div class="flex justify-center py-10"><span class="loading loading-dots loading-md"></span></div> }
}

/// Abschnittskopf mit Link „Alle“.
#[component]
fn SectionHead(title: &'static str, href: &'static str, more: &'static str) -> impl IntoView {
    view! {
        <div class="mb-3.5 flex items-center justify-between">
            <h2 class="font-display text-[1.4rem] font-bold">{title}</h2>
            <a href=href class="inline-flex min-h-11 items-center text-[15px] font-medium text-primary" aria-label=more>
                "Alle"
            </a>
        </div>
    }
}

#[component]
fn Live() -> impl IntoView {
    let data = Resource::new(|| (), |_| radio_live());
    view! {
        <Suspense fallback=spinner>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(info) => {
                        let on_air = info.on_air().cloned();
                        view! { <LiveCard stream_url=info.stream_url on_air compact=true /> }.into_any()
                    }
                    Err(e) => view! { <div role="alert" class="alert alert-error">{server_error_text(&e)}</div> }.into_any(),
                }
            })}
        </Suspense>
        <a href="/radio" class="mt-3 inline-flex min-h-11 items-center text-[15px] font-medium text-primary">
            "Sendeplan und Archiv"
        </a>
    }
    .into_any()
}

/// Die nächsten Events (kurz, mit Datumsblock).
#[component]
fn NextEvents() -> impl IntoView {
    let data = Resource::new(|| (), |_| upcoming_events(EventRange::All));
    view! {
        <section>
            <SectionHead title="Nächste Events" href="/eventliste" more="Alle Events" />
            <Suspense fallback=spinner>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(l) if l.items.is_empty() => view! {
                            <p class="rounded-2xl border border-base-300 bg-base-200 p-4 text-muted">{event_msg::PUBLIC_EMPTY}</p>
                        }.into_any(),
                        Ok(l) => view! {
                            <ul class="flex flex-col gap-3">
                                {l.items.into_iter().take(HOME_EVENTS).map(|e| view! { <EventItem event=e /> }).collect_view()}
                            </ul>
                        }.into_any(),
                        Err(e) => view! { <div role="alert" class="alert alert-error">{server_error_text(&e)}</div> }.into_any(),
                    }
                })}
            </Suspense>
        </section>
    }
    .into_any()
}

#[component]
fn EventItem(event: EventCard) -> impl IntoView {
    let link = format!("/eventliste/{}", event.id);
    let time = event.starts_at.format("%H:%M").to_string();
    view! {
        <li class="flex gap-3.5 rounded-2xl border border-base-300 bg-base-200 p-3.5">
            <DateBlock starts_at=event.starts_at />
            <div class="flex min-w-0 flex-col justify-center gap-1">
                <h3 class="font-semibold leading-snug break-words">
                    <a href=link class="hover:underline">{event.title}</a>
                </h3>
                <p class="text-sm text-muted">{event.venue} " · " {event.city} " · " {time} " Uhr"</p>
                <div><TicketBadge status=event.ticket_status /></div>
            </div>
        </li>
    }
    .into_any()
}

#[component]
fn News() -> impl IntoView {
    let data = Resource::new(|| (), |_| latest_news(HOME_NEWS));
    view! {
        <SectionHead title="Neueste News" href="/news" more="Alle News" />
        <Suspense fallback=spinner>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(list) if list.is_empty() => view! {
                        <p class="rounded-2xl border border-base-300 bg-base-200 p-4 text-muted">{article_msg::NEWS_EMPTY}</p>
                    }.into_any(),
                    Ok(list) => view! {
                        <ul class="flex flex-col gap-3">
                            {list.into_iter().map(|a| view! { <li><NewsRow article=a /></li> }).collect_view()}
                        </ul>
                    }.into_any(),
                    Err(e) => view! { <div role="alert" class="alert alert-error">{server_error_text(&e)}</div> }.into_any(),
                }
            })}
        </Suspense>
    }
    .into_any()
}

#[component]
fn Merch() -> impl IntoView {
    let data = Resource::new(|| (), |_| shop_products(1, ShopSort::Newest, None));
    view! {
        <section class="mt-10">
            <SectionHead title="Merch" href="/shop" more="Alle Produkte" />
            <Suspense fallback=spinner>
                {move || Suspend::new(async move {
                    match data.await {
                        Ok(p) if p.items.is_empty() => view! {
                            <p class="rounded-2xl border border-base-300 bg-base-200 p-4 text-muted">{product_msg::SHOP_EMPTY}</p>
                        }.into_any(),
                        Ok(p) => view! {
                            <ul class="grid grid-cols-2 gap-3 md:grid-cols-4">
                                {p.items.into_iter().take(HOME_MERCH).map(|i| view! { <MerchCard product=i /> }).collect_view()}
                            </ul>
                        }.into_any(),
                        Err(e) => view! { <div role="alert" class="alert alert-error">{server_error_text(&e)}</div> }.into_any(),
                    }
                })}
            </Suspense>
        </section>
    }
    .into_any()
}

#[component]
fn MerchCard(product: ShopItem) -> impl IntoView {
    let link = format!("/shop/{}", product.id);
    view! {
        <li class="flex flex-col gap-2.5 rounded-2xl border border-base-300 bg-base-200 p-2.5">
            <a href=link.clone() class="block overflow-hidden rounded-xl" tabindex="-1" aria-hidden="true">
                <ProductCover id=product.id name=product.name.clone() image=product.image.clone() class="aspect-square" letter_size="text-7xl" show_credit=false />
            </a>
            <h3 class="text-[15px] font-semibold leading-snug break-words">
                <a href=link class="hover:underline">{product.name.clone()}</a>
            </h3>
            <div class="mt-auto flex min-h-11 items-center justify-between gap-2">
                <span class="text-[15px] text-muted whitespace-nowrap tabular-nums">{format_chf(product.price_cents)}</span>
                <AddToCartButton id=product.id name=product.name price_cents=product.price_cents stock=product.stock />
            </div>
        </li>
    }
    .into_any()
}
