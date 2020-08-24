//! module for the kind of graph item.

/// kind of graph item.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum GraphItemKind {
    /// Group. This is a grouping item for other graph item.
    Group,
    /// Node. This is a vertex of graph.
    Node,
    /// Edge. This is a bridge from other graph item to other graph item.
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

/// add methods of getter for graph item kind.
pub trait HasGraphItemKind {
    /// get the kind of graph item.
    fn kind() -> GraphItemKind;
    /// helper method. self.get_kind() == Self::kind().
    fn get_kind(&self) -> GraphItemKind {
        Self::kind()
    }
}
