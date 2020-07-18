//! module for Edge item builder

use crate::grafo::core::item::edge::{EdgeItem, EdgeItemError};
use crate::grafo::core::item::{
    HasItemKind, ItemBuildResult, ItemBuilderBase, ItemBuilderBaseBuilderMethod,
};
use crate::grafo::core::layout::LayoutReference;
use crate::grafo::core::refindex::NameReference;
use crate::util::item_kind::ItemKind;

#[derive(Debug, Clone)]
pub struct EdgeItemBuilder {
    // TODO
}

impl HasItemKind for EdgeItemBuilder {
    fn kind() -> ItemKind {
        ItemKind::Edge
    }
}

impl ItemBuilderBase for EdgeItemBuilder {
    type Item = EdgeItem;
    type ItemOption = ();
    // TODO
    type BuildFailError = EdgeItemError;

    fn set_group_id(&mut self, group_id: usize) -> &mut Self {
        unimplemented!()
    }

    fn get_group_id(&self) -> usize {
        unimplemented!()
    }
}

impl ItemBuilderBaseBuilderMethod for EdgeItemBuilder {
    fn build(self) -> ItemBuildResult<Self::Item, Self::ItemOption, Self::BuildFailError> {
        unimplemented!()
    }
}
