//! ANF-006 Teil B: Öffentliche Eventseite – `/eventliste/<id>`
//!
//! Unbekannte IDs zeigen die 404-Seite (Route mit `SsrMode::Async`).

use leptos::prelude::*;
use leptos_meta::Title;

use crate::articles::model::paragraphs;
use crate::components::event::{EventCover, TicketBadge, TicketButton};
use crate::datetime::format_when;
use crate::events::model::Event;
use crate::pages::event_detail::{event_from_params, with_event};

#[component]
pub fn EventListeEventPage() -> impl IntoView {
    with_event(event_from_params(), |e| view! { <PublicEvent event=e /> })
}

#[component]
fn PublicEvent(event: Event) -> impl IntoView {
    let lineup = (!event.lineup.is_empty()).then(|| event.lineup.clone());
    view! {
        <Title text=format!("{} – Events – Saintwhoo", event.title) />
        <article class="mx-auto max-w-3xl py-6 md:py-10">
            <a href="/eventliste" class="text-sm font-medium text-muted hover:text-base-content">
                "← Zurück zur Eventliste"
            </a>

            <figure class="mt-6 overflow-hidden rounded-box">
                <EventCover
                    id=event.id
                    title=event.title.clone()
                    image=event.image.clone()
                    class="aspect-[16/9]"
                    letter_size="text-[10rem] md:text-[16rem]"
                    show_credit=false
                    eager=true
                />
            </figure>
            {event.image.clone().map(|i| view! {
                <p class="mt-2 text-right text-xs text-muted">{i.credit}</p>
            })}

            <header class="mt-6 flex flex-col gap-2">
                <div class="flex flex-wrap items-center gap-2 text-sm text-muted">
                    <span>{format_when(event.starts_at)}</span>
                    <TicketBadge status=event.ticket_status />
                </div>
                <h1 class="font-display text-4xl font-bold leading-tight break-words md:text-5xl">
                    {event.title.clone()}
                </h1>
                <p class="text-lg text-muted">{event.venue.clone()} " · " {event.city.clone()}</p>
                {lineup.map(|l| view! {
                    <p class="text-lg"><span class="sr-only">"Lineup: "</span>{l}</p>
                })}
            </header>

            <div class="mt-6 flex gap-2.5">
                <TicketButton status=event.ticket_status url=event.ticket_url.clone() title=event.title.clone() label="Tickets sichern" />
            </div>

            <div class="mt-8 space-y-5 text-lg leading-relaxed">
                {paragraphs(&event.description)
                    .into_iter()
                    .map(|p| view! { <p class="whitespace-pre-line break-words">{p}</p> })
                    .collect_view()}
            </div>
        </article>
    }
    .into_any()
}
