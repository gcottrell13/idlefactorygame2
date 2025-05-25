use std::sync::Arc;
use base64::prelude::*;
use crate::types::ItemName;
use crate::types::runtime_state_1::ItemState;

type Tick = Option<Box<&'static dyn Fn(f32, &mut ItemState)>>;

macro_rules! item_stats {
    (name=$name: expr, sprite_path=$sprite: expr) => {
        ItemStats::new($name.into(), include_bytes!($sprite), None)
    };
    (name=$name: expr, sprite_path=$sprite: expr, on_tick=$on_tick: expr) => {
        ItemStats::new($name.into(), include_bytes!($sprite), Some($on_tick))
    }
}


pub(crate) use item_stats;

pub struct ItemStats {
    pub name: ItemName,
    pub sprite_content: String,
    /// used to update the amount of this item at all times, even when there are no buildings
    pub on_tick: Tick,
}

impl ItemStats {
    pub fn new(name: ItemName, sprite_content: &'static [u8], on_tick: Option<&'static dyn Fn(f32, &mut ItemState)>) -> Arc<ItemStats> {
        Arc::new(ItemStats {
            name,
            sprite_content: "data:image/png;base64,".to_string() + BASE64_STANDARD.encode(sprite_content).as_str(),
            on_tick: on_tick.map(|f| Box::new(f)),
        })
    }
}