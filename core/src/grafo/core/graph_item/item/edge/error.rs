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
                write!(f, "not specify belong group for edge {}", item_id)
            }
            EdgeItemError::FailResolveBelongGroup(item_id, Some(name)) => write!(
                f,
                "not found belong group \"{}\" for edge {}",
                name, item_id
            ),
            EdgeItemError::NotSpecifyStartEndpoint(item_id, None) => {
                write!(f, "not specify start endpoint for edge {}", item_id)
            }
            EdgeItemError::NotSpecifyStartEndpoint(item_id, Some((kind, name))) => write!(
                f,
                "not found {} \"{}\" as start endpoint for edge {}",
                kind.to_string().to_lowercase(),
                name,
                item_id
            ),
            EdgeItemError::FailResolveStartEndpoint(item_id, None) => {
                write!(f, "not specify start endpoint for edge {}", item_id)
            }
            EdgeItemError::FailResolveStartEndpoint(item_id, Some((kind, name))) => write!(
                f,
                "not found {} \"{}\" as start endpoint for edge {}",
                kind.to_string().to_lowercase(),
                name,
                item_id
            ),
            EdgeItemError::NotSpecifyEndEndpoint(item_id, None) => {
                write!(f, "not specify end endpoint  for edge {}", item_id)
            }
            EdgeItemError::NotSpecifyEndEndpoint(item_id, Some((kind, name))) => write!(
                f,
                "not found {} item \"{}\" as end endpoint for edge {}",
                kind.to_string().to_lowercase(),
                name,
                item_id
            ),
            EdgeItemError::FailResolveEndEndpoint(item_id, None) => {
                write!(f, "not specify end endpoint for edge {}", item_id)
            }
            EdgeItemError::FailResolveEndEndpoint(item_id, Some((kind, name))) => {
                write!(f,
                       "not found {} \"{}\" as end endpoint for edge {}",
                       kind.to_string().to_lowercase(),
                       name,
                       item_id)
            }
            EdgeItemError::NameIdError(item_id, e) => write!(f, "{} for edge {}", e, item_id),
            EdgeItemError::CannotSpecifyBelongGroupAsEndpoint(item_id, name) => write!(
                f,
                "cannot specify same belong group \"{}\" for edge {}",
                name, item_id
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
