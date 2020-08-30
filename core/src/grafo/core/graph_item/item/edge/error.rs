//! module for Edge item builder's error

use crate::grafo::core::graph_item::item::edge::EdgeItem;
use crate::grafo::core::graph_item::GraphBuilderErrorBase;
use crate::grafo::{NameIdError, ResolverError};
use crate::util::alias::ItemId;
use crate::util::item_base::{FromWithItemId, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::error::Error;

/// error for Edge item's builder
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EdgeItemError<Name: NameType> {
    /// not found belonging group by the name or not found root group. Argument is (item_id, item's name, belong group's name)
    FailResolveBelongGroup(ItemId, Option<Name>, Option<Name>),
    /// fail resolve from specify start endpoint. Argument is (item_id, item's name, start endpoint's kind and name)
    NotSpecifyStartEndpoint(ItemId, Option<Name>, Option<(GraphItemKind, Name)>),
    /// fail build start endpoint. Argument is (item_id, item's name, start endpoint's kind and name)
    FailResolveStartEndpoint(ItemId, Option<Name>, Option<(GraphItemKind, Name)>),
    /// fail resolve from specify end endpoint. Argument is (item_id, item's name, end endpoint's kind and name)
    NotSpecifyEndEndpoint(ItemId, Option<Name>, Option<(GraphItemKind, Name)>),
    /// fail build end endpoint. Argument is (item_id, item's name, end endpoint's kind and name)
    FailResolveEndEndpoint(ItemId, Option<Name>, Option<(GraphItemKind, Name)>),
    /// cannot specify endpoint belonging self group or it's ancestor belong group.
    /// Argument is (item_id, item's name, belong group's name for endpoint)
    CannotSpecifyBelongGroupAsEndpoint(ItemId, Option<Name>, Name),
    /// this error occurred when item's belong group is not endpoint's belong group or group endpoint.
    /// Argument is (item_id, item's name, belong group's name for item)
    InappropriateGroup(ItemId, Option<Name>, Option<Name>),
    /// error for name reference. Argument is (item_id, item's name, error of NameIdError for Edge item)
    NameIdError(ItemId, Option<Name>, NameIdError<Name, GraphItemKind>),
    /// error for resolver. Argument is (item_id, item's name, error of ResolverError for Edge item)
    ResolverError(ItemId, Option<Name>, ResolverError),
}

impl<Name: NameType> HasGraphItemKind for EdgeItemError<Name> {
    fn kind() -> GraphItemKind {
        EdgeItem::kind()
    }
}

impl<Name: NameType> std::fmt::Display for EdgeItemError<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeItemError::FailResolveBelongGroup(_, _, None) => {
                self.fmt_header(f)?;
                write!(f, "not specify belong group")
            }
            EdgeItemError::FailResolveBelongGroup(_, _, Some(name)) => {
                self.fmt_header(f)?;
                write!(f, "not found belong group \"{}\"", name)
            }
            EdgeItemError::NotSpecifyStartEndpoint(_, _, None) => {
                self.fmt_header(f)?;
                write!(f, "not specify start endpoint")
            }
            EdgeItemError::NotSpecifyStartEndpoint(_, _, Some((kind, name))) => {
                self.fmt_header(f)?;
                write!(
                    f,
                    "not found {} \"{}\" as start endpoint",
                    kind.to_string().to_lowercase(),
                    name
                )
            }
            EdgeItemError::FailResolveStartEndpoint(_, _, None) => {
                self.fmt_header(f)?;
                write!(f, "not specify start endpoint")
            }
            EdgeItemError::FailResolveStartEndpoint(_, _, Some((kind, name))) => {
                self.fmt_header(f)?;
                write!(
                    f,
                    "not found {} \"{}\" as start endpoint",
                    kind.to_string().to_lowercase(),
                    name
                )
            }
            EdgeItemError::NotSpecifyEndEndpoint(_, _, None) => {
                self.fmt_header(f)?;
                write!(f, "not specify end endpoint")
            }
            EdgeItemError::NotSpecifyEndEndpoint(_, _, Some((kind, name))) => {
                self.fmt_header(f)?;
                write!(
                    f,
                    "not found {} item \"{}\" as end endpoint",
                    kind.to_string().to_lowercase(),
                    name
                )
            }
            EdgeItemError::FailResolveEndEndpoint(_, _, None) => {
                self.fmt_header(f)?;
                write!(f, "not specify end endpoint")
            }
            EdgeItemError::FailResolveEndEndpoint(_, _, Some((kind, name))) => {
                self.fmt_header(f)?;
                write!(
                    f,
                    "not found {} \"{}\" as end endpoint",
                    kind.to_string().to_lowercase(),
                    name
                )
            }
            EdgeItemError::CannotSpecifyBelongGroupAsEndpoint(_, _, name) => {
                self.fmt_header(f)?;
                write!(
                    f,
                    "cannot specify self belong group \"{}\" or it's ancestor belong group",
                    name
                )
            }
            EdgeItemError::InappropriateGroup(_, _, Some(name)) => {
                self.fmt_header(f)?;
                write!(
                    f,
                    "item's belong group \"{}\" is not endpoint's belong group or group endpoint",
                    name
                )
            }
            EdgeItemError::InappropriateGroup(_, _, None) => {
                self.fmt_header(f)?;
                write!(
                    f,
                    "root group which item belong to is not endpoint's belong group or group endpoint",
                )
            }
            EdgeItemError::NameIdError(_, _, e) => {
                self.fmt_header(f)?;
                write!(f, "{}", e)
            }
            EdgeItemError::ResolverError(_, _, e) => {
                self.fmt_header(f)?;
                write!(f, "{}", e)
            }
        }
    }
}

