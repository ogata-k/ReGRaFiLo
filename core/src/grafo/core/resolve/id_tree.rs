use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
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
}

impl<Id: Debug + Eq + Copy> IdTree<Id> {
    pub fn get_root_id(&self) -> Id {
        match self {
            IdTree::Root(root) => root.get_root_id(),
            IdTree::None => panic!("{}", IdTreeError::<Id>::NotInitialized),
        }
    }

    pub fn push_id(&mut self, parent: Id, child: Id) {
        match self {
            IdTree::Root(root) => {
                if let Err(e) = root.push_id(parent, child) {
                    panic!("{}", e);
                }
            }
            IdTree::None => panic!("{}", IdTreeError::<Id>::NotInitialized),
        };
    }
}

/// This Error is always panic!!
#[derive(Debug, Eq, PartialEq, Clone)]
enum IdTreeError<Id> {
    NotInitialized,
    NotFindParentId(Id),
    AlreadyExistId(Id),
}
impl<Id: Debug> Display for IdTreeError<Id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
impl<Id: Debug> Error for IdTreeError<Id> {}

#[derive(Debug, Clone)]
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
}

impl<Id: Debug + Eq + Copy> IdTreeRoot<Id> {
    fn push_id(&mut self, parent: Id, child: Id) -> Result<(), IdTreeError<Id>> {
        self.root.push_id(parent, child)
    }
}

#[derive(Debug, Clone)]
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
