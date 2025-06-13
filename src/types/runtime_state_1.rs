use crate::types::{ItemName, RecipeName};
use malachite::Integer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use semver::Version;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeRuntimeBuildingState {
    pub enabled: bool,
    pub progress: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeRuntimeState {
    pub name: RecipeName,
    pub buildings: HashMap<ItemName, RecipeRuntimeBuildingState>,
}

/// Features unlocked in-game
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Features {
    MultiBuy(Integer),
    AutoAssign,
}

pub type ItemState = HashMap<String, Integer>;

pub type RecipeState = HashMap<RecipeName, RecipeRuntimeState>;

///
/// Keeps the entire runtime state.
/// de/serialize to save and load from storage.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeState1 {
    pub item_amounts: ItemState,
    pub recipes: RecipeState,
    pub auto_building_assign: HashMap<ItemName, Vec<RecipeName>>,
    pub time_elapsed: i64,
    pub current_version: Version,
    pub unlocked_features: Vec<Features>,
}

impl RuntimeState1 {
    pub fn default() -> Self {
        RuntimeState1 {
            item_amounts: HashMap::new(),
            recipes: HashMap::new(),
            auto_building_assign: HashMap::new(),
            time_elapsed: 0,
            current_version: Version::from_str(include_str!("../../version.txt")).unwrap(),
            unlocked_features: Vec::new(),
        }
    }
}