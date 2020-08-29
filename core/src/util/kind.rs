//! kind of Grafo's item

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

/// kind of layout item.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum LayoutGraphItemKind {
    /// This layout kind is group item.
    Group,
    /// This layout kind is node item.
    Node,
    /// This layout kind is edge item.
    Edge,
}

impl std::fmt::Display for LayoutGraphItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutGraphItemKind::Group => write!(f, "Group layout"),
            LayoutGraphItemKind::Node => write!(f, "Node layout"),
            LayoutGraphItemKind::Edge => write!(f, "Edge layout"),
        }
    }
}

impl From<GraphItemKind> for LayoutGraphItemKind {
    fn from(graph_item_kind: GraphItemKind) -> Self {
        use LayoutGraphItemKind::*;
        match graph_item_kind {
            GraphItemKind::Group => Group,
            GraphItemKind::Node => Node,
            GraphItemKind::Edge => Edge,
        }
    }
}

/// add methods of getter for layout graph item kind.
pub trait HasLayoutGraphItemKind: HasGraphItemKind {
    /// get the kind of graph layout item.
    fn layout_kind() -> LayoutGraphItemKind {
        Self::kind().into()
    }
    /// helper method. self.get_layout_kind() == Self::layout_kind().
    fn get_layout_kind(&self) -> LayoutGraphItemKind {
        Self::layout_kind()
    }
}

#[cfg(test)]
pub mod test {
    use crate::util::kind::GraphItemKind;

    pub fn graph_item_check_list() -> Vec<GraphItemKind> {
        use GraphItemKind::*;
        vec![Group, Node, Edge]
    }
}
