use std::error::Error;
use std::fmt::Formatter;

use crate::grafo::core::graph_item::GraphItemBase;
use crate::grafo::{GrafoError, IdTree, NameIdError, NameRefIndex};
use crate::util::alias::{GroupId, ItemId};
use crate::util::either::Either;
use crate::util::kind::{AttributeKind, GraphItemKind, LayoutItemKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ResolverError {
    FailSetRootGraphId,
    NotInitialized,
    NotFindParentId(GroupId),
    AlreadyExistId(GroupId),
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Error for ResolverError {}

impl Into<GrafoError> for ResolverError {
    fn into(self) -> GrafoError {
        unimplemented!()
    }
}

/// reference indexes for names
#[derive(Debug, Clone)]
pub struct Resolver<'a> {
    group_id_tree: IdTree<GroupId>,
    /// names reference indexes name:(group_id, item_id)
    graph_items: NameRefIndex<'a, GraphItemKind, (GroupId, ItemId)>,
    /// layout reference indexes layout_type:value
    layouts: NameRefIndex<'a, LayoutItemKind, ItemId>,
}

impl<'a> Default for Resolver<'a> {
    fn default() -> Self {
        Self {
            group_id_tree: IdTree::None,
            graph_items: Default::default(),
            layouts: Default::default(),
        }
    }
}

impl<'a> Resolver<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    //
    // for root group
    //
    pub(crate) fn set_root_group_id(&mut self, group_id: GroupId) -> Result<(), ResolverError> {
        if self.group_id_tree.is_some() {
            return Err(ResolverError::FailSetRootGraphId);
        }
        self.group_id_tree = IdTree::new(group_id);
        Ok(())
    }

    pub(crate) fn get_root_group_id(&self) -> Result<GroupId, ResolverError> {
        match self.group_id_tree.get_root_id() {
            Ok(id) => Ok(id),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_belong_group<'b: 'a>(
        &'a self,
        name: Option<&'b str>,
    ) -> Result<(GroupId, ItemId), Either<NameIdError<GraphItemKind>, ResolverError>> {
        if let Some(n) = name {
            self.get_graph_item_id_pair(GraphItemKind::Group, n)
                .map_err(Either::Left)
        } else {
            let root_id = self.get_root_group_id().map_err(Either::Right)?;
            Ok((root_id, root_id))
        }
    }

    //
    // for item
    //

    pub(crate) fn push_graph_item_value<S: Into<String>>(
        &mut self,
        item_kind: GraphItemKind,
        name: S,
        group_id: GroupId,
        item_id: ItemId,
    ) -> Result<(), NameIdError<GraphItemKind>> {
        self.graph_items
            .push_value(item_kind, name.into(), (group_id, item_id))
    }

    pub fn get_graph_item_id_pair<'b: 'a>(
        &'a self,
        item_kind: GraphItemKind,
        name: &'b str,
    ) -> Result<(GroupId, ItemId), NameIdError<GraphItemKind>> {
        let id_pair = self
            .graph_items
            .get_value(item_kind, name)
            .ok_or_else(|| NameIdError::NotExist(item_kind, name.to_string()))?;
        Ok(*id_pair)
    }

    pub fn get_graph_item_name_by_item<I: GraphItemBase>(&'a self, item: &I) -> Option<&'a str> {
        self.get_graph_item_name(
            item.get_kind(),
            item.get_belong_group_id(),
            item.get_item_id(),
        )
    }

    pub fn get_graph_item_name(
        &'a self,
        item_kind: GraphItemKind,
        group_id: GroupId,
        item_id: ItemId,
    ) -> Option<&'a str> {
        self.graph_items.get_name(item_kind, (group_id, item_id))
    }

    pub fn contains_name_graph_item<'b: 'a>(
        &'a self,
        item_kind: GraphItemKind,
        name: &'b str,
    ) -> bool {
        self.graph_items.contains_name(item_kind, name)
    }

    pub fn count_names_graph_item_by(&self, item_kind: GraphItemKind) -> usize {
        self.graph_items.count_names_by(item_kind)
    }

    //
    // for layout with graph item
    //

    pub(crate) fn push_layout_value_for_graph_item<S: Into<String>>(
        &mut self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: S,
        layout_item_id: ItemId,
    ) -> Result<(), NameIdError<LayoutItemKind>> {
        self.layouts.push_value(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            name.into(),
            layout_item_id,
        )
    }

    pub fn get_layout_item_id_for_graph_item<'b: 'a>(
        &'a self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> Result<&'a ItemId, NameIdError<LayoutItemKind>> {
        let kind = LayoutItemKind::new_with_item(item_kind, attribute_kind);
        self.layouts
            .get_value(kind, name)
            .ok_or_else(|| NameIdError::NotExist(kind, name.to_string()))
    }

    pub fn get_layout_item_name_for_graph_item(
        &'a self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        item_id: ItemId,
    ) -> Option<&'a str> {
        self.layouts.get_name(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            item_id,
        )
    }

    pub fn contains_name_layout_item_for_graph_item<'b: 'a>(
        &'a self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> bool {
        self.layouts.contains_name(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            name,
        )
    }

    pub fn count_names_layout_item_for_graph_item_by(
        &self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
    ) -> usize {
        self.layouts
            .count_names_by(LayoutItemKind::new_with_item(item_kind, attribute_kind))
    }

    //
    //  for layout without graph item
    //

    pub(crate) fn push_layout_value<S: Into<String>>(
        &mut self,
        attribute_kind: AttributeKind,
        name: S,
        layout_item_id: ItemId,
    ) -> Result<(), NameIdError<LayoutItemKind>> {
        self.layouts.push_value(
            LayoutItemKind::new(attribute_kind),
            name.into(),
            layout_item_id,
        )
    }

    pub fn get_layout_item_id<'b: 'a>(
        &'a self,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> Result<&'a ItemId, NameIdError<LayoutItemKind>> {
        let kind = LayoutItemKind::new(attribute_kind);
        self.layouts
            .get_value(kind, name)
            .ok_or_else(|| NameIdError::NotExist(kind, name.to_string()))
    }

    pub fn get_layout_item_name(
        &'a self,
        attribute_kind: AttributeKind,
        item_id: ItemId,
    ) -> Option<&'a str> {
        self.layouts
            .get_name(LayoutItemKind::new(attribute_kind), item_id)
    }

    pub fn contains_name_layout_item<'b: 'a>(
        &'a self,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> bool {
        self.layouts
            .contains_name(LayoutItemKind::new(attribute_kind), name)
    }

    pub fn count_names_layout_item_by(&self, attribute_kind: AttributeKind) -> usize {
        self.layouts
            .count_names_by(LayoutItemKind::new(attribute_kind))
    }
}
