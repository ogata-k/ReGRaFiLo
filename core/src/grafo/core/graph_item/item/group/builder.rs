//! module for Group item's builder

use crate::grafo::core::graph_item::group::{GroupItem, GroupItemError};
use crate::grafo::core::graph_item::item::group::GroupItemOption;
use crate::grafo::core::graph_item::GraphItemBuilderBase;
use crate::grafo::core::resolve::Resolver;
use crate::grafo::graph_item::group::GroupItemStyle;
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
    label: Option<String>,
    style: Option<GroupItemStyle>,
}

impl<Name: NameType> ItemBuilderBase<Name> for GroupItemBuilder<Name> {
    type Item = GroupItem;
    type ItemError = GroupItemError<Name>;
}

impl<Name: NameType> GraphItemBuilderBase<Name> for GroupItemBuilder<Name> {
    type ItemStyle = GroupItemStyle;

    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self {
        self.belong_group = Some(group.into());
        self
    }

    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    fn set_label<S: Into<String>>(&mut self, label: S) -> &mut Self {
        self.label = Some(label.into());
        self
    }

    fn set_item_style(&mut self, style: Self::ItemStyle) -> &mut Self {
        self.style = Some(style);
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
        let (item, option): (Option<GroupItem>, GroupItemOption<Name>) =
            self.resolve_item(item_id, resolver, &mut errors, belong_group);
        match item {
            Some(i) => (Some((i, option)), errors),
            None => (None, errors),
        }
    }
}

impl<Name: NameType> Default for GroupItemBuilder<Name> {
    fn default() -> Self {
        Self {
            belong_group: None,
            name: None,
            label: None,
            style: None,
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
                errors.push(
                    GroupItemError::CannotSpecifyBelongGroupForRoot(
                        item_id,
                        self.name.clone(),
                        n.clone(),
                    )
                    .into(),
                );
                None
            } else {
                Some(item_id)
            };
        }
        match resolver.get_belong_group(self.belong_group.as_ref()) {
            Ok(group) => Some(group),
            Err(Either::Left(e)) => {
                errors.push(GroupItemError::from_with_id(item_id, self.name.clone(), e).into());
                None
            }
            Err(Either::Right(e)) => {
                errors.push(GroupItemError::from_with_id(item_id, self.name.clone(), e).into());
                None
            }
        }
    }

    /// resolve Group item from builder's parameter
    fn resolve_item(
        self,
        item_id: ItemId,
        resolver: &Resolver<Name>,
        errors: &mut Vec<GrafoError<Name>>,
        resolved_belong_group: Option<ItemId>,
    ) -> (Option<GroupItem>, GroupItemOption<Name>) {
        let mut validate = true;
        let GroupItemBuilder {
            belong_group,
            name,
            label,
            style,
        } = self;

        if resolved_belong_group.is_none() {
            if item_id != DEFAULT_ITEM_ID {
                errors.push(
                    GroupItemError::FailResolveBelongGroup(item_id, name.clone(), belong_group)
                        .into(),
                );
            }
            validate = false;
        }

        // todo?? if self use outer file, check file exist. but not fail build.

        let item = if validate {
            Some(GroupItem::new(
                resolved_belong_group.unwrap(),
                item_id,
                label,
                style.unwrap_or_default(),
            ))
        } else {
            None
        };

        // option
        if let Some(n) = &name {
            if resolver.is_usable_graph_item_name(GroupItem::kind(), n) {
                errors.push(
                    GroupItemError::from_with_id(
                        item_id,
                        Some(n.clone()),
                        NameIdError::AlreadyExist(GroupItem::kind(), n.clone()),
                    )
                    .into(),
                );
            }
        }
        (item, GroupItemOption { name })
    }
}

impl<Name: NameType> GroupItemBuilder<Name> {
    /// initializer for Group item's builder
    pub fn new() -> Self {
        Self::default()
    }
}
