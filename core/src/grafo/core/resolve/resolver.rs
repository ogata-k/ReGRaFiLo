//! resolver for hierarchical group and name

use std::borrow::Borrow;
use std::error::Error;
use std::hash::Hash;

use crate::grafo::core::graph_item::GraphItemBase;
use crate::grafo::graph_item::edge::Endpoint;
use crate::grafo::layout_item::LayoutItemBase;
use crate::grafo::{IdTree, IdTreeError, NameIdError, NameRefIndex};
use crate::util::alias::{GroupId, ItemId, LayoutItemId};
use crate::util::either::Either;
use crate::util::iter::IterLimitedByOneGroup;
use crate::util::kind::{GraphItemKind, LayoutGraphItemKind};
use crate::util::name_type::NameType;
use crate::util::writer::DisplayAsJson;

/// error for Resolver
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ResolverError {
    /// fail set root group id
    FailSetRootGraphId,
    /// occurred error when use group tree, before initialized the group tree
    NotInitialized,
    /// not found parent id for target id
    NotFindParentId(GroupId),
    /// specified id already exist
    AlreadyExistId(GroupId),
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverError::FailSetRootGraphId => write!(f, "fail set group as root group"),
            ResolverError::NotInitialized => write!(f, "hierarchy of group is not initialized"),
            ResolverError::NotFindParentId(group_id) => {
                write!(f, "not found parent group by id {}", group_id)
            }
            ResolverError::AlreadyExistId(group_id) => {
                write!(f, "group with id {} already exist", group_id)
            }
        }
    }
}

impl Error for ResolverError {}

impl From<IdTreeError<GroupId>> for ResolverError {
    fn from(e: IdTreeError<GroupId>) -> ResolverError {
        match e {
            IdTreeError::NotInitialized => ResolverError::NotInitialized,
            IdTreeError::NotFindParentId(id) => ResolverError::NotFindParentId(id),
            IdTreeError::AlreadyExistId(id) => ResolverError::AlreadyExistId(id),
        }
    }
}

/// reference indexes for names and hierarchy tree for group id
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Resolver<Name: NameType> {
    group_id_tree: IdTree<GroupId>,
    /// names reference indexes name:(group_id, graph_item_id)
    graph_items: NameRefIndex<Name, GraphItemKind, (GroupId, ItemId)>,
    /// layout reference indexes layout_type: layout_item_id
    layout_items: NameRefIndex<Name, LayoutGraphItemKind, ItemId>,
}

impl<Name: NameType> Default for Resolver<Name> {
    fn default() -> Self {
        Self {
            group_id_tree: IdTree::None,
            graph_items: NameRefIndex::new(),
            layout_items: NameRefIndex::initialize_without_no_name(),
        }
    }
}

impl<Name: NameType> DisplayAsJson for Resolver<Name> {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"group_tree\": \"{}\", \"graph_items\": ",
            self.group_id_tree
        )?;
        self.graph_items.fmt_as_json(f)?;
        write!(f, ", \"layout_items\": ")?;
        self.layout_items.fmt_as_json(f)?;
        write!(f, "}}")
    }
}

impl<Name: NameType> std::fmt::Display for Resolver<Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReferenceResolver",)?;
        self.fmt_as_json(f)
    }
}

impl<Name: NameType> Resolver<Name> {
    /// initialize as default
    pub fn new() -> Self {
        Default::default()
    }

    //
    // for root group
    //
    /// initialize as group id tree with specify id
    pub(crate) fn set_root_group_id(&mut self, group_id: GroupId) -> Result<(), ResolverError> {
        match self.group_id_tree {
            IdTree::Root(_) => Err(ResolverError::FailSetRootGraphId),
            IdTree::None => {
                self.group_id_tree = IdTree::new(group_id);
                Ok(())
            }
        }
    }

    /// insert child group for parent group
    pub(crate) fn insert_group(
        &mut self,
        parent: GroupId,
        child: GroupId,
    ) -> Result<(), ResolverError> {
        self.group_id_tree
            .insert_id(parent, child)
            .map_err(|e| e.into())
    }

    /// get root id
    pub(crate) fn get_root_group_id(&self) -> Result<GroupId, ResolverError> {
        match self.group_id_tree.get_root_id() {
            Ok(id) => Ok(id),
            Err(e) => Err(e.into()),
        }
    }

