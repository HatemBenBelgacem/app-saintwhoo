//! ANF-007: Radio-Player, der beim Seitenwechsel weiterläuft.
//!
//! Der Player sitzt in `App` außerhalb der `<Routes>`. Seiten starten etwas
//! über den Kontext [`Player`]; das `<audio>`-Element bleibt beim Navigieren
//! bestehen, die Wiedergabe läuft also weiter.

use leptos::prelude::*;

/// Was gerade gespielt wird.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Track {
    pub title: String,
    /// z. B. DJ oder „Live-Stream“
    pub subtitle: String,
    pub url: String,
    pub live: bool,
}

/// Gemeinsamer Zustand des Players (im Kontext von `App`).
#[derive(Clone, Copy)]
pub struct Player {
    pub track: RwSignal<Option<Track>>,
}

impl Player {
    pub fn new() -> Self {
        Self { track: RwSignal::new(None) }
    }

    pub fn play(&self, track: Track) {
        self.track.set(Some(track));
    }

    pub fn stop(&self) {
        self.track.set(None);
    }

    /// Läuft gerade genau diese Adresse?
    pub fn is_playing(&self, url: &str) -> bool {
        self.track.with(|t| t.as_ref().is_some_and(|t| t.url == url))
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

pub fn use_player() -> Player {
    expect_context::<Player>()
}

/// Knopf „Abspielen“ / „Stopp“ für eine Aufnahme oder den Stream.
#[component]
pub fn PlayButton(
    track: Track,
    /// Beschriftung für Screenreader, z. B. „Set abspielen: Nachtschicht #13“
    label: String,
    #[prop(default = "size-11")] size: &'static str,
    /// Heller Knopf statt Akzentfarbe (Archiv-Liste)
    #[prop(optional)]
    light: bool,
) -> impl IntoView {
    let player = use_player();
    let url = track.url.clone();
    let playing = Memo::new(move |_| player.is_playing(&url));
    let stop_label = format!("Stopp: {}", track.title);
    let toggle = move |_| {
        if playing.get_untracked() {
            player.stop();
        } else {
            player.play(track.clone());
        }
    };
    view! {
        <button
            type="button"
            on:click=toggle
            class=format!("btn btn-circle shrink-0 border-0 {size}")
            class=("btn-primary", move || !light)
            class=("btn-neutral", move || light)
            aria-pressed=move || if playing.get() { "true" } else { "false" }
            aria-label=move || if playing.get() { stop_label.clone() } else { label.clone() }
        >
            {move || {
                if playing.get() {
                    view! {
                        <svg width="20" height="20" viewBox="0 0 24 24" aria-hidden="true">
                            <path d="M7 5h4v14H7zM13 5h4v14h-4z" fill="currentColor" />
                        </svg>
                    }
                    .into_any()
                } else {
                    view! {
                        <svg width="20" height="20" viewBox="0 0 24 24" aria-hidden="true">
                            <path d="M7 4.5v15l12-7.5z" fill="currentColor" />
                        </svg>
                    }
                    .into_any()
                }
            }}
        </button>
    }
}

/// Leiste mit dem laufenden Titel. Auf dem Handy über der Navigationsleiste.
#[component]
pub fn PlayerBar() -> impl IntoView {
    let player = use_player();
    // ANF-011 (Kann): Titel auf dem Sperrbildschirm, Play/Pause von dort
    Effect::new(move |_| {
        let current = player.track.with(|t| t.as_ref().map(|t| (t.title.clone(), t.subtitle.clone())));
        crate::components::pwa::set_media_session(current);
    });
    view! {
        {move || {
            player
                .track
                .get()
                .map(|t| {
                    view! {
                        // Platzhalter, damit die Leiste keinen Inhalt verdeckt
                        <div class="h-28" aria-hidden="true"></div>
                        <section
                            aria-label="Player"
                            class="fixed inset-x-0 bottom-[calc(4.25rem+max(1rem,env(safe-area-inset-bottom)))] z-30 px-3 md:bottom-4"
                        >
                            <div class="container mx-auto flex flex-wrap items-center gap-3 rounded-box border border-base-300 bg-base-200 p-3 sm:flex-nowrap">
                                <div class="min-w-0 flex-1">
                                    <p class="flex items-center gap-2 text-sm text-muted">
                                        {t.live.then(|| view! {
                                            <span class="badge badge-primary badge-sm font-bold tracking-wider">"LIVE"</span>
                                        })}
                                        <span class="truncate">{t.subtitle.clone()}</span>
                                    </p>
                                    <p class="truncate font-semibold">{t.title.clone()}</p>
                                </div>
                                <audio
                                    src=t.url.clone()
                                    controls=true
                                    autoplay=true
                                    preload="none"
                                    class="h-10 w-full sm:w-80"
                                    aria-label=format!("Wiedergabe: {}", t.title)
                                ></audio>
                                <button
                                    type="button"
                                    class="btn btn-ghost btn-circle size-11"
                                    aria-label="Player schließen"
                                    on:click=move |_| player.stop()
                                >
                                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">
                                        <path d="M6 6l12 12M18 6L6 18" />
                                    </svg>
                                </button>
                            </div>
                        </section>
                    }
                })
        }}
    }
    .into_any()
}
