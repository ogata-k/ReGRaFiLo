//! module for Edge item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::GroupId;
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

/// Edge Item
#[derive(Debug, Clone)]
pub struct EdgeItem {
    // TODO
}

impl HasGraphItemKind for EdgeItem {
    fn kind() -> GraphItemKind {
        GraphItemKind::Edge
    }
}

impl ItemBase for EdgeItem {}

impl GraphItemBase for EdgeItem {
    fn get_group_id(&self) -> GroupId {
        unimplemented!()
    }
}
