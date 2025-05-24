use crate::types::ItemName;
use crate::types::runtime_state::ItemState;

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
    item: ItemName,
    sprite_content: &'static [u8],
    /// used to update the amount of this item at all times, even when there are no buildings
    on_tick: Tick,
}

impl ItemStats {
    pub fn new(item: ItemName, sprite_content: &'static [u8], on_tick: Option<&'static dyn Fn(f32, &mut ItemState)>) -> ItemStats {
        ItemStats {
            item,
            sprite_content,
            on_tick: on_tick.map(|f| Box::new(f)),
        }
    }
}