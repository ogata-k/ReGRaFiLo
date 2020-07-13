//! module for Group item

use crate::grafo::core::item::{HasItemKind, ItemBase};
use crate::util::item_kind::ItemKind;

/// Group item
#[derive(Debug, Clone)]
pub struct GroupItem {
    // todo
}

impl HasItemKind for GroupItem {
    fn kind() -> ItemKind {
        ItemKind::Group
    }
}

impl ItemBase for GroupItem {
    fn get_group_id(&self) -> usize {
        unimplemented!()
    }

    fn get_item_id(&self) -> usize {
        unimplemented!()
    }
}
