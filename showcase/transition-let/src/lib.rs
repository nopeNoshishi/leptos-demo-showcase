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
        leptos::logging::debug_log!("Fetching 1s resource...");
        let result = fetch_after(1_000, "1s", 100).await;
        leptos::logging::debug_log!("1s resource fetched and count will be updated");

        count_1s.update(|n| *n += 1);

        result
    });
    let resource_2s = LocalResource::new(move || async move {
        leptos::logging::debug_log!("Fetching 2s resource...");
        let result = fetch_after(2_000, "2s", 999).await;
        leptos::logging::debug_log!("2s resource fetched and count will be updated");

        count_2s.update(|n| *n += 1);

        result
    });
    let resource_err = LocalResource::new(move || async move {
        leptos::logging::debug_log!("Fetching error resource...");
        let result = fetch_fail_after(3_000).await;
        leptos::logging::debug_log!("Error resource fetched and count will be updated");

        count_err.update(|n| *n += 1);

        result
    });

    let on_refetch = move || {
        leptos::logging::debug_log!("Refetching all resources...");
        resource_1s.refetch();
        resource_2s.refetch();
        resource_err.refetch();
    };

    view! {
        <div class="min-h-screen bg-gray-50 text-gray-900">
            <PageHeader on_refetch />
            <ResourceGrid
                count_1s resource_1s
                count_2s resource_2s
                count_err resource_err
            />
        </div>
    }
}

#[component]
fn PageHeader(on_refetch: impl Fn() + 'static) -> impl IntoView {
    view! {
        <header class="border-b bg-white">
            <div class="mx-auto max-w-4xl px-4 py-4 flex items-center justify-between">
                <h1 class="text-xl font-semibold">"TransitionLet Demo"</h1>
                <button
                    class="rounded-md border px-3 py-1.5 text-sm hover:bg-gray-50"
                    on:click=move |_| on_refetch()
                >
                    "Refetch"
                </button>
            </div>
        </header>
    }
}

#[component]
fn ResourceGrid(
    count_1s: RwSignal<u32>,
    resource_1s: LocalResource<Result<FetchResult, String>>,
    count_2s: RwSignal<u32>,
    resource_2s: LocalResource<Result<FetchResult, String>>,
    count_err: RwSignal<u32>,
    resource_err: LocalResource<Result<FetchResult, String>>,
) -> impl IntoView {
    view! {
        <main class="mx-auto max-w-4xl px-4 py-8">
            <div class="grid grid-cols-1 gap-6 md:grid-cols-3">
                <ResourceCard title="Success after 1s" count=count_1s resource=resource_1s />
                <ResourceCard title="Success after 2s" count=count_2s resource=resource_2s />
                <ResourceCard title="Error after 3s" count=count_err resource=resource_err />
            </div>
        </main>
    }
}

#[component]
fn ResourceCard(
    title: &'static str,
    count: RwSignal<u32>,
    resource: LocalResource<Result<FetchResult, String>>,
) -> impl IntoView {
    let loading_fallback =
        || view! { <p class="text-sm text-gray-400 animate-pulse">"Loading..."</p> };
    let error_view = |e: String| view! { <p class="text-sm text-red-500">{e}</p> };
    let pending = RwSignal::new(false);

    view! {
        <div class="card">
            <header class="p-4 border-b flex items-center justify-between">
                <h2 class="text-sm font-semibold">{title}</h2>
                <span class="badge-outline text-xs px-2 py-0.5">"× " {count}</span>
            </header>
            <div
                class="p-4 transition-opacity duration-200"
                class:opacity-30=move || pending.get()
            >
                <TransitionLet
                    resource=resource
                    fallback=loading_fallback
                    error_view=error_view
                    set_pending=pending
                    let:data
                >
                    <p class="text-sm">"Fetched after " {data.label}</p>
                    <p class="text-2xl font-bold text-indigo-600">{data.value}</p>
                </TransitionLet>
            </div>
        </div>
    }
}
