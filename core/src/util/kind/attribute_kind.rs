use std::fmt::{Display, Formatter};

/// kind of Attribute
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum AttributeKind {
    Form,
    Group,
}

fn item_kind_to_str(attribute_kind: &AttributeKind) -> &str {
    match attribute_kind {
        AttributeKind::Form => "Form",
        AttributeKind::Group => "Group",
    }
}

impl Display for AttributeKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
