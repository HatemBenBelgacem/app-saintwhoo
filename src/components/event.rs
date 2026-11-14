//! UI-Bausteine für Events (ANF-006).

use chrono::NaiveDateTime;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::components::article::{Field, server_error_text};
use crate::components::magazine::CoverImage;
use crate::datetime::{month_short, weekday_short};
use crate::events::api::DeleteEvent;
use crate::events::model::{DeleteOutcome, Event, EventImage, FieldErrors, TicketStatus, msg};

/// Zähler, der nach Änderungen hochgezählt wird, damit Listen neu laden.
#[derive(Clone, Copy)]
pub struct EventsVersion(pub RwSignal<u32>);

/// Die Formularfelder eines Events. Ohne `initial` leer, mit `initial`
/// vorausgefüllt (Bearbeiten).
#[component]
pub fn EventFields(#[prop(optional)] initial: Option<Event>, errors: Memo<FieldErrors>) -> impl IntoView {
    let (image_url, image_credit) = match initial.as_ref().and_then(|e| e.image.clone()) {
        Some(i) => (i.url, i.credit),
        None => (String::new(), String::new()),
    };
    let (title, description, date, time, venue, city, lineup, ticket_url, ticket_status) = match initial {
        Some(e) => (
            e.title,
            e.description,
            e.starts_at.format("%Y-%m-%d").to_string(),
            e.starts_at.format("%H:%M").to_string(),
            e.venue,
            e.city,
            e.lineup,
            e.ticket_url.unwrap_or_default(),
            e.ticket_status,
        ),
        None => Default::default(),
    };
    let err = move |f: fn(&FieldErrors) -> Option<String>| Signal::derive(move || f(&errors.get()));

    view! {
        <Field label="Titel" error=err(|e| e.title.clone())>
            <input
                type="text"
                name="title"
                value=title
                class="input w-full"
                class:input-error=move || errors.get().title.is_some()
                placeholder="z. B. Nachtschicht #14"
                aria-required="true"
            />
        </Field>

        <div class="grid gap-4 sm:grid-cols-2">
            <Field label="Datum" error=err(|e| e.date.clone())>
                <input
                    type="date"
                    name="date"
                    value=date
                    class="input w-full"
                    class:input-error=move || errors.get().date.is_some()
                    aria-required="true"
                />
            </Field>
            <Field label="Beginn" hint="Ortszeit" error=err(|e| e.time.clone())>
                <input
                    type="time"
                    name="time"
                    value=time
                    class="input w-full"
                    class:input-error=move || errors.get().time.is_some()
                    aria-required="true"
                />
            </Field>
        </div>

        <div class="grid gap-4 sm:grid-cols-2">
            <Field label="Ort" hint="Club oder Location" error=err(|e| e.venue.clone())>
                <input
                    type="text"
                    name="venue"
                    value=venue
                    class="input w-full"
                    class:input-error=move || errors.get().venue.is_some()
                    placeholder="z. B. Kellerraum 9"
                    aria-required="true"
                />
            </Field>
            <Field label="Stadt" error=err(|e| e.city.clone())>
                <input
                    type="text"
                    name="city"
                    value=city
                    class="input w-full"
                    class:input-error=move || errors.get().city.is_some()
                    placeholder="z. B. Zürich"
                    aria-required="true"
                />
            </Field>
        </div>

        <Field label="Lineup" hint="optional, Namen mit Komma trennen" error=err(|e| e.lineup.clone())>
            <input
                type="text"
                name="lineup"
                value=lineup
                class="input w-full"
                class:input-error=move || errors.get().lineup.is_some()
                placeholder="Mara Voss, Jonas Keller"
            />
        </Field>

        <Field label="Beschreibung" error=err(|e| e.description.clone())>
            <textarea
                name="description"
                rows="6"
                class="textarea w-full"
                class:textarea-error=move || errors.get().description.is_some()
                placeholder="Was erwartet die Gäste? Leerzeilen trennen Absätze."
                aria-required="true"
            >
                {description}
            </textarea>
        </Field>

        <div class="grid gap-4 sm:grid-cols-3">
            <div class="sm:col-span-2">
                <Field label="Ticket-Adresse" hint="optional, https://…" error=err(|e| e.ticket_url.clone())>
                    <input
                        type="text"
                        inputmode="url"
                        name="ticket_url"
                        value=ticket_url
                        class="input w-full"
                        class:input-error=move || errors.get().ticket_url.is_some()
                        placeholder="https://…"
                    />
                </Field>
            </div>
            <Field label="Tickets" error=err(|e| e.ticket_status.clone())>
                <select
                    name="ticket_status"
                    class="select w-full"
                    class:select-error=move || errors.get().ticket_status.is_some()
                >
                    {TicketStatus::ALL
                        .into_iter()
                        .map(|s| view! { <option value=s.as_str() selected=s == ticket_status>{s.label()}</option> })
                        .collect_view()}
                </select>
            </Field>
        </div>

        <div class="grid gap-4 sm:grid-cols-2">
            <Field label="Bild-Adresse" hint="optional" error=err(|e| e.image_url.clone())>
                <input
                    type="text"
                    inputmode="url"
                    name="image_url"
                    value=image_url
                    class="input w-full"
                    class:input-error=move || errors.get().image_url.is_some()
                    placeholder="https://… oder /images/events/…"
                />
            </Field>
            <Field label="Bildnachweis" hint="Pflicht mit Bild" error=err(|e| e.image_credit.clone())>
                <input
                    type="text"
                    name="image_credit"
                    value=image_credit
                    class="input w-full"
                    class:input-error=move || errors.get().image_credit.is_some()
                    placeholder="Foto: Name / Quelle (Lizenz)"
                />
            </Field>
        </div>
    }
    .into_any()
}

