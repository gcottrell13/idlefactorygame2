use leptos::prelude::*;
use malachite::base::num::arithmetic::traits::{ModPowerOf2};
use malachite::{Integer};

#[component]
pub fn SimpleCounter(start: Integer, up: Integer, down: Integer) -> impl IntoView {
    // create a reactive signal with the initial value
    let init = start.clone();
    let (value, set_value) = signal(init.clone());

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let d = down.clone();
    let u = up.clone();
    // update the state in all three different ways:
    let clear = move |_ev| set_value.set(init.clone());
    let decrement = move |_ev| set_value.update(|value| *value -= down.clone());
    let increment = move |_ev| {
        *set_value.write() += up.clone();
    };

    let get_progress = move || value.get().mod_power_of_2(5).to_string();

    view! {
        <div
    class:red=move || value.get().mod_power_of_2(1) == 1
        >
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-" {d.clone().to_string()}</button>
            <span>"Value: " {move || value.get().to_string()} "!"</span>
            <button on:click=increment>"+" {u.to_string()}</button>
        <ProgressBar max=50 progress=Signal::derive(get_progress) />
        </div>
    }
}

#[component]
fn ProgressBar(
    /// the maximum value
    #[prop(default = 100)]
    max: u16,

    /// the value of the progress bar, as a reactive signal
    #[prop(into)]
    progress: Signal<String>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
        // Add a line-break to avoid overlap
        <br/>
    }
}
