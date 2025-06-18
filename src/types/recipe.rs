use crate::types::runtime_state_1::Features;
use malachite::Integer;
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

#[derive(Clone)]
pub struct Building {
    pub item: ItemName,
    pub power_per_second: Option<Vec<ItemAmount>>,
    pub crafting_time: f32,
}

#[derive(Clone, Copy)]
pub struct ByHand {
    pub name: &'static str,
    // pub crafting_time: f32,
    pub button_class: &'static str,
}

pub struct Recipe {
    pub name: RecipeName,
    pub image: Option<ItemName>,
    pub description: &'static str,
    pub inputs: Vec<ItemAmount>,
    pub outputs: Output,
    pub buildings: Option<Vec<Building>>,
    pub by_hand: Option<ByHand>,
    pub on_hand_buy_features_unlocked: Option<Vec<Features>>,
}

macro_rules! simple {
    [$($input: expr),+] => {
        Output::Simple(vec![$(ItemAmount($input.into(), Integer::ONE)),+])
    };
}
macro_rules! amount {
    ($base: literal, $exponent: literal, $input: expr) => {
        ItemAmount(
            $input.into(),
            Integer::from($base) * Integer::from(10).pow($exponent),
        )
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

pub(crate) use amount;
pub(crate) use simple;
pub(crate) use weighted;

impl Into<RecipeName> for &str {
    fn into(self) -> RecipeName {
        RecipeName(self.to_string())
    }
}

impl Default for Recipe {
    fn default() -> Self {
        Recipe {
            name: "default".into(),
            description: "default item",
            inputs: vec![],
            outputs: Output::Simple(vec![]),
            by_hand: None,
            buildings: None,
            image: None,
            on_hand_buy_features_unlocked: None,
        }
    }
}

impl Default for ByHand {
    fn default() -> Self {
        ByHand {
            name: "",
            // crafting_time: 0.0,
            button_class: "",
        }
    }
}