//! module for Edge item builder

use crate::grafo::core::graph_item::edge::{EdgeItem, EdgeItemError};
use crate::grafo::core::graph_item::item::edge::EdgeItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::{GroupId, ItemId};
use crate::util::either::Either;
use crate::util::item_base::{
    FromWithItemId, HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult,
};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct EdgeItemBuilder<Name: NameType> {
    // TODO
    belong_group: Option<Name>,
    name: Option<Name>,
    start: Option<(GraphItemKind, Name)>,
    end: Option<(GraphItemKind, Name)>,
}

impl<Name: NameType> ItemBuilderBase<Name> for EdgeItemBuilder<Name> {
    type Item = EdgeItem;
    type ItemError = EdgeItemError<Name>;
}

impl<Name: NameType> GraphItemBuilderBase<Name> for EdgeItemBuilder<Name> {
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self {
        unimplemented!()
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        unimplemented!()
    }
}

impl<Name: NameType> HasItemBuilderMethod<Name> for EdgeItemBuilder<Name> {
    type ItemOption = EdgeItemOption<Name>;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
    ) -> ItemBuilderResult<Name, Self::Item, Self::ItemOption> {
        let mut errors: Vec<GrafoError<Name>> = Vec::new();
        let belong_group: Option<ItemId> =
            self.resolve_belong_group(item_id, resolver, &mut errors);
        let start: Option<(GraphItemKind, (GroupId, ItemId))> = if let Some(bg) = belong_group {
            self.resolve_endpoint(bg, item_id, &self.start, resolver, &mut errors)
        } else {
            None
        };
        let end: Option<(GraphItemKind, (GroupId, ItemId))> = if let Some(bg) = belong_group {
            self.resolve_endpoint(bg, item_id, &self.end, resolver, &mut errors)
        } else {
            None
        };
        let item: Option<EdgeItem> =
            self.resolve_item(item_id, &mut errors, belong_group, start, end);
        let item_option: Option<EdgeItemOption<Name>> =
            self.into_item_option(item_id, resolver, &mut errors);

        match (item, item_option) {
            (Some(i), Some(o)) => (Some((i, o)), errors),
            (_, _) => (None, errors),
        }
    }
}

impl<Name: NameType> EdgeItemBuilder<Name> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            belong_group: None,
            name: None,
            start: None,
            end: None,
        }
    }

    fn resolve_belong_group(
        &self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
        errors: &mut Vec<GrafoError<Name>>,
    ) -> Option<ItemId> {
        match resolver.get_belong_group(self.belong_group.as_ref()) {
            Ok(group) => Some(group),
            Err(Either::Left(e)) => {
                errors.push(EdgeItemError::from_with_id(item_id, e).into());
                None
            }
            Err(Either::Right(e)) => {
                errors.push(e.into());
                None
            }
        }
    }

    fn resolve_endpoint(
        &self,
        group_id: GroupId,
        item_id: ItemId,
        endpoint: &Option<(GraphItemKind, Name)>,
        resolver: &Resolver<Name>,
        errors: &mut Vec<GrafoError<Name>>,
    ) -> Option<(GraphItemKind, (GroupId, ItemId))> {
        if let Some((kind, name)) = endpoint {
            match resolver.get_graph_item_id_pair(*kind, &name) {
                Ok((endpoint_group_id, endpoint_item_id)) => {
                    if *kind == GraphItemKind::Group {
                        let mut cannot_specify = group_id == endpoint_item_id;
                        if !cannot_specify {
                            match resolver.get_ancestor_ids(endpoint_item_id) {
                                None => {
                                    // not stored graph id in id_tree
                                    cannot_specify = true;
                                }
                                Some(ancestor_ids) => {
                                    cannot_specify = ancestor_ids.contains(&group_id);
                                }
                            }
                        }

                        if cannot_specify {
                            errors.push(
                                EdgeItemError::CannotSpecifyBelongGroupAsEndpoint(
                                    item_id,
                                    self.belong_group.clone(),
                                )
                                .into(),
                            );
                            None
                        } else {
                            Some((*kind, (endpoint_group_id, endpoint_item_id)))
                        }
                    } else {
                        Some((*kind, (endpoint_group_id, endpoint_item_id)))
                    }
                }
                Err(e) => {
                    errors.push(EdgeItemError::from_with_id(item_id, e).into());
                    None
                }
            }
        } else {
            errors.push(EdgeItemError::NotSpecifyEndpoint(item_id).into());
            None
        }
    }

    fn resolve_item(
        &self,
        item_id: ItemId,
        errors: &mut Vec<GrafoError<Name>>,
        resolved_belong_group: Option<ItemId>,
        start: Option<(GraphItemKind, (GroupId, ItemId))>,
        end: Option<(GraphItemKind, (GroupId, ItemId))>,
    ) -> Option<EdgeItem> {
        let mut validate = true;
        if resolved_belong_group.is_none() {
            errors.push(EdgeItemError::FailResolveBelongGroup(item_id).into());
            validate = false;
        }

        if start.is_none() {
            errors.push(EdgeItemError::FailResolveStartEndpoint().into());
            validate = false;
        }

        if end.is_none() {
            errors.push(EdgeItemError::FailResolveEndEndpoint().into());
            validate = false;
        }

        if validate {
            Some(EdgeItem::new(
                resolved_belong_group.unwrap(),
                item_id,
                start.unwrap(),
                end.unwrap(),
            ))
        } else {
            None
        }
    }

    fn into_item_option(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
        errors: &mut Vec<GrafoError<Name>>,
    ) -> Option<EdgeItemOption<Name>> {
        let EdgeItemBuilder {
            belong_group: _,
            name,
            start: _,
            end: _,
        } = self;
        if let Some(n) = &name {
            if resolver.is_usable_graph_item_name(EdgeItem::kind(), n) {
                errors.push(
                    EdgeItemError::from_with_id(
                        item_id,
                        NameIdError::AlreadyExist(EdgeItem::kind(), n.to_owned()),
                    )
                    .into(),
                );
            }
        }

        Some(EdgeItemOption { name })
    }
}
