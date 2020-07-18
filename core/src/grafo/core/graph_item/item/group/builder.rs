//! module for Group item builder

use crate::grafo::core::graph_item::group::{GroupItem, GroupItemError};
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::name_refindex::NameReference;
use crate::util::alias::GroupId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

#[derive(Debug, Clone)]
pub struct GroupItemBuilder {
    // TODO
}

impl HasGraphItemKind for GroupItemBuilder {
    fn kind() -> GraphItemKind {
        GraphItemKind::Group
    }
}

impl ItemBuilderBase for GroupItemBuilder {
    type Item = GroupItem;
    // TODO
    type ItemOption = ();
    type BuildFailError = GroupItemError;
}

impl GraphItemBuilderBase for GroupItemBuilder {
    fn set_group_id(&mut self, group_id: GroupId) -> &mut Self {
        unimplemented!()
    }

    fn get_group_id(&self) -> GroupId {
        unimplemented!()
    }
}

impl HasItemBuilderMethod for GroupItemBuilder {
    fn build(
        self,
        name_ref: &NameReference,
    ) -> ItemBuilderResult<Self::Item, Self::ItemOption, Self::BuildFailError> {
        unimplemented!()
    }
}

// TODO Test check kind eq
