//! base of layout item and layout item

use crate::util::item_base::ItemBase;
use crate::util::kind::HasLayoutGraphItemKind;

/// layout item's base
pub trait LayoutItemBase: ItemBase + HasLayoutGraphItemKind {}
