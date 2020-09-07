//! module of hierarchical id's tree

use std::error::Error;
use std::fmt::Debug;

/// error of hierarchical id's tree
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdTreeError<Id> {
    /// error occurred when use self, before not initialized yet
    NotInitialized,
    /// not found target's parent id
    NotFindParentId(Id),
    /// already exist specified target id
    AlreadyExistId(Id),
}

impl<Id: std::fmt::Display> std::fmt::Display for IdTreeError<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdTreeError::NotInitialized => write!(f, "hierarchy of ids is not initialized"),
            IdTreeError::NotFindParentId(id) => write!(f, "not found parent by id {}", id),
            IdTreeError::AlreadyExistId(id) => write!(f, "insert id {} already exist", id),
        }
    }
}

impl<Id: Debug + std::fmt::Display> Error for IdTreeError<Id> {}

/// hierarchical id's tree<br/>
/// The hierarchical tree structure of id that cannot be used unless initialized
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdTree<Id: Eq + Copy> {
    /// root node for this tree
    Root(IdTreeRoot<Id>),
    /// not initialized tree
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
    /// initializer for id's hierarchical tree with root's id which is specified
    pub fn new(root: Id) -> Self {
        Self::Root(IdTreeRoot::new(root))
    }

    /// check self tree contains specified id
    pub fn contains_id(&self, id: Id) -> bool {
        match self {
            IdTree::Root(root) => root.contains_id(id),
            IdTree::None => false,
        }
    }

    /// get parent and ancestors id without target id
    pub fn get_ancestor_ids(&self, id: Id) -> Vec<Id> {
        match self {
            IdTree::Root(tree) => tree.get_ancestor_ids(id),
            IdTree::None => Default::default(),
        }
    }

    /// get children's id list
    pub fn get_child_ids(&self, id: Id) -> Vec<Id> {
        match self {
            IdTree::Root(tree) => tree.get_child_ids(id),
            IdTree::None => Default::default(),
        }
    }

    /// get id list of children and children's children
    pub fn get_descendant_ids(&self, id: Id) -> Vec<Id> {
        match self {
            IdTree::Root(tree) => tree.get_descendant_ids(id),
            IdTree::None => Default::default(),
        }
    }
}

impl<Id: Debug + Eq + Copy> IdTree<Id> {
    /// insert id as parent's child
    pub fn insert_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
        match self {
            IdTree::Root(root) => root.insert_id(parent, child),
            IdTree::None => Err(IdTreeError::<Id>::NotInitialized),
        }
    }

    /// get self tree's root id
    pub fn get_root_id(&self) -> Result<Id, IdTreeError<Id>> {
        match self {
            IdTree::Root(root) => Ok(root.get_root_id()),
            IdTree::None => Err(IdTreeError::<Id>::NotInitialized),
        }
    }
}

/// root node for IDTree
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
    /// initializer for root node of IDTree
    fn new(root: Id) -> Self {
        Self {
            root: UniqueTree::new(root),
        }
    }

    /// check root node and it's children contains specified id
    fn contains_id(&self, id: Id) -> bool {
        self.root.contains_id(id)
    }

    /// get root id for self tree
    fn get_root_id(&self) -> Id {
        self.root.node
    }

    /// get ids from root to target id without target id
    fn get_ancestor_ids(&self, target_id: Id) -> Vec<Id> {
        let mut ids: Vec<Id> = Vec::new();
        if self.root.collect_ids_self_to(&mut ids, target_id) {
            ids
        } else {
            Default::default()
        }
    }

    /// get children's id list
    fn get_child_ids(&self, target_id: Id) -> Vec<Id> {
        self.root
            .find(target_id)
            .map(|node| node.get_children().iter().map(|child| child.node).collect())
            .unwrap_or_else(Default::default)
    }

    /// get id list of children and children's children
    fn get_descendant_ids(&self, target_id: Id) -> Vec<Id> {
        let mut result = vec![];
        if let Some(tree) = self.root.find(target_id) {
            for child in tree.children.iter() {
                child.get_descendant_ids(&mut result);
            }
        }
        result
    }
}

impl<Id: Debug + Eq + Copy> IdTreeRoot<Id> {
    /// insert parent's child
    fn insert_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
        self.root.insert_id(parent, child)
    }
}

/// this structure is rooted directed tree which can have unique id as node
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
    /// initializer with specified id as root node
    fn new(root: Id) -> Self {
        Self {
            node: root,
            children: Vec::new(),
        }
    }

    /// check self tree has specified id
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

    /// getter for children
    fn get_children(&self) -> &[Box<UniqueTree<Id>>] {
        self.children.as_ref()
    }

    /// get id list of children and children's children
    fn get_descendant_ids(&self, ids: &mut Vec<Id>) {
        ids.push(self.node);
        for child in self.children.iter() {
            child.get_descendant_ids(ids);
        }
    }

    /// find node by specified id
    fn find(&self, id: Id) -> Option<&Self> {
        if self.node == id {
            return Some(self);
        }
        for child in self.children.as_slice() {
            let child_result = child.find(id);
            if child_result.is_some() {
                return child_result;
            }
        }
        None
    }

    /// find node as mut by specified id
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
    /// insert parent's child
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
        assert_eq!(tree.get_ancestor_ids(0), vec![]);
    }

    #[test]
    fn not_found_ancestor_in_tree_template() {
        let tree = new_tree_template();
        assert!(!tree.contains_id(100));
        assert_eq!(tree.get_ancestor_ids(100), vec![]);
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
        assert_eq!(tree.get_ancestor_ids(0), vec![]);
    }

    #[test]
    fn found_ancestor1() {
        let tree = new_tree_template();
        assert!(tree.contains_id(4));
        assert_eq!(tree.get_ancestor_ids(4), vec!(0, 1));
    }

    #[test]
    fn found_ancestor2() {
        let tree = new_tree_template();
        assert!(tree.contains_id(5));
        assert_eq!(tree.get_ancestor_ids(5), vec!(0, 1));
    }

    #[test]
    fn found_ancestor3() {
        let tree = new_tree_template();
        assert!(tree.contains_id(3));
        assert_eq!(tree.get_ancestor_ids(3), vec!(0));
    }

    #[test]
    fn found_deep_ancestor() {
        let tree = new_tree_template();
        assert!(tree.contains_id(7));
        assert_eq!(tree.get_ancestor_ids(7), vec!(0, 1, 4));
    }

    #[test]
    fn has_no_children() {
        let tree = new_tree_template();
        assert_eq!(tree.get_child_ids(7), vec![]);
    }

    #[test]
    fn get_children() {
        let tree = new_tree_template();
        assert_eq!(tree.get_child_ids(0), vec![1, 2, 3]);
    }

    #[test]
    fn has_no_descendant() {
        let tree = new_tree_template();
        assert_eq!(tree.get_descendant_ids(7), vec![]);
    }

    #[test]
    fn get_descendant() {
        let tree = new_tree_template();
        assert_eq!(tree.get_descendant_ids(0), vec![1, 4, 7, 5, 2, 6, 3]);
    }
}
