use leptos::prelude::*;

/// Wraps a `LocalResource<Result<Data, Error>>` in a `<Transition>` and exposes the
/// success value via a `let:` binding.
///
/// - While loading: shows `fallback` (defaults to empty).
/// - On `Ok(data)`: calls `children(data)` — bind the value with `let:name`.
/// - On `Err(e)`: calls `error_view(e)`.
/// - While refetching: `set_pending` is set to `true`.
///
/// # Example
///
/// ```rust
/// view! {
///     <TransitionLet
///         resource=my_resource
///         fallback=|| view! { <p>"Loading..."</p> }
///         error_view=|e: String| view! { <p>{e}</p> }
///         let:data
///     >
///         <p>{data.name}</p>
///     </TransitionLet>
/// }
/// ```
#[component(transparent)]
pub fn TransitionLet<Data, Error, CnFn, ErrFn, View, ErrView>(
    /// The Load Resource to read and expose via `let:`.
    resource: LocalResource<Result<Data, Error>>,
    /// A function that takes an error and returns a view to display in case of `Err`.
    error_view: ErrFn,
    /// Will be displayed while resources are pending. By default this is the empty view.
    #[prop(optional, into)]
    fallback: ViewFnOnce,
    /// A function that will be called when the component transitions into or out of
    /// the `pending` state, with its argument indicating whether it is pending (`true`)
    /// or not pending (`false`).
    #[prop(optional, into)]
    set_pending: Option<SignalSetter<bool>>,

    children: CnFn,
) -> impl IntoView
where
    Data: Clone + Send + Sync + 'static,
    Error: Clone + Send + Sync + 'static,
    CnFn: Fn(Data) -> View + Send + Clone + 'static,
    ErrFn: Fn(Error) -> ErrView + Send + Clone + 'static,
    View: IntoView + 'static,
    ErrView: IntoView + 'static,
{
    // If `set_pending` is not provided, we create a local signal to track pending state,
    let pending_setter = set_pending.unwrap_or_else(|| RwSignal::new(false).into());

    view! {
        <Transition fallback=fallback set_pending=pending_setter>
            {move || {
                resource
                    .get()
                    .map(|res| match res {
                        Ok(data) => children(data).into_any(),
                        Err(e) => error_view(e).into_any(),
                    })
            }}
        </Transition>
    }
}
