//! module for Node item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::GroupId;
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

/// Node Item
#[derive(Debug, Clone)]
pub struct NodeItem {
    belong_group_id: GroupId,
}

impl HasGraphItemKind for NodeItem {
    fn kind() -> GraphItemKind {
        GraphItemKind::Node
    }
}

impl ItemBase for NodeItem {}

impl GraphItemBase for NodeItem {
    fn get_belong_group_id(&self) -> GroupId {
        self.belong_group_id
    }
}

impl NodeItem {
    pub(crate) fn new(belong_group: GroupId) -> Self {
        Self {
            belong_group_id: belong_group,
        }
    }
}
