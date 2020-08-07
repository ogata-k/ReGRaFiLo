//! module for Group item builder

use crate::grafo::core::graph_item::group::{GroupItem, GroupItemError};
use crate::grafo::core::graph_item::item::group::GroupItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::util::alias::ItemId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::name_type::NameType;

#[derive(Debug, Clone)]
pub struct GroupItemBuilder<Name: NameType> {
    // TODO
    belong_group: Option<Name>,
    name: Option<Name>,
}

impl<Name: NameType> ItemBuilderBase<Name> for GroupItemBuilder<Name> {
    type Item = GroupItem;
    type ItemError = GroupItemError<Name>;
}

impl<Name: NameType> GraphItemBuilderBase<Name> for GroupItemBuilder<Name> {
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self {
        unimplemented!()
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        unimplemented!()
    }
}

impl<Name: NameType> HasItemBuilderMethod<Name> for GroupItemBuilder<Name> {
    type ItemOption = GroupItemOption<Name>;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
    ) -> ItemBuilderResult<Name, Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

impl<Name: NameType> Default for GroupItemBuilder<Name> {
    fn default() -> Self {
        unimplemented!()
    }
}

impl<Name: NameType> GroupItemBuilder<Name> {
    pub fn new() -> Self {
        Self {
            belong_group: None,
            name: None,
        }
    }
}
