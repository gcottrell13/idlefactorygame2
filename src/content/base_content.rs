use crate::types::*;
use malachite::Integer;
use malachite::base::num::arithmetic::traits::Pow;
use malachite::base::num::basic::traits::One;
use malachite::base::strings::ToDebugString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops;
use std::sync::Arc;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
enum Item {
    A,
    B,
    C,
}

impl ops::Mul<i32> for Item {
    type Output = ItemAmount;
    fn mul(self, rhs: i32) -> Self::Output {
        ItemAmount(self.into(), Integer::from(rhs))
    }
}
impl Into<ItemAmount> for Item {
    fn into(self) -> ItemAmount {
        ItemAmount(self.into(), Integer::ONE)
    }
}
impl Into<ItemName> for Item {
    fn into(self) -> ItemName {
        ItemName(self.to_debug_string().into())
    }
}
impl Into<RecipeName> for Item {
    fn into(self) -> RecipeName {
        RecipeName(self.to_debug_string().into())
    }
}
impl Into<Option<ItemName>> for Item {
    fn into(self) -> Option<ItemName> {
        Some(ItemName(self.to_debug_string().into()))
    }
}

pub fn get_recipes() -> RecipeInfo {
    Arc::new(HashMap::from_iter(
        vec![
            new!(
                Recipe,
                name: Item::A,
                image: Item::A,
                description: "item A",
                inputs: vec![amount!(Item::A)],
                outputs: simple![Item::A, Item::B],
            ),
            new!(
                Recipe,
                name: "buy A",
                image: Item::A,
                description: "item A",
                inputs: vec![amount!(1, 10, Item::A)],
                outputs: simple![Item::A, Item::B, Item::C],
                by_hand: new!(ByHand, name: "Buy one", button_class: "red-button", ),
            ),
            new!(
                Recipe,
                name:"B #2",
                image: Item::B,
                description: "item B",
                inputs: vec![amount!(Item::A)],
                outputs:weighted!(
                    1: Item::B
                ),
            ),
        ]
        .drain(..)
        .map(|item| (item.name.clone(), item)),
    ))
}


pub fn get_item_info() -> ItemInfo {
    Arc::new(HashMap::from_iter(
        vec![
            item_stats!(name = Item::A, sprite_path = "sprites/iron-bar.png"),
            item_stats!(name = Item::B, sprite_path = "sprites/iron-gear.png"),
        ]
        .drain(..)
        .map(|item| (item.name.clone(), item)),
    ))
}
