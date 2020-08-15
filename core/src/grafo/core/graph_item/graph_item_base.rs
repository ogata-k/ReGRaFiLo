//! base of item and item builder

use crate::grafo::NameIdError;
use crate::util::alias::GroupId;
use crate::util::item_base::{FromWithItemId, ItemBase, ItemBuilderBase, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;

/// Item Builder's base set
pub trait GraphItemBuilderBase<Name: NameType>: ItemBuilderBase<Name> {
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self;
    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self;
}

/// Item's base set
pub trait GraphItemBase: Copy + ItemBase + HasGraphItemKind {
    fn get_belong_group_id(&self) -> GroupId;
}

pub trait GraphBuilderErrorBase<Name: NameType>:
    ItemErrorBase<Name> + HasGraphItemKind + FromWithItemId<NameIdError<Name, GraphItemKind>>
{
}
