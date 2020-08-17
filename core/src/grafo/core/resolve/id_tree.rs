use std::error::Error;
use std::fmt::Debug;

/// This Error is always panic!!
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdTreeError<Id> {
    NotInitialized,
    NotFindParentId(Id),
    AlreadyExistId(Id),
}

impl<Id: std::fmt::Display> std::fmt::Display for IdTreeError<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO
        unimplemented!()
    }
}

impl<Id: Debug + std::fmt::Display> Error for IdTreeError<Id> {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdTree<Id: Eq + Copy> {
    Root(IdTreeRoot<Id>),
    None,
}

impl<Id: Eq + Copy + std::fmt::Display> std::fmt::Display for IdTree<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            IdTree::Root(t) => write!(f, "Tree{}", t),
            IdTree::None => write!(f, "Empty"),
        }
    }
}

impl<Id: Eq + Copy> Default for IdTree<Id> {
    fn default() -> Self {
        IdTree::None
    }
}

impl<Id: Eq + Copy> IdTree<Id> {
    pub fn new(root: Id) -> Self {
        Self::Root(IdTreeRoot::new(root))
    }

    pub fn contains_id(&self, id: Id) -> bool {
        match self {
            IdTree::Root(root) => root.contains_id(id),
            IdTree::None => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            IdTree::Root(_) => false,
            IdTree::None => true,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            IdTree::Root(_) => true,
            IdTree::None => false,
        }
    }

    /// get parent and ancestors id without target id
    pub fn get_ancestor_ids(&self, id: Id) -> Option<Vec<Id>> {
        match self {
            IdTree::Root(tree) => tree.get_ancestor_ids(id),
            IdTree::None => None,
        }
    }
}

impl<Id: Debug + Eq + Copy> IdTree<Id> {
    pub fn insert_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
        match self {
            IdTree::Root(root) => root.insert_id(parent, child),
            IdTree::None => Err(IdTreeError::<Id>::NotInitialized),
        }
    }

    pub fn get_root_id(&self) -> Result<Id, IdTreeError<Id>> {
        match self {
            IdTree::Root(root) => Ok(root.get_root_id()),
            IdTree::None => Err(IdTreeError::<Id>::NotInitialized),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IdTreeRoot<Id: Eq + Copy> {
    root: UniqueTree<Id>,
}

impl<Id: Eq + Copy + std::fmt::Display> std::fmt::Display for IdTreeRoot<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.root.children.is_empty() {
            write!(f, "({})", self.root)
        } else {
            write!(f, "{}", self.root)
        }
    }
}

impl<Id: Eq + Copy> IdTreeRoot<Id> {
    fn new(root: Id) -> Self {
        Self {
            root: UniqueTree::new(root),
        }
    }

    fn contains_id(&self, id: Id) -> bool {
        self.root.contains_id(id)
    }

    fn get_root_id(&self) -> Id {
        self.root.node
    }

    /// get ids from root to target id without target id
    fn get_ancestor_ids(&self, target_id: Id) -> Option<Vec<Id>> {
        let mut ids: Vec<Id> = Vec::new();
        if self.root.collect_ids_self_to(&mut ids, target_id) {
            Some(ids)
        } else {
            None
        }
    }
}

impl<Id: Debug + Eq + Copy> IdTreeRoot<Id> {
    fn insert_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
        self.root.insert_id(parent, child)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct UniqueTree<Id: Eq + Copy> {
    node: Id,
    children: Vec<Box<UniqueTree<Id>>>,
}

impl<Id: Eq + Copy + std::fmt::Display> std::fmt::Display for UniqueTree<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.children.len() {
            0 => write!(f, "{}", self.node),
            1 => {
                let child = &self.children[0];
                match child.children.len() {
                    0 => write!(f, "({}: {})", self.node, child.node),
                    _ => write!(f, "({}: {})", self.node, child),
                }
            }
            _ => {
                write!(f, "({}: (", self.node)?;
                for (i, child) in self.children.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{}", child)?;
                    } else {
                        write!(f, ", {}", child)?;
                    }
                }
                write!(f, "))")
            }
        }
    }
}

impl<Id: Eq + Copy> UniqueTree<Id> {
    fn new(root: Id) -> Self {
        Self {
            node: root,
            children: Vec::new(),
        }
    }

    fn contains_id(&self, id: Id) -> bool {
        if self.node == id {
            return true;
        }
        for child in self.children.as_slice() {
            if child.contains_id(id) {
                return true;
            }
        }
        false
    }

    fn find_as_mut(&mut self, id: Id) -> Option<&mut Self> {
        if self.node == id {
            return Some(self);
        }
        for child in self.children.as_mut_slice() {
            let child_result = child.find_as_mut(id);
            if child_result.is_some() {
                return child_result;
            }
        }
        None
    }

