//! ANF-006: Detailseite (Verwaltung) – `/events/<id>`

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::articles::model::{format_timestamp, paragraphs};
use crate::components::article::server_error_text;
use crate::components::event::{DeleteEventButton, EventCover, EventFlash, TicketBadge};
use crate::datetime::format_when;
use crate::events::api::get_event;
use crate::events::model::Event;
use crate::pages::not_found::NotFound;

type EventResource = Resource<Result<Option<Event>, ServerFnError>>;

/// Liest `:id` aus der Adresse und lädt das Event. Eine ungültige ID
/// verhält sich wie eine unbekannte (404).
pub fn event_from_params() -> EventResource {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").and_then(|s| s.parse::<i64>().ok()));
    Resource::new(id, |id| async move {
        match id {
            Some(id) => get_event(id).await,
            None => Ok(None),
        }
    })
}

/// Zeigt Laden / Fehler / 404 und ruft sonst `view` mit dem Event auf.
pub fn with_event<V: IntoView + 'static>(event: EventResource, view: fn(Event) -> V) -> impl IntoView {
    view! {
        <Suspense fallback=|| view! { <span class="loading loading-dots loading-lg"></span> }>
            {move || Suspend::new(async move {
                match event.await {
                    Ok(Some(e)) => view(e).into_any(),
                    Ok(None) => view! { <NotFound /> }.into_any(),
                    Err(e) => view! {
                        <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
                    }.into_any(),
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn EventDetailPage() -> impl IntoView {
    with_event(event_from_params(), |e| view! { <EventDetail event=e /> })
}

#[component]
fn EventDetail(event: Event) -> impl IntoView {
    let id = event.id;
    let lineup = (!event.lineup.is_empty()).then(|| event.lineup.clone());
    view! {
        <Title text=format!("{} – Events – Saintwhoo", event.title) />
        <div class="mx-auto max-w-3xl">
            <a href="/events" class="btn btn-ghost btn-sm mb-4">"← Zurück zur Liste"</a>

            <EventFlash />

            <article class="card overflow-hidden border border-base-300 bg-base-200">
                <figure class="flex-col">
                    <EventCover id title=event.title.clone() image=event.image.clone() class="aspect-[16/9]" show_credit=false />
                    {event.image.clone().map(|i| view! {
                        <figcaption class="w-full px-8 pt-2 text-right text-xs text-muted">{i.credit}</figcaption>
                    })}
                </figure>
                <div class="card-body">
                    <div class="flex flex-wrap items-center gap-2 text-sm text-muted">
                        <span>{format_when(event.starts_at)}</span>
                        <TicketBadge status=event.ticket_status />
                    </div>
                    <h1 class="font-display text-3xl font-bold break-words">{event.title.clone()}</h1>
                    <p class="text-muted">{event.venue.clone()} " · " {event.city.clone()}</p>
                    {lineup.map(|l| view! { <p>{l}</p> })}

                    <div class="mt-2 space-y-4 leading-relaxed">
                        {paragraphs(&event.description)
                            .into_iter()
                            .map(|p| view! { <p class="whitespace-pre-line break-words">{p}</p> })
                            .collect_view()}
                    </div>

                    <dl class="mt-4 grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-sm text-muted">
                        <dt>"Tickets"</dt>
                        <dd class="break-all">
                            {event.ticket_status.label()}
                            {event.ticket_url.clone().map(|u| { let text = u.clone(); view! { " · " <a href=u class="link">{text}</a> } })}
                        </dd>
                        <dt>"Erstellt am"</dt>
                        <dd>{format_timestamp(event.created_at)}</dd>
                        <dt>"Zuletzt geändert"</dt>
                        <dd>{format_timestamp(event.updated_at)}</dd>
                    </dl>

                    <div class="card-actions mt-4 justify-end">
                        <a href=format!("/eventliste/{id}") class="btn btn-ghost">"In der Eventliste ansehen"</a>
                        <a href=format!("/events/{id}/bearbeiten") class="btn btn-primary">"Bearbeiten"</a>
                        <DeleteEventButton id title=event.title />
                    </div>
                </div>
            </article>
        </div>
    }
    .into_any()
}
