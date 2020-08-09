use std::error::Error;
use std::fmt::Formatter;

use crate::grafo::core::graph_item::GraphItemBase;
use crate::grafo::{GrafoError, IdTree, NameIdError, NameRefIndex};
use crate::util::alias::{GroupId, ItemId};
use crate::util::either::Either;
use crate::util::kind::{AttributeKind, GraphItemKind, LayoutItemKind};
use crate::util::name_type::NameType;
use std::borrow::Borrow;
use std::hash::Hash;

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

impl<Name: NameType> Into<GrafoError<Name>> for ResolverError {
    fn into(self) -> GrafoError<Name> {
        unimplemented!()
    }
}

/// reference indexes for names
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Resolver<Name: NameType> {
    group_id_tree: IdTree<GroupId>,
    /// names reference indexes name:(group_id, item_id)
    graph_items: NameRefIndex<Name, GraphItemKind, (GroupId, ItemId)>,
    /// layout reference indexes layout_type:value
    layout_items: NameRefIndex<Name, LayoutItemKind, ItemId>,
}

impl<Name: NameType> Default for Resolver<Name> {
    fn default() -> Self {
        Self {
            group_id_tree: IdTree::None,
            graph_items: NameRefIndex::new(),
            layout_items: NameRefIndex::new(),
        }
    }
}

impl<Name: NameType> Resolver<Name> {
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

