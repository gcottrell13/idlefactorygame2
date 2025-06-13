use std::collections::HashMap;
use std::sync::Arc;
use leptos::ev;
use leptos::prelude::*;
use leptos::html::*;
use malachite::base::strings::ToDebugString;
use semver::Version;
use web_sys::window;
use crate::components::item_image_display::item_image_display;
use crate::content::{get_item_info, get_recipes};
use crate::engine::increase_amounts_from_recipe_image;
use crate::types::{*};

#[component]
pub fn FactoryApp() -> impl IntoView {
    match load() {
        Ok(loaded) => {
            let recipes = get_recipes();
            let item_info = get_item_info();
            factory_app(loaded, recipes, item_info).into_any()
        },
        Err(err) => view!{<h1>{err.to_debug_string()}</h1>}.into_any(),
    }
}
pub fn counter(initial_value: i32, step: i32) -> impl IntoView {
    let (count, set_count) = signal(initial_value);
    div().child((
        button()
            // typed events found in leptos::ev
            // 1) prevent typos in event names
            // 2) allow for correct type inference in callbacks
            .on(ev::click, move |_| set_count.set(0))
            .child("Clear"),
        button()
            .on(ev::click, move |_| *set_count.write() -= step)
            .child("-1"),
        span().child(("Value: ", move || count.get(), "!")),
        button()
            .on(ev::click, move |_| *set_count.write() += step)
            .child("+1"),
    ))
}

fn factory_app(_loaded: RuntimeState, recipes: RecipeInfo, item_info: ItemInfo) -> impl IntoView {
    let (loaded, set_loaded) = signal(_loaded);
    div().child((
        h1().child("Factory application"),
        h2().child(loaded.get().current_version.to_string()),
        For(ForProps {
            each: move || loaded.get().item_amounts,
            key: |state| state.0.clone(),
            children: {
                let v = Arc::clone(&item_info);
                move |(name, amount)| span().child((
                    item_image_display(name, v.clone()),
                    move || amount.get().to_string(),
                ))
            }
        }),
        For(ForProps {
            each: move || loaded.get().recipes,
            key: |state| state.0.clone(),
            children: move |(name, recipe_state)| div().child((
                {
                    let recipes = recipes.clone();
                    let item_info = item_info.clone();
                    item_image_display(recipes.get(&name).unwrap().image.as_ref().unwrap().clone(), item_info)
                },
                name.to_debug_string(),
                button()
                    .on(ev::click, {
                        let recipes = recipes.clone();
                        move |_| increase_amounts_from_recipe_image(recipes.get(&name).unwrap(), &loaded.read().item_amounts)
                    })
                    .child("add one")
            ))
        })
    ))

    // view! {
    //     <div>
    //         <h1>"Factory application:"</h1>
    //         <h2>{}</h2>
    //         <For
    //             each=move || loaded.get().recipes
    //             key=|state| state.0
    //             let(child)
    //         >
    //             <RecipeDisplay recipe_state=child.1.clone() recipes=Arc::clone(&recipes) />
    //         </For>
    //         {
    //             loaded.read().recipes.iter().map(move |(name, item)| {
    //                 let image = &b.get(&name).unwrap().image;
    //                 view!{
    //                     <div>
    //                         {name.to_debug_string()}
    //                         <ItemImageDisplay name=image item_info=Arc::clone(&item_info) />
    //                         <button on:click=move |_| {
    //                             increase_amounts_from_recipe_image(recipes.get(name).unwrap(), &loaded.read().item_amounts);
    //                         }>
    //                             add one
    //                         </button>
    //                     </div>
    //                 }
    //             }).collect_view()
    //         }
    //         <button on:click=move |_| save(&loaded.read())> Save </button>
    //     </div>
    // }
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
        n if n < Version::new(0, 0, 1) => RuntimeState0{}.update().into(),
        n => RuntimeState1 {
            auto_building_assign: load!(storage, auto_building_assign),
            current_version: n,
            item_amounts: load!(storage, item_amounts),
            recipes: load!(storage, recipes),
            time_elapsed: load!(storage, time_elapsed),
            unlocked_features: load!(storage, unlocked_features),
        }.into()
    })
}

impl RuntimeState0 {
    fn update(self) -> RuntimeState1 {
        RuntimeState1::default()
    }
}

impl Into<RuntimeState> for RuntimeState1 {
    fn into(self) -> RuntimeState {
        RuntimeState {
            item_amounts: HashMap::from_iter(self.item_amounts.iter().map(|(key, value)| {
                (ItemName(key.clone()), RwSignal::new(value.clone()))
            })),
            recipes: HashMap::from_iter(self.recipes.iter().map(|(key, value)| {
                (key.clone(), RwSignal::new(value.clone()))
            })),
            auto_building_assign: HashMap::from_iter(self.auto_building_assign.iter().map(|(key, value)| {
                (key.clone(), value.iter().map(|recipe| RwSignal::new(recipe.clone())).collect())
            })),
            current_version: self.current_version,
            unlocked_features: RwSignal::new(self.unlocked_features.clone()),
            time_elapsed: RwSignal::new(self.time_elapsed),
        }
    }
}
