//! module for type of item

/// type of item
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum ItemKind {
    Group,
    Node,
    Edge,
}

fn item_kind_to_str(item_kind: &ItemKind) -> &str {
    match item_kind {
        ItemKind::Group => "Group",
        ItemKind::Node => "Node",
        ItemKind::Edge => "Edge",
    }
}
impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", item_kind_to_str(&self))
    }
}

#[cfg(test)]
pub mod test {
    use crate::util::item_kind::ItemKind;

    pub fn check_list() -> Vec<ItemKind> {
        use ItemKind::*;
        vec![Group, Node, Edge]
    }
}
