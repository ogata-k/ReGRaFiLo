//! base of layout item and layout item's builder

use crate::util::item_base::ItemBase;
use crate::util::kind::{HasAttributeKind, HasAttributeKindDependOnGraph};

/// Item's base set depending on graph item
pub trait LayoutItemBase: ItemBase + HasAttributeKind {}

/// Item's base set depending on graph item
pub trait LayoutItemBaseDependOnGraph: ItemBase + HasAttributeKindDependOnGraph {}
