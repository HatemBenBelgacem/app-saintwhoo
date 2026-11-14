//! ANF-006: Eventliste (Verwaltung) – `/events?seite=2`

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

use crate::articles::model::format_timestamp;
use crate::components::article::server_error_text;
use crate::components::event::{DeleteEventButton, EventCover, EventFlash, EventsVersion};
use crate::datetime::format_when;
use crate::events::api::list_events;
use crate::events::model::{EventPage, EventSummary, msg};

fn list_href(page: u32) -> String {
    if page > 1 { format!("/events?seite={page}") } else { "/events".into() }
}

#[component]
pub fn EventListPage() -> impl IntoView {
    let query = use_query_map();
    let version = expect_context::<EventsVersion>().0;
    let page = move || query.with(|q| q.get("seite").and_then(|s| s.parse::<u32>().ok()).unwrap_or(1));
    let data = Resource::new(move || (page(), version.get()), |(page, _)| list_events(page));

    view! {
        <Title text="Events verwalten – Saintwhoo" />
        <div class="mb-6 flex flex-wrap items-center justify-between gap-4">
            <h1 class="font-display text-3xl font-bold">"Events"</h1>
            <div class="flex gap-2">
                <a href="/eventliste" class="btn btn-ghost">"Zur Eventliste"</a>
                <a href="/events/neu" class="btn btn-primary">"Neues Event"</a>
            </div>
        </div>

        <EventFlash />

        <Suspense fallback=|| view! { <span class="loading loading-dots loading-lg"></span> }>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(p) => view! { <EventTable data=p /> }.into_any(),
                    Err(e) => view! {
                        <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
                    }.into_any(),
                }
            })}
        </Suspense>
    }
    .into_any()
}

#[component]
fn EventTable(data: EventPage) -> impl IntoView {
    if data.items.is_empty() {
        return view! {
            <div class="card border border-base-300 bg-base-200">
                <div class="card-body items-center text-center">
                    <p>{msg::LIST_EMPTY}</p>
                    <a href="/events/neu" class="btn btn-primary">"Neues Event"</a>
                </div>
            </div>
        }
        .into_any();
    }

    let items = data.items.clone();
    let (page, total_pages, total) = (data.page, data.total_pages, data.total);
    view! {
        // Desktop: Tabelle
        <div class="hidden overflow-x-auto rounded-box border border-base-300 bg-base-200 md:block">
            <table class="table">
                <thead>
                    <tr>
                        <th>"Bild"</th>
                        <th>"Titel"</th>
                        <th>"Beginn"</th>
                        <th>"Zuletzt geändert"</th>
                        <th class="text-right">"Aktionen"</th>
                    </tr>
                </thead>
                <tbody>
                    {data.items.into_iter().map(|e| view! { <EventRow event=e /> }).collect_view()}
                </tbody>
            </table>
        </div>

        // Handy: Karten
        <div class="flex flex-col gap-3 md:hidden">
            {items.into_iter().map(|e| view! { <EventCardAdmin event=e /> }).collect_view()}
        </div>

        <div class="mt-4 flex flex-wrap items-center justify-between gap-2">
            <span class="text-sm text-muted">{total} " Events"</span>
            <Show when=move || { total_pages > 1 }>
                <nav class="join" aria-label="Seiten">
                    {(1..=total_pages)
                        .map(|n| view! {
                            <a
                                href=list_href(n)
                                class="join-item btn btn-sm"
                                class:btn-active=n == page
                                aria-current=(n == page).then_some("page")
                            >
                                {n}
                            </a>
                        })
                        .collect_view()}
                </nav>
            </Show>
        </div>
    }
    .into_any()
}

#[component]
fn EventRow(event: EventSummary) -> impl IntoView {
    let id = event.id;
    view! {
        <tr class="hover:bg-base-300/40">
            <td class="w-20">
                <EventCover id title=event.title.clone() image=event.image.clone() class="aspect-square w-14 rounded-lg" letter_size="text-4xl" show_credit=false />
            </td>
            <td>
                <a href=format!("/events/{id}") class="link link-hover font-medium">{event.title.clone()}</a>
            </td>
            <td class="whitespace-nowrap text-sm">{format_when(event.starts_at)} " · " {event.city}</td>
            <td class="text-sm text-muted">{format_timestamp(event.updated_at)}</td>
            <td>
                <div class="flex justify-end gap-2">
                    <a href=format!("/events/{id}/bearbeiten") class="btn btn-sm">"Bearbeiten"</a>
                    <DeleteEventButton id title=event.title key="t" small=true />
                </div>
            </td>
        </tr>
    }
    .into_any()
}

#[component]
fn EventCardAdmin(event: EventSummary) -> impl IntoView {
    let id = event.id;
    view! {
        <div class="card border border-base-300 bg-base-200">
            <div class="card-body gap-2 p-4">
                <div class="flex gap-3">
                    <EventCover id title=event.title.clone() image=event.image.clone() class="aspect-square w-16 shrink-0 rounded-lg" letter_size="text-4xl" show_credit=false />
                    <div class="min-w-0">
                        <a href=format!("/events/{id}") class="card-title link link-hover break-words">{event.title.clone()}</a>
                        <p class="text-sm">{format_when(event.starts_at)} " · " {event.city}</p>
                    </div>
                </div>
                <p class="text-xs text-muted">"Zuletzt geändert: " {format_timestamp(event.updated_at)}</p>
                <div class="card-actions justify-end">
                    <a href=format!("/events/{id}/bearbeiten") class="btn btn-sm">"Bearbeiten"</a>
                    <DeleteEventButton id title=event.title key="k" small=true />
                </div>
            </div>
        </div>
    }
    .into_any()
}
