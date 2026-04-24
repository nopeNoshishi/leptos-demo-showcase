use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router base="/leptos-demo-showcase">
            <Routes fallback=|| view! { <p>"Not Found"</p> }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/leptos-fetch") view=showcase_leptos_fetch::Page />
                <Route path=path!("/transition-let") view=showcase_transition_let::Page />
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 text-gray-900">
            <header class="border-b bg-white">
                <div class="mx-auto max-w-4xl px-4 py-6">
                    <h1 class="text-2xl font-semibold">"Leptos Demo Showcase"</h1>
                </div>
            </header>
            <main class="mx-auto max-w-4xl px-4 py-8">
                <ul class="space-y-2">
                    <li>
                        <A href="/leptos-fetch" attr:class="text-indigo-600 hover:underline">
                            "leptos-fetch"
                        </A>
                    </li>
                    <li>
                        <A href="/transition-let" attr:class="text-indigo-600 hover:underline">
                            "transition-let"
                        </A>
                    </li>
                </ul>
            </main>
        </div>
    }
}
