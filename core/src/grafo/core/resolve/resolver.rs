use crate::grafo::core::resolve::{NameRefIndex, ResolverError};
use crate::util::alias::{GraphItemId, GroupId, ItemId, LayoutItemId};
use crate::util::kind::{AttributeKind, GraphItemKind, LayoutItemKind};

/// reference indexes for names
#[derive(Debug, Clone)]
pub struct Resolver<'a> {
    // TODO Group構造の管理（GroupTree）
    root_group_id: Option<GroupId>,
    /// names reference indexes name:(group_id, item_id)
    names: NameRefIndex<'a, GraphItemKind, (GroupId, GraphItemId)>,
    /// attribute reference indexes attribute_type:value
    attributes: NameRefIndex<'a, LayoutItemKind, LayoutItemId>,
}

impl<'a> Default for Resolver<'a> {
    fn default() -> Self {
        Self {
            // TODO できればroot_group_idは決め打ちのを使いたい
            root_group_id: None,
            names: Default::default(),
            attributes: Default::default(),
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
    pub(crate) fn set_root_group_id(&mut self, group_id: GroupId) {
        if self.root_group_id.is_some() {
            panic!("already set root group");
        }
        self.root_group_id = Some(group_id);
    }

    pub(crate) fn get_root_group_id(&self) -> GroupId {
        self.root_group_id.expect("root group not set")
    }

    //
    // for item
    //

    pub fn push_item_name<S: Into<String>>(
        &mut self,
        item_kind: GraphItemKind,
        name: S,
        group_id: GroupId,
        item_id: ItemId,
    ) -> Result<(), ResolverError<GraphItemKind>> {
        self.names
            .push_value(item_kind, name.into(), (group_id, item_id))
    }

    pub fn get_item_id_pair<'b: 'a>(
        &'a self,
        item_kind: GraphItemKind,
        name: &'b str,
    ) -> Result<&'a (GroupId, ItemId), ResolverError<GraphItemKind>> {
        self.names.get_value(item_kind, name)
    }

    pub fn contains_item_name<'b: 'a>(&'a self, item_kind: GraphItemKind, name: &'b str) -> bool {
        self.names.contains_key(item_kind, name)
    }

    pub fn item_name_count_by(&self, item_kind: GraphItemKind) -> usize {
        self.names.count_by(item_kind)
    }

    //
    // for attribute
    //

    pub fn push_attribute_name_for_graph_item<S: Into<String>>(
        &mut self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: S,
        layout_item_id: LayoutItemId,
    ) -> Result<(), ResolverError<LayoutItemKind>> {
        self.attributes.push_value(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            name.into(),
            layout_item_id,
        )
    }

    pub fn push_attribute_name<S: Into<String>>(
        &mut self,
        attribute_kind: AttributeKind,
        name: S,
        layout_item_id: LayoutItemId,
    ) -> Result<(), ResolverError<LayoutItemKind>> {
        self.attributes.push_value(
            LayoutItemKind::new(attribute_kind),
            name.into(),
            layout_item_id,
        )
    }

    pub fn get_layout_item_id_for_graph_item<'b: 'a>(
        &'a self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> Result<&'a LayoutItemId, ResolverError<LayoutItemKind>> {
        self.attributes.get_value(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            name,
        )
    }

    pub fn get_layout_item_id<'b: 'a>(
        &'a self,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> Result<&'a LayoutItemId, ResolverError<LayoutItemKind>> {
        self.attributes
            .get_value(LayoutItemKind::new(attribute_kind), name)
    }

    pub fn contains_layout_item_name_for_graph_item<'b: 'a>(
        &'a self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> bool {
        self.attributes.contains_key(
            LayoutItemKind::new_with_item(item_kind, attribute_kind),
            name,
        )
    }

    pub fn contains_layout_item_name<'b: 'a>(
        &'a self,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> bool {
        self.attributes
            .contains_key(LayoutItemKind::new(attribute_kind), name)
    }

    pub fn attribute_name_count_for_graph_item_by(
        &self,
        item_kind: GraphItemKind,
        attribute_kind: AttributeKind,
    ) -> usize {
        self.attributes
            .count_by(LayoutItemKind::new_with_item(item_kind, attribute_kind))
    }

    pub fn attribute_name_count_item_by(&self, attribute_kind: AttributeKind) -> usize {
        self.attributes
            .count_by(LayoutItemKind::new(attribute_kind))
    }
}
