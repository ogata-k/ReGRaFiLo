//! module for Node builder

use crate::grafo::core::graph_item::node::{NodeItem, NodeItemError};
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::name_refindex::NameReference;
use crate::util::alias::GroupId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

#[derive(Debug, Clone)]
pub struct NodeItemBuilder {
    // TODO
}

impl HasGraphItemKind for NodeItemBuilder {
    fn kind() -> GraphItemKind {
        GraphItemKind::Node
    }
}

impl ItemBuilderBase for NodeItemBuilder {
    type Item = NodeItem;
    // TODO
    type ItemOption = ();
    type BuildFailError = NodeItemError;
}

impl GraphItemBuilderBase for NodeItemBuilder {
    fn set_group_id(&mut self, group_id: GroupId) -> &mut Self {
        unimplemented!()
    }

    fn get_group_id(&self) -> GroupId {
        unimplemented!()
    }
}

impl HasItemBuilderMethod for NodeItemBuilder {
    fn build(
        self,
        name_ref: &NameReference,
    ) -> ItemBuilderResult<Self::Item, Self::ItemOption, Self::BuildFailError> {
        unimplemented!()
    }
}

// TODO Test check kind eq
