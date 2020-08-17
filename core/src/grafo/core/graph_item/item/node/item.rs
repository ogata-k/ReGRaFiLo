//! module for Node item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GroupId, ItemId};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::writer::WriteAsJson;

/// Node Item
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct NodeItem {
    belong_group_id: GroupId,
    item_id: ItemId,
}

impl WriteAsJson for NodeItem {
    fn write_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}}}",
            &self.get_kind(),
            &self.belong_group_id,
            &self.item_id
        )
    }
}

impl std::fmt::Display for NodeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node")?;
        self.write_as_json(f)
    }
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
