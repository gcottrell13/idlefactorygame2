use crate::types::runtime_state::ItemState;

pub struct ItemStats<T>{
    item: T,
    sprite_name: String,
    /// used to update the amount of this item at all times, even when there are no buildings
    on_tick: Option<Box<dyn FnMut(f32, &mut ItemState<T>)>>,
}