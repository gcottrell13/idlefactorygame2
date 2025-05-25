use leptos::prelude::*;
use std::sync::Arc;
use malachite::base::strings::ToDebugString;
use semver::Version;
use web_sys::window;
use crate::components::RecipeDisplay;
use crate::content::{get_item_stats, get_recipes};
use crate::types::{RuntimeState, RuntimeState0};

#[component]
pub fn FactoryApp() -> impl IntoView {
    match load() {
        Ok(loaded) => factory_app(loaded).into_any(),
        Err(err) => view!{<h1>{err.to_debug_string()}</h1>}.into_any(),
    }
}


fn factory_app(loaded: RuntimeState) -> impl IntoView {
    let (read_state, write_state) = signal(loaded);

    let recipes = Arc::new(get_recipes());
    let item_states = Arc::new(get_item_stats());

    view! {
        <div>
            <h1>"Factory application:"</h1>
            <h2>{read_state.read_untracked().current_version.to_string()}</h2>
            {
                move || read_state.read().recipes.values().map(|recipe| {
                    let info = recipes.get(&recipe.name).unwrap();
                    view!{<RecipeDisplay recipe_state=Arc::clone(recipe) recipe_info=Arc::clone(info) />}
                }).collect_view()
            }
            {
                item_states.iter().map(|(name, item)| {
                    view!{
                        <div>{name.to_debug_string()} <img src=item.sprite_content.clone() /></div>
                    }
                }).collect_view()
            }
            <button on:click=move |_| save(&read_state.get())> Save </button>
        </div>
    }
}




macro_rules! save {
    ($state:ident, $storage:ident, $name: ident) => {
        $storage.set_item(stringify!($name), serde_json::to_string(&$state.$name).unwrap().as_str()).unwrap();
    };
}
fn save(state: &RuntimeState) {
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
        serde_json::from_str($storage.get_item(stringify!($name)).unwrap_or_else(|_| Some("".to_string())).unwrap_or("0".into()).as_str())?
    };
}

fn load() -> Result<RuntimeState, serde_json::Error> {
    let window = window().expect("window must exist");
    let storage = window.local_storage().expect("storage must exist").unwrap();

    let ver = storage.get_item("current_version").unwrap().unwrap_or("\"0.0.0\"".to_string());
    let version = serde_json::from_str::<Version>(ver.as_str())?;

    Ok(match version {
        n if n < Version::new(0, 0, 1) => RuntimeState0{}.into(),
        n => RuntimeState {
            auto_building_assign: load!(storage, auto_building_assign),
            current_version: n,
            item_amounts: load!(storage, item_amounts),
            recipes: load!(storage, recipes),
            time_elapsed: load!(storage, time_elapsed),
            unlocked_features: load!(storage, unlocked_features),
            error: None,
        }
    })
}
