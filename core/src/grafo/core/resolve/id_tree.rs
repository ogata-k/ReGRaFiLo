use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::grafo::ResolverError;
use crate::util::alias::GroupId;

/// This Error is always panic!!
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdTreeError<Id> {
    NotInitialized,
    NotFindParentId(Id),
    AlreadyExistId(Id),
}

impl<Id: Debug + Display> Display for IdTreeError<Id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Id: Debug + Display> Error for IdTreeError<Id> {}

impl Into<ResolverError> for IdTreeError<GroupId> {
    fn into(self) -> ResolverError {
        match self {
            IdTreeError::NotInitialized => ResolverError::NotInitialized,
            IdTreeError::NotFindParentId(id) => ResolverError::NotFindParentId(id),
            IdTreeError::AlreadyExistId(id) => ResolverError::AlreadyExistId(id),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum IdTree<Id: Eq + Copy> {
    Root(IdTreeRoot<Id>),
    None,
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
    pub fn push_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
        match self {
            IdTree::Root(root) => root.push_id(parent, child),
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
    fn push_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
        self.root.push_id(parent, child)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct UniqueTree<Id: Eq + Copy> {
    node: Id,
    children: Vec<Box<UniqueTree<Id>>>,
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
    fn push_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
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

    fn new_none_tree() -> IdTree<u8> {
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
        tree.push_id(0, 1);
        tree.push_id(0, 2);
        tree.push_id(0, 3);
        tree.push_id(1, 4);
        tree.push_id(1, 5);
        tree.push_id(2, 6);
        tree.push_id(4, 7);
        tree
    }

    #[test]
    fn not_found_ancestor_in_none_tree() {
        let tree = new_none_tree();
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
