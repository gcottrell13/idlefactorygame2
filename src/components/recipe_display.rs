use std::sync::Arc;
use leptos::prelude::*;
use malachite::base::strings::ToDebugString;
use crate::types::{Recipe, RecipeRuntimeState};

#[component]
pub fn RecipeDisplay(recipe_state: Arc<RecipeRuntimeState>, recipe_info: Arc<Recipe>) -> impl IntoView {
    view!{
        <li><b>{recipe_info.name.to_debug_string()}</b>: {recipe_info.description}</li>
    }
}