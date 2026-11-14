//! ANF-007: Radio – `/radio` (Live + Sendeplan) und `/radio?ansicht=archiv`

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_query_map;

use crate::components::article::server_error_text;
use crate::components::player::{PlayButton, Track};
use crate::components::ui::{AccentTag, PageHeader};
use crate::radio::api::{radio_archive, radio_live};
use crate::radio::model::{LiveInfo, Mix, RadioView, Show, format_duration, msg};

#[component]
pub fn RadioPage() -> impl IntoView {
    let query = use_query_map();
    let view_mode = move || query.with(|q| q.get("ansicht").map(|s| RadioView::parse(&s)).unwrap_or_default());

    view! {
        <Title text="Radio – Saintwhoo" />
        <div class="mx-auto max-w-3xl py-6 md:py-10">
            <PageHeader title="Radio" subtitle="Livesets hören, Sendeplan und Archiv" />

            <nav aria-label="Ansicht" class="mt-5 flex rounded-full border border-base-300 bg-base-200 p-1">
                <ViewTab href="/radio" label="Live" active=Signal::derive(move || view_mode() == RadioView::Live) />
                <ViewTab href="/radio?ansicht=archiv" label="Archiv" active=Signal::derive(move || view_mode() == RadioView::Archive) />
            </nav>

            {move || match view_mode() {
                RadioView::Live => view! { <LiveView /> }.into_any(),
                RadioView::Archive => view! { <ArchiveView /> }.into_any(),
            }}
        </div>
    }
    .into_any()
}

#[component]
fn ViewTab(href: &'static str, label: &'static str, active: Signal<bool>) -> impl IntoView {
    view! {
        <a
            href=href
            class="flex min-h-11 flex-1 items-center justify-center rounded-full text-[15px]"
            class=("bg-primary", move || active.get())
            class=("text-primary-content", move || active.get())
            class=("font-semibold", move || active.get())
            class=("font-medium", move || !active.get())
            aria-current=move || active.get().then_some("page")
        >
            {label}
        </a>
    }
}

fn loading() -> impl IntoView {
    view! { <div class="flex justify-center py-16"><span class="loading loading-dots loading-lg"></span></div> }
}

#[component]
fn LiveView() -> impl IntoView {
    let data = Resource::new(|| (), |_| radio_live());
    view! {
        <Suspense fallback=loading>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(info) => view! { <Live info /> }.into_any(),
                    Err(e) => view! { <div role="alert" class="alert alert-error mt-6">{server_error_text(&e)}</div> }.into_any(),
                }
            })}
        </Suspense>
    }
    .into_any()
}

#[component]
fn Live(info: LiveInfo) -> impl IntoView {
    let now = info.now;
    let on_air = info.on_air().cloned();
    let today = info.today.clone();
    view! {
        <LiveCard stream_url=info.stream_url.clone() on_air />

        <h2 class="mt-8 mb-3.5 font-display text-2xl font-bold">"Sendeplan heute"</h2>
        {if today.is_empty() {
            view! { <p class="rounded-2xl border border-base-300 bg-base-200 p-4 text-muted">{msg::SCHEDULE_EMPTY}</p> }.into_any()
        } else {
            view! {
                <ul class="divide-y divide-base-300 rounded-2xl border border-base-300 bg-base-200 px-4">
                    {today.into_iter().map(|s| view! { <ScheduleRow show=s now /> }).collect_view()}
                </ul>
            }
            .into_any()
        }}
    }
    .into_any()
}

