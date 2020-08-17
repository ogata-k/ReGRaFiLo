//! module for Edge item

use crate::grafo::core::graph_item::GraphItemBase;
use crate::util::alias::{GroupId, ItemId};
use crate::util::item_base::ItemBase;
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::writer::WriteAsJson;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Endpoint {
    kind: GraphItemKind,
    group_id: GroupId,
    item_id: ItemId,
}

impl WriteAsJson for Endpoint {
    fn write_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}}}",
            self.kind, self.group_id, self.item_id
        )
    }
}

impl Endpoint {
    fn new(kind: GraphItemKind, id_pair: (GroupId, ItemId)) -> Self {
        Self {
            kind,
            group_id: id_pair.0,
            item_id: id_pair.1,
        }
    }
}

/// Edge Item
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct EdgeItem {
    // TODO Align can use RelativeAlign and AbsoluteAlign
    belong_group_id: GroupId,
    item_id: ItemId,
    start: (GraphItemKind, (GroupId, ItemId)),
    end: (GraphItemKind, (GroupId, ItemId)),
}

impl WriteAsJson for EdgeItem {
    fn write_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}, \"start_endpoint\": ",
            self.get_kind(),
            self.belong_group_id,
            self.item_id
        )?;
        let start = Endpoint::new(self.start.0, self.start.1);
        start.write_as_json(f)?;
        write!(f, ", \"end_endpoint\": ")?;
        let end = Endpoint::new(self.end.0, self.end.1);
        end.write_as_json(f)?;
        write!(f, "}}")
    }
}

impl std::fmt::Display for EdgeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge")?;
        self.write_as_json(f)
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
    pub(crate) fn new(
        belong_group: GroupId,
        item_id: ItemId,
        start: (GraphItemKind, (GroupId, ItemId)),
        end: (GraphItemKind, (GroupId, ItemId)),
    ) -> Self {
        Self {
            belong_group_id: belong_group,
            item_id,
            start,
            end,
        }
    }

    pub fn get_start_item_id(&self) -> (GraphItemKind, (GroupId, ItemId)) {
        self.start
    }

    pub fn get_end_item_id(&self) -> (GraphItemKind, (GroupId, ItemId)) {
        self.end
    }
}
