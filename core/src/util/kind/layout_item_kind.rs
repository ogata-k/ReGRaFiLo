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
    fn kind() -> AttributeKind;
    fn get_kind(&self) -> AttributeKind {
        Self::kind()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum WithItemLayoutKind {
    // TODO
}

impl std::fmt::Display for WithItemLayoutKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        unimplemented!()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum LayoutItemKind {
    IsolateAttribute(AttributeKind),
    WithItemAttribute(GraphItemKind, WithItemLayoutKind),
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

    pub fn new_layout(item_kind: GraphItemKind, layout_kind: WithItemLayoutKind) -> Self {
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
