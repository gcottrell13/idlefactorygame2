use crate::types::{ItemInfo, ItemName};
use leptos::prelude::*;

pub fn item_image_display(
    name: ItemName,
    item_info: ItemInfo,
) -> impl IntoView {
    view! { <img src=item_info.get(&name).unwrap().sprite_content.clone() /> }
}
