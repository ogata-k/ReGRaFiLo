//! base of item and item builder

use std::error::Error;

use crate::grafo::core::refindex::NameReference;
use crate::grafo::GrafoError;
use crate::util::alias::GroupId;
use crate::util::item_kind::ItemKind;

pub trait HasItemKind {
    fn kind() -> ItemKind;
}

/// Item Builder's base set
pub trait ItemBuilderBase: HasItemKind {
    type Item: ItemBase;
    type ItemOption;
    type BuildFailError: ItemErrorBase;

    fn set_group_id(&mut self, group_id: GroupId) -> &mut Self;
    fn get_group_id(&self) -> GroupId;
}

pub(crate) type ItemBuildResult<Item, Option, BuildError> = Result<(Item, Option), Vec<BuildError>>;
pub(crate) trait ItemBuilderBaseBuilderMethod: ItemBuilderBase {
    fn build(self) -> ItemBuildResult<Self::Item, Self::ItemOption, Self::BuildFailError>;
}

/// Item's base set
pub trait ItemBase: HasItemKind {
    fn get_group_id(&self) -> GroupId;
    fn get_item_id(&self) -> GroupId;
}

pub trait ItemErrorBase: HasItemKind + Error + Into<GrafoError> {}
