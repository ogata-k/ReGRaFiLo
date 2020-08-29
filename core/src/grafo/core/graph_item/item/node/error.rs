//! module for Node item builder's error

use crate::grafo::graph_item::node::NodeItem;
use crate::grafo::graph_item::GraphBuilderErrorBase;
use crate::grafo::{NameIdError, ResolverError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;

/// error for Node item's builder
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NodeItemError<Name: NameType> {
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

impl<Name: NameType> HasGraphItemKind for NodeItemError<Name> {
    fn kind() -> GraphItemKind {
        NodeItem::kind()
    }
}

impl<Name: NameType> std::fmt::Display for NodeItemError<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeItemError::FailResolveBelongGroup(_, _, None) => {
                self.fmt_header(f)?;
                write!(f, "not specify belong group")
            }
            NodeItemError::FailResolveBelongGroup(_, _, Some(name)) => {
                self.fmt_header(f)?;
                write!(f, "not found belong group \"{}\"", name)
            }
            NodeItemError::NameIdError(_, _, e) => {
                self.fmt_header(f)?;
                write!(f, "{}", e)
            }
            NodeItemError::ResolverError(_, _, e) => {
                self.fmt_header(f)?;
                write!(f, "{}", e)
            }
        }
    }
}

impl<Name: NameType> Error for NodeItemError<Name> {}

impl<Name: NameType> ItemErrorBase<Name> for NodeItemError<Name> {}

impl<Name: NameType> FromWithItemId<NameIdError<Name, GraphItemKind>, Name>
    for NodeItemError<Name>
{
    fn from_with_id(
        item_id: ItemId,
        name: Option<Name>,
        from: NameIdError<Name, GraphItemKind>,
    ) -> Self {
        NodeItemError::NameIdError(item_id, name, from)
    }
}

impl<Name: NameType> FromWithItemId<ResolverError, Name> for NodeItemError<Name> {
    fn from_with_id(item_id: ItemId, name: Option<Name>, from: ResolverError) -> Self {
        NodeItemError::ResolverError(item_id, name, from)
    }
}

impl<Name: NameType> GraphBuilderErrorBase<Name> for NodeItemError<Name> {
    fn get_item_id(&self) -> &ItemId {
        match &self {
            NodeItemError::FailResolveBelongGroup(i, _, _) => i,
            NodeItemError::NameIdError(i, _, _) => i,
            NodeItemError::ResolverError(i, _, _) => i,
        }
    }

    fn get_item_name(&self) -> &Option<Name> {
        match self {
            NodeItemError::FailResolveBelongGroup(_, name, _) => name,
            NodeItemError::NameIdError(_, name, _) => name,
            NodeItemError::ResolverError(_, name, _) => name,
        }
    }
}
