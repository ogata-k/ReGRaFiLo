//! module for Group item's builder

use crate::grafo::core::graph_item::group::{GroupItem, GroupItemError};
use crate::grafo::core::graph_item::item::group::GroupItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::grafo::{GrafoError, NameIdError};
use crate::util::alias::{GroupId, ItemId, DEFAULT_ITEM_ID};
use crate::util::either::Either;
use crate::util::item_base::{
    FromWithItemId, HasItemBuilderMethod, ItemBuilderBase, ItemBuilderResult,
};
use crate::util::kind::HasGraphItemKind;
use crate::util::name_type::NameType;

/// builder for Group item
#[derive(Debug, Clone)]
pub struct GroupItemBuilder<Name: NameType> {
    belong_group: Option<Name>,
    name: Option<Name>,
}

impl<Name: NameType> ItemBuilderBase<Name> for GroupItemBuilder<Name> {
    type Item = GroupItem;
    type ItemError = GroupItemError<Name>;
}

impl<Name: NameType> GraphItemBuilderBase<Name> for GroupItemBuilder<Name> {
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self {
        self.belong_group = Some(group.into());
        self
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }
}

impl<Name: NameType> HasItemBuilderMethod<Name> for GroupItemBuilder<Name> {
    type ItemOption = GroupItemOption<Name>;
    fn build(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
    ) -> ItemBuilderResult<Name, Self::Item, Self::ItemOption> {
        let mut errors: Vec<GrafoError<Name>> = Vec::new();
        let belong_group: Option<GroupId> =
            self.resolve_belong_group(item_id, resolver, &mut errors);
        let item: Option<GroupItem> = self.resolve_item(item_id, &mut errors, belong_group);
        let item_option: Option<GroupItemOption<Name>> =
            self.into_item_option(item_id, resolver, &mut errors);

        match (item, item_option) {
            (Some(i), Some(o)) => (Some((i, o)), errors),
            (_, _) => (None, errors),
        }
    }
}

impl<Name: NameType> Default for GroupItemBuilder<Name> {
    fn default() -> Self {
        Self {
            belong_group: None,
            name: None,
        }
    }
}

// resolver
impl<Name: NameType> GroupItemBuilder<Name> {
    /// resolve belong group from builder's parameter
    fn resolve_belong_group(
        &self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
        errors: &mut Vec<GrafoError<Name>>,
    ) -> Option<ItemId> {
        if item_id == DEFAULT_ITEM_ID {
            return if let Some(n) = &self.belong_group {
                errors.push(GroupItemError::CannotSpecifyBelongGroupForRoot(n.clone()).into());
                None
            } else {
                Some(item_id)
            };
        }
        match resolver.get_belong_group(self.belong_group.as_ref()) {
            Ok(group) => Some(group),
            Err(Either::Left(e)) => {
                errors.push(GroupItemError::from_with_id(item_id, e).into());
                None
            }
            Err(Either::Right(e)) => {
                errors.push(e.into());
                None
            }
        }
    }

    /// resolve Group item from builder's parameter
    fn resolve_item(
        &self,
        item_id: ItemId,
        errors: &mut Vec<GrafoError<Name>>,
        resolved_belong_group: Option<ItemId>,
    ) -> Option<GroupItem> {
        let mut validate = true;
        if resolved_belong_group.is_none() {
            if item_id != DEFAULT_ITEM_ID {
                errors.push(
                    GroupItemError::FailResolveBelongGroup(item_id, self.belong_group.clone())
                        .into(),
                );
            }
            validate = false;
        }

        if validate {
            Some(GroupItem::new(resolved_belong_group.unwrap(), item_id))
        } else {
            None
        }
    }

    /// resolve Group item's option from builder's parameter
    fn into_item_option(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
        errors: &mut Vec<GrafoError<Name>>,
    ) -> Option<GroupItemOption<Name>> {
        let GroupItemBuilder {
            belong_group: _,
            name,
        } = self;
        if let Some(n) = &name {
            if resolver.is_usable_graph_item_name(GroupItem::kind(), n) {
                errors.push(
                    GroupItemError::from_with_id(
                        item_id,
                        NameIdError::AlreadyExist(GroupItem::kind(), n.clone()),
                    )
                    .into(),
                );
            }
        }

        Some(GroupItemOption { name })
    }
}

impl<Name: NameType> GroupItemBuilder<Name> {
    /// initializer for Group item's builder
    pub fn new() -> Self {
        Self::default()
    }
}
