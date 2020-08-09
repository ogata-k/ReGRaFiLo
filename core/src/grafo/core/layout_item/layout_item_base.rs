use crate::util::item_base::ItemBase;
use crate::util::kind::HasAttributeKind;

// TODO
/// Item's base set
pub trait LayoutItemBase: ItemBase + HasAttributeKind {}
