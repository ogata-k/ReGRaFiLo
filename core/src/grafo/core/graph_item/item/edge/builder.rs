//! module for Edge item builder

use crate::grafo::core::graph_item::edge::{EdgeItem, EdgeItemError};
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::name_refindex::NameReference;
use crate::util::alias::GroupId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

#[derive(Debug, Clone)]
pub struct EdgeItemBuilder {
    // TODO
}

impl HasGraphItemKind for EdgeItemBuilder {
    fn kind() -> GraphItemKind {
        GraphItemKind::Edge
    }
}

impl ItemBuilderBase for EdgeItemBuilder {
    type Item = EdgeItem;
    // TODO
    type ItemOption = ();
    type BuildFailError = EdgeItemError;
}

impl GraphItemBuilderBase for EdgeItemBuilder {
    fn set_group_id(&mut self, group_id: GroupId) -> &mut Self {
        unimplemented!()
    }

    fn get_group_id(&self) -> GroupId {
        unimplemented!()
    }
}

impl HasItemBuilderMethod for EdgeItemBuilder {
    fn build(
        self,
        name_ref: &NameReference,
    ) -> ItemBuilderResult<Self::Item, Self::ItemOption, Self::BuildFailError> {
        unimplemented!()
    }
}

// TODO Test check kind eq
