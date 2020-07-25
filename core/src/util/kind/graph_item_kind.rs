//! module for type of item

/// type of item
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum GraphItemKind {
    Group,
    Node,
    Edge,
}

pub trait HasGraphItemKind {
    fn kind() -> GraphItemKind;
    fn get_kind(&self) -> GraphItemKind {
        Self::kind()
    }
}

fn item_kind_to_str(item_kind: &GraphItemKind) -> &str {
    match item_kind {
        GraphItemKind::Group => "Group",
        GraphItemKind::Node => "Node",
        GraphItemKind::Edge => "Edge",
    }
}
impl std::fmt::Display for GraphItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", item_kind_to_str(&self))
    }
}
