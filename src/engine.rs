use leptos::prelude::*;
use malachite::Integer;
use malachite::base::num::basic::traits::One;
use crate::types::{ItemAmount, Recipe, RuntimeItemAmounts, RuntimeState};

pub fn tick<T: Into<ItemAmount>>(state: &mut RuntimeState) {

}

pub fn increase_amounts_from_recipe_image<'a>(recipe: &'a Recipe, state: &'a RuntimeItemAmounts) {
    let image = &recipe.image;
    if let Some(image) = image {
        *state.get(image).unwrap().write() += Integer::ONE;
    }
}