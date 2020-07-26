//! module for Edge item builder

use crate::grafo::core::graph_item::edge::{EdgeItem, EdgeItemError};
use crate::grafo::core::graph_item::item::edge::EdgeItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};

#[derive(Debug, Clone)]
pub struct EdgeItemBuilder {
    // TODO
}

impl ItemBuilderBase for EdgeItemBuilder {
    type Item = EdgeItem;
    type ItemError = EdgeItemError;
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
    type ItemOption = EdgeItemOption;
    fn build(self, resolver: &Resolver) -> ItemBuilderResult<Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

impl EdgeItemBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