    /// get group id by specify name
    pub fn get_belong_group<S: ?Sized>(
        &self,
        name: Option<&S>,
    ) -> Result<GroupId, Either<NameIdError<Name, GraphItemKind>, ResolverError>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        match name {
            None => {
                let root_id = self.get_root_group_id().map_err(Either::Right)?;
                Ok(root_id)
            }
            Some(n) => self
                .get_graph_item_id_pair(GraphItemKind::Group, n)
                .map(|(_, item_id)| item_id)
                .map_err(Either::Left),
        }
    }

    /// check specified group is exist
    pub fn contains_group(&self, group_id: GroupId) -> bool {
        self.group_id_tree.contains_id(group_id)
    }

    /// get parent and ancestors id
    pub fn get_ancestor_ids(&self, group_id: GroupId) -> Vec<GroupId> {
        self.group_id_tree.get_ancestor_ids(group_id)
    }

    /// get children's id list
    pub fn get_child_ids(&self, group_id: GroupId) -> Vec<GroupId> {
        self.group_id_tree.get_child_ids(group_id)
    }

    /// get id list of children and children's children
    pub fn get_descendant_ids(&self, group_id: GroupId) -> Vec<GroupId> {
        self.group_id_tree.get_descendant_ids(group_id)
    }

    /// get group tree but type as string
    pub fn group_tree_string(&self) -> String {
        self.group_id_tree.to_string()
    }

    //
    // for graph item
    //

    /// inset id for graph item as the kind. But override value when name exist.
    pub(crate) fn insert_graph_item_id_or_override<S: Into<Name>>(
        &mut self,
        item_kind: GraphItemKind,
        name: Option<S>,
        group_id: GroupId,
        item_id: ItemId,
    ) -> Result<(), NameIdError<Name, GraphItemKind>> {
        self.graph_items
            .insert_value_or_override(item_kind, name, (group_id, item_id))
    }

    /// get pair of belonging group_id and item_id by name
    pub fn get_graph_item_id_pair<S: ?Sized>(
        &self,
        item_kind: GraphItemKind,
        name: &S,
    ) -> Result<(GroupId, ItemId), NameIdError<Name, GraphItemKind>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.graph_items
            .get_value(item_kind, name)
            .ok_or_else(|| NameIdError::NotExist(item_kind, name.to_owned()))
    }

    /// get graph item's name as the kind.
    pub fn get_graph_item_name_by(
        &self,
        item_kind: GraphItemKind,
        group_id: GroupId,
        item_id: ItemId,
    ) -> Option<&Name> {
        self.graph_items.get_name(item_kind, (group_id, item_id))
    }

    /// get the graph item's name as the kind by the item.
    pub fn get_graph_item_name_by_item<I: GraphItemBase>(&self, item: &I) -> Option<&Name> {
        self.graph_items.get_name(
            item.get_kind(),
            (item.get_belong_group_id(), item.get_item_id()),
        )
    }

    /// get the graph item's name as the kind by the endpoint.
    pub fn get_graph_item_name_by_endpoint(&self, endpoint: &Endpoint) -> Option<&Name> {
        self.graph_items.get_name(
            endpoint.get_kind(),
            (endpoint.get_belong_group_id(), endpoint.get_item_id()),
        )
    }

    /// check the graph item is already registered
    pub fn is_already_registered_graph_item(
        &self,
        item_kind: GraphItemKind,
        group_id: GroupId,
        item_id: ItemId,
    ) -> bool {
        self.graph_items
            .is_already_registered(item_kind, (group_id, item_id))
    }

    /// check the name usable as reference key for graph item's key.
    pub fn is_usable_graph_item_name<S: ?Sized>(&self, item_kind: GraphItemKind, name: &S) -> bool
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.graph_items.is_usable_name(item_kind, name)
    }

    /// check the graph item as the kind has a name for graph item's key.
    pub fn has_registered_graph_item_name(
        &self,
        item_kind: GraphItemKind,
        group_id: GroupId,
        item_id: ItemId,
    ) -> bool {
        self.graph_items
            .has_registered_name(item_kind, (group_id, item_id))
    }

    /// count all graph items.
    pub fn count_all_registered_graph_items(&self) -> usize {
        self.graph_items.count_all_registered()
    }

    /// count all graph items limited by kind.
    pub fn count_all_graph_items_by(&self, item_kind: GraphItemKind) -> usize {
        self.graph_items.count_all_registered_by(item_kind)
    }

    /// count all usable names as reference key for graph item's key.
    pub fn count_usable_graph_item_names(&self) -> usize {
        self.graph_items.count_usable_names_all()
    }

    /// count all usable names as reference key limited by the kind for graph item's key.
    pub fn count_usable_graph_item_names_by(&self, item_kind: GraphItemKind) -> usize {
        self.graph_items.count_usable_names_by(item_kind)
    }

    /// count all graph items having name.
    pub fn count_registered_graph_item_names(&self) -> usize {
        self.graph_items.count_registered_names_all()
    }

    /// count all graph items having name limited by specify kind.
    pub fn count_registered_graph_item_names_by(&self, item_kind: GraphItemKind) -> usize {
        self.graph_items.count_registered_names_by(item_kind)
    }

    /// iter for graph item grouped by the item_kind
    pub fn iter_graph_item_by(
        &self,
        item_kind: GraphItemKind,
    ) -> IterLimitedByOneGroup<GraphItemKind, (GroupId, ItemId), Name> {
        self.graph_items.iter_by_kind(item_kind)
    }

    //
    // for layout with graph item
    //

    /// insert item id for layout item. layout item always has name because the no name item cannot be specified.
    pub(crate) fn insert_layout_id<S: Into<Name>>(
        &mut self,
        item_kind: GraphItemKind,
        name: S,
        layout_item_id: LayoutItemId,
    ) -> Result<(), NameIdError<Name, LayoutGraphItemKind>> {
        self.layout_items
            .insert_value_or_override(item_kind.into(), Some(name), layout_item_id)
    }

    /// get item id for layout item
    pub fn get_layout_item_id<S: ?Sized>(
        &self,
        item_kind: GraphItemKind,
        name: &S,
    ) -> Result<ItemId, NameIdError<Name, LayoutGraphItemKind>>
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        let layout_kind = item_kind.into();
        self.layout_items
            .get_value(layout_kind, name)
            .ok_or_else(|| NameIdError::NotExist(layout_kind, name.to_owned()))
    }

    /// get layout item's name
    pub fn get_layout_item_name_by(
        &self,
        item_kind: GraphItemKind,
        item_id: ItemId,
    ) -> Option<&Name> {
        self.layout_items.get_name(item_kind.into(), item_id)
    }

    /// get layout item's name by specified layout item
    pub fn get_layout_item_name_by_item<I: LayoutItemBase>(&self, item: &I) -> Option<&Name> {
        self.layout_items
            .get_name(item.get_layout_kind(), item.get_item_id())
    }

    /// check the layout item is already registered
    pub fn is_already_registered_layout_item(
        &self,
        item_kind: GraphItemKind,
        item_id: ItemId,
    ) -> bool {
        self.layout_items
            .is_already_registered(item_kind.into(), item_id)
    }

    /// check the name usable as reference key for layout item
    pub fn is_usable_layout_item_name<S: ?Sized>(&self, item_kind: GraphItemKind, name: &S) -> bool
    where
        Name: Borrow<S>,
        S: ToOwned<Owned = Name> + Hash + Eq,
    {
        self.layout_items.is_usable_name(item_kind.into(), name)
    }

    /// check the layout item as the kind has a name for the layout item's key.
    pub fn has_registered_layout_item_name(
        &self,
        item_kind: GraphItemKind,
        item_id: ItemId,
    ) -> bool {
        self.layout_items
            .has_registered_name(item_kind.into(), item_id)
    }

    /// count all usable names as reference key for layout item.
    pub fn count_usable_layout_item_names(&self) -> usize {
        self.layout_items.count_usable_names_all()
    }

    /// count usable names as reference key for layout item limited by specify kind.
    pub fn count_usable_graph_item_layout_names_by(&self, item_kind: GraphItemKind) -> usize {
        self.layout_items.count_usable_names_by(item_kind.into())
    }

    /// count all layout items as the kind has a name for the layout item's key.
    pub fn count_registered_graph_item_layout_names(&self) -> usize {
        self.layout_items.count_registered_names_all()
    }

    /// count all layout items as the kind has a name for the layout item's key limited by specify kind.
    pub fn count_registered_graph_item_layout_names_by(&self, item_kind: GraphItemKind) -> usize {
        self.layout_items
            .count_registered_names_by(item_kind.into())
    }

    /// iter for layout item grouped by the item_kind
    pub fn iter_layout_item_by(
        &self,
        item_kind: LayoutGraphItemKind,
    ) -> IterLimitedByOneGroup<LayoutGraphItemKind, ItemId, Name> {
        self.layout_items.iter_by_kind(item_kind)
    }
}
