use crate::grafo::core::graph_item::item::edge::EdgeItem;
use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::NameIdError;
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EdgeItemError<Name: NameType> {
    FailResolveBelongGroup(ItemId, Option<Name>),
    NotSpecifyStartEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    FailResolveStartEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    NotSpecifyEndEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    FailResolveEndEndpoint(ItemId, Option<(GraphItemKind, Name)>),
    NameIdError(ItemId, NameIdError<Name, GraphItemKind>),
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
