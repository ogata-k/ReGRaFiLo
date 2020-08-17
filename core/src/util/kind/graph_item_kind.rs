//! module for type of item

/// type of item
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum GraphItemKind {
    Group,
    Node,
    Edge,
}

impl std::fmt::Display for GraphItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphItemKind::Group => write!(f, "Group"),
            GraphItemKind::Node => write!(f, "Node"),
            GraphItemKind::Edge => write!(f, "Edge"),
        }
    }
}

pub trait HasGraphItemKind {
    fn kind() -> GraphItemKind;
    fn get_kind(&self) -> GraphItemKind {
        Self::kind()
    }
}
