use leptos::prelude::*;

/// Minimal component demonstrating the `let:` binding pattern.
///
/// Multiplies `value` by 3 and passes the result to `children`.
/// Use `let:name` to bind the computed value in the view:
///
/// ```rust
/// view! {
///     <MiniI32TripleLet value=14 let:x>
///         <p>{x}</p>  // x = 42
///     </MiniI32TripleLet>
/// }
/// ```
#[component]
pub fn MiniI32TripleLet<CnFn, View>(value: i32, children: CnFn) -> impl IntoView
where
    CnFn: Fn(i32) -> View + Send + Clone + 'static,
    View: IntoView + 'static,
{
    children(value * 3)
}

/// Usage example for [`MiniI32TripleLet`] — referenced by `cargo expand` in the blog.
#[component]
pub fn MiniI32TripleLetUsage() -> impl IntoView {
    view! {
        <MiniI32TripleLet value=42 let:x>
            <p>{x}</p>
        </MiniI32TripleLet>
    }
}
