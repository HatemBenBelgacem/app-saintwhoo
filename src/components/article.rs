//! Gemeinsame UI-Bausteine für Artikel (ANF-001, ANF-002).

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::articles::api::DeleteArticle;
use crate::articles::model::{Article, ArticleCategory, ArticleStatus, DeleteOutcome, FieldErrors, msg};

/// Zähler, der nach Änderungen hochgezählt wird, damit Listen neu laden.
#[derive(Clone, Copy)]
pub struct ArticlesVersion(pub RwSignal<u32>);

/// Text für technische Fehler aus Server-Funktionen.
pub fn server_error_text(e: &ServerFnError) -> String {
    match e {
        ServerFnError::ServerError(msg) => msg.clone(),
        _ => "Der Server ist nicht erreichbar. Bitte später erneut versuchen.".into(),
    }
}

/// Die vier Formularfelder eines Artikels. Ohne `initial` leer (Erstellen),
/// mit `initial` vorausgefüllt (Bearbeiten).
#[component]
pub fn ArticleFields(#[prop(optional)] initial: Option<Article>, errors: Memo<FieldErrors>) -> impl IntoView {
    let category = initial.as_ref().map(|a| a.category).unwrap_or_default();
    let (title, body, date, status, image_url, image_credit) = match initial {
        Some(a) => (
            a.title,
            a.body,
            a.published_on.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default(),
            a.status,
            a.image.as_ref().map(|i| i.url.clone()).unwrap_or_default(),
            a.image.map(|i| i.credit).unwrap_or_default(),
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
                placeholder="Titel des Artikels"
                aria-required="true"
            />
        </Field>

        // ANF-008: Rubrik für den News-Filter
        <Field label="Rubrik" error=err(|e| e.category.clone())>
            <select
                name="category"
                class="select w-full"
                class:select-error=move || errors.get().category.is_some()
            >
                {ArticleCategory::ALL
                    .into_iter()
                    .map(|c| view! { <option value=c.as_str() selected=c == category>{c.label()}</option> })
                    .collect_view()}
            </select>
        </Field>

        <Field label="Text" error=err(|e| e.body.clone())>
            <textarea
                name="body"
                rows="10"
                class="textarea w-full"
                class:textarea-error=move || errors.get().body.is_some()
                placeholder="Text des Artikels"
                aria-required="true"
            >
                {body}
            </textarea>
        </Field>

        <div class="grid gap-4 sm:grid-cols-2">
            <Field label="Status" error=err(|e| e.status.clone())>
                <select
                    name="status"
                    class="select w-full"
                    class:select-error=move || errors.get().status.is_some()
                >
                    {ArticleStatus::ALL
                        .into_iter()
                        .map(|s| view! { <option value=s.as_str() selected=s == status>{s.label()}</option> })
                        .collect_view()}
                </select>
            </Field>

            <Field
                label="Veröffentlichungsdatum"
                hint="Pflicht bei „Veröffentlicht“"
                error=err(|e| e.published_on.clone())
            >
                <input
                    type="date"
                    name="published_on"
                    value=date
                    class="input w-full"
                    class:input-error=move || errors.get().published_on.is_some()
                />
            </Field>
        </div>

        // ANF-004
        <div class="grid gap-4 sm:grid-cols-2">
            <Field label="Bild-Adresse" hint="optional" error=err(|e| e.image_url.clone())>
                <input
                    type="text"
                    inputmode="url"
                    name="image_url"
                    value=image_url
                    class="input w-full"
                    class:input-error=move || errors.get().image_url.is_some()
                    placeholder="https://… oder /images/…"
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
}

/// Formularfeld mit Beschriftung, optionalem Hinweis und Fehlermeldung.
/// Wird auch für Produkte (ANF-005) verwendet.
#[component]
pub fn Field(
    label: &'static str,
    #[prop(optional)] hint: Option<&'static str>,
    #[prop(into)] error: Signal<Option<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <label class="fieldset w-full">
            <span class="fieldset-legend">
                {label}
                {hint.map(|h| view! { <span class="font-normal text-base-content/60">" (" {h} ")"</span> })}
            </span>
            {children()}
            <Show when=move || error.get().is_some()>
                <span class="text-error text-sm" role="alert">{move || error.get()}</span>
            </Show>
        </label>
    }
}

/// Status als farbiges Badge: Entwurf grau, Veröffentlicht grün.
#[component]
pub fn StatusBadge(status: ArticleStatus) -> impl IntoView {
    let class = match status {
        ArticleStatus::Draft => "badge badge-ghost",
        ArticleStatus::Published => "badge badge-success",
    };
    view! { <span class=class>{status.label()}</span> }
}

/// Erfolgsmeldung nach einer Weiterleitung (`?meldung=gespeichert|geloescht`).
#[component]
pub fn Flash() -> impl IntoView {
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

/// Löschen-Knopf mit Bestätigungsdialog (ANF-002, B5).
///
/// Der Dialog nutzt DaisyUIs `modal-toggle` (Checkbox) und funktioniert
/// daher auch ohne JavaScript. `key` macht die ID eindeutig, wenn derselbe
/// Artikel mehrmals auf einer Seite vorkommt (Tabelle + Karten).
#[component]
pub fn DeleteArticleButton(
    id: i64,
    title: String,
    #[prop(optional)] key: &'static str,
    #[prop(optional)] small: bool,
) -> impl IntoView {
    let action = ServerAction::<DeleteArticle>::new();
    let version = use_context::<ArticlesVersion>();
    Effect::new(move |_| {
        if let Some(Ok(DeleteOutcome::Deleted)) = action.value().get()
            && let Some(v) = version
        {
            v.0.update(|n| *n += 1);
        }
    });

    let modal_id = format!("loeschen-{key}{id}");
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
                <h3 id=heading_id class="text-lg font-bold">"Artikel löschen"</h3>
                <p class="py-4">
                    "Artikel „" {title} "“ wirklich löschen? Das kann nicht rückgängig gemacht werden."
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
