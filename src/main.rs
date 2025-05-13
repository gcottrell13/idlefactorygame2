use leptos::prelude::*;
use malachite::base::num::arithmetic::traits::{Factorial};
use malachite::{Integer, Natural};

mod components;
use components::{NumericInput, SimpleCounter, EmailForm};

use web_sys::{window, };


fn main() {
    console_error_panic_hook::set_once();

    let hundred: Integer = Natural::factorial(100).into();
    let values = vec![0, 1, 2];
    
    let storage = window().expect("window must exist").local_storage().expect("storage must exist").expect("storage must exist");
    let v = storage.get("hi");
    storage.set("hi", "10");

    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters
        .map(|count| {
            view! {
            <li>
                <button
                    on:click=move |_| *count.write() += 1
                >
                    {count}
                </button>
            </li>
        }
        })
        .collect_view();

    mount_to_body(|| view! {
        <div>
            <SimpleCounter start=hundred up=Integer::from(2) down=Integer::from(3) />
        <hr />
            <ul>
                {values.into_iter()
                    .map(|n| view! { <li>{n}</li>})
                    .collect::<Vec<_>>()}
            </ul>
            <ul>{counter_buttons}</ul>
        <hr />
            <EmailForm />
        <hr />
            <NumericInput />
        </div>

    })
}
