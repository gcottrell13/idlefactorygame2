use std::collections::HashMap;
use std::sync::Arc;
use crate::types::{Recipe, RecipeName, RecipeRuntimeState};
use leptos::prelude::*;
use malachite::base::strings::ToDebugString;

pub fn recipe_display(
    recipe_state: RwSignal<RecipeRuntimeState>,
    recipes: Arc<HashMap<RecipeName, Recipe>>,
) -> impl IntoView {
    let recipe_info = recipes.get(&recipe_state.read().name).unwrap();

    view! {
        <li><b>{recipe_info.name.to_debug_string()}</b>: {recipe_info.description}</li>
    }
}
