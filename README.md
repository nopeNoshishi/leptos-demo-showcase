# Leptos Demo Showcase

A collection of small Leptos demos, each exploring a specific pattern or library in the Leptos ecosystem.

## Repository Structure

```
leptos-demo-showcase/
├── Cargo.toml              # workspace root (shared dependencies)
├── app/                    # main Leptos SPA — routing & Trunk build
└── showcase/
    ├── leptos-fetch/       # leptos-fetch: query cache, devtools, refetch
    └── transition-let/     # let: binding pattern with TransitionLet
```

Each showcase under `showcase/` is a library crate that exposes a `#[component] pub fn Page()`. The `app/` crate imports them and routes between pages via `leptos_router`.

## Watch and Serve

```sh
# Requires `trunk` — install via https://trunkrs.dev
cd app
trunk serve
```

## Showcases

| Directory | What it demonstrates |
|---|---|
| [`showcase/leptos-fetch/`](./showcase/leptos-fetch/) | [`leptos-fetch`](https://github.com/nicholasgasior/leptos-fetch) — query cache, auto refresh, force refetch, devtools |
| [`showcase/transition-let/`](./showcase/transition-let/) | `let:` binding pattern — building components that expose async results via `let:name` |
