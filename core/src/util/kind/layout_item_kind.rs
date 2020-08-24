//! module for the kind of layout item.

use crate::util::kind::GraphItemKind;

/// The kind of Attribute. Attribute is layout item without depending on graph item.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum AttributeKind {
    // TODO ex.Color
}

impl std::fmt::Display for AttributeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

/// add methods of getter for attribute kind.
pub trait HasAttributeKind {
    /// get the attribute kind.
    fn attribute_kind() -> AttributeKind;
    /// helper method. self.get_attribute_kind() == Self::attribute_kind().
    fn get_attribute_kind(&self) -> AttributeKind {
        Self::attribute_kind()
    }
}

/// The kind of attribute depending on graph item.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum AttributeKindDependOnGraph {
    // TODO ex.Form
}

impl std::fmt::Display for AttributeKindDependOnGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

/// add methods of getter for the kind of attribute depending on graph item.
pub trait HasAttributeKindDependOnGraph {
    /// get the graph item kind.
    fn graph_kind() -> GraphItemKind;
    /// get the attribute kind.
    fn attribute_kind() -> AttributeKindDependOnGraph;
    /// helper method. self.get_graph_kind() == Self::graph_kind().
    fn get_graph_kind(&self) -> GraphItemKind {
        Self::graph_kind()
    }
    /// helper method. self.get_attribute_kind() == Self::attribute_kind().
    fn get_attribute_kind(&self) -> AttributeKindDependOnGraph {
        Self::attribute_kind()
    }
}

/// layout kind. The kind is the kind of attribute or the kind of attribute depending on graph item.  
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum LayoutItemKind {
    /// The kind is attribute kind.
    IsolateAttribute(AttributeKind),
    /// The kind is a kind of attribute depending on graph item.
    WithItemAttribute(GraphItemKind, AttributeKindDependOnGraph),
}

/// add methods of getter for the layout kind.<br/>
/// Item implementing trait "HasAttributeKind" or "HasAttributeKindDependOnGraph" need impl this trait.
pub trait HasLayoutKind {
    /// get the layout kind.
    fn layout_kind() -> LayoutItemKind;
    /// helper method. self.get_layout_kind() == Self::layout_kind().
    fn get_layout_kind(&self) -> LayoutItemKind {
        Self::layout_kind()
    }
}

impl std::fmt::Display for LayoutItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

impl LayoutItemKind {
    /// make attribute kind as layout kind.
    pub fn new_attribute(attribute_kind: AttributeKind) -> Self {
        Self::IsolateAttribute(attribute_kind)
    }

    /// make kind of attribute depending on graph item as layout item.
    pub fn new_layout(item_kind: GraphItemKind, layout_kind: AttributeKindDependOnGraph) -> Self {
        Self::WithItemAttribute(item_kind, layout_kind)
    }

    /// check self layout kind equal attribute kind.
    pub fn is_attribute(&self) -> bool {
        match self {
            LayoutItemKind::IsolateAttribute(_) => true,
            LayoutItemKind::WithItemAttribute(_, _) => false,
        }
    }

    /// check self layout kind equal kind of attribute depending on graph item.
    pub fn need_graph_item(&self) -> bool {
        match self {
            LayoutItemKind::IsolateAttribute(_) => false,
            LayoutItemKind::WithItemAttribute(_, _) => true,
        }
    }
}
