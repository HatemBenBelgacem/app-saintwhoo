//! Systemstatus – prüft die Verbindung zu PostgreSQL.

use leptos::prelude::*;

/// Gibt die PostgreSQL-Version zurück.
#[server]
pub async fn db_version() -> Result<String, ServerFnError> {
    let pool = crate::db::pool()?;
    let (version,): (String,) = sqlx::query_as("SELECT version()")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(version)
}

#[component]
pub fn StatusPage() -> impl IntoView {
    let version = Resource::new(|| (), |_| db_version());

    view! {
        <div class="max-w-2xl mx-auto">
            <h1 class="text-3xl font-bold mb-6">"Systemstatus"</h1>
            <div class="card bg-base-100 shadow">
                <div class="card-body">
                    <h2 class="card-title">"Datenbank"</h2>
                    <Suspense fallback=|| view! { <span class="loading loading-dots"></span> }>
                        {move || version.get().map(|res| match res {
                            Ok(v) => view! {
                                <div role="alert" class="alert alert-success">
                                    <span>"Verbunden: " {v}</span>
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <div role="alert" class="alert alert-error">
                                    <span>"Fehler: " {e.to_string()}</span>
                                </div>
                            }.into_any(),
                        })}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}
