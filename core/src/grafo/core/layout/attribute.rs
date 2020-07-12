//! attribute of ReGRaFiLo's item

use crate::event::Event::{OverrideValue, PushValue};
use crate::event::{Event, ItemEventKind, Visitor};
use crate::grafo::core::layout::create_layout_key;
use crate::util::alias::{ItemIndex, RefIndex};
use crate::util::kind_key::KeyWithKind;
use crate::util::util_trait::KindBase;

/// triple of ItemKind, Index, Key
type AttributeRefKey<ItemKindKey> = KeyWithKind<ItemKindKey, KeyWithKind<ItemIndex, AttributeKey>>;

/// key of Attribute
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum AttributeKey {
    Form,
    Group,
}

impl KindBase for AttributeKey {}

/// value of Attribute. but user wouldn't use
#[derive(Debug, Eq, PartialEq, Clone)]
enum AttributeValue {
    String(String),
}

/// reference of Attribute
pub struct AttributeRefIndex<ItemKindKey: KindBase> {
    reference_index: RefIndex<AttributeRefKey<ItemKindKey>, AttributeValue>,
}

impl<ItemKindKey: KindBase + Into<ItemEventKind>> AttributeRefIndex<ItemKindKey> {
    /// initialize
    pub fn new<V: Visitor>(visitor: &mut V) -> Self {
        visitor.visit(&Event::InitializeAttribute);
        AttributeRefIndex::default()
    }

    //
    // helper
    //

    /// helper for a setter of string attribute
    fn push_attribute_string<V: Visitor>(
        &mut self,
        visitor: &mut V,
        key: AttributeKey,
        item_kind: ItemKindKey,
        index: ItemIndex,
        value: String,
    ) -> Option<String> {
        visitor.visit(&PushValue(item_kind.into(), index, &value));
        let result = self.reference_index.insert(
            create_layout_key(item_kind, key, index),
            AttributeValue::String(value),
        );
        result.map(|v| {
            if let AttributeValue::String(s) = v {
                visitor.visit(&OverrideValue(item_kind.into(), index, &s));
                return s;
            }
            unreachable!(
                "inconsistent attribute value: ({:?},{},{:?})",
                item_kind.into(),
                index,
                v
            );
        })
    }

    /// helper for getter of string attribute
    fn get_attribute_string(
        &self,
        key: AttributeKey,
        item_kind: ItemKindKey,
        index: ItemIndex,
    ) -> Option<&str> {
        let result = self
            .reference_index
            .get(&create_layout_key(item_kind, key, index));
        result.map(|v| {
            if let AttributeValue::String(s) = v {
                return s.as_str();
            }
            unreachable!(
                "inconsistent attribute value: ({:?},{},{:?})",
                item_kind.into(),
                index,
                v
            );
        })
    }

    /// helper for count by kind
    fn count_by(&self, item_kind: ItemKindKey, key: AttributeKey) -> usize {
        self.reference_index
            .iter()
            .filter(|(k, _)| k.is_kind(item_kind) && k.key.key == key)
            .count()
    }

    //
    // setter
    //

    /// set form attribute
    pub fn push_form_name<V: Visitor>(
        &mut self,
        visitor: &mut V,
        item_kind: ItemKindKey,
        index: ItemIndex,
        name: String,
    ) -> Option<String> {
        self.push_attribute_string(visitor, AttributeKey::Form, item_kind, index, name)
    }

    /// set group attribute
    pub fn push_group_name<V: Visitor>(
        &mut self,
        visitor: &mut V,
        item_kind: ItemKindKey,
        index: ItemIndex,
        name: String,
    ) -> Option<String> {
        self.push_attribute_string(visitor, AttributeKey::Group, item_kind, index, name)
    }

    //
    // getter
    //

    /// get form attribute
    pub fn get_form_name(&self, item_kind: ItemKindKey, index: ItemIndex) -> Option<&str> {
        self.get_attribute_string(AttributeKey::Form, item_kind, index)
    }

    /// get group attribute
    pub fn get_group_name(&self, item_kind: ItemKindKey, index: ItemIndex) -> Option<&str> {
        self.get_attribute_string(AttributeKey::Group, item_kind, index)
    }

    //
    // reference
    //

    /// count for Form value
    pub fn count_form(&self, item_kind: ItemKindKey) -> usize {
        self.count_by(item_kind, AttributeKey::Form)
    }

    /// count for Group value
    pub fn count_group(&self, item_kind: ItemKindKey) -> usize {
        self.count_by(item_kind, AttributeKey::Group)
    }
}

impl<ItemKindKey: KindBase> Default for AttributeRefIndex<ItemKindKey> {
    /// initialize without log
    fn default() -> Self {
        AttributeRefIndex {
            reference_index: RefIndex::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::event::test::{check_list, Kind, Visitor, ITERATE_COUNT};
    use crate::grafo::core::layout::attribute::AttributeRefIndex;

    #[test]
    fn is_empty() {
        let mut v = Visitor::new();
        let ref_index = AttributeRefIndex::<Kind>::new(&mut v);
        for key in check_list().iter() {
            assert_eq!(ref_index.count_form(*key), 0);
            assert_eq!(ref_index.count_group(*key), 0);
        }
    }

    #[test]
    fn form_count() {
        let mut v = Visitor::new();
        let mut ref_index_mut = AttributeRefIndex::<Kind>::new(&mut v);
        let checker = Kind::Node;
        for i in 0..ITERATE_COUNT {
            ref_index_mut.push_form_name(&mut v, checker, i, format!("{}", i));
        }
        let ref_index = ref_index_mut;
        for key in check_list().iter() {
            assert_eq!(
                ref_index.count_form(*key),
                if *key == checker { ITERATE_COUNT } else { 0 }
            );
            assert_eq!(ref_index.count_group(*key), 0);
        }
    }

    #[test]
    fn form_each_eq() {
        let mut v = Visitor::new();
        let mut ref_index_mut = AttributeRefIndex::<Kind>::new(&mut v);
        let checker = Kind::Node;
        for i in 0..ITERATE_COUNT {
            ref_index_mut.push_form_name(&mut v, checker, i, format!("{}", i));
        }
        let ref_index = ref_index_mut;
        for key in check_list().iter() {
            for i in 0..ITERATE_COUNT {
                if *key == checker {
                    let result = ref_index.get_form_name(*key, i);
                    assert!(result.is_some());
                    assert_eq!(result.unwrap(), format!("{}", i));
                } else {
                    assert_eq!(ref_index.get_form_name(*key, i), None);
                }
                assert_eq!(ref_index.get_group_name(*key, i), None);
            }
        }
    }
}
