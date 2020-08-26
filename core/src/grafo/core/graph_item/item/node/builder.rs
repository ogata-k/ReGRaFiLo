//! module for Node item's builder

use crate::grafo::core::graph_item::node::{NodeItem, NodeItemError, NodeItemOption};
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::{GroupId, ItemId};
use crate::util::either::Either;
use crate::util::item_base::{
    FromWithItemId, HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult,
};
use crate::util::kind::HasGraphItemKind;
use crate::util::name_type::NameType;

/// builder for Node item
#[derive(Debug, Clone)]
pub struct NodeItemBuilder<Name: NameType> {
    belong_group: Option<Name>,
    name: Option<Name>,
}

impl<Name: NameType> ItemBuilderBase<Name> for NodeItemBuilder<Name> {
    type Item = NodeItem;
    type ItemError = NodeItemError<Name>;
}

impl<Name: NameType> GraphItemBuilderBase<Name> for NodeItemBuilder<Name> {
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self {
        self.belong_group = Some(group.into());
        self
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }
}

impl<Name: NameType> HasItemBuilderMethod<Name> for NodeItemBuilder<Name> {
    type ItemOption = NodeItemOption<Name>;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
    ) -> ItemBuilderResult<Name, Self::Item, Self::ItemOption> {
        let mut errors: Vec<GrafoError<Name>> = Vec::new();
        let belong_group: Option<GroupId> =
            self.resolve_belong_group(item_id, resolver, &mut errors);
        let item: Option<NodeItem> = self.resolve_item(item_id, &mut errors, belong_group);
        let item_option: Option<NodeItemOption<Name>> =
            self.into_item_option(item_id, resolver, &mut errors);

        match (item, item_option) {
            (Some(i), Some(o)) => (Some((i, o)), errors),
            (_, _) => (None, errors),
        }
    }
}

// resolver
impl<Name: NameType> NodeItemBuilder<Name> {
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
                errors.push(NodeItemError::from_with_id(item_id, e).into());
                None
            }
            Err(Either::Right(e)) => {
                errors.push(e.into());
                None
            }
        }
    }

    /// resolve Node item from builder's parameter
    fn resolve_item(
        &self,
        item_id: ItemId,
        errors: &mut Vec<GrafoError<Name>>,
        resolved_belong_group: Option<ItemId>,
    ) -> Option<NodeItem> {
        let mut validate = true;
        if resolved_belong_group.is_none() {
            errors.push(
                NodeItemError::FailResolveBelongGroup(item_id, self.belong_group.clone()).into(),
            );
            validate = false;
        }

        if validate {
            Some(NodeItem::new(resolved_belong_group.unwrap(), item_id))
        } else {
            None
        }
    }

    /// resolve Node item's option from builder's parameter
    fn into_item_option(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
        errors: &mut Vec<GrafoError<Name>>,
    ) -> Option<NodeItemOption<Name>> {
        let NodeItemBuilder {
            belong_group: _,
            name,
        } = self;
        if let Some(n) = &name {
            if resolver.is_usable_graph_item_name(NodeItem::kind(), n) {
                errors.push(
                    NodeItemError::from_with_id(
                        item_id,
                        NameIdError::AlreadyExist(NodeItem::kind(), n.clone()),
                    )
                    .into(),
                );
            }
        }

        Some(NodeItemOption { name })
    }
}

impl<Name: NameType> NodeItemBuilder<Name> {
    /// initializer for Node item's builder
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            belong_group: None,
            name: None,
        }
    }
}
