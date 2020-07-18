//! base of item and item builder

use crate::util::alias::GroupId;
use crate::util::item_base::{ItemBase, ItemBuilderBase, ItemErrorBase};
use crate::util::kind::HasGraphItemKind;

/// Item Builder's base set
pub trait GraphItemBuilderBase: ItemBuilderBase + HasGraphItemKind {
    fn set_group_id(&mut self, group_id: GroupId) -> &mut Self;
    fn get_group_id(&self) -> GroupId;
}

/// Item's base set
pub trait GraphItemBase: ItemBase + HasGraphItemKind {
    fn get_group_id(&self) -> GroupId;
}

pub trait GraphItemErrorBase: ItemErrorBase + HasGraphItemKind {}
