//! module for Group item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::grafo::graph_item::group::GroupItemStyle;
use crate::grafo::graph_item::WithMutable;
use crate::util::alias::{GroupId, ItemId, DEFAULT_ITEM_ID};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::writer::DisplayAsJson;

/// Group item
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GroupItem {
    belong_group_id: GroupId,
    item_id: ItemId,
    label: Option<String>,
    style: GroupItemStyle,
}

impl DisplayAsJson for GroupItem {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}, \"label\": \"{}\"",
            &self.get_kind(),
            &self.belong_group_id,
            &self.item_id,
            self.label.as_deref().unwrap_or_else(|| ""),
        )?;
        write!(f, ", \"style\": ")?;
        self.style.fmt_as_json(f)?;
        write!(f, "}}")
    }
}

impl std::fmt::Display for GroupItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group")?;
        self.fmt_as_json(f)
    }
}

impl HasGraphItemKind for GroupItem {
    fn kind() -> GraphItemKind {
        GraphItemKind::Group
    }
}

impl ItemBase for GroupItem {
    fn get_item_id(&self) -> ItemId {
        self.item_id
    }
}

impl GraphItemBase for GroupItem {
    type ItemStyle = GroupItemStyle;

    fn get_belong_group_id(&self) -> GroupId {
        self.belong_group_id
    }

    fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn get_item_style(&self) -> &Self::ItemStyle {
        &self.style
    }
}

impl Default for GroupItem {
    fn default() -> Self {
        Self {
            belong_group_id: DEFAULT_ITEM_ID,
            item_id: DEFAULT_ITEM_ID,
            label: None,
            style: GroupItemStyle::default(),
        }
    }
}

impl WithMutable for GroupItem {
    fn set_label<S: Into<String>>(&mut self, label: Option<S>) -> &mut Self {
        self.label = label.map(|s| s.into());
        self
    }
}

impl GroupItem {
    /// initializer for Group item
    pub(crate) fn new(
        belong_group: GroupId,
        item_id: ItemId,
        label: Option<String>,
        style: GroupItemStyle,
    ) -> Self {
        Self {
            belong_group_id: belong_group,
            item_id,
            label,
            style,
        }
    }
}
