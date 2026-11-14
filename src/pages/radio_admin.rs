//! ANF-007: Radio verwalten – `/radio/verwaltung`
//!
//! Sendungen für den Sendeplan und Mixe fürs Archiv anlegen und löschen.

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

use crate::components::article::{Field, server_error_text};
use crate::radio::api::{CreateMix, CreateShow, DeleteMix, DeleteShow, radio_admin};
use crate::datetime::format_when;
use crate::radio::model::{
    DeleteOutcome, Mix, MixErrors, MixOutcome, RadioOverview, Show, ShowErrors, ShowOutcome, format_duration, msg,
};

/// Zähler: nach Speichern/Löschen hochzählen → Liste und Formulare neu.
#[derive(Clone, Copy)]
struct RadioVersion(RwSignal<u32>);

impl RadioVersion {
    fn bump(&self) {
        self.0.update(|n| *n += 1);
    }
}

#[component]
pub fn RadioAdminPage() -> impl IntoView {
    let version = RadioVersion(RwSignal::new(0));
    provide_context(version);
    let data = Resource::new(move || version.0.get(), |_| radio_admin());

    view! {
        <Title text="Radio verwalten – Saintwhoo" />
        <div class="mb-6 flex flex-wrap items-center justify-between gap-4">
            <h1 class="font-display text-3xl font-bold">"Radio"</h1>
            <a href="/radio" class="btn btn-ghost">"Zum Radio"</a>
        </div>

        <Flash />

        <div class="grid gap-6 lg:grid-cols-2">
            <section class="card border border-base-300 bg-base-200">
                <div class="card-body">
                    <h2 class="card-title font-display text-2xl">"Sendeplan"</h2>
                    {move || {
                        version.0.track();
                        view! { <ShowForm /> }
                    }}
                </div>
            </section>
            <section class="card border border-base-300 bg-base-200">
                <div class="card-body">
                    <h2 class="card-title font-display text-2xl">"Archiv"</h2>
                    {move || {
                        version.0.track();
                        view! { <MixForm /> }
                    }}
                </div>
            </section>
        </div>

        <Suspense fallback=|| view! { <span class="loading loading-dots loading-lg"></span> }>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(d) => view! { <Lists data=d /> }.into_any(),
                    Err(e) => view! { <div role="alert" class="alert alert-error mt-6">{server_error_text(&e)}</div> }.into_any(),
                }
            })}
        </Suspense>
    }
    .into_any()
}

#[component]
fn Flash() -> impl IntoView {
    let query = use_query_map();
    move || {
        query
            .with(|q| match q.get("meldung").as_deref() {
                Some("sendung-gespeichert") => Some(msg::SHOW_SAVED),
                Some("sendung-geloescht") => Some(msg::SHOW_DELETED),
                Some("mix-gespeichert") => Some(msg::MIX_SAVED),
                Some("mix-geloescht") => Some(msg::MIX_DELETED),
                _ => None,
            })
            .map(|t| view! { <div role="status" class="alert alert-success mb-4"><span>{t}</span></div> })
    }
}

