//! kind for layout

/// kind of Attribute
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum AttributeKind {
    Form,
    Group,
}

impl Into<LayoutKind> for AttributeKind {
    fn into(self) -> LayoutKind {
        match self {
            AttributeKind::Form => LayoutKind::Form,
            AttributeKind::Group => LayoutKind::Group,
        }
    }
}

/// kind of Layout
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum LayoutKind {
    Form,
    Group,
}
