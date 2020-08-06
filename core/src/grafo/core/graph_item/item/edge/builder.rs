//! module for Edge item builder

use crate::grafo::core::graph_item::edge::{EdgeItem, EdgeItemError};
use crate::grafo::core::graph_item::item::edge::EdgeItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::util::alias::ItemId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::name_type::{NameType, StoredNameType};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct EdgeItemBuilder<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    // TODO
    stored_name: PhantomData<StoredName>,
    belong_group: Option<Name>,
    name: Option<Name>,
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> ItemBuilderBase<Name, StoredName>
    for EdgeItemBuilder<Name, StoredName>
{
    type Item = EdgeItem;
    type ItemError = EdgeItemError<Name, StoredName>;
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    GraphItemBuilderBase<Name, StoredName> for EdgeItemBuilder<Name, StoredName>
{
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self {
        unimplemented!()
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    HasItemBuilderMethod<Name, StoredName> for EdgeItemBuilder<Name, StoredName>
{
    type ItemOption = EdgeItemOption<Name, StoredName>;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name, StoredName>,
    ) -> ItemBuilderResult<Name, StoredName, Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>
    EdgeItemBuilder<Name, StoredName>
{
    pub fn new() -> Self {
        Self {
            stored_name: PhantomData,
            belong_group: None,
            name: None,
        }
    }
}
