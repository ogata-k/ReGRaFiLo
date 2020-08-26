//! module for Group item builder's error

use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::graph_item::group::GroupItem;
use crate::grafo::NameIdError;
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;

/// error for Group item's builder
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GroupItemError<Name: NameType> {
    /// specify belong group for root group, because root group is not belong to other group.
    CannotSpecifyBelongGroupForRoot(Name),
    /// not found belonging group by the name or not found root group
    FailResolveBelongGroup(ItemId, Option<Name>),
    /// error for name reference
    NameIdError(ItemId, NameIdError<Name, GraphItemKind>),
}

impl<Name: NameType> HasGraphItemKind for GroupItemError<Name> {
    fn kind() -> GraphItemKind {
        GroupItem::kind()
    }
}

impl<Name: NameType> std::fmt::Display for GroupItemError<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupItemError::CannotSpecifyBelongGroupForRoot(name) => {
                write!(f, "Root Group: cannot specify belong group \"{}\"", name)
            }
            GroupItemError::FailResolveBelongGroup(item_id, None) => {
                write!(f, "Group {}: not specify belong group", item_id)
            }
            GroupItemError::FailResolveBelongGroup(item_id, Some(name)) => {
                write!(f, "Group {}: not found belong group \"{}\"", item_id, name)
            }
            GroupItemError::NameIdError(item_id, e) => write!(f, "Group {}: {}", item_id, e),
        }
    }
}

impl<Name: NameType> Error for GroupItemError<Name> {}
impl<Name: NameType> ItemErrorBase<Name> for GroupItemError<Name> {}
impl<Name: NameType> FromWithItemId<NameIdError<Name, GraphItemKind>> for GroupItemError<Name> {
    fn from_with_id(item_id: ItemId, from: NameIdError<Name, GraphItemKind>) -> Self {
        Self::NameIdError(item_id, from)
    }
}
impl<Name: NameType> GraphBuilderErrorBase<Name> for GroupItemError<Name> {}
