# leptos-demo-showcase

Showcase repository that demonstrate Leptos mini app.

## Repository Structure

```
leptos-demo-showcase/
├── Cargo.toml              # workspace root
├── app/                    # main Leptos app — routing & Trunk build
└── showcase/
    └── leptos-fetch/       # lib crate — leptos-fetch showcase page
```

Each showcase under `showcase/` is a library crate that exposes a `Page` component. The `app/` crate imports them and routes between pages via `leptos_router`.

## Watch and Serve

```sh
# Requires `trunk` — install via https://trunkrs.dev
cd app
trunk serve
```

## Showcases

| Directory | Description |
|---|---|
| [`showcase/leptos-fetch/`](./showcase/leptos-fetch/) | leptos-fetch demo — query cache, auto refresh, force refetch, devtools |
