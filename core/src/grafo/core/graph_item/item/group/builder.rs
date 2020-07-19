//! module for Group item builder

use crate::grafo::core::graph_item::group::{GroupItem, GroupItemError};
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::name_refindex::NameReference;
use crate::grafo::GrafoError;
use crate::util::alias::GroupId;
use crate::util::item_base::{HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

#[derive(Debug, Clone)]
pub struct GroupItemBuilder {
    // TODO
}

impl HasGraphItemKind for GroupItemBuilder {
    fn kind() -> GraphItemKind {
        GraphItemKind::Group
    }
}

impl ItemBuilderBase for GroupItemBuilder {
    type Item = GroupItem;
    // TODO
    type ItemOption = ();
    type BuilderError = GroupItemError;
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
    fn build(self, name_ref: &NameReference) -> ItemBuilderResult<Self::Item, Self::ItemOption> {
        unimplemented!()
    }
}

// TODO Test check kind eq
