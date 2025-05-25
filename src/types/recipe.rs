use malachite::Integer;
use crate::types::runtime_state_1::{Features, ItemState, RecipeState};
use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct RecipeName(pub String);

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct ItemName(pub String);

pub struct OutputWeight(pub Vec<ItemAmount>, pub f32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemAmount(pub ItemName, pub Integer);


pub enum Output {
    Simple(Vec<ItemAmount>),
    Weighted(Vec<OutputWeight>),
}

pub struct Building {
    pub item: ItemName,
    pub power_per_second: Option<Vec<ItemAmount>>,
    pub crafting_time: f32,
}

pub struct ByHand {
    pub name: &'static str,
    pub crafting_time: f32,
}


pub struct Recipe {
    pub name: RecipeName,
    pub description: &'static str,
    pub inputs: Vec<ItemAmount>,
    pub outputs: Output,
    pub buildings: Option<Vec<Building>>,
    pub by_hand: Option<ByHand>,
    /// used to update options on this recipe:
    /// FnMut(delta f32, item state, mutable recipe state)
    // pub on_tick: Option<Box<dyn FnMut(f32, &ItemState, &mut RecipeState)>>,
    pub on_hand_buy_features_unlocked: Option<Vec<Features>>,
}

macro_rules! recipe {
    (name=$name: expr, description=$description: literal, inputs=[$($input: expr),+] , outputs=$outputs: expr, ) => {
        // provided: name, description, inputs, outputs. no buildings or by_hand, so by_hand set to default: "Create"
        recipe!(name=$name, description=$description, inputs=[$($input),+], outputs=$outputs, by_hand=Some(ByHand{name: "Create", crafting_time: 1f32}), )
    };
    (name=$name: expr, description=$description: literal, inputs=[$($input: expr),+], outputs=$outputs: expr, buildings=$buildings:expr,) => {
        // provided: name, description, inputs, outputs, buildings. by_hand set to None
        recipe!(name=$name, description=$description, inputs=[$($input),+], outputs=$outputs, buildings=$buildings, by_hand=None, )
    };
    (name=$name: expr, description=$description: literal, inputs=[$($input: expr),+], outputs=$outputs: expr, by_hand=$by_hand:expr,) => {
        // provided: name, description, inputs, outputs, by_hand
        recipe!(name=$name, description=$description, inputs=[$($input),+], outputs=$outputs, buildings=None, by_hand=$by_hand, )
    };

    (name=$name: expr, description=$description: literal, inputs=[$($input: expr),+], outputs=$outputs: expr, buildings=$buildings:expr, by_hand=$by_hand:expr,) => {
        // provided: name, description, inputs, outputs, buildings, by_hand. on_tick set to None
        recipe!(name=$name, description=$description, inputs=[$($input),+], outputs=$outputs, buildings=$buildings, by_hand=$by_hand, on_tick=None, )
    };

    (name=$name: expr, description=$description: literal, inputs=[$($input: expr),+], outputs=$outputs: expr, buildings=$buildings:expr, by_hand=$by_hand:expr, on_tick=$on_tick: expr, ) => {
        // provided: name, description, inputs, outputs, buildings, by_hand, on_tick. on_hand_buy_features_unlocked set to None
        recipe!(name=$name, description=$description, inputs=[$($input),+], outputs=$outputs, buildings=$buildings, by_hand=$by_hand, on_tick=$on_tick, on_hand_buy_features_unlocked=None)
    };
    (name=$name: expr, description=$description: literal, inputs=[$($input: expr),+], outputs=$outputs: expr, buildings=$buildings:expr, by_hand=$by_hand:expr, on_hand_buy_features_unlocked=$on_hand_buy_features_unlocked: expr, ) => {
        // provided: name, description, inputs, outputs, buildings, by_hand, on_hand_buy_features_unlocked. on_tick set to None
        recipe!(name=$name, description=$description, inputs=[$($input),+], outputs=$outputs, buildings=$buildings, by_hand=$by_hand, on_tick=None, on_hand_buy_features_unlocked=$on_hand_buy_features_unlocked)
    };

    (name=$name: expr, description=$description: literal,
    inputs=[$($input: expr),+], outputs=$outputs: expr,
    buildings=$buildings:expr, by_hand=$by_hand:expr,
    on_tick=$on_tick:expr, on_hand_buy_features_unlocked=$on_hand_buy_features_unlocked:expr) => {
        // provided: all
        Arc::new(Recipe {
            name: $name.into(),
            description: $description,
            inputs: vec![$($input),+],
            outputs: $outputs,
            buildings: $buildings,
            by_hand: $by_hand,
            // on_tick: $on_tick,
            on_hand_buy_features_unlocked: $on_hand_buy_features_unlocked,
        })
    };
}

macro_rules! simple {
    [$($input: expr),+] => {
        Output::Simple(vec![$(ItemAmount($input.into(), Integer::ONE)),+])
    };
}
macro_rules! amount {
    ($base: literal, $exponent: literal, $input: expr) => {
        ItemAmount($input.into(), Integer::from($base) * Integer::from(10).pow($exponent))
    };
    ($amount: literal, $input: expr) => {
        ItemAmount($input.into(), Integer::from($amount))
    };
    ($input: expr) => {
        ItemAmount($input.into(), Integer::ONE)
    };
}

macro_rules! weighted {
    {$($weight:literal: $($item: expr),+)+} => {
        Output::Weighted(vec![$(OutputWeight(vec![$($item.into()),+], $weight as f32)),+])
    };
}

pub(crate) use recipe;
pub(crate) use amount;
pub(crate) use simple;
pub(crate) use weighted;


impl Into<RecipeName> for &str {
    fn into(self) -> RecipeName {
        RecipeName(self.to_string())
    }
}