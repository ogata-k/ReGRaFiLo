use crate::util::kind::GraphItemKind;

/// kind of Attribute
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum AttributeKind {
    Form,
    Group,
}

impl std::fmt::Display for AttributeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

pub trait HasAttributeKind {
    fn attribute_kind() -> AttributeKind;
    fn layout_kind() -> LayoutItemKind {
        LayoutItemKind::IsolateAttribute(Self::attribute_kind())
    }
    fn get_attribute_kind(&self) -> AttributeKind {
        Self::attribute_kind()
    }
    fn get_layout_kind(&self) -> LayoutItemKind {
        Self::layout_kind()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum AttributeKindDependOnGraph {
    // TODO
}

impl std::fmt::Display for AttributeKindDependOnGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

pub trait HasAttributeKindDependOnGraph {
    fn graph_kind() -> GraphItemKind;
    fn attribute_kind() -> AttributeKindDependOnGraph;
    fn layout_kind() -> LayoutItemKind {
        LayoutItemKind::WithItemAttribute(Self::graph_kind(), Self::attribute_kind())
    }
    fn get_graph_kind(&self) -> GraphItemKind {
        Self::graph_kind()
    }
    fn get_attribute_kind(&self) -> AttributeKindDependOnGraph {
        Self::attribute_kind()
    }
    fn get_layout_kind(&self) -> LayoutItemKind  {
        Self::layout_kind()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum LayoutItemKind {
    IsolateAttribute(AttributeKind),
    WithItemAttribute(GraphItemKind, AttributeKindDependOnGraph),
}

impl std::fmt::Display for LayoutItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

impl LayoutItemKind {
    pub fn new_attribute(attribute_kind: AttributeKind) -> Self {
        Self::IsolateAttribute(attribute_kind)
    }

    pub fn new_layout(item_kind: GraphItemKind, layout_kind: AttributeKindDependOnGraph) -> Self {
        Self::WithItemAttribute(item_kind, layout_kind)
    }

    pub fn is_attribute(&self) -> bool {
        match self {
            LayoutItemKind::IsolateAttribute(_) => true,
            LayoutItemKind::WithItemAttribute(_, _) => false,
        }
    }

    pub fn need_graph_item(&self) -> bool {
        match self {
            LayoutItemKind::IsolateAttribute(_) => false,
            LayoutItemKind::WithItemAttribute(_, _) => true,
        }
    }
}