    /// get ids from root to target id without target id
    fn collect_ids_self_to(&self, collected_ids: &mut Vec<Id>, target_id: Id) -> bool {
        if target_id == self.node {
            return true;
        }
        for child in self.children.iter() {
            if child.collect_ids_self_to(collected_ids, target_id) {
                collected_ids.insert(0, self.node);
                return true;
            }
        }
        false
    }
}

impl<Id: Debug + Eq + Copy> UniqueTree<Id> {
    fn insert_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
        if self.contains_id(child) {
            return Err(IdTreeError::AlreadyExistId(child));
        }
        if let Some(parent_node) = self.find_as_mut(parent) {
            parent_node.children.push(Box::new(UniqueTree {
                node: child,
                children: Vec::new(),
            }));
            return Ok(());
        }
        Err(IdTreeError::NotFindParentId(parent))
    }
}

#[cfg(test)]
mod test {
    use crate::grafo::IdTree;

    fn new_empty_tree() -> IdTree<u8> {
        IdTree::None
    }

    #[allow(unused_must_use)]
    fn new_tree_template() -> IdTree<u8> {
        //
        //       0
        //     / | \
        //    1  2  3
        //   /|  |
        //  4 5  6
        //  |
        //  7
        //

        let mut tree = IdTree::new(0);
        tree.insert_id(0, 1);
        tree.insert_id(0, 2);
        tree.insert_id(0, 3);
        tree.insert_id(1, 4);
        tree.insert_id(1, 5);
        tree.insert_id(2, 6);
        tree.insert_id(4, 7);
        tree
    }

    #[test]
    fn not_found_ancestor_in_none_tree() {
        let tree = new_empty_tree();
        assert!(!tree.contains_id(0));
        assert_eq!(tree.get_ancestor_ids(0), None);
    }

    #[test]
    fn not_found_ancestor_in_tree_template() {
        let tree = new_tree_template();
        assert!(!tree.contains_id(100));
        assert_eq!(tree.get_ancestor_ids(100), None);
    }

    #[test]
    fn empty_tree_to_string() {
        let tree = new_empty_tree();
        assert_eq!(tree.to_string(), "Empty".to_string());
    }

    #[test]
    fn only_root_tree_to_string() {
        let tree = IdTree::new(0);
        assert_eq!(tree.to_string(), "Tree(0)".to_string());
    }

    #[test]
    fn root_has_one_child_tree_to_string() {
        let mut tree = IdTree::new(0);
        tree.insert_id(0, 1).unwrap();
        assert_eq!(tree.to_string(), "Tree(0: 1)".to_string());
    }

    #[test]
    fn root_has_two_child_tree_to_string() {
        let mut tree = IdTree::new(0);
        tree.insert_id(0, 1).unwrap();
        tree.insert_id(0, 2).unwrap();
        assert_eq!(tree.to_string(), "Tree(0: (1, 2))".to_string());
    }

    #[test]
    fn root_has_child_has_child_tree_to_string() {
        let mut tree = IdTree::new(0);
        tree.insert_id(0, 1).unwrap();
        tree.insert_id(1, 2).unwrap();
        assert_eq!(tree.to_string(), "Tree(0: (1: 2))".to_string());
    }

    #[test]
    fn not_empty_tree_to_string() {
        let tree = new_tree_template();
        assert_eq!(
            tree.to_string(),
            "Tree(0: ((1: ((4: 7), 5)), (2: 6), 3))".to_string()
        );
    }

    #[test]
    fn found_root() {
        let tree = new_tree_template();
        assert!(tree.contains_id(0));
        assert_eq!(tree.get_ancestor_ids(0), Some(vec!()));
    }

    #[test]
    fn found_ancestor1() {
        let tree = new_tree_template();
        assert!(tree.contains_id(4));
        assert_eq!(tree.get_ancestor_ids(4), Some(vec!(0, 1)));
    }

    #[test]
    fn found_ancestor2() {
        let tree = new_tree_template();
        assert!(tree.contains_id(5));
        assert_eq!(tree.get_ancestor_ids(5), Some(vec!(0, 1)));
    }

    #[test]
    fn found_ancestor3() {
        let tree = new_tree_template();
        assert!(tree.contains_id(3));
        assert_eq!(tree.get_ancestor_ids(3), Some(vec!(0)));
    }

    #[test]
    fn found_deep_ancestor() {
        let tree = new_tree_template();
        assert!(tree.contains_id(7));
        assert_eq!(tree.get_ancestor_ids(7), Some(vec!(0, 1, 4)));
    }
}
