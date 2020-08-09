//! module for Edge item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GroupId, ItemId};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

/// Edge Item
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EdgeItem {
    // TODO Align can use RelativeAlign and AbsoluteAlign
    belong_group_id: GroupId,
    item_id: ItemId,
    start: (GraphItemKind, ItemId),
    end: (GraphItemKind, ItemId),
}

impl HasGraphItemKind for EdgeItem {
    fn kind() -> GraphItemKind {
        GraphItemKind::Edge
    }
}

impl ItemBase for EdgeItem {
    fn get_item_id(&self) -> ItemId {
        self.item_id
    }
}

impl GraphItemBase for EdgeItem {
    fn get_belong_group_id(&self) -> GroupId {
        self.belong_group_id
    }
}

impl EdgeItem {
    pub(crate) fn get_start_item_kind_id(&self) -> (GraphItemKind, ItemId) {
        self.start
    }

    pub(crate) fn get_end_item_kind_id(&self) -> (GraphItemKind, ItemId) {
        self.end
    }
}
