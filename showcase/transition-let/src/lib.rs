pub mod mini_let;
mod transition_let;

use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;
use transition_let::TransitionLet;

#[derive(Clone)]
struct FetchResult {
    label: &'static str,
    value: u32,
}

async fn fetch_after(ms: u32, label: &'static str, value: u32) -> Result<FetchResult, String> {
    TimeoutFuture::new(ms).await;
    Ok(FetchResult { label, value })
}

async fn fetch_fail_after(ms: u32) -> Result<FetchResult, String> {
    TimeoutFuture::new(ms).await;
    Err(format!("Failed after {}ms", ms))
}

#[component]
pub fn Page() -> impl IntoView {
    let count_1s = RwSignal::new(0u32);
    let count_2s = RwSignal::new(0u32);
    let count_err = RwSignal::new(0u32);

    let resource_1s = LocalResource::new(move || async move {
        let result = fetch_after(1_000, "1s", 100).await;
        count_1s.update(|n| *n += 1);
        result
    });
    let resource_2s = LocalResource::new(move || async move {
        let result = fetch_after(2_000, "2s", 999).await;
        count_2s.update(|n| *n += 1);
        result
    });
    let resource_err = LocalResource::new(move || async move {
        let result = fetch_fail_after(3_000).await;
        count_err.update(|n| *n += 1);
        result
    });

    let loading_fallback =
        || view! { <p class="text-sm text-gray-400 animate-pulse">"Loading..."</p> };
    let error_view = |e: String| view! { <p class="text-sm text-red-500">{e}</p> }.into_any();

    view! {
        <div class="min-h-screen bg-gray-50 text-gray-900">
            <header class="border-b bg-white">
                <div class="mx-auto max-w-4xl px-4 py-4 flex items-center justify-between">
                    <h1 class="text-xl font-semibold">"TransitionLet Demo"</h1>
                    <button
                        class="rounded-md border px-3 py-1.5 text-sm hover:bg-gray-50"
                        on:click=move |_| {
                            resource_1s.refetch();
                            resource_2s.refetch();
                            resource_err.refetch();
                        }
                    >
                        "Refetch"
                    </button>
                </div>
            </header>
            <main class="mx-auto max-w-4xl px-4 py-8">
                <div class="grid grid-cols-1 gap-6 md:grid-cols-3">

                    <div class="card">
                        <header class="p-4 border-b flex items-center justify-between">
                            <h2 class="text-sm font-semibold">"Success after 1s"</h2>
                            <span class="badge-outline text-xs px-2 py-0.5">"× " {count_1s}</span>
                        </header>
                        <div class="p-4">
                            <TransitionLet
                                resource=resource_1s
                                fallback=loading_fallback
                                error_view=error_view
                                let:data
                            >
                                <p class="text-sm">"Fetched after " {data.label}</p>
                                <p class="text-2xl font-bold text-indigo-600">{data.value}</p>
                            </TransitionLet>
                        </div>
                    </div>

                    <div class="card">
                        <header class="p-4 border-b flex items-center justify-between">
                            <h2 class="text-sm font-semibold">"Success after 2s"</h2>
                            <span class="badge-outline text-xs px-2 py-0.5">"× " {count_2s}</span>
                        </header>
                        <div class="p-4">
                            <TransitionLet
                                resource=resource_2s
                                fallback=loading_fallback
                                error_view=error_view
                                let:data
                            >
                                <p class="text-sm">"Fetched after " {data.label}</p>
                                <p class="text-2xl font-bold text-indigo-600">{data.value}</p>
                            </TransitionLet>
                        </div>
                    </div>

                    <div class="card">
                        <header class="p-4 border-b flex items-center justify-between">
                            <h2 class="text-sm font-semibold">"Error after 3s"</h2>
                            <span class="badge-outline text-xs px-2 py-0.5">"× " {count_err}</span>
                        </header>
                        <div class="p-4">
                            <TransitionLet
                                resource=resource_err
                                fallback=loading_fallback
                                error_view=error_view
                                let:data
                            >
                                <p class="text-sm">"Fetched after " {data.label}</p>
                                <p class="text-2xl font-bold text-indigo-600">{data.value}</p>
                            </TransitionLet>
                        </div>
                    </div>

                </div>
            </main>
        </div>
    }
}
