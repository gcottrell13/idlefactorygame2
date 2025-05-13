use leptos::prelude::*;
use malachite::base::num::conversion::traits::FromStringBase;
use malachite::Natural;
use thiserror::Error;

#[derive(Error, Debug)]
struct MyError(String);

// Implement `Display` for `MyError`.
impl std::fmt::Display for MyError {
    // https://docs.rs/leptos/latest/leptos/error/struct.ErrorBoundaryProps.html#beginners-tip-errorboundary-requires-your-error-to-implement-stderrorerror
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.0)
    }
}

#[component]
pub fn NumericInput() -> impl IntoView {
    let (value, set_value) = signal(Some(Natural::from(0u16)));
    let v = move || value.get().map(|x| x.to_string()).ok_or(MyError("not a natural".to_string()));

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number"
                on:input:target=move |ev| {
                // when input changes, try to parse a number from the input
                set_value.set(Natural::from_string_base(10, ev.target().value().as_str()))
            }/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect_view()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{v}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }

}
