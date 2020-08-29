//! module for Group item builder's error

use std::error::Error;

use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::graph_item::group::GroupItem;
use crate::grafo::{NameIdError, ResolverError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;

/// error for Group item's builder
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum GroupItemError<Name: NameType> {
    /// specify belong group for root group, because root group is not belong to other group.
    /// Argument is (item_id, item's name, belong group's name)
    CannotSpecifyBelongGroupForRoot(ItemId, Option<Name>, Name),
    /// not found belonging group by the name or not found root group.
    /// Argument is (item_id, item's name, belong group's name)
    FailResolveBelongGroup(ItemId, Option<Name>, Option<Name>),
    /// error for name reference.
    /// Argument is (item_id, item's name, error of NameIdError for Group item)
    NameIdError(ItemId, Option<Name>, NameIdError<Name, GraphItemKind>),
    /// error for resolver.
    /// Argument is (item_id, item's name, error of ResolverError for Group item)
    ResolverError(ItemId, Option<Name>, ResolverError),
}

impl<Name: NameType> HasGraphItemKind for GroupItemError<Name> {
    fn kind() -> GraphItemKind {
        GroupItem::kind()
    }
}

impl<Name: NameType> std::fmt::Display for GroupItemError<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupItemError::CannotSpecifyBelongGroupForRoot(_, _, name) => {
                self.fmt_header(f)?;
                write!(
                    f,
                    "this Root Group cannot specify belong group \"{}\"",
                    name
                )
            }
            GroupItemError::FailResolveBelongGroup(_, _, None) => {
                self.fmt_header(f)?;
                write!(f, "not specify belong group")
            }
            GroupItemError::FailResolveBelongGroup(_, _, Some(name)) => {
                self.fmt_header(f)?;
                write!(f, "not found belong group \"{}\"", name)
            }
            GroupItemError::NameIdError(_, _, e) => {
                self.fmt_header(f)?;
                write!(f, "{}", e)
            }
            GroupItemError::ResolverError(_, _, e) => {
                self.fmt_header(f)?;
                write!(f, "{}", e)
            }
        }
    }
}

impl<Name: NameType> Error for GroupItemError<Name> {}

impl<Name: NameType> ItemErrorBase<Name> for GroupItemError<Name> {}

impl<Name: NameType> FromWithItemId<NameIdError<Name, GraphItemKind>, Name>
    for GroupItemError<Name>
{
    fn from_with_id(
        item_id: ItemId,
        name: Option<Name>,
        from: NameIdError<Name, GraphItemKind>,
    ) -> Self {
        Self::NameIdError(item_id, name, from)
    }
}

impl<Name: NameType> FromWithItemId<ResolverError, Name> for GroupItemError<Name> {
    fn from_with_id(item_id: ItemId, name: Option<Name>, from: ResolverError) -> Self {
        Self::ResolverError(item_id, name, from)
    }
}

impl<Name: NameType> GraphBuilderErrorBase<Name> for GroupItemError<Name> {
    fn get_item_id(&self) -> &ItemId {
        match self {
            GroupItemError::CannotSpecifyBelongGroupForRoot(i, _, _) => i,
            GroupItemError::FailResolveBelongGroup(i, _, _) => i,
            GroupItemError::NameIdError(i, _, _) => i,
            GroupItemError::ResolverError(i, _, _) => i,
        }
    }

    fn get_item_name(&self) -> &Option<Name> {
        match self {
            GroupItemError::CannotSpecifyBelongGroupForRoot(_, name, _) => name,
            GroupItemError::FailResolveBelongGroup(_, name, _) => name,
            GroupItemError::NameIdError(_, name, _) => name,
            GroupItemError::ResolverError(_, name, _) => name,
        }
    }
}
