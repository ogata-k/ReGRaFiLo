//! module for Group item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::GroupId;
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

/// Group item
#[derive(Debug, Clone)]
pub struct GroupItem {
    // todo
}

impl HasGraphItemKind for GroupItem {
    fn kind() -> GraphItemKind {
        GraphItemKind::Group
    }
}

impl ItemBase for GroupItem {}

impl GraphItemBase for GroupItem {
    fn get_belong_group_id(&self) -> GroupId {
        unimplemented!()
    }
}

impl Default for GroupItem {
    fn default() -> Self {
        unimplemented!()
    }
}
