//! module for Edge item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GraphItemId, GroupId};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

/// Edge Item
#[derive(Debug, Clone)]
pub struct EdgeItem {
    // TODO Align can use RelativeAlign and AbsoluteAlign
    belong_group_id: GroupId,
    start: (GraphItemKind, GraphItemId),
    end: (GraphItemKind, GraphItemId),
}

impl HasGraphItemKind for EdgeItem {
    fn get_kind(&self) -> GraphItemKind {
        GraphItemKind::Edge
    }
}

impl ItemBase for EdgeItem {}

impl GraphItemBase for EdgeItem {
    fn get_belong_group_id(&self) -> GroupId {
        self.belong_group_id
    }
}

impl EdgeItem {
    pub(crate) fn get_start_item_kind_id(&self) -> (GraphItemKind, GraphItemId) {
        self.start
    }

    pub(crate) fn get_end_item_kind_id(&self) -> (GraphItemKind, GraphItemId) {
        self.end
    }
}
