//! module for Node item

use crate::grafo::core::item::{HasItemKind, ItemBase};
use crate::util::item_kind::ItemKind;

/// Node Item
#[derive(Debug, Clone)]
pub struct NodeItem {
    // TODO
}

impl HasItemKind for NodeItem {
    fn kind() -> ItemKind {
        ItemKind::Node
    }
}

impl ItemBase for NodeItem {
    fn get_group_id(&self) -> usize {
        unimplemented!()
    }

    fn get_item_id(&self) -> usize {
        unimplemented!()
    }
}
