//! reference indexes

use crate::grafo::core::layout::attribute::AttributeRefIndex;
use crate::util::alias::{RefIndex, ItemIndex, GroupIndex};
use crate::util::item_kind::ItemKind;
use crate::util::kind_key::KeyWithKind;

// TODO
/// reference indexes for layout
#[derive(Debug, Clone)]
pub struct LayoutReference {
    /// names reference indexes name:(group_id, item_id)
    names: RefIndex<KeyWithKind<ItemKind, String>, (GroupIndex, ItemIndex)>,

    /// attribute reference indexes attribute_type:value
    attributes: AttributeRefIndex,
    // TODO each layout item without
}

impl Default for LayoutReference {
    fn default() -> Self {
        Self {
            names: Default::default(),
            attributes: Default::default(),
        }
    }
}
