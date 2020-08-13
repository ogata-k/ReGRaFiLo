//! module for Group item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GroupId, ItemId, DEFAULT_ITEM_ID};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

/// Group item
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct GroupItem {
    // todo
    belong_group_id: GroupId,
    item_id: ItemId,
}

impl HasGraphItemKind for GroupItem {
    fn kind() -> GraphItemKind {
        GraphItemKind::Group
    }
}

impl ItemBase for GroupItem {
    fn get_item_id(&self) -> ItemId {
        self.item_id
    }
}

impl GraphItemBase for GroupItem {
    fn get_belong_group_id(&self) -> GroupId {
        self.belong_group_id
    }
}

impl Default for GroupItem {
    fn default() -> Self {
        Self {
            belong_group_id: DEFAULT_ITEM_ID,
            item_id: DEFAULT_ITEM_ID,
        }
    }
}