impl<Name: NameType> Error for EdgeItemError<Name> {}
impl<Name: NameType> ItemErrorBase<Name> for EdgeItemError<Name> {}
impl<Name: NameType> FromWithItemId<NameIdError<Name, GraphItemKind>, Name>
    for EdgeItemError<Name>
{
    fn from_with_id(
        item_id: ItemId,
        name: Option<Name>,
        from: NameIdError<Name, GraphItemKind>,
    ) -> Self {
        EdgeItemError::NameIdError(item_id, name, from)
    }
}
impl<Name: NameType> FromWithItemId<ResolverError, Name> for EdgeItemError<Name> {
    fn from_with_id(item_id: ItemId, name: Option<Name>, from: ResolverError) -> Self {
        EdgeItemError::ResolverError(item_id, name, from)
    }
}
impl<Name: NameType> GraphBuilderErrorBase<Name> for EdgeItemError<Name> {
    fn get_item_id(&self) -> &ItemId {
        match self {
            EdgeItemError::FailResolveBelongGroup(i, _, _) => i,
            EdgeItemError::NotSpecifyStartEndpoint(i, _, _) => i,
            EdgeItemError::FailResolveStartEndpoint(i, _, _) => i,
            EdgeItemError::NotSpecifyEndEndpoint(i, _, _) => i,
            EdgeItemError::FailResolveEndEndpoint(i, _, _) => i,
            EdgeItemError::CannotSpecifyBelongGroupAsEndpoint(i, _, _) => i,
            EdgeItemError::InappropriateGroup(i, _, _) => i,
            EdgeItemError::NameIdError(i, _, _) => i,
            EdgeItemError::ResolverError(i, _, _) => i,
        }
    }

    fn get_item_name(&self) -> &Option<Name> {
        match self {
            EdgeItemError::FailResolveBelongGroup(_, name, _) => name,
            EdgeItemError::NotSpecifyStartEndpoint(_, name, _) => name,
            EdgeItemError::FailResolveStartEndpoint(_, name, _) => name,
            EdgeItemError::NotSpecifyEndEndpoint(_, name, _) => name,
            EdgeItemError::FailResolveEndEndpoint(_, name, _) => name,
            EdgeItemError::CannotSpecifyBelongGroupAsEndpoint(_, name, _) => name,
            EdgeItemError::InappropriateGroup(_, name, _) => name,
            EdgeItemError::NameIdError(_, name, _) => name,
            EdgeItemError::ResolverError(_, name, _) => name,
        }
    }
}
