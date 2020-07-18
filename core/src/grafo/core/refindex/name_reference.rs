use crate::grafo::core::refindex::{NameRefIndex, NameRefWarning};
use crate::util::alias::{GraphItemId, GroupId, ItemId, LayoutItemId};
use crate::util::item_kind::ItemKind;
use crate::util::layout_kind::{AttributeKind, LayoutKind};

/// reference indexes for names
#[derive(Debug, Clone)]
pub struct NameReference<'a> {
    /// names reference indexes name:(group_id, item_id)
    names: NameRefIndex<'a, ItemKind, (GroupId, GraphItemId)>,
    /// attribute reference indexes attribute_type:value
    attributes: NameRefIndex<'a, LayoutKind, LayoutItemId>,
}

impl<'a> Default for NameReference<'a> {
    fn default() -> Self {
        Self {
            names: Default::default(),
            attributes: Default::default(),
        }
    }
}

// TODO impl insert
impl<'a> NameReference<'a> {
    // TODO
    pub fn new() -> Self {
        Default::default()
    }

    //
    // for item
    //

    pub fn push_item_name<S: Into<String>>(
        &mut self,
        item_kind: ItemKind,
        name: S,
        group_id: GroupId,
        item_id: ItemId,
    ) -> Result<(), NameRefWarning<ItemKind>> {
        self.names
            .push_value(item_kind, name.into(), (group_id, item_id))
    }

    pub fn get_item_id_pair<'b: 'a>(
        &'a self,
        item_kind: ItemKind,
        name: &'b str,
    ) -> Result<&'a (GroupId, ItemId), NameRefWarning<ItemKind>> {
        self.names.get_value(item_kind, name)
    }

    pub fn item_name_count_by(&self, item_kind: ItemKind) -> usize {
        self.names.count_by(item_kind)
    }

    //
    // for attribute
    //

    pub fn push_attribute_name<S: Into<String>>(
        &mut self,
        item_kind: ItemKind,
        attribute_kind: AttributeKind,
        name: S,
        layout_item_id: LayoutItemId,
    ) -> Result<(), NameRefWarning<LayoutKind>> {
        self.attributes.push_value(
            LayoutKind::new(item_kind, attribute_kind),
            name.into(),
            layout_item_id,
        )
    }

    pub fn get_layout_item_id<'b: 'a>(
        &'a self,
        item_kind: ItemKind,
        attribute_kind: AttributeKind,
        name: &'b str,
    ) -> Result<&'a LayoutItemId, NameRefWarning<LayoutKind>> {
        self.attributes
            .get_value(LayoutKind::new(item_kind, attribute_kind), name)
    }

    pub fn attribute_name_count_by(
        &self,
        item_kind: ItemKind,
        attribute_kind: AttributeKind,
    ) -> usize {
        self.attributes
            .count_by(LayoutKind::new(item_kind, attribute_kind))
    }
}
