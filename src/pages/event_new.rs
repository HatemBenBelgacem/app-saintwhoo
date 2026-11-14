//! ANF-006: Neues Event – `/events/neu`

use leptos::prelude::*;
use leptos_meta::Title;

use crate::components::article::server_error_text;
use crate::components::event::{EventFields, EventsVersion};
use crate::events::api::CreateEvent;
use crate::events::model::{CreateOutcome, FieldErrors};

#[component]
pub fn EventNewPage() -> impl IntoView {
    let action = ServerAction::<CreateEvent>::new();
    let version = use_context::<EventsVersion>();

    let errors = Memo::new(move |_| match action.value().get() {
        Some(Ok(CreateOutcome::Invalid(e))) => e,
        _ => FieldErrors::default(),
    });

    Effect::new(move |_| {
        if let Some(Ok(CreateOutcome::Created { .. })) = action.value().get()
            && let Some(v) = version
        {
            v.0.update(|n| *n += 1);
        }
    });

    view! {
        <Title text="Neues Event – Saintwhoo" />
        <div class="mx-auto max-w-2xl">
            <div class="mb-6 flex flex-wrap items-center justify-between gap-4">
                <h1 class="font-display text-3xl font-bold">"Neues Event"</h1>
                <a href="/events" class="btn btn-ghost btn-sm">"← Zur Liste"</a>
            </div>

            {move || match action.value().get() {
                Some(Ok(CreateOutcome::Invalid(_))) => Some(view! {
                    <div role="alert" class="alert alert-warning mb-4">
                        <span>"Bitte die markierten Felder korrigieren."</span>
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
                        <fieldset class="fieldset" disabled=move || action.pending().get()>
                            <EventFields errors />
                            <div class="mt-4 flex justify-end gap-2">
                                <a href="/events" class="btn btn-ghost">"Abbrechen"</a>
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
