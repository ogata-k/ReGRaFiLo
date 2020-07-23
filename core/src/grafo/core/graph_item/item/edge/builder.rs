//! module for Edge item builder

use crate::grafo::core::graph_item::edge::{EdgeItem, EdgeItemError};
use crate::grafo::core::graph_item::item::edge::EdgeItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
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
    type ItemOption = EdgeItemOption;
    type BuilderError = EdgeItemError;
}

impl GraphItemBuilderBase for EdgeItemBuilder {
    fn set_belong_group<S: Into<String>>(&mut self, group: S) -> &mut Self {
        unimplemented!()
    }

    fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        unimplemented!()
    }
}

impl HasItemBuilderMethod for EdgeItemBuilder {
    fn build(self, resolver: &Resolver) -> ItemBuilderResult<Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

// TODO Test check kind eq