/// Bild eines Events oder Platzhalter mit Anfangsbuchstabe.
#[component]
pub fn EventCover(
    id: i64,
    title: String,
    #[prop(default = None)] image: Option<EventImage>,
    #[prop(optional)] class: &'static str,
    #[prop(default = "text-8xl")] letter_size: &'static str,
    #[prop(default = true)] show_credit: bool,
    #[prop(optional)] eager: bool,
) -> impl IntoView {
    view! { <CoverImage id title image class letter_size show_credit eager label="Bild zum Event" /> }
}

/// Badge „Wenige Tickets“. „Verfügbar“ braucht keinen Hinweis, „Ausverkauft“
/// zeigt der Ticket-Knopf.
#[component]
pub fn TicketBadge(status: TicketStatus) -> impl IntoView {
    (status == TicketStatus::Few).then(|| view! { <span class="badge badge-outline">{status.label()}</span> })
}

/// Knopf „Tickets“: Link zur Ticket-Adresse, ausgegraut wenn ausverkauft,
/// nichts ohne Adresse.
#[component]
pub fn TicketButton(
    status: TicketStatus,
    url: Option<String>,
    title: String,
    #[prop(default = "Tickets")] label: &'static str,
    #[prop(optional)] wide: bool,
) -> impl IntoView {
    if status == TicketStatus::SoldOut {
        return Some(
            view! {
                <button type="button" class="btn min-h-11" class:flex-1=wide disabled=true>
                    "Ausverkauft"
                </button>
            }
            .into_any(),
        );
    }
    url.map(|u| {
        view! {
            <a
                href=u
                target="_blank"
                rel="noopener noreferrer"
                class="btn btn-primary min-h-11"
                class:flex-1=wide
                aria-label=format!("{label} für {title} (öffnet neues Fenster)")
            >
                {label}
            </a>
        }
        .into_any()
    })
}

/// Datumsblock wie im Design: `Fr / 20 / Nov`.
#[component]
pub fn DateBlock(starts_at: NaiveDateTime) -> impl IntoView {
    let d = starts_at.date();
    view! {
        <div class="flex h-19 w-16 shrink-0 flex-col items-center justify-center gap-0.5 rounded-xl border border-base-300 bg-base-100">
            <span class="text-xs font-semibold uppercase tracking-wider text-muted">{weekday_short(d)}</span>
            <span class="font-display text-2xl font-bold leading-none">{d.format("%d").to_string()}</span>
            <span class="text-xs font-semibold uppercase tracking-wider text-muted">{month_short(d)}</span>
        </div>
    }
    .into_any()
}

/// Erfolgsmeldung nach einer Weiterleitung (`?meldung=gespeichert|geloescht`).
#[component]
pub fn EventFlash() -> impl IntoView {
    let query = use_query_map();
    move || {
        let text = query.with(|q| match q.get("meldung").as_deref() {
            Some("gespeichert") => Some(msg::SAVED),
            Some("geloescht") => Some(msg::DELETED),
            _ => None,
        });
        text.map(|t| {
            view! {
                <div role="status" class="alert alert-success mb-4">
                    <span>{t}</span>
                </div>
            }
        })
    }
}

/// Löschen-Knopf mit Bestätigungsdialog (funktioniert auch ohne JavaScript).
#[component]
pub fn DeleteEventButton(
    id: i64,
    title: String,
    #[prop(optional)] key: &'static str,
    #[prop(optional)] small: bool,
) -> impl IntoView {
    let action = ServerAction::<DeleteEvent>::new();
    let version = use_context::<EventsVersion>();
    Effect::new(move |_| {
        if let Some(Ok(DeleteOutcome::Deleted)) = action.value().get()
            && let Some(v) = version
        {
            v.0.update(|n| *n += 1);
        }
    });

    let modal_id = format!("event-loeschen-{key}{id}");
    let heading_id = format!("{modal_id}-titel");
    let (for_open, for_cancel, for_backdrop) = (modal_id.clone(), modal_id.clone(), modal_id.clone());
    let labelledby = heading_id.clone();

    view! {
        <label for=for_open class="btn btn-error btn-outline" class:btn-sm=small>
            "Löschen"
        </label>
        <input type="checkbox" id=modal_id class="modal-toggle" />
        <div class="modal" role="dialog" aria-labelledby=labelledby>
            <div class="modal-box text-left">
                <h3 id=heading_id class="text-lg font-bold">"Event löschen"</h3>
                <p class="py-4">
                    "Event „" {title} "“ wirklich löschen? Das kann nicht rückgängig gemacht werden."
                </p>
                {move || match action.value().get() {
                    Some(Ok(DeleteOutcome::NotFound)) => Some(view! {
                        <div role="alert" class="alert alert-error">{msg::NOT_FOUND}</div>
                    }.into_any()),
                    Some(Err(e)) => Some(view! {
                        <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
                    }.into_any()),
                    _ => None,
                }}
                <div class="modal-action">
                    <label for=for_cancel class="btn">"Abbrechen"</label>
                    <ActionForm action>
                        <input type="hidden" name="id" value=id />
                        <button type="submit" class="btn btn-error" disabled=move || action.pending().get()>
                            <Show when=move || action.pending().get()>
                                <span class="loading loading-spinner loading-sm"></span>
                            </Show>
                            "Löschen"
                        </button>
                    </ActionForm>
                </div>
            </div>
            <label class="modal-backdrop" for=for_backdrop>"Schließen"</label>
        </div>
    }
    .into_any()
}
