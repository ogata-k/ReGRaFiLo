//! type alias

/// index of item
pub type ItemId = usize;

/// item id for default item
pub(crate) const DEFAULT_ITEM_ID: ItemId = 0;

/// alias for item id for belong group of the item. <br/>
/// GroupId is **NOT** ItemId for Group as GraphItem
pub type GroupId = ItemId;

/// alias for layout item id
pub type LayoutItemId = ItemId;
