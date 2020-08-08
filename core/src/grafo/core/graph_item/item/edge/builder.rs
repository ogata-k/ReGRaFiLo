//! module for Edge item builder

use crate::grafo::core::graph_item::edge::{EdgeItem, EdgeItemError};
use crate::grafo::core::graph_item::item::edge::EdgeItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::util::alias::ItemId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::name_type::NameType;

#[derive(Debug, Clone)]
pub struct EdgeItemBuilder<Name: NameType> {
    // TODO
    belong_group: Option<Name>,
    name: Option<Name>,
}

impl<Name: NameType> ItemBuilderBase<Name> for EdgeItemBuilder<Name> {
    type Item = EdgeItem;
    type ItemError = EdgeItemError<Name>;
}

impl<Name: NameType> GraphItemBuilderBase<Name> for EdgeItemBuilder<Name> {
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self {
        unimplemented!()
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        unimplemented!()
    }
}

impl<Name: NameType> HasItemBuilderMethod<Name> for EdgeItemBuilder<Name> {
    type ItemOption = EdgeItemOption<Name>;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
    ) -> ItemBuilderResult<Name, Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

impl<Name: NameType> EdgeItemBuilder<Name> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            belong_group: None,
            name: None,
        }
    }
}
