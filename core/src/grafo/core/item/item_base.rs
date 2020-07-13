//! base of item and item builder

use std::error::Error;

use crate::grafo::core::layout::LayoutReference;
use crate::grafo::GrafoError;
use crate::util::alias::GroupIndex;
use crate::util::item_kind::ItemKind;

pub trait HasItemKind {
    fn kind() -> ItemKind;
}

/// Item Builder's base set
pub trait ItemBuilderBase: HasItemKind {
    type Item: ItemBase;
    type ItemOption;
    type BuildFailError: ItemErrorBase;

    fn set_group_id(&mut self, group_id: GroupIndex) -> &mut Self;
    fn get_group_id(&self) -> GroupIndex;
}

pub(crate) trait ItemBuilderBaseBuilderMethod: ItemBuilderBase {
    fn build(
        self,
        layout: &LayoutReference,
    ) -> Result<(Self::Item, Self::ItemOption), Vec<Self::BuildFailError>>;
}

/// Item's base set
pub trait ItemBase: HasItemKind {
    fn get_group_id(&self) -> GroupIndex;
    fn get_item_id(&self) -> GroupIndex;
}

pub trait ItemErrorBase: HasItemKind + Error + Into<GrafoError> {}