    pub fn get_belong_group<S: ?Sized>(
        &self,
        name: Option<&S>,
    ) -> Result<(GroupId, ItemId), Either<NameIdError<Name, GraphItemKind>, ResolverError>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        match name {
            None => {
                let root_id = self.get_root_group_id().map_err(Either::Right)?;
                Ok((root_id, root_id))
            }
            Some(n) => self
                .get_graph_item_id_pair(GraphItemKind::Group, n)
                .map_err(Either::Left),
        }
    }

    //
    // for item
    //

    pub(crate) fn push_graph_item_value<S: Into<Name>>(
        &mut self,
        item_kind: GraphItemKind,
        name: S,
        group_id: GroupId,
        item_id: ItemId,
    ) -> Result<(), NameIdError<Name, GraphItemKind>> {
        self.graph_items
            .push_value(item_kind, name, (group_id, item_id))
    }

    pub fn get_graph_item_id_pair<S: ?Sized>(
        &self,
        item_kind: GraphItemKind,
        name: &S,
    ) -> Result<(GroupId, ItemId), NameIdError<Name, GraphItemKind>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        let id_pair = self
            .graph_items
            .get_value(item_kind, name)
            .ok_or_else(|| NameIdError::NotExist(item_kind, name.to_owned()))?;
        Ok(*id_pair)
    }

    pub fn get_graph_item_name_by_item<I: GraphItemBase>(&self, item: &I) -> Option<&Name> {
        self.get_graph_item_name_by(
            item.get_kind(),
            item.get_belong_group_id(),
            item.get_item_id(),
        )
    }

    pub fn get_graph_item_name_by(
        &self,
        item_kind: GraphItemKind,
        group_id: GroupId,
        item_id: ItemId,
    ) -> Option<&Name> {
        self.graph_items.get_name(item_kind, (group_id, item_id))
    }

    pub fn is_usable_name_graph_item<S: ?Sized>(&self, item_kind: GraphItemKind, name: &S) -> bool
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.graph_items.is_usable_name(item_kind, name)
    }

    pub fn has_registered_name_graph_item(
        &self,
        item_kind: GraphItemKind,
        group_id: GroupId,
        item_id: ItemId,
    ) -> bool {
        self.graph_items
            .has_registered_name(item_kind, (group_id, item_id))
    }

    pub fn count_usable_names_graph_item_by(&self, item_kind: GraphItemKind) -> usize {
        self.graph_items.count_usable_names_by(item_kind)
    }

    pub fn count_usable_names_graph_item(&self) -> usize {
        self.graph_items.count_usable_names_all()
    }

    pub fn count_registered_names_graph_item_by(&self, item_kind: GraphItemKind) -> usize {
        self.graph_items.count_registered_names_by(item_kind)
    }

    pub fn count_registered_names_graph_item(&self) -> usize {
        self.graph_items.count_registered_names_all()
    }

    //
    // for layout with graph item
    //

    pub(crate) fn push_layout_value_for_graph_item<S: Into<Name>>(
        &mut self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: S,
        layout_item_id: ItemId,
    ) -> Result<(), NameIdError<Name, LayoutItemKind>> {
        self.layout_items.push_value(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            name.into(),
            layout_item_id,
        )
    }

    pub fn get_layout_item_id_for_graph_item<S: ?Sized>(
        &self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: &S,
    ) -> Result<ItemId, NameIdError<Name, LayoutItemKind>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        let kind = LayoutItemKind::new_with_item(item_kind, attribute_kind);
        self.layout_items
            .get_value(kind, name)
            .copied()
            .ok_or_else(|| NameIdError::NotExist(kind, name.to_owned()))
    }

    pub fn get_layout_item_name_for_graph_item_by(
        &self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        item_id: ItemId,
    ) -> Option<&Name> {
        self.layout_items.get_name(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            item_id,
        )
    }

    pub fn get_layout_item_name_for_graph_item_by_item<I: GraphItemBase>(
        &self,
        attribute_kind: AttributeKind,
        item: &I,
    ) -> Option<&Name> {
        self.layout_items.get_name(
            LayoutItemKind::new_with_item(item.get_kind(), attribute_kind),
            item.get_item_id(),
        )
    }

    pub fn is_usable_name_layout_item_for_graph_item<S: ?Sized>(
        &self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: &S,
    ) -> bool
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.layout_items.is_usable_name(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            name,
        )
    }

    pub fn has_registered_name_graph_item_name_layout_item_for_graph_item(
        &self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        item_id: ItemId,
    ) -> bool {
        self.layout_items.has_registered_name(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            item_id,
        )
    }

    pub fn count_usable_names_layout_item_for_graph_item_by(
        &self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
    ) -> usize {
        self.layout_items
            .count_usable_names_by(LayoutItemKind::new_with_item(item_kind, attribute_kind))
    }

    pub fn count_usable_names_layout_item_for_graph_item(&self) -> usize {
        self.layout_items.count_usable_names_all()
    }

    pub fn count_registered_names_layout_item_for_graph_item_by(
        &self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
    ) -> usize {
        self.layout_items
            .count_registered_names_by(LayoutItemKind::new_with_item(item_kind, attribute_kind))
    }

    pub fn count_registered_names_layout_item_for_graph_item(&self) -> usize {
        self.layout_items.count_registered_names_all()
    }

    //
    //  for layout without graph item
    //

    pub(crate) fn push_layout_item_value<S: Into<Name>>(
        &mut self,
        attribute_kind: AttributeKind,
        name: S,
        layout_item_id: ItemId,
    ) -> Result<(), NameIdError<Name, LayoutItemKind>> {
        self.layout_items
            .push_value(LayoutItemKind::new(attribute_kind), name, layout_item_id)
    }

    pub fn get_layout_item_id<S: ?Sized>(
        &self,
        attribute_kind: AttributeKind,
        name: &S,
    ) -> Result<ItemId, NameIdError<Name, LayoutItemKind>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        let kind = LayoutItemKind::new(attribute_kind);
        self.layout_items
            .get_value(kind, name)
            .copied()
            .ok_or_else(|| NameIdError::NotExist(kind, name.to_owned()))
    }
    /*
       TODO 下の実装のGroupItemなしのlayout専用版
       pub fn get_graph_item_name_by_item<I: GraphItemBase>(&self, item: &I) -> Option<&Name> {
           self.get_graph_item_name_by(
               item.get_kind(),
               item.get_belong_group_id(),
               item.get_item_id(),
           )
       }

    */
    pub fn get_layout_item_name_by(
        &self,
        attribute_kind: AttributeKind,
        item_id: ItemId,
    ) -> Option<&Name> {
        self.layout_items
            .get_name(LayoutItemKind::new(attribute_kind), item_id)
    }

    pub fn is_usable_name_layout_item<S: ?Sized>(
        &self,
        attribute_kind: AttributeKind,
        name: &S,
    ) -> bool
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.layout_items
            .is_usable_name(LayoutItemKind::new(attribute_kind), name)
    }

    pub fn has_registered_name_layout_item(
        &self,
        attribute_kind: AttributeKind,
        item_id: ItemId,
    ) -> bool {
        self.layout_items
            .has_registered_name(LayoutItemKind::new(attribute_kind), item_id)
    }

    pub fn count_usable_names_layout_item_by(&self, attribute_kind: AttributeKind) -> usize {
        self.layout_items
            .count_usable_names_by(LayoutItemKind::new(attribute_kind))
    }

    pub fn count_usable_names_layout_item(&self) -> usize {
        self.layout_items.count_usable_names_all()
    }

    pub fn count_registered_names_layout_item_by(&self, attribute_kind: AttributeKind) -> usize {
        self.layout_items
            .count_registered_names_by(LayoutItemKind::new(attribute_kind))
    }

    pub fn count_registered_names_layout_item(&self) -> usize {
        self.layout_items.count_registered_names_all()
    }
}
