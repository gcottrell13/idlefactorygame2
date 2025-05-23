use crate::types::recipe::Recipe;
use malachite::Integer;
use std::collections::HashMap;
use crate::types::ItemAmount;

pub type ItemState<T> = HashMap<T, Integer>;
pub type RecipeState<T> = HashMap<Recipe<T>, RecipeRuntimeState<T>>;

pub struct RecipeRuntimeBuildingState {
    enabled: bool,
}

pub struct RecipeRuntimeState<T> {
    buildings: HashMap<T, RecipeRuntimeBuildingState>,
}



/// Features unlocked in-game
pub enum Features {
    MultiBuy(Integer),
    AutoAssign,
}


///
/// Keeps the entire runtime state.
/// de/serialize to save and load from storage.
/// 
pub struct RuntimeState<T> 
where T : Into<ItemAmount<T>> {
    item_amounts: ItemState<T>,
    recipes: RecipeState<T>,
    auto_building_assign: HashMap<T, Vec<Recipe<T>>>,
    time_elapsed: i64,
    current_version: String,
    unlocked_features: Vec<Features>,
}
