//! module for Node builder

use crate::grafo::core::item::node::{NodeItem, NodeItemError};
use crate::grafo::core::item::{HasItemKind, ItemBuilderBase, ItemBuilderBaseBuilderMethod};
use crate::grafo::core::refindex::NameReference;
use crate::util::item_kind::ItemKind;

#[derive(Debug, Clone)]
pub struct NodeItemBuilder {
    // TODO
}

impl HasItemKind for NodeItemBuilder {
    fn kind() -> ItemKind {
        ItemKind::Node
    }
}

impl ItemBuilderBase for NodeItemBuilder {
    type Item = NodeItem;
    type ItemOption = ();
    // TODO
    type BuildFailError = NodeItemError;

    fn set_group_id(&mut self, group_id: usize) -> &mut Self {
        unimplemented!()
    }

    fn get_group_id(&self) -> usize {
        unimplemented!()
    }
}

impl ItemBuilderBaseBuilderMethod for NodeItemBuilder {
    fn build(
        self,
        name_ref: &NameReference,
    ) -> Result<(Self::Item, Self::ItemOption), Vec<Self::BuildFailError>> {
        unimplemented!()
    }
}
