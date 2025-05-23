use crate::types::{*};
use malachite::Integer;
use malachite::base::num::basic::traits::{One};

pub enum Item {
    A,
    B,
    C,
}

use std::ops;
impl ops::Mul<i32> for Item {
    type Output = ItemAmount<Item>;
    fn mul(self, rhs: i32) -> Self::Output {
        ItemAmount(self, Integer::from(rhs))
    }
}
impl Into<ItemAmount<Item>> for Item {
    fn into(self) -> ItemAmount<Item> {
        ItemAmount(self, Integer::ONE)
    }
}


pub fn get_recipes() -> Vec<Recipe<Item>> {
    vec![
        recipe!(
            name="A",
            description="item A",
            inputs=[amount!(Item::A)],
            outputs=simple![Item::A, Item::B],
        ),
        recipe!(
            name="A #2",
            description="item A",
            inputs=[amount!(Item::A)],
            outputs=weighted!(
                1: Item::A
            ),
        ),
    ]
}
