//! attribute of ReGRaFiLo's item

use crate::grafo::core::layout::create_layout_key;
use crate::util::alias::{ItemIndex, RefIndex};
use crate::util::item_kind::ItemKind;
use crate::util::kind_key::KeyWithKind;
use crate::util::layout_kind::AttributeKind;
use std::borrow::Borrow;

/// triple of ItemKind, Index, Key
type AttributeRefKey<ItemKindKey> = KeyWithKind<ItemKindKey, KeyWithKind<ItemIndex, AttributeKind>>;

/// reference of Attribute
#[derive(Debug, Clone)]
pub struct AttributeRefIndex {
    reference_index: RefIndex<AttributeRefKey<ItemKind>, String>,
}

impl AttributeRefIndex {
    /// initialize
    pub fn new() -> Self {
        AttributeRefIndex::default()
    }

    //
    // helper
    //

    /// helper for a setter of string attribute
    pub fn push_attribute(
        &mut self,
        item_kind: ItemKind,
        attribute_kind: AttributeKind,
        index: ItemIndex,
        value: String,
    ) -> Option<String> {
        self.reference_index
            .insert(create_layout_key(item_kind, attribute_kind, index), value)
        // TODO return is overrided value. so return as Warning when use outside
    }

    /// helper for getter of string attribute
    pub fn get_attribute(
        &self,
        item_kind: ItemKind,
        attribute_kind: AttributeKind,
        index: ItemIndex,
    ) -> Option<&str> {
        self.reference_index
            .get(&create_layout_key(item_kind, attribute_kind, index))
            .map(|s| s.borrow())
    }

    /// helper for count by kind
    pub fn count_by(&self, item_kind: ItemKind, attribute_kind: AttributeKind) -> usize {
        self.reference_index
            .iter()
            .filter(|(k, _)| k.is_kind(item_kind) && k.key.key == attribute_kind)
            .count()
    }
}

impl Default for AttributeRefIndex {
    /// initialize without log
    fn default() -> Self {
        AttributeRefIndex {
            reference_index: RefIndex::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grafo::core::layout::attribute::AttributeRefIndex;
    use crate::util::item_kind::test::check_list;
    use crate::util::item_kind::ItemKind;
    use crate::util::layout_kind::AttributeKind;

    const ITERATE_COUNT: usize = 10;

    #[test]
    fn is_empty() {
        let ref_index = AttributeRefIndex::new();
        for key in check_list().iter() {
            assert_eq!(ref_index.count_by(*key, AttributeKind::Form), 0);
            assert_eq!(ref_index.count_by(*key, AttributeKind::Group), 0);
        }
    }

    #[test]
    fn form_count() {
        let mut ref_index_mut = AttributeRefIndex::new();
        let checker = ItemKind::Node;
        for i in 0..ITERATE_COUNT {
            ref_index_mut.push_attribute(checker, AttributeKind::Form, i, format!("{}", i));
        }
        let ref_index = ref_index_mut;
        for key in check_list().iter() {
            assert_eq!(
                ref_index.count_by(*key, AttributeKind::Form),
                if *key == checker { ITERATE_COUNT } else { 0 }
            );
            assert_eq!(ref_index.count_by(*key, AttributeKind::Group), 0);
        }
    }

    #[test]
    fn form_each_eq() {
        let mut ref_index_mut = AttributeRefIndex::new();
        let checker = ItemKind::Node;
        for i in 0..ITERATE_COUNT {
            ref_index_mut.push_attribute(checker, AttributeKind::Form, i, format!("{}", i));
        }
        let ref_index = ref_index_mut;
        for key in check_list().iter() {
            for i in 0..ITERATE_COUNT {
                if *key == checker {
                    let result = ref_index.get_attribute(*key, AttributeKind::Form, i);
                    assert!(result.is_some());
                    assert_eq!(result.unwrap(), format!("{}", i));
                } else {
                    assert_eq!(ref_index.get_attribute(*key, AttributeKind::Form, i), None);
                }
                assert_eq!(ref_index.get_attribute(*key, AttributeKind::Group, i), None);
            }
        }
    }
}
