use base64::prelude::*;
use crate::types::ItemName;

macro_rules! item_stats {
    (name=$name: expr, sprite_path=$sprite: expr) => {
        ItemStats::new($name.into(), include_bytes!($sprite))
    };
}


pub(crate) use item_stats;

pub struct ItemStats {
    pub name: ItemName,
    pub sprite_content: String,
}

impl ItemStats {
    pub fn new(name: ItemName, sprite_content: &'static [u8]) -> ItemStats {
        ItemStats {
            name,
            sprite_content: "data:image/png;base64,".to_string() + BASE64_STANDARD.encode(sprite_content).as_str(),
        }
    }
}