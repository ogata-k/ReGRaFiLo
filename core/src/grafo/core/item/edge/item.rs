//! module for Edge item

use crate::grafo::core::item::{HasItemKind, ItemBase};
use crate::util::item_kind::ItemKind;

/// Edge Item
#[derive(Debug, Clone)]
pub struct EdgeItem {
    // TODO
}

impl HasItemKind for EdgeItem {
    fn kind() -> ItemKind {
        ItemKind::Edge
    }
}

impl ItemBase for EdgeItem {
    fn get_group_id(&self) -> usize {
        unimplemented!()
    }

    fn get_item_id(&self) -> usize {
        unimplemented!()
    }
}
