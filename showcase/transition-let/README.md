# showcase-transition-let

A showcase demonstrating the `let:` binding pattern in Leptos using a custom `<TransitionLet>` component.

## What this shows

- How to build a component that exposes an async result via `let:` binding
- Using `LocalResource` with simulated delays to demonstrate loading and error states
- The `<Transition>` component keeping the previous value visible while re-fetching

## Components

### `TransitionLet`

Wraps `LocalResource<Result<T, E>>` in a `<Transition>` and exposes the success value via `let:`.

```rust
view! {
    <TransitionLet
        resource=my_resource
        fallback=|| view! { <p>"Loading..."</p> }
        error_view=|e: String| view! { <p>{e}</p> }.into_any()
        let:data
    >
        <p>{data.name}</p>
    </TransitionLet>
}
```

### `MiniI32TripleLet`

A minimal example showing how any `children: Fn(T) -> V` prop enables `let:` binding.

## Watch and Serve

Run from the workspace root:

```sh
cd app
trunk serve
```
