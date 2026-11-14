//! ANF-006: Event bearbeiten – `/events/<id>/bearbeiten`

use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::article::server_error_text;
use crate::components::event::{EventFields, EventsVersion};
use crate::events::api::UpdateEvent;
use crate::events::model::{Event, FieldErrors, UpdateOutcome, msg};
use crate::pages::event_detail::{event_from_params, with_event};

#[component]
pub fn EventEditPage() -> impl IntoView {
    with_event(event_from_params(), |e| view! { <EventEditForm event=e /> })
}

#[component]
fn EventEditForm(event: Event) -> impl IntoView {
    let action = ServerAction::<UpdateEvent>::new();
    let version = use_context::<EventsVersion>();
    let id = event.id;

    let errors = Memo::new(move |_| match action.value().get() {
        Some(Ok(UpdateOutcome::Invalid(e))) => e,
        _ => FieldErrors::default(),
    });

    Effect::new(move |_| {
        if let Some(Ok(UpdateOutcome::Updated { .. })) = action.value().get()
            && let Some(v) = version
        {
            v.0.update(|n| *n += 1);
        }
    });

    view! {
        <Title text=format!("{} bearbeiten – Saintwhoo", event.title) />
        <div class="mx-auto max-w-2xl">
            <h1 class="mb-6 font-display text-3xl font-bold">"Event bearbeiten"</h1>

            {move || match action.value().get() {
                Some(Ok(UpdateOutcome::Invalid(_))) => Some(view! {
                    <div role="alert" class="alert alert-warning mb-4">
                        <span>"Bitte die markierten Felder korrigieren."</span>
                    </div>
                }.into_any()),
                Some(Ok(UpdateOutcome::NotFound)) => Some(view! {
                    <div role="alert" class="alert alert-error mb-4">
                        <span>{msg::NOT_FOUND}</span>
                        <a href="/events" class="btn btn-sm">"Zur Liste"</a>
                    </div>
                }.into_any()),
                Some(Err(e)) => Some(view! {
                    <div role="alert" class="alert alert-error mb-4">
                        <span>{server_error_text(&e)}</span>
                    </div>
                }.into_any()),
                _ => None,
            }}

            <div class="card border border-base-300 bg-base-200">
                <div class="card-body">
                    <ActionForm action attr:novalidate=true>
                        <input type="hidden" name="id" value=id />
                        <fieldset class="fieldset" disabled=move || action.pending().get()>
                            <EventFields initial=event errors />
                            <div class="mt-4 flex justify-end gap-2">
                                <a href=format!("/events/{id}") class="btn btn-ghost">"Abbrechen"</a>
                                <button type="submit" class="btn btn-primary">
                                    <Show when=move || action.pending().get()>
                                        <span class="loading loading-spinner loading-sm"></span>
                                    </Show>
                                    "Speichern"
                                </button>
                            </div>
                        </fieldset>
                    </ActionForm>
                </div>
            </div>
        </div>
    }
    .into_any()
}
