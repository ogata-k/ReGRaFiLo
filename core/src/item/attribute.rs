//! attribute of ReGRaFiLo's item

use regrafilo_util::log::{KindGroup4Logger, KindKey4Logger, Logger};

use crate::util::item_arena::ItemIndex;
use crate::util::kind_key::KindKey;
use crate::util::RefIndex;

pub type AttributeRefKey<ItemKindKey> = KindKey<ItemKindKey, KindKey<ItemIndex, AttributeKey>>;

/// helper for make reference key
fn create_ref_key<ItemKindKey: Copy>(
    item_kind: ItemKindKey,
    key: AttributeKey,
    index: ItemIndex,
) -> AttributeRefKey<ItemKindKey> {
    KindKey::new(item_kind, KindKey::new(index, key))
}

/// helper for to format of logger
fn logger_format<ItemKindKey: KindKey4Logger>(item_kind: ItemKindKey) -> String {
    format!(
        "{} for form of {}",
        AttributeKey::kind_group(),
        item_kind.get_kind_string()
    )
}

/// key of Attribute
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum AttributeKey {
    Form,
    Group,
}

impl KindGroup4Logger for AttributeKey {
    fn kind_group() -> &'static str {
        "attribute"
    }
}

/// value of Attribute. but user wouldn't use
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AttributeValue {
    String(String),
}

/// builder for reference of Attribute
pub struct AttributeRefIndexBuilder<ItemKindKey: Ord + Eq + Copy> {
    ref_index: RefIndex<AttributeRefKey<ItemKindKey>, AttributeValue>,
}

impl<ItemKindKey: Ord + Eq + Copy> AttributeRefIndexBuilder<ItemKindKey> {
    /// initializer
    pub fn new() -> Self {
        Logger::builder_start_log(AttributeKey::kind_group());
        AttributeRefIndexBuilder {
            ref_index: RefIndex::new(),
        }
    }

    /// build
    pub fn build(self) -> AttributeRefIndex<ItemKindKey> {
        let ari = AttributeRefIndex {
            ref_index: self.ref_index,
        };
        Logger::builder_finish_log(AttributeKey::kind_group());
        ari
    }
}

impl<ItemKindKey: Ord + Eq + Copy + KindKey4Logger> AttributeRefIndexBuilder<ItemKindKey> {
    //
    // helper
    //

    /// helper for setter of string attribute
    fn push_attribute_string(
        &mut self,
        key: AttributeKey,
        item_kind: ItemKindKey,
        index: ItemIndex,
        value: String,
    ) -> Option<String> {
        let f = logger_format(item_kind);
        Logger::with_name_push_log(&f, &value, index);
        let result = self.ref_index.insert(
            create_ref_key(item_kind, key, index),
            AttributeValue::String(value),
        );
        result.map(|v| {
            if let AttributeValue::String(s) = v {
                Logger::override_log(&f, &s);
                return s;
            }
            Logger::inconsistent(&f, v);
            unreachable!();
        })
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
}

/// reference of Attribute
pub struct AttributeRefIndex<ItemKindKey: Ord + Eq + Copy> {
    ref_index: RefIndex<AttributeRefKey<ItemKindKey>, AttributeValue>,
}

impl<ItemKindKey: Ord + Eq + Copy + KindKey4Logger> AttributeRefIndex<ItemKindKey> {
    //
    // helper
    //

    /// helper for getter of string attribute
    fn get_attribute_string(
        &self,
        key: AttributeKey,
        item_kind: ItemKindKey,
        index: ItemIndex,
    ) -> Option<&str> {
        let result = self.ref_index.get(&create_ref_key(item_kind, key, index));
        result.map(|v| {
            if let AttributeValue::String(s) = v {
                return s.as_str();
            }
            Logger::inconsistent(&logger_format(item_kind), v);
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
    // count
    //

    pub fn count_form(&self, item_kind: ItemKindKey) -> usize {
        self.count_by(item_kind, AttributeKey::Form)
    }

    pub fn count_group(&self, item_kind: ItemKindKey) -> usize {
        self.count_by(item_kind, AttributeKey::Group)
    }
}

#[cfg(test)]
mod test {
    use regrafilo_util::log::{KindKey4Logger, Logger};

    use crate::item::AttributeRefIndexBuilder;

    const COUNT: usize = 10;

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
    enum ItemKindKey {
        Group,
        Node,
        Edge,
    }

    impl KindKey4Logger for ItemKindKey {
        fn get_kind_string(&self) -> String {
            match self {
                ItemKindKey::Group => "group",
                ItemKindKey::Node => "node",
                ItemKindKey::Edge => "edge",
            }
            .to_string()
        }
    }

    fn check_key_list() -> Vec<ItemKindKey> {
        use ItemKindKey::*;
        vec![Group, Node, Edge]
    }

    #[test]
    fn is_empty() {
        Logger::init(true);
        let ref_index = AttributeRefIndexBuilder::<ItemKindKey>::new().build();
        for key in check_key_list().iter() {
            assert_eq!(ref_index.count_form(*key), 0);
            assert_eq!(ref_index.count_group(*key), 0);
        }
    }

    #[test]
    fn form_count() {
        Logger::init(true);
        let mut builder = AttributeRefIndexBuilder::<ItemKindKey>::new();
        let checker = ItemKindKey::Node;
        for i in 0..COUNT {
            builder.push_form_name(checker, i, format!("{}", i));
        }
        let ref_index = builder.build();
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
        let mut builder = AttributeRefIndexBuilder::<ItemKindKey>::new();
        let checker = ItemKindKey::Node;
        for i in 0..COUNT {
            builder.push_form_name(checker, i, format!("{}", i));
        }
        let ref_index = builder.build();
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