#[component]
fn ShowForm() -> impl IntoView {
    let action = ServerAction::<CreateShow>::new();
    let version = expect_context::<RadioVersion>();
    let errors = Memo::new(move |_| match action.value().get() {
        Some(Ok(ShowOutcome::Invalid(e))) => e,
        _ => ShowErrors::default(),
    });
    Effect::new(move |_| {
        if let Some(Ok(ShowOutcome::Created { .. })) = action.value().get() {
            version.bump();
        }
    });
    let err = move |f: fn(&ShowErrors) -> Option<String>| Signal::derive(move || f(&errors.get()));

    view! {
        {move || action.value().get().and_then(|r| r.err()).map(|e| view! {
            <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
        })}
        <ActionForm action attr:novalidate=true>
            <fieldset class="fieldset" disabled=move || action.pending().get()>
                <div class="grid gap-x-4 sm:grid-cols-2">
                    <Field label="Titel" error=err(|e| e.title.clone())>
                        <input type="text" name="title" class="input w-full" placeholder="Nachtschicht #15" aria-required="true" />
                    </Field>
                    <Field label="DJ" error=err(|e| e.host.clone())>
                        <input type="text" name="host" class="input w-full" placeholder="Mara Voss" aria-required="true" />
                    </Field>
                </div>
                <div class="grid gap-x-4 sm:grid-cols-3">
                    <Field label="Datum" error=err(|e| e.date.clone())>
                        <input type="date" name="date" class="input w-full" aria-required="true" />
                    </Field>
                    <Field label="Beginn" error=err(|e| e.start.clone())>
                        <input type="time" name="start" class="input w-full" aria-required="true" />
                    </Field>
                    <Field label="Ende" hint="früher = nächster Tag" error=err(|e| e.end.clone())>
                        <input type="time" name="end" class="input w-full" aria-required="true" />
                    </Field>
                </div>
                <div class="mt-3 flex justify-end">
                    <button type="submit" class="btn btn-primary">"Sendung speichern"</button>
                </div>
            </fieldset>
        </ActionForm>
    }
    .into_any()
}

#[component]
fn MixForm() -> impl IntoView {
    let action = ServerAction::<CreateMix>::new();
    let version = expect_context::<RadioVersion>();
    let errors = Memo::new(move |_| match action.value().get() {
        Some(Ok(MixOutcome::Invalid(e))) => e,
        _ => MixErrors::default(),
    });
    Effect::new(move |_| {
        if let Some(Ok(MixOutcome::Created { .. })) = action.value().get() {
            version.bump();
        }
    });
    let err = move |f: fn(&MixErrors) -> Option<String>| Signal::derive(move || f(&errors.get()));

    view! {
        {move || action.value().get().and_then(|r| r.err()).map(|e| view! {
            <div role="alert" class="alert alert-error">{server_error_text(&e)}</div>
        })}
        <ActionForm action attr:novalidate=true>
            <fieldset class="fieldset" disabled=move || action.pending().get()>
                <div class="grid gap-x-4 sm:grid-cols-2">
                    <Field label="Titel" error=err(|e| e.title.clone())>
                        <input type="text" name="title" class="input w-full" placeholder="Acid Morning #12" aria-required="true" />
                    </Field>
                    <Field label="DJ" error=err(|e| e.host.clone())>
                        <input type="text" name="host" class="input w-full" placeholder="Lea Stern" aria-required="true" />
                    </Field>
                </div>
                <div class="grid gap-x-4 sm:grid-cols-3">
                    <Field label="Dauer (Min.)" error=err(|e| e.duration_minutes.clone())>
                        <input type="number" name="duration_minutes" min="1" max="600" inputmode="numeric" class="input w-full" aria-required="true" />
                    </Field>
                    <div class="sm:col-span-2">
                        <Field label="Adresse der Aufnahme" hint="https://… oder /audio/…" error=err(|e| e.audio_url.clone())>
                            <input type="text" name="audio_url" inputmode="url" class="input w-full" placeholder="/audio/mix.mp3" aria-required="true" />
                        </Field>
                    </div>
                </div>
                <div class="mt-3 flex justify-end">
                    <button type="submit" class="btn btn-primary">"Mix speichern"</button>
                </div>
            </fieldset>
        </ActionForm>
    }
    .into_any()
}

#[component]
fn Lists(data: RadioOverview) -> impl IntoView {
    view! {
        <div class="mt-8 grid gap-6 lg:grid-cols-2">
            <section>
                <h2 class="mb-3 font-display text-xl font-bold">"Kommende Sendungen"</h2>
                {if data.shows.is_empty() {
                    view! { <p class="text-muted">"Keine kommenden Sendungen."</p> }.into_any()
                } else {
                    view! {
                        <ul class="divide-y divide-base-300 rounded-box border border-base-300 bg-base-200 px-4">
                            {data.shows.into_iter().map(|s| view! { <ShowItem show=s /> }).collect_view()}
                        </ul>
                    }.into_any()
                }}
            </section>
            <section>
                <h2 class="mb-3 font-display text-xl font-bold">"Mixe im Archiv"</h2>
                {if data.mixes.is_empty() {
                    view! { <p class="text-muted">{msg::ARCHIVE_EMPTY}</p> }.into_any()
                } else {
                    view! {
                        <ul class="divide-y divide-base-300 rounded-box border border-base-300 bg-base-200 px-4">
                            {data.mixes.into_iter().map(|m| view! { <MixItem mix=m /> }).collect_view()}
                        </ul>
                    }.into_any()
                }}
            </section>
        </div>
    }
    .into_any()
}

