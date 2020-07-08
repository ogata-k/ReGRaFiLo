//! attribute of ReGRaFiLo's item

use regrafilo_util::log::{GroupKind4Logger, KeyKind4Logger, KindBase, Logger};

use crate::util::item_arena::ItemIndex;
use crate::util::kind_key::KindKey;
use crate::util::RefIndex;

/// triple of ItemKind, Index, Key
pub type AttributeRefKey<ItemKindKey> = KindKey<ItemKindKey, KindKey<ItemIndex, AttributeKey>>;

/// helper for make reference key
fn create_ref_key<ItemKindKey: Copy>(
    item_kind: ItemKindKey,
    key: AttributeKey,
    index: ItemIndex,
) -> AttributeRefKey<ItemKindKey> {
    KindKey::new(item_kind, KindKey::new(index, key))
}

/// key of Attribute
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum AttributeKey {
    Form,
    Group,
}

impl GroupKind4Logger for AttributeKey {
    fn group_kind_string() -> &'static str {
        "Attribute"
    }
}

impl KeyKind4Logger for AttributeKey {
    fn key_kind_string(&self) -> &'static str {
        use AttributeKey::*;
        match self {
            Form => "Form",
            Group => "Group",
        }
    }
}

impl KindBase for AttributeKey {}

/// value of Attribute. but user wouldn't use
#[derive(Debug, Eq, PartialEq, Clone)]
enum AttributeValue {
    String(String),
}

/// reference of Attribute
pub struct AttributeRefIndex<ItemKindKey: KindBase> {
    ref_index: RefIndex<AttributeRefKey<ItemKindKey>, AttributeValue>,
}

impl<ItemKindKey: KindBase> AttributeRefIndex<ItemKindKey> {
    /// initialize
    pub fn new() -> Self {
        Logger::initializer_log(
            AttributeKey::group_kind_string(),
            Some(ItemKindKey::group_kind_string()),
        );
        AttributeRefIndex::default()
    }

    //
    // helper
    //

    /// helper for a setter of string attribute
    fn push_attribute_string(
        &mut self,
        key: AttributeKey,
        item_kind: ItemKindKey,
        index: ItemIndex,
        value: String,
    ) -> Option<String> {
        Logger::with_name_push_log(
            AttributeKey::group_kind_string(),
            item_kind.key_kind_string(),
            &value,
            index,
        );
        let result = self.ref_index.insert(
            create_ref_key(item_kind, key, index),
            AttributeValue::String(value),
        );
        result.map(|v| {
            #[allow(irrefutable_let_patterns)]
            if let AttributeValue::String(s) = v {
                Logger::override_value_log(
                    AttributeKey::group_kind_string(),
                    item_kind.key_kind_string(),
                    &s,
                );
                return s;
            }
            Logger::inconsistent(
                AttributeKey::group_kind_string(),
                item_kind.key_kind_string(),
                v,
            );
            unreachable!();
        })
    }

    /// helper for getter of string attribute
    fn get_attribute_string(
        &self,
        key: AttributeKey,
        item_kind: ItemKindKey,
        index: ItemIndex,
    ) -> Option<&str> {
        let result = self.ref_index.get(&create_ref_key(item_kind, key, index));
        result.map(|v| {
            #[allow(irrefutable_let_patterns)]
            if let AttributeValue::String(s) = v {
                return s.as_str();
            }
            Logger::inconsistent(
                AttributeKey::group_kind_string(),
                item_kind.key_kind_string(),
                v,
            );
            unreachable!();
        })
    }

    /// helper for count by kind
    fn count_by(&self, item_kind: ItemKindKey, key: AttributeKey) -> usize {
        self.ref_index
            .iter()
            .filter(|(k, _)| k.is_kind(item_kind) && k.key.key == key)
            .count()
    }

    //
    // setter
    //

    /// set form attribute
    pub fn push_form_name(
        &mut self,
        item_kind: ItemKindKey,
        index: ItemIndex,
        name: String,
    ) -> Option<String> {
        self.push_attribute_string(AttributeKey::Form, item_kind, index, name)
    }

    /// set group attribute
    pub fn push_group_name(
        &mut self,
        item_kind: ItemKindKey,
        index: ItemIndex,
        name: String,
    ) -> Option<String> {
        self.push_attribute_string(AttributeKey::Group, item_kind, index, name)
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
            ref_index: RefIndex::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::layout::AttributeRefIndex;
    use regrafilo_util::log::{GroupKind4Logger, KeyKind4Logger, KindBase, Logger};

    const COUNT: usize = 10;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
    enum ItemKindKey {
        Group,
        Node,
        Edge,
    }

    impl GroupKind4Logger for ItemKindKey {
        fn group_kind_string() -> &'static str {
            "Attribute"
        }
    }

    impl KeyKind4Logger for ItemKindKey {
        fn key_kind_string(&self) -> &'static str {
            use ItemKindKey::*;
            match self {
                Group => "Group",
                Node => "Node",
                Edge => "Edge",
            }
        }
    }

    impl KindBase for ItemKindKey {}

    fn check_key_list() -> Vec<ItemKindKey> {
        use ItemKindKey::*;
        vec![Group, Node, Edge]
    }

    #[test]
    fn is_empty() {
        Logger::init(true);
        let ref_index = AttributeRefIndex::<ItemKindKey>::new();
        for key in check_key_list().iter() {
            assert_eq!(ref_index.count_form(*key), 0);
            assert_eq!(ref_index.count_group(*key), 0);
        }
    }

    #[test]
    fn form_count() {
        Logger::init(true);
        let mut ref_index_mut = AttributeRefIndex::<ItemKindKey>::new();
        let checker = ItemKindKey::Node;
        for i in 0..COUNT {
            ref_index_mut.push_form_name(checker, i, format!("{}", i));
        }
        let ref_index = ref_index_mut;
        for key in check_key_list().iter() {
            assert_eq!(
                ref_index.count_form(*key),
                if *key == checker { COUNT } else { 0 }
            );
            assert_eq!(ref_index.count_group(*key), 0);
        }
    }

    #[test]
    fn form_each_eq() {
        Logger::init(true);
        let mut ref_index_mut = AttributeRefIndex::<ItemKindKey>::new();
        let checker = ItemKindKey::Node;
        for i in 0..COUNT {
            ref_index_mut.push_form_name(checker, i, format!("{}", i));
        }
        let ref_index = ref_index_mut;
        for key in check_key_list().iter() {
            for i in 0..COUNT {
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
