//! module for Group item builder

use crate::grafo::core::graph_item::group::{GroupItem, GroupItemError};
use crate::grafo::core::graph_item::item::group::GroupItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::util::alias::ItemId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};

#[derive(Debug, Clone)]
pub struct GroupItemBuilder {
    // TODO
}

impl ItemBuilderBase for GroupItemBuilder {
    type Item = GroupItem;
    type ItemError = GroupItemError;
}

impl GraphItemBuilderBase for GroupItemBuilder {
    fn set_belong_group<S: Into<String>>(&mut self, group: S) -> &mut Self {
        unimplemented!()
    }

    fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        unimplemented!()
    }
}

impl HasItemBuilderMethod for GroupItemBuilder {
    type ItemOption = GroupItemOption;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver,
    ) -> ItemBuilderResult<Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

impl Default for GroupItemBuilder {
    fn default() -> Self {
        unimplemented!()
    }
}

impl GroupItemBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
