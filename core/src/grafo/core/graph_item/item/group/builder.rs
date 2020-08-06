//! module for Group item builder

use crate::grafo::core::graph_item::group::{GroupItem, GroupItemError};
use crate::grafo::core::graph_item::item::group::GroupItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::util::alias::ItemId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::name_type::{NameType, StoredNameType};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GroupItemBuilder<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    // TODO
    stored_name: PhantomData<StoredName>,
    belong_group: Option<Name>,
    name: Option<Name>,
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> ItemBuilderBase<Name, StoredName>
    for GroupItemBuilder<Name, StoredName>
{
    type Item = GroupItem;
    type ItemError = GroupItemError<Name, StoredName>;
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    GraphItemBuilderBase<Name, StoredName> for GroupItemBuilder<Name, StoredName>
{
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self {
        unimplemented!()
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    HasItemBuilderMethod<Name, StoredName> for GroupItemBuilder<Name, StoredName>
{
    type ItemOption = GroupItemOption<Name, StoredName>;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name, StoredName>,
    ) -> ItemBuilderResult<Name, StoredName, Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Default
    for GroupItemBuilder<Name, StoredName>
{
    fn default() -> Self {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    GroupItemBuilder<Name, StoredName>
{
    pub fn new() -> Self {
        Self {
            stored_name: PhantomData,
            belong_group: None,
            name: None,
        }
    }
}
