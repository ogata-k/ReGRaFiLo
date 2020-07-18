use crate::grafo::core::refindex::NameRefIndex;
use crate::util::alias::{GraphItemId, GroupId, LayoutItemId};
use crate::util::item_kind::ItemKind;
use crate::util::layout_kind::AttributeKind;

/// reference indexes for names
#[derive(Debug, Clone)]
pub struct NameReference<'a> {
    /// names reference indexes name:(group_id, item_id)
    names: NameRefIndex<'a, ItemKind, (GroupId, GraphItemId)>,
    /// attribute reference indexes attribute_type:value
    attributes: NameRefIndex<'a, (ItemKind, AttributeKind), LayoutItemId>,
}

impl<'a> Default for NameReference<'a> {
    fn default() -> Self {
        Self {
            names: Default::default(),
            attributes: Default::default(),
        }
    }
}

// TODO imple insert
impl<'a> NameReference<'a> {
    // TODO
}
