//! module for Edge item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GroupId, ItemId};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::writer::DisplayAsJson;

/// endpoint is graph item for Edge's endpoint
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Endpoint {
    kind: GraphItemKind,
    group_id: GroupId,
    item_id: ItemId,
}

impl DisplayAsJson for Endpoint {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}}}",
            self.kind, self.group_id, self.item_id
        )
    }
}

impl Endpoint {
    /// initializer for endpoint
    pub fn new(kind: GraphItemKind, group_id: GroupId, item_id: ItemId) -> Self {
        Self {
            kind,
            group_id,
            item_id,
        }
    }

    /// endpoint's graph item kind
    pub fn get_kind(&self) -> GraphItemKind {
        self.kind
    }

    /// belonging group for endpoint's item
    pub fn get_belong_group_id(&self) -> GroupId {
        self.group_id
    }

    /// item id for endpoint
    pub fn get_item_id(&self) -> ItemId {
        self.item_id
    }
}

/// Edge Item
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct EdgeItem {
    // TODO Align can use RelativeAlign and AbsoluteAlign
    belong_group_id: GroupId,
    item_id: ItemId,
    start: Endpoint,
    end: Endpoint,
}

impl DisplayAsJson for EdgeItem {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}, \"start_endpoint\": ",
            self.get_kind(),
            self.belong_group_id,
            self.item_id
        )?;
        self.start.fmt_as_json(f)?;
        write!(f, ", \"end_endpoint\": ")?;
        self.end.fmt_as_json(f)?;
        write!(f, "}}")
    }
}

impl std::fmt::Display for EdgeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge")?;
        self.fmt_as_json(f)
    }
}

impl HasGraphItemKind for EdgeItem {
    fn kind() -> GraphItemKind {
        GraphItemKind::Edge
    }
}

impl ItemBase for EdgeItem {
    fn get_item_id(&self) -> ItemId {
        self.item_id
    }
}

impl GraphItemBase for EdgeItem {
    fn get_belong_group_id(&self) -> GroupId {
        self.belong_group_id
    }
}

impl EdgeItem {
    /// initializer for Edge item
    pub(crate) fn new(
        belong_group: GroupId,
        item_id: ItemId,
        start: Endpoint,
        end: Endpoint,
    ) -> Self {
        Self {
            belong_group_id: belong_group,
            item_id,
            start,
            end,
        }
    }

    /// get endpoint of Edge's start endpoint
    pub fn get_start_endpoint(&self) -> Endpoint {
        self.start
    }

    /// get endpoint of Edge's end endpoint
    pub fn get_end_endpoint(&self) -> Endpoint {
        self.end
    }
}
