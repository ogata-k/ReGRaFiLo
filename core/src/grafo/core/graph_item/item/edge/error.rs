//! module for Edge item builder's error

use crate::grafo::core::graph_item::item::edge::EdgeItem;
use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::NameIdError;
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;

/// error for Edge item's builder
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EdgeItemError<Name: NameType> {
    /// not found belonging group by the name or not found root group
    FailResolveBelongGroup(ItemId, Option<Name>),
    /// fail resolve from specify start endpoint
    NotSpecifyStartEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    /// fail build start endpoint
    FailResolveStartEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    /// fail resolve from specify end endpoint
    NotSpecifyEndEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    /// fail build end endpoint
    FailResolveEndEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    /// error for name reference
    NameIdError(ItemId, NameIdError<Name, GraphItemKind>),
    /// cannot specify endpoint belonging self group or it's ancestor belong group
    CannotSpecifyBelongGroupAsEndpoint(ItemId, Name),
}

impl<Name: NameType> HasGraphItemKind for EdgeItemError<Name> {
    fn kind() -> GraphItemKind {
        EdgeItem::kind()
    }
}

impl<Name: NameType> std::fmt::Display for EdgeItemError<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeItemError::FailResolveBelongGroup(item_id, None) => {
                write!(f, "Edge {}: not specify belong group", item_id)
            }
            EdgeItemError::FailResolveBelongGroup(item_id, Some(name)) => {
                write!(f, "Edge {}: not found belong group \"{}\"", item_id, name)
            }
            EdgeItemError::NotSpecifyStartEndpoint(item_id, None) => {
                write!(f, "Edge {}: not specify start endpoint", item_id)
            }
            EdgeItemError::NotSpecifyStartEndpoint(item_id, Some((kind, name))) => write!(
                f,
                "Edge {}: not found {} \"{}\" as start endpoint",
                item_id,
                kind.to_string().to_lowercase(),
                name
            ),
            EdgeItemError::FailResolveStartEndpoint(item_id, None) => {
                write!(f, "Edge {}: not specify start endpoint", item_id)
            }
            EdgeItemError::FailResolveStartEndpoint(item_id, Some((kind, name))) => write!(
                f,
                "Edge {}: not found {} \"{}\" as start endpoint",
                item_id,
                kind.to_string().to_lowercase(),
                name
            ),
            EdgeItemError::NotSpecifyEndEndpoint(item_id, None) => {
                write!(f, "Edge {}: not specify end endpoint", item_id)
            }
            EdgeItemError::NotSpecifyEndEndpoint(item_id, Some((kind, name))) => write!(
                f,
                "Edge {}: not found {} item \"{}\" as end endpoint",
                item_id,
                kind.to_string().to_lowercase(),
                name
            ),
            EdgeItemError::FailResolveEndEndpoint(item_id, None) => {
                write!(f, "Edge {}: not specify end endpoint", item_id)
            }
            EdgeItemError::FailResolveEndEndpoint(item_id, Some((kind, name))) => write!(
                f,
                "Edge {}: not found {} \"{}\" as end endpoint",
                item_id,
                kind.to_string().to_lowercase(),
                name
            ),
            EdgeItemError::NameIdError(item_id, e) => write!(f, "Edge {}: {}", item_id, e),
            EdgeItemError::CannotSpecifyBelongGroupAsEndpoint(item_id, name) => write!(
                f,
                "Edge {}: cannot specify self belong group \"{}\" or it's ancestor belong group",
                item_id, name
            ),
        }
    }
}

impl<Name: NameType> Error for EdgeItemError<Name> {}
impl<Name: NameType> ItemErrorBase<Name> for EdgeItemError<Name> {}
impl<Name: NameType> FromWithItemId<NameIdError<Name, GraphItemKind>> for EdgeItemError<Name> {
    fn from_with_id(item_id: ItemId, from: NameIdError<Name, GraphItemKind>) -> Self {
        EdgeItemError::NameIdError(item_id, from)
    }
}
impl<Name: NameType> GraphBuilderErrorBase<Name> for EdgeItemError<Name> {}
