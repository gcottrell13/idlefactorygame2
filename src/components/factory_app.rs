use leptos::prelude::*;
use std::error::Error;
use leptos::task::{spawn_local};
use malachite::{Integer};
use malachite::base::num::conversion::traits::FromStringBase;
use std::time::{Duration};
use async_std::task;
use malachite::base::num::basic::traits::{One, Zero};
use serde::{Serialize};
use web_sys::wasm_bindgen::JsValue;
use web_sys::window;
use crate::types::RuntimeState;

#[component]
pub fn FactoryApp() -> impl IntoView {
    let state = load().unwrap_or_else(RuntimeState::default);

    
}





macro_rules! save {
    ($state:ident, $storage:ident, $name: ident) => {
        $storage.set_item(stringify!($name), serde_json::to_string(&$state.$name).unwrap().as_str()).unwrap();
    };
}
fn save<T>(state: &RuntimeState) {
    let window = window().expect("window must exist");
    let storage = window.local_storage().expect("storage must exist").unwrap();
    save!(state, storage, auto_building_assign);
    save!(state, storage, current_version);
    save!(state, storage, item_amounts);
    save!(state, storage, recipes);
    save!(state, storage, time_elapsed);
    save!(state, storage, unlocked_features);
}

macro_rules! load {
    ($storage:ident, $name: ident) => {
        serde_json::from_str($storage.get_item(stringify!($name)).unwrap_or_else(|_| Some("".to_string())).unwrap().as_str())?
    };
}
fn load() -> Result<RuntimeState, serde_json::Error> {
    let window = window().expect("window must exist");
    let storage = window.local_storage().expect("storage must exist").unwrap();
    Ok(RuntimeState {
        auto_building_assign: load!(storage, auto_building_assign),
        current_version: load!(storage, current_version),
        item_amounts: load!(storage, item_amounts),
        recipes: load!(storage, recipes),
        time_elapsed: load!(storage, time_elapsed),
        unlocked_features: load!(storage, unlocked_features),
        error: None,
    })
}