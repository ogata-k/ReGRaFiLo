//! reference indexes

use crate::grafo::core::layout::attribute::AttributeRefIndex;
use crate::util::alias::RefIndexOfItem;
use crate::util::item_kind::ItemKind;

// TODO
/// reference indexes for layout
#[derive(Debug, Clone)]
pub struct LayoutReference {
    // indexes name:id
    names: RefIndexOfItem<ItemKind, String>,

    // layout
    attribute: AttributeRefIndex,
}
