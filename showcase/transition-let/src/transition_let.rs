use leptos::prelude::*;
use std::sync::Arc;

/// A clonable wrapper around an error-rendering function `Fn(E) -> AnyView`.
///
/// Used as the `error_view` prop of [`TransitionLet`]. Accepts any closure via
/// the `From` impl, so you can pass `|e| view! { ... }.into_any()` directly.
pub struct ErrorViewFn<E>(Arc<dyn Fn(E) -> AnyView + Send + Sync + 'static>);

impl<E> Clone for ErrorViewFn<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// Defaults to rendering nothing on error.
impl<E> Default for ErrorViewFn<E> {
    fn default() -> Self {
        Self(Arc::new(|_| ().into_any()))
    }
}

impl<E> ErrorViewFn<E> {
    fn run(&self, e: E) -> AnyView {
        (self.0)(e)
    }
}

impl<E, F> From<F> for ErrorViewFn<E>
where
    F: Fn(E) -> AnyView + Send + Sync + 'static,
{
    fn from(f: F) -> Self {
        Self(Arc::new(f))
    }
}

/// Wraps a `LocalResource<Result<T, E>>` in a `<Transition>` and exposes the
/// success value via a `let:` binding.
///
/// - While loading: shows `fallback` (defaults to empty).
/// - On `Ok(data)`: calls `children(data)` — bind the value with `let:name`.
/// - On `Err(e)`: calls `error_view(e)` (defaults to empty).
///
/// # Example
///
/// ```rust
/// view! {
///     <TransitionLet
///         resource=my_resource
///         fallback=|| view! { <p>"Loading..."</p> }
///         error_view=|e: String| view! { <p>{e}</p> }.into_any()
///         let:data
///     >
///         <p>{data.name}</p>
///     </TransitionLet>
/// }
/// ```
#[component]
pub fn TransitionLet<T, E, F, V>(
    resource: LocalResource<Result<T, E>>,
    children: F,
    #[prop(optional, into)] fallback: ViewFnOnce,
    #[prop(optional, into)] error_view: ErrorViewFn<E>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    E: Clone + Send + Sync + 'static,
    F: Fn(T) -> V + Send + Clone + 'static,
    V: IntoView + 'static,
{
    view! {
        <Transition fallback=fallback>
            {move || {
                let children = children.clone();
                let error_view = error_view.clone();
                resource
                    .get()
                    .map(|res: Result<T, E>| match res {
                        Ok(data) => children(data).into_any(),
                        Err(e) => error_view.run(e),
                    })
            }}
        </Transition>
    }
}
