//! base of item and item builder

use crate::grafo::NameIdError;
use crate::util::alias::GroupId;
use crate::util::item_base::{FromWithItemId, ItemBase, ItemBuilderBase, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::{NameType, StoredNameType};

/// Item Builder's base set
pub trait GraphItemBuilderBase<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>:
    ItemBuilderBase<Name, StoredName>
{
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self;
    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self;
}

/// Item's base set
pub trait GraphItemBase: ItemBase + HasGraphItemKind {
    fn get_belong_group_id(&self) -> GroupId;
}

pub trait GraphBuilderErrorBase<Name: NameType<StoredName>, StoredName: StoredNameType<Name>>:
    ItemErrorBase<Name, StoredName>
    + HasGraphItemKind
    + FromWithItemId<NameIdError<Name, StoredName, GraphItemKind>>
{
}
