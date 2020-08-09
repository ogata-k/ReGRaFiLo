use std::error::Error;
use std::fmt::Formatter;

use crate::grafo::core::graph_item::GraphItemBase;
use crate::grafo::layout_item::LayoutItemBase;
use crate::grafo::{GrafoError, IdTree, NameIdError, NameRefIndex};
use crate::util::alias::{GroupId, ItemId};
use crate::util::either::Either;
use crate::util::kind::{AttributeKind, GraphItemKind, LayoutItemKind, WithItemLayoutKind};
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

    pub fn as_graph_item_ref(&self, kind: GraphItemKind) -> GraphItemRef<Name> {
        GraphItemRef::from_resolver(kind, self)
    }

    pub fn as_attribute_ref(&self, kind: AttributeKind) -> AttributeRef<Name> {
        AttributeRef::from_resolver(kind, self)
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
                .as_graph_item_ref(GraphItemKind::Group)
                .get_id_pair(n)
                .map_err(Either::Left),
        }
    }

    //
    // for graph item
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

    pub(crate) fn push_graph_item_layout_value<S: Into<Name>>(
        &mut self,
        item_kind: GraphItemKind,
        layout_kind: WithItemLayoutKind,
        name: S,
        layout_item_id: ItemId,
    ) -> Result<(), NameIdError<Name, LayoutItemKind>> {
        self.layout_items.push_value(
            LayoutItemKind::new_layout(item_kind, layout_kind),
            name.into(),
            layout_item_id,
        )
    }

    pub(crate) fn push_attribute_value<S: Into<Name>>(
        &mut self,
        attribute_kind: AttributeKind,
        name: S,
        layout_item_id: ItemId,
    ) -> Result<(), NameIdError<Name, LayoutItemKind>> {
        self.layout_items.push_value(
            LayoutItemKind::new_attribute(attribute_kind),
            name,
            layout_item_id,
        )
    }

    // TODO graph_itemを使わない形にしたい
    pub fn get_graph_item_name_by_item<I: GraphItemBase>(&self, item: &I) -> Option<&Name> {
        self.graph_items.get_name(
            item.get_kind(),
            (item.get_belong_group_id(), item.get_item_id()),
        )
    }

    // TODO layout_itemを使わない形にしたい
    pub fn get_graph_item_layout_name_by_item<I: GraphItemBase>(
        &self,
        layout_kind: WithItemLayoutKind,
        item: &I,
    ) -> Option<&Name> {
        self.layout_items.get_name(
            LayoutItemKind::WithItemAttribute(item.get_kind(), layout_kind),
            item.get_item_id(),
        )
    }

    // TODO layout_itemを使わない形にしたい
    pub fn get_attribute_name_by_item<I: LayoutItemBase>(&self, item: &I) -> Option<&Name> {
        self.layout_items.get_name(
            LayoutItemKind::IsolateAttribute(item.get_kind()),
            item.get_item_id(),
        )
    }

    pub fn count_usable_graph_item_names(&self) -> usize {
        self.graph_items.count_usable_names_all()
    }

    pub fn count_usable_whole_layout_item_names(&self) -> usize {
        self.layout_items.count_usable_names_all()
    }

    pub fn count_usable_graph_item_layout_names(&self) -> usize {
        self.layout_items
            .count_usable_names_filtered_by(|k| k.need_graph_item())
    }

    pub fn count_usable_attribute_names(&self) -> usize {
        self.layout_items
            .count_usable_names_filtered_by(|k| k.is_attribute())
    }

    pub fn count_registered_graph_item_names(&self) -> usize {
        self.graph_items.count_registered_names_all()
    }

    pub fn count_registered_whole_layout_names(&self) -> usize {
        self.layout_items.count_registered_names_all()
    }

    pub fn count_registered_graph_item_layout_names(&self) -> usize {
        self.layout_items
            .count_registered_names_filtered_by(|k| k.need_graph_item())
    }

    pub fn count_registered_attribute_names(&self) -> usize {
        self.layout_items
            .count_registered_names_filtered_by(|k| k.is_attribute())
    }
}

// TODO 内部に持ってしまっているので外側に外せるようにしたい
#[derive(Debug)]
pub struct GraphItemRef<'a, Name: NameType> {
    kind: GraphItemKind,
    resolver: &'a Resolver<Name>,
}

impl<'a, Name: NameType> GraphItemRef<'a, Name> {
    fn from_resolver(kind: GraphItemKind, resolver: &'a Resolver<Name>) -> Self {
        Self { kind, resolver }
    }

    pub fn as_attribute_ref(&self, kind: AttributeKind) -> AttributeRef<Name> {
        AttributeRef::from_resolver(kind, self.resolver)
    }

    //
    // for item
    //

