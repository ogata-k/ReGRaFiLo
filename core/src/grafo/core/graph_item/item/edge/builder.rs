//! module for Edge item's builder

use crate::grafo::core::graph_item::edge::{EdgeItem, EdgeItemError, EdgeItemOption};
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::grafo::graph_item::edge::Endpoint;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::{GroupId, ItemId};
use crate::util::either::Either;
use crate::util::item_base::{
    FromWithItemId, HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult,
};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;

/// builder for Group item
#[derive(Debug, Clone)]
pub struct EdgeItemBuilder<Name: NameType> {
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
        self.belong_group = Some(group.into());
        self
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
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
        let belong_group: Option<GroupId> =
            self.resolve_belong_group(item_id, resolver, &mut errors);
        let start: Option<(GraphItemKind, (GroupId, ItemId))> = if let Some(bg) = belong_group {
            self.resolve_endpoint(bg, item_id, &self.start, resolver, &mut errors, |item_id| {
                EdgeItemError::NotSpecifyStartEndpoint(item_id, self.start.clone())
            })
        } else {
            None
        };
        let end: Option<(GraphItemKind, (GroupId, ItemId))> = if let Some(bg) = belong_group {
            self.resolve_endpoint(bg, item_id, &self.end, resolver, &mut errors, |item_id| {
                EdgeItemError::NotSpecifyEndEndpoint(item_id, self.end.clone())
            })
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

// resolver
impl<Name: NameType> EdgeItemBuilder<Name> {
    /// resolve belong group from builder's parameter
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

    /// resolve endpoint from builder's parameter
    fn resolve_endpoint<F>(
        &self,
        group_id: GroupId,
        item_id: ItemId,
        endpoint: &Option<(GraphItemKind, Name)>,
        resolver: &Resolver<Name>,
        errors: &mut Vec<GrafoError<Name>>,
        not_specify_error: F,
    ) -> Option<(GraphItemKind, (GroupId, ItemId))>
    where
        F: FnOnce(ItemId) -> EdgeItemError<Name>,
    {
        if let Some((kind, name)) = endpoint {
            match resolver.get_graph_item_id_pair(*kind, &name) {
                Ok((endpoint_group_id, endpoint_item_id)) => {
                    if *kind == GraphItemKind::Group {
                        let mut cannot_specify = group_id == endpoint_item_id;
                        if !cannot_specify {
                            match resolver.get_ancestor_ids(endpoint_item_id) {
                                None => {
                                    // not stored graph id in id_tree
                                    // usually unreachable!!
                                    errors.push(
                                        EdgeItemError::from_with_id(
                                            item_id,
                                            NameIdError::NotExist(
                                                GraphItemKind::Group,
                                                name.clone(),
                                            ),
                                        )
                                        .into(),
                                    );
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
                                    name.clone(),
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
            errors.push(not_specify_error(item_id).into());
            None
        }
    }

    /// resolve Edge item from builder's parameter
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
            errors.push(
                EdgeItemError::FailResolveBelongGroup(item_id, self.belong_group.clone()).into(),
            );
            validate = false;
        }

        if resolved_belong_group.is_some() && start.is_none() {
            errors
                .push(EdgeItemError::FailResolveStartEndpoint(item_id, self.start.clone()).into());
            validate = false;
        }

        if resolved_belong_group.is_some() && end.is_none() {
            errors.push(EdgeItemError::FailResolveEndEndpoint(item_id, self.end.clone()).into());
            validate = false;
        }

        if validate {
            let (s_kind, (s_belong_group, s_item_id)) = start.unwrap();
            let (e_kind, (e_belong_group, e_item_id)) = end.unwrap();
            Some(EdgeItem::new(
                resolved_belong_group.unwrap(),
                item_id,
                Endpoint::new(s_kind, s_belong_group, s_item_id),
                Endpoint::new(e_kind, e_belong_group, e_item_id),
            ))
        } else {
            None
        }
    }

    /// resolve Edge item's option from builder's parameter
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
                        NameIdError::AlreadyExist(EdgeItem::kind(), n.clone()),
                    )
                    .into(),
                );
            }
        }

        Some(EdgeItemOption { name })
    }
}

impl<Name: NameType> EdgeItemBuilder<Name> {
    /// initializer for Edge item's builder
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            belong_group: None,
            name: None,
            start: None,
            end: None,
        }
    }

    /// setter for start endpoint
    pub fn set_start_endpoint<S: Into<Name>>(&mut self, kind: GraphItemKind, name: S) -> &mut Self {
        self.start = Some((kind, name.into()));
        self
    }

    /// setter for end endpoint
    pub fn set_end_endpoint<S: Into<Name>>(&mut self, kind: GraphItemKind, name: S) -> &mut Self {
        self.end = Some((kind, name.into()));
        self
    }
}
