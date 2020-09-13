//! module for Node item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GroupId, ItemId};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::writer::DisplayAsJson;

/// Node Item
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NodeItem {
    belong_group_id: GroupId,
    item_id: ItemId,
    label: Option<String>,
}

impl DisplayAsJson for NodeItem {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}, \"label\": \"{}\"}}",
            &self.get_kind(),
            &self.belong_group_id,
            &self.item_id,
            self.label.as_deref().unwrap_or_else(|| ""),
        )
    }
}

impl std::fmt::Display for NodeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node")?;
        self.fmt_as_json(f)
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

    fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl NodeItem {
    /// initializer for Node item
    pub(crate) fn new(belong_group: GroupId, item_id: ItemId, label: Option<String>) -> Self {
        Self {
            belong_group_id: belong_group,
            item_id,
            label,
        }
    }
}
