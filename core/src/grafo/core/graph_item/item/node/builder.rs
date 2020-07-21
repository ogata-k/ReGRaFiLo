//! module for Node builder

use crate::grafo::core::graph_item::node::{NodeItem, NodeItemError};
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::grafo::core::graph_item::item::node::NodeItemOption;

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
    type ItemOption = NodeItemOption;
    type BuilderError = NodeItemError;
}

impl GraphItemBuilderBase for NodeItemBuilder {
    fn set_belong_group<S: Into<String>>(&mut self, group: S) -> &mut Self {
        unimplemented!()
    }

    fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        unimplemented!()
    }
}

impl HasItemBuilderMethod for NodeItemBuilder {
    fn build(self, resolver: &Resolver) -> ItemBuilderResult<Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

// TODO Test check kind eq
