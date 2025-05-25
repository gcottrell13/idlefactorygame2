use serde::{Deserialize, Serialize};

///
/// Keeps the entire runtime state.
/// de/serialize to save and load from storage.
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeState0 {}

impl RuntimeState0 {
    pub fn default(_err: serde_json::Error) -> Self {
        RuntimeState0 {}
    }
}