    pub fn get_id_pair<S: ?Sized>(
        &self,
        name: &S,
    ) -> Result<(GroupId, ItemId), NameIdError<Name, GraphItemKind>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        let id_pair = self
            .resolver
            .graph_items
            .get_value(self.kind, name)
            .ok_or_else(|| NameIdError::NotExist(self.kind, name.to_owned()))?;
        Ok(*id_pair)
    }

    pub fn get_name_by(&self, group_id: GroupId, item_id: ItemId) -> Option<&Name> {
        self.resolver
            .graph_items
            .get_name(self.kind, (group_id, item_id))
    }

    pub fn is_usable_name<S: ?Sized>(&self, name: &S) -> bool
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.resolver.graph_items.is_usable_name(self.kind, name)
    }

    pub fn has_registered_name(&self, group_id: GroupId, item_id: ItemId) -> bool {
        self.resolver
            .graph_items
            .has_registered_name(self.kind, (group_id, item_id))
    }

    pub fn count_usable_names(&self) -> usize {
        self.resolver.graph_items.count_usable_names_by(self.kind)
    }

    pub fn count_registered_names(&self) -> usize {
        self.resolver
            .graph_items
            .count_registered_names_by(self.kind)
    }

    //
    // for layout with graph item
    //

    pub fn get_layout_item_id<S: ?Sized>(
        &self,
        kind: WithItemLayoutKind,
        name: &S,
    ) -> Result<ItemId, NameIdError<Name, LayoutItemKind>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        let kind = LayoutItemKind::new_layout(self.kind, kind);
        self.resolver
            .layout_items
            .get_value(kind, name)
            .copied()
            .ok_or_else(|| NameIdError::NotExist(kind, name.to_owned()))
    }

    pub fn get_layout_name_by(&self, kind: WithItemLayoutKind, item_id: ItemId) -> Option<&Name> {
        self.resolver
            .layout_items
            .get_name(LayoutItemKind::new_layout(self.kind, kind), item_id)
    }

    pub fn is_usable_layout_name<S: ?Sized>(&self, kind: WithItemLayoutKind, name: &S) -> bool
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.resolver
            .layout_items
            .is_usable_name(LayoutItemKind::new_layout(self.kind, kind), name)
    }

    pub fn has_registered_layout_name(&self, kind: WithItemLayoutKind, item_id: ItemId) -> bool {
        self.resolver
            .layout_items
            .has_registered_name(LayoutItemKind::new_layout(self.kind, kind), item_id)
    }

    pub fn count_usable_layout_names_by(&self, kind: WithItemLayoutKind) -> usize {
        self.resolver
            .layout_items
            .count_usable_names_by(LayoutItemKind::new_layout(self.kind, kind))
    }

    pub fn count_registered_layout_names_by(&self, kind: WithItemLayoutKind) -> usize {
        self.resolver
            .layout_items
            .count_registered_names_by(LayoutItemKind::new_layout(self.kind, kind))
    }
}

// TODO 内部に持ってしまっているので外側に外せるようにしたい
#[derive(Debug)]
pub struct AttributeRef<'a, Name: NameType> {
    kind: AttributeKind,
    resolver: &'a Resolver<Name>,
}

impl<'a, Name: NameType> AttributeRef<'a, Name> {
    fn from_resolver(kind: AttributeKind, resolver: &'a Resolver<Name>) -> Self {
        Self { kind, resolver }
    }

    //
    //  for layout without graph item
    //

    pub fn get_item_id<S: ?Sized>(
        &self,
        name: &S,
    ) -> Result<ItemId, NameIdError<Name, LayoutItemKind>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        let kind = LayoutItemKind::new_attribute(self.kind);
        self.resolver
            .layout_items
            .get_value(kind, name)
            .copied()
            .ok_or_else(|| NameIdError::NotExist(kind, name.to_owned()))
    }

    pub fn get_name_by(&self, item_id: ItemId) -> Option<&Name> {
        self.resolver
            .layout_items
            .get_name(LayoutItemKind::new_attribute(self.kind), item_id)
    }

    pub fn is_usable_name<S: ?Sized>(&self, name: &S) -> bool
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.resolver
            .layout_items
            .is_usable_name(LayoutItemKind::new_attribute(self.kind), name)
    }

    pub fn has_registered_name(&self, item_id: ItemId) -> bool {
        self.resolver
            .layout_items
            .has_registered_name(LayoutItemKind::new_attribute(self.kind), item_id)
    }

    pub fn count_usable_names_by(&self) -> usize {
        self.resolver
            .layout_items
            .count_usable_names_by(LayoutItemKind::new_attribute(self.kind))
    }

    pub fn count_registered_names_by(&self) -> usize {
        self.resolver
            .layout_items
            .count_registered_names_by(LayoutItemKind::new_attribute(self.kind))
    }
}
