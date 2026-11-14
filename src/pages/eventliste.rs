//! ANF-006 Teil B: Öffentliche Eventliste – `/eventliste?zeitraum=wochenende`
//!
//! Layout nach dem Designvorschlag „Techno App“: Filter-Chips, das nächste
//! Event groß, danach „Demnächst“ mit Datumsblock.

use chrono::NaiveDate;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

use crate::components::article::server_error_text;
use crate::components::event::{DateBlock, EventCover, TicketBadge, TicketButton};
use crate::components::ui::{Chip, PageHeader};
use crate::datetime::format_when;
use crate::events::api::upcoming_events;
use crate::events::model::{EventCard, EventListing, EventRange, msg};

fn range_href(r: EventRange) -> String {
    match r {
        EventRange::All => "/eventliste".into(),
        r => format!("/eventliste?zeitraum={}", r.as_str()),
    }
}

#[component]
pub fn EventListePage() -> impl IntoView {
    let query = use_query_map();
    let range = move || query.with(|q| q.get("zeitraum").map(|s| EventRange::parse(&s)).unwrap_or_default());
    let data = Resource::new(range, upcoming_events);

    view! {
        <Title text="Events – Saintwhoo" />
        <div class="py-6 md:py-10">
            <PageHeader title="Events" subtitle="Termine von DJs und Clubs" />

            <nav aria-label="Zeitraum" class="-mx-4 mt-5 flex gap-2 overflow-x-auto px-4 pb-1">
                {EventRange::ALL
                    .into_iter()
                    .map(|r| view! { <Chip href=range_href(r) active=Signal::derive(move || range() == r) label=r.label() /> })
                    .collect_view()}
            </nav>

            <Suspense fallback=|| view! {
                <div class="flex justify-center py-24"><span class="loading loading-dots loading-lg"></span></div>
            }>
                {move || {
                    let r = range();
                    Suspend::new(async move {
                        match data.await {
                            Ok(l) if l.items.is_empty() => {
                                let text = if r == EventRange::All { msg::PUBLIC_EMPTY } else { msg::PUBLIC_EMPTY_RANGE };
                                view! { <p class="py-24 text-center font-display text-2xl">{text}</p> }.into_any()
                            }
                            Ok(l) => view! { <Listing listing=l /> }.into_any(),
                            Err(e) => view! {
                                <div role="alert" class="alert alert-error mt-6">{server_error_text(&e)}</div>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
    .into_any()
}

#[component]
fn Listing(listing: EventListing) -> impl IntoView {
    let today = listing.today;
    let mut items = listing.items.into_iter();
    let first = items.next().expect("nicht leer");
    let rest: Vec<_> = items.collect();
    let has_rest = !rest.is_empty();

    view! {
        <div class="mt-6">
            <Highlight event=first today />
        </div>
        <Show when=move || has_rest>
            <h2 class="mt-10 mb-4 font-display text-2xl font-bold">"Demnächst"</h2>
            <ul class="grid gap-3 md:grid-cols-2 xl:grid-cols-3">
                {rest.clone().into_iter().map(|e| view! { <Row event=e /> }).collect_view()}
            </ul>
        </Show>
    }
    .into_any()
}

/// Das nächste Event, groß mit Flyer.
#[component]
fn Highlight(event: EventCard, today: NaiveDate) -> impl IntoView {
    let link = format!("/eventliste/{}", event.id);
    let is_today = event.starts_at.date() == today;
    let lineup = (!event.lineup.is_empty()).then(|| event.lineup.clone());
    view! {
        <article class="grid overflow-hidden rounded-box border border-base-300 bg-base-200 md:grid-cols-2">
            <a href=link.clone() tabindex="-1" aria-hidden="true" class="block">
                <EventCover
                    id=event.id
                    title=event.title.clone()
                    image=event.image.clone()
                    class="aspect-[16/9] h-full md:aspect-auto md:min-h-72"
                    letter_size="text-[10rem]"
                    eager=true
                />
            </a>
            <div class="flex flex-col gap-2.5 p-5 md:p-8">
                <div class="flex flex-wrap items-center gap-2">
                    <Show when=move || is_today>
                        <span class="badge badge-primary font-bold tracking-wider">"HEUTE"</span>
                    </Show>
                    <TicketBadge status=event.ticket_status />
                </div>
                <p class="text-sm text-muted">{format_when(event.starts_at)}</p>
                <h2 class="font-display text-3xl font-bold leading-tight break-words md:text-4xl">
                    <a href=link class="hover:underline">{event.title.clone()}</a>
                </h2>
                <p class="text-muted">{event.venue.clone()} " · " {event.city.clone()}</p>
                {lineup.map(|l| view! { <p>{l}</p> })}
                <p class="text-muted line-clamp-3">{event.teaser.clone()}</p>
                <div class="mt-2 flex gap-2.5">
                    <TicketButton
                        status=event.ticket_status
                        url=event.ticket_url.clone()
                        title=event.title.clone()
                        label="Tickets sichern"
                        wide=true
                    />
                </div>
            </div>
        </article>
    }
    .into_any()
}

/// Kompakter Eintrag mit Datumsblock.
#[component]
fn Row(event: EventCard) -> impl IntoView {
    let link = format!("/eventliste/{}", event.id);
    let time = event.starts_at.format("%H:%M").to_string();
    let lineup = (!event.lineup.is_empty()).then(|| event.lineup.clone());
    view! {
        <li class="flex gap-3.5 rounded-2xl border border-base-300 bg-base-200 p-3.5">
            <DateBlock starts_at=event.starts_at />
            <div class="flex min-w-0 flex-1 flex-col gap-1">
                <h3 class="font-semibold leading-snug break-words">
                    <a href=link class="hover:underline">{event.title.clone()}</a>
                </h3>
                <p class="text-sm text-muted">{event.venue.clone()} " · " {event.city.clone()} " · " {time} " Uhr"</p>
                {lineup.map(|l| view! { <p class="text-sm">{l}</p> })}
                <div class="mt-2 flex flex-wrap items-center gap-2">
                    <TicketButton status=event.ticket_status url=event.ticket_url.clone() title=event.title.clone() />
                    <TicketBadge status=event.ticket_status />
                </div>
            </div>
        </li>
    }
    .into_any()
}
