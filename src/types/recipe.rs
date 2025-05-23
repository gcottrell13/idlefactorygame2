use malachite::Integer;
use crate::types::runtime_state::{Features, ItemState, RecipeState};
use malachite::base::num::basic::traits::{One};
use malachite::base::num::arithmetic::traits::{Pow};

pub struct OutputWeight<T>(pub Vec<ItemAmount<T>>, pub f32);

pub struct ItemAmount<T>(pub T, pub Integer);


pub enum Output<T> {
    Simple(Vec<ItemAmount<T>>),
    Weighted(Vec<OutputWeight<T>>),
}

pub struct Building<T> {
    pub item: T,
    pub power_per_second: Vec<ItemAmount<T>>,
}

pub struct ByHand {
    pub name: &'static str,
}


pub struct Recipe<T> 
where T : Into<ItemAmount<T>> {
    pub name: &'static str,
    pub description: &'static str,
    pub inputs: Vec<ItemAmount<T>>,
    pub outputs: Output<T>,
    pub buildings: Option<Vec<Building<T>>>,
    pub by_hand: Option<ByHand>,
    /// used to update options on this recipe:
    /// FnMut(delta f32, item state, mutable recipe state)
    pub on_tick: Option<Box<dyn FnMut(f32, &ItemState<T>, &mut RecipeState<T>)>>,
    pub on_hand_buy_features_unlocked: Option<Vec<Features>>,
}

macro_rules! recipe {
    (name=$name: expr, description=$description: literal, inputs=[$($input: expr),+] , outputs=$outputs: expr, ) => {
        // provided: name, description, inputs, outputs. no buildings or by_hand, so by_hand set to default: "Create"
        recipe!(name=$name, description=$description, inputs=[$($input),+], outputs=$outputs, by_hand=Some(ByHand{name: "Create"}), )
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
        Recipe {
            name: $name,
            description: $description,
            inputs: vec![$($input),+],
            outputs: $outputs,
            buildings: $buildings,
            by_hand: $by_hand,
            on_tick: $on_tick,
            on_hand_buy_features_unlocked: $on_hand_buy_features_unlocked,
        }
    };
}

macro_rules! simple {
    [$($input: expr),+] => {
        Output::Simple(vec![$(ItemAmount($input, Integer::ONE)),+])
    };
}
macro_rules! amount {
    ($base: literal, $exponent: literal, $input: expr) => {
        ItemAmount($input, Integer::from($base) * Integer::from(10).pow($exponent))
    };
    ($amount: literal, $input: expr) => {
        ItemAmount($input, Integer::from($amount))
    };
    ($input: expr) => {
        ItemAmount($input, Integer::ONE)
    };
}

macro_rules! weighted {
    {$($weight: literal: $($item: expr),+)+} => {
        Output::Weighted(vec![$(OutputWeight(vec![$($item.into()),+], $weight as f32)),+])
    };
}

pub(crate) use recipe;
pub(crate) use amount;
pub(crate) use simple;
pub(crate) use weighted;


enum TestEnum {
    A,
    B,
}
use std::ops;
impl ops::Mul<i32> for TestEnum {
    type Output = ItemAmount<TestEnum>;
    fn mul(self, rhs: i32) -> Self::Output {
        ItemAmount(self, Integer::from(rhs))
    }
}
impl Into<ItemAmount<TestEnum>> for TestEnum {
    fn into(self) -> ItemAmount<TestEnum> {
        ItemAmount(self, Integer::ONE)
    }
}

fn test() {
    let a = recipe!(
        name="A",
        description="there",
        inputs=[amount!(100, 10, TestEnum::A), amount!(155, TestEnum::B)],
        outputs=weighted!{
            1: TestEnum::A * 2
            2: TestEnum::B, TestEnum::A
        },
    );
}