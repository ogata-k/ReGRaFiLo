//! Module for flatten ids for Node and NodeGrouping

use crate::util::Identity;

/// ids helper to flatten ids
#[derive(Eq, PartialEq, Clone)]
pub struct FlattenIds<'a, Id: 'a + Identity> {
    is_group: bool,
    root: &'a Id,
    children: Vec<&'a Id>,
}

impl<'a, Id: 'a + Identity> FlattenIds<'a, Id> {
    // ---
    // constructor
    // ---

    /// constructor for child
    pub(crate) fn _create_as_point(child_id: &'a Id) -> Self {
        FlattenIds {
            is_group: false,
            root: child_id,
            children: Vec::new(),
        }
    }

    /// constructor for flatten group
    pub(crate) fn _create_as_group(root_id: &'a Id, mut children: Vec<&'a Id>) -> Self {
        // Do sort to use binary search
        children.sort();
        // Do unique. But did deduped because uniqueness is ensured by the way of construction.
        // children.dedup();

        FlattenIds {
            is_group: true,
            root: root_id,
            children: children,
        }
    }

    // ---
    // getter
    // ---

    /// get root id
    pub fn get_root(&self) -> &Id {
        self.root
    }

    /// get grouping children
    pub fn get_children(&self) -> &[&'a Id] {
        self.children.as_slice()
    }

    // ---
    // setter
    // ---
    /// drain from the other
    pub fn drain_one(&mut self, other: Self) {
        let FlattenIds {
            is_group: _,
            root,
            children,
        } = other;

        self.is_group = true;
        self.children.push(root);
        self.children.extend(children);
        self.children.sort();
    }

    /// drain from the others
    pub fn drain<I: IntoIterator<Item = Self>>(&mut self, others: I) {
        for other in others.into_iter() {
            let FlattenIds {
                is_group: _,
                root,
                children,
            } = other;

            self.is_group = true;
            self.children.push(root);
            self.children.extend(children);
        }
        self.children.sort();
    }

    // ---
    // checker
    // ---

    /// check can be use as point
    pub fn is_point(&self) -> bool {
        !self.is_group
    }

    /// check can use as group
    pub fn is_group(&self) -> bool {
        self.is_group
    }

    // ---
    // check: self /\ other
    // ---

    /// check self root and children in other root and children
    pub fn is_part_of_other(&self, other: &Self) -> bool {
        if !(self.root == other.root || other.children.binary_search(&self.root).is_ok()) {
            return false;
        }
        for self_child in self.children.iter() {
            // self child is not other root, because self root is in other root or children and self child belongs to self root.
            if !(other.children.binary_search(&self_child).is_ok()) {
                return false;
            }
        }

        true
    }

    /// check other root and children in self root and children
    pub fn contains_other(&self, other: &Self) -> bool {
        other.is_part_of_other(self)
    }

    /// check intersection for self root and children and other root and children not exist
    pub fn has_no_intersection_other(&self, other: &Self) -> bool {
        if self.root == other.root || other.children.binary_search(&self.root).is_ok() {
            return false;
        }
        for self_child in self.children.iter() {
            if self_child == &other.root || other.children.binary_search(&self_child).is_ok() {
                return false;
            }
        }

        true
    }

    // ---
    // check: self /\ other children
    // ---

    /// check self root and children in other children
    pub fn is_part_of_other_children(&self, other: &Self) -> bool {
        if !(other.children.binary_search(&self.root).is_ok()) {
            return false;
        }
        for self_child in self.children.iter() {
            // self child is not other root, because self root is in other root or children and self child belongs to self root.
            if !(other.children.binary_search(&self_child).is_ok()) {
                return false;
            }
        }

        true
    }

    /// check other children in self root and children
    pub fn contains_other_children(&self, other: &Self) -> bool {
        other.children_is_part_of_other(self)
    }

    /// check intersection for self root and children and other chilren not exist
    pub fn has_no_intersection_other_children(&self, other: &Self) -> bool {
        if other.children.binary_search(&self.root).is_ok() {
            return false;
        }
        for self_child in self.children.iter() {
            if other.children.binary_search(&self_child).is_ok() {
                return false;
            }
        }

        true
    }

    // ---
    // check: self children /\ other
    // ---

    /// check self children in other root and children
    pub fn children_is_part_of_other(&self, other: &Self) -> bool {
        for self_child in self.children.iter() {
            if !(self_child == &other.root || other.children.binary_search(&self_child).is_ok()) {
                return false;
            }
        }

        true
    }

    /// check other root and children in self children
    pub fn children_contains_other(&self, other: &Self) -> bool {
        other.is_part_of_other_children(self)
    }

    /// check intersection for self children and other root and children not exist
    pub fn children_has_no_intersection_other(&self, other: &Self) -> bool {
        for self_child in self.children.iter() {
            if self_child == &other.root || other.children.binary_search(&self_child).is_ok() {
                return false;
            }
        }

        true
    }

    // ---
    // check: self children /\ other children
    // ---

    /// check self children in other children
    pub fn children_is_part_of_other_children(&self, other: &Self) -> bool {
        for self_child in self.children.iter() {
            if !(other.children.binary_search(&self_child).is_ok()) {
                return false;
            }
        }

        true
    }

    /// check other children in self children
    pub fn children_contains_other_children(&self, other: &Self) -> bool {
        other.children_is_part_of_other_children(self)
    }

    /// check intersection for self children and other children not exist
    pub fn children_has_no_intersection_other_children(&self, other: &Self) -> bool {
        for self_child in self.children.iter() {
            if other.children.binary_search(&self_child).is_ok() {
                return false;
            }
        }

        true
    }

    // ---
    // delete
    // ---
}
