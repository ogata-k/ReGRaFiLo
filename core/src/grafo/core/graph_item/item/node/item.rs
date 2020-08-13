//! module for Node item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GroupId, ItemId};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

/// Node Item
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct NodeItem {
    belong_group_id: GroupId,
    item_id: ItemId,
}

impl HasGraphItemKind for NodeItem {
    fn kind() -> GraphItemKind {
        GraphItemKind::Node
    }
}

impl ItemBase for NodeItem {
    fn get_item_id(&self) -> ItemId {
        self.item_id
    }
}

impl GraphItemBase for NodeItem {
    fn get_belong_group_id(&self) -> GroupId {
        self.belong_group_id
    }
}

impl NodeItem {
    pub(crate) fn new(belong_group: GroupId, item_id: ItemId) -> Self {
        Self {
            belong_group_id: belong_group,
            item_id,
        }
    }
}
