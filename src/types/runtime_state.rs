use crate::types::{ItemName, RecipeName};
use malachite::Integer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use semver::Version;

pub type ItemState = HashMap<String, Integer>;

pub type RecipeState = HashMap<RecipeName, RecipeRuntimeState>;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeRuntimeBuildingState {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeRuntimeState {
    pub buildings: HashMap<String, RecipeRuntimeBuildingState>,
}

/// Features unlocked in-game
#[derive(Serialize, Deserialize, Debug)]
pub enum Features {
    MultiBuy(Integer),
    AutoAssign,
}

///
/// Keeps the entire runtime state.
/// de/serialize to save and load from storage.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeState {
    pub item_amounts: ItemState,
    pub recipes: RecipeState,
    pub auto_building_assign: HashMap<ItemName, Vec<RecipeName>>,
    pub time_elapsed: i64,
    pub current_version: Version,
    pub unlocked_features: Vec<Features>,

    pub error: Option<String>,
}

impl RuntimeState {
    pub fn default(err: serde_json::Error) -> Self {
        RuntimeState {
            item_amounts: HashMap::new(),
            recipes: HashMap::new(),
            auto_building_assign: HashMap::new(),
            time_elapsed: 0,
            current_version: Version::from_str(include_str!("../../version.txt")).unwrap(),
            unlocked_features: Vec::new(),
            error: Some(err.to_string()),
        }
    }
}