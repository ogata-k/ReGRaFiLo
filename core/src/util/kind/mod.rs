mod attribute_kind;
mod graph_item_kind;
mod layout_item_kind;
mod name_kind;

pub use attribute_kind::*;
pub use graph_item_kind::*;
pub use layout_item_kind::*;
pub use name_kind::*;

#[cfg(test)]
pub mod test {
    use crate::util::kind::GraphItemKind;

    pub fn graph_item_check_list() -> Vec<GraphItemKind> {
        use GraphItemKind::*;
        vec![Group, Node, Edge]
    }
}
