mod recipe;
mod item_stats;
mod runtime_state_1;
mod runtime_state_0;

use std::str::FromStr;

pub(crate) use recipe::{*};
pub(crate) use item_stats::{*};
pub(crate) use runtime_state_0::{*};
pub(crate) use runtime_state_1::{*};
pub(crate) use RuntimeState1 as RuntimeState;

impl Into<RuntimeState> for RuntimeState0 {
    fn into(self) -> RuntimeState {
        RuntimeState::default(None)
    }
}