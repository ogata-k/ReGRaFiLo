//! module for Group item builder

use crate::grafo::core::item::group::{GroupItem, GroupItemError};
use crate::grafo::core::item::{HasItemKind, ItemBuilderBase, ItemBuilderBaseBuilderMethod};
use crate::grafo::core::layout::LayoutReference;
use crate::grafo::core::refindex::NameReference;
use crate::util::item_kind::ItemKind;

#[derive(Debug, Clone)]
pub struct GroupItemBuilder {
    // TODO
}

impl HasItemKind for GroupItemBuilder {
    fn kind() -> ItemKind {
        ItemKind::Group
    }
}

impl ItemBuilderBase for GroupItemBuilder {
    type Item = GroupItem;
    type ItemOption = ();
    // TODO
    type BuildFailError = GroupItemError;

    fn set_group_id(&mut self, group_id: usize) -> &mut Self {
        unimplemented!()
    }

    fn get_group_id(&self) -> usize {
        unimplemented!()
    }
}

impl ItemBuilderBaseBuilderMethod for GroupItemBuilder {
    fn build(
        self,
        layout: &NameReference,
    ) -> Result<(Self::Item, Self::ItemOption), Vec<Self::BuildFailError>> {
        unimplemented!()
    }
}
