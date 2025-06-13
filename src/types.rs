mod recipe;
mod item_stats;
mod runtime_state_1;
mod runtime_state_0;

use std::collections::HashMap;
use std::sync::Arc;
use leptos::prelude::RwSignal;
use malachite::Integer;
use semver::Version;
use serde::{Deserialize, Serialize};

pub(crate) use recipe::{*};
pub(crate) use item_stats::{*};
pub(crate) use runtime_state_0::{*};
pub(crate) use runtime_state_1::{*};

macro_rules! new
{
    ($($strct:ident)::* , $($field:ident: $val:expr, )*) =>
    {
        $($strct)::*
        {
            $($field: $val .into(),)*
            ..$($strct)::*::default()
        }
    }
}
pub(crate) use new;

pub(crate) type RuntimeItemAmounts = HashMap<ItemName, RwSignal<Integer>>;
pub(crate) type RuntimeRecipes = HashMap<RecipeName, RwSignal<RecipeRuntimeState>>;
pub(crate) type RuntimeBuildingAssign = HashMap<ItemName, Vec<RwSignal<RecipeName>>>;
pub(crate) type RuntimeUnlockedFeatures = RwSignal<Vec<Features>>;

pub(crate) type RecipeInfo = Arc<HashMap<RecipeName, Recipe>>;
pub(crate) type ItemInfo = Arc<HashMap<ItemName, ItemStats>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RuntimeState {
    pub item_amounts: RuntimeItemAmounts,
    pub recipes: RuntimeRecipes,
    pub auto_building_assign: RuntimeBuildingAssign,
    pub time_elapsed: RwSignal<i64>,
    pub current_version: Version,
    pub unlocked_features: RuntimeUnlockedFeatures,
}