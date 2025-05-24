use leptos::prelude::*;
use leptos::task::{spawn_local};
use malachite::{Integer};
use malachite::base::num::conversion::traits::FromStringBase;
use std::time::{Duration};
use async_std::task;
use malachite::base::num::basic::traits::{One, Zero};
use web_sys::window;


#[component]
pub fn NumberGoUp() -> impl IntoView {

    let window = window().expect("window must exist");
    let storage = window.local_storage().expect("storage must exist").unwrap();
    let value = match storage.get("value").unwrap() {
        Some(value) => Integer::from_string_base(10, &value).unwrap(),
        None => Integer::from(0u8),
    };

    let all_values = vec![value];

    let (incrementers, set_incrementers) = signal(all_values);

    let counters = move || incrementers.get()
        .into_iter()
        .enumerate()
        .map(|(i, count)| {
            view! {
            <li>
                {move || count.to_string()}
            </li>
        }
        })
        .collect_view();

    spawn_local(async move {
        loop {
            task::sleep(Duration::from_millis(1)).await;
            let current_value = incrementers.get_untracked();
            current_value.clone()
                .into_iter()
                .enumerate()
                .for_each(|(index, count)| {
                    (*set_incrementers.write())[index] += match current_value.get(index + 1) {
                        Some(value) => value.clone(),
                        None => Integer::ONE
                    }
                });
        }
    });

    spawn_local(async move {
        loop {
            task::sleep(Duration::from_secs(20)).await;
            let current = incrementers.get_untracked().get(0).unwrap().clone();
            storage.set("value", &current.to_string().to_string()).expect("TODO: panic message");
        }
    });

    view! {
        <div>
            {counters}
        <br/>
            <button on:click=move |_| (*set_incrementers.write()).push(Integer::ZERO)>
                Add Row
            </button>
        </div>
    }
}