#[component]
fn ShowItem(show: Show) -> impl IntoView {
    let action = ServerAction::<DeleteShow>::new();
    let version = expect_context::<RadioVersion>();
    Effect::new(move |_| {
        if let Some(Ok(DeleteOutcome::Deleted)) = action.value().get() {
            version.bump();
        }
    });
    let text = format!("Sendung „{}“ wirklich löschen?", show.title);
    view! {
        <li class="flex items-center gap-3 py-3">
            <div class="min-w-0 flex-1">
                <p class="font-semibold">{show.title.clone()}</p>
                <p class="text-sm text-muted">{show.host.clone()} " · " {format_when(show.starts_at)} " bis " {show.ends_at.format("%H:%M").to_string()}</p>
            </div>
            <ConfirmDelete modal_id=format!("sendung-{}", show.id) text>
                <ActionForm action>
                    <input type="hidden" name="id" value=show.id />
                    <button type="submit" class="btn btn-error">"Löschen"</button>
                </ActionForm>
            </ConfirmDelete>
        </li>
    }
    .into_any()
}

#[component]
fn MixItem(mix: Mix) -> impl IntoView {
    let action = ServerAction::<DeleteMix>::new();
    let version = expect_context::<RadioVersion>();
    Effect::new(move |_| {
        if let Some(Ok(DeleteOutcome::Deleted)) = action.value().get() {
            version.bump();
        }
    });
    let text = format!("Mix „{}“ wirklich löschen?", mix.title);
    view! {
        <li class="flex items-center gap-3 py-3">
            <div class="min-w-0 flex-1">
                <p class="font-semibold">{mix.title.clone()}</p>
                <p class="text-sm text-muted break-all">{mix.host.clone()} " · " {format_duration(mix.duration_minutes)} " · " {mix.audio_url.clone()}</p>
            </div>
            <ConfirmDelete modal_id=format!("mix-{}", mix.id) text>
                <ActionForm action>
                    <input type="hidden" name="id" value=mix.id />
                    <button type="submit" class="btn btn-error">"Löschen"</button>
                </ActionForm>
            </ConfirmDelete>
        </li>
    }
    .into_any()
}

/// Löschen-Knopf mit Bestätigungsdialog (DaisyUI `modal-toggle`, ohne JavaScript nutzbar).
/// `children` ist das Formular mit dem eigentlichen Löschen-Knopf.
#[component]
fn ConfirmDelete(modal_id: String, text: String, children: Children) -> impl IntoView {
    let heading_id = format!("{modal_id}-titel");
    let (for_open, for_cancel, for_backdrop, labelledby) =
        (modal_id.clone(), modal_id.clone(), modal_id.clone(), heading_id.clone());
    view! {
        <label for=for_open class="btn btn-error btn-outline btn-sm">"Löschen"</label>
        <input type="checkbox" id=modal_id class="modal-toggle" />
        <div class="modal" role="dialog" aria-labelledby=labelledby>
            <div class="modal-box">
                <h3 id=heading_id class="text-lg font-bold">"Löschen"</h3>
                <p class="py-4">{text} " Das kann nicht rückgängig gemacht werden."</p>
                <div class="modal-action">
                    <label for=for_cancel class="btn">"Abbrechen"</label>
                    {children()}
                </div>
            </div>
            <label class="modal-backdrop" for=for_backdrop>"Schließen"</label>
        </div>
    }
    .into_any()
}