/// Karte „Jetzt auf Sendung“ mit großem Play-Knopf.
#[component]
pub fn LiveCard(stream_url: Option<String>, on_air: Option<Show>, #[prop(optional)] compact: bool) -> impl IntoView {
    let (title, subtitle) = match &on_air {
        Some(s) => (s.title.clone(), format!("{} · {} Uhr", s.host, s.time_range())),
        None => ("Saintwhoo Radio".to_string(), "Livestream".to_string()),
    };
    let label = if on_air.is_some() { "Jetzt auf Sendung" } else { "Gerade keine Sendung im Plan" };
    let track = stream_url.map(|url| Track {
        title: title.clone(),
        subtitle: on_air.as_ref().map(|s| s.host.clone()).unwrap_or_else(|| "Livestream".into()),
        url,
        live: true,
    });

    view! {
        <section aria-label="Livestream" class="mt-4 flex flex-col gap-4 rounded-box border border-base-300 bg-base-200 p-5">
            <div class="flex items-center gap-2.5 text-sm text-muted">
                <AccentTag label="LIVE" />
                <span>{label}</span>
            </div>
            <div class="flex flex-col gap-1">
                <h2 class="font-display text-[1.65rem] font-bold leading-tight">{title.clone()}</h2>
                <p class="text-[15px] text-muted">{subtitle}</p>
            </div>
            <Show when=move || !compact>
                <Bars />
            </Show>
            {match track {
                Some(t) => view! {
                    <div class="flex items-center gap-4">
                        <PlayButton track=t label="Livestream abspielen".to_string() size="size-14" />
                        <p class="text-[15px] leading-snug">
                            "Stream starten" <br />
                            <span class="text-muted">"Läuft beim Seitenwechsel weiter"</span>
                        </p>
                    </div>
                }
                .into_any(),
                None => view! { <p class="text-muted">{msg::STREAM_OFF}</p> }.into_any(),
            }}
        </section>
    }
    .into_any()
}

/// Dekorative Pegel-Balken wie im Design.
#[component]
fn Bars() -> impl IntoView {
    const HEIGHTS: [u8; 48] = [
        18, 30, 44, 26, 52, 38, 60, 34, 22, 48, 66, 40, 28, 54, 36, 62, 46, 24, 50, 32, 58, 42, 20, 56, 64, 30,
        44, 52, 26, 38, 60, 34, 48, 22, 54, 40, 28, 62, 36, 46, 24, 58, 32, 50, 42, 20, 30, 18,
    ];
    view! {
        <div class="flex h-18 items-center gap-[3px] overflow-hidden" aria-hidden="true">
            {HEIGHTS
                .iter()
                .enumerate()
                .map(|(i, h)| {
                    // Die erste Hälfte in Akzentfarbe (bereits gespielt)
                    let color = if i < 24 { "bg-primary" } else { "bg-base-300" };
                    view! { <div class=format!("w-1 shrink-0 rounded-sm {color}") style=format!("height:{h}px")></div> }
                })
                .collect_view()}
        </div>
    }
    .into_any()
}

#[component]
fn ScheduleRow(show: Show, now: chrono::NaiveDateTime) -> impl IntoView {
    let on_air = show.is_on_air(now);
    view! {
        <li class="flex items-center gap-3.5 py-3.5">
            <span class="w-24 shrink-0 text-sm text-muted tabular-nums">{show.time_range()}</span>
            <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                <span class="font-semibold">{show.title}</span>
                <span class="text-sm text-muted">{show.host}</span>
            </div>
            <Show when=move || on_air>
                <AccentTag label="JETZT" />
            </Show>
        </li>
    }
}

#[component]
fn ArchiveView() -> impl IntoView {
    let data = Resource::new(|| (), |_| radio_archive());
    view! {
        <h2 class="mt-8 mb-3.5 font-display text-2xl font-bold">"Archiv"</h2>
        <Suspense fallback=loading>
            {move || Suspend::new(async move {
                match data.await {
                    Ok(list) if list.is_empty() => view! {
                        <p class="rounded-2xl border border-base-300 bg-base-200 p-4 text-muted">{msg::ARCHIVE_EMPTY}</p>
                    }.into_any(),
                    Ok(list) => view! {
                        <ul class="flex flex-col gap-3">
                            {list.into_iter().map(|m| view! { <MixRow mix=m /> }).collect_view()}
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
fn MixRow(mix: Mix) -> impl IntoView {
    let label = format!("Set abspielen: {}", mix.title);
    let track = Track { title: mix.title.clone(), subtitle: mix.host.clone(), url: mix.audio_url.clone(), live: false };
    view! {
        <li class="flex items-center gap-3.5 rounded-2xl border border-base-300 bg-base-200 p-3.5">
            <PlayButton track label light=true />
            <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                <span class="font-semibold">{mix.title}</span>
                <span class="text-sm text-muted">{mix.host} " · " {format_duration(mix.duration_minutes)}</span>
            </div>
        </li>
    }
}
