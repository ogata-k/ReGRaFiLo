//! kind for layout

use crate::util::item_kind::ItemKind;
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct LayoutKind {
    pub item_kind: ItemKind,
    pub attribute_kind: AttributeKind,
}

impl LayoutKind {
    pub fn new(item_kind: ItemKind, attribute_kind: AttributeKind) -> Self {
        Self {
            item_kind,
            attribute_kind,
        }
    }
}

impl Display for LayoutKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
