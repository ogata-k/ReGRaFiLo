//! module for Group item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GroupId, ItemId, DEFAULT_ITEM_ID};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::writer::DisplayAsJson;

/// Group item
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct GroupItem {
    belong_group_id: GroupId,
    item_id: ItemId,
}

impl DisplayAsJson for GroupItem {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}}}",
            &self.get_kind(),
            &self.belong_group_id,
            &self.item_id
        )
    }
}

impl std::fmt::Display for GroupItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group")?;
        self.fmt_as_json(f)
    }
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

impl GroupItem {
    pub(crate) fn new(belong_group: GroupId, item_id: ItemId) -> Self {
        Self {
            belong_group_id: belong_group,
            item_id,
        }
    }
}
