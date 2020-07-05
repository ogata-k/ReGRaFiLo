//! attribute of ReGRaFiLo's item

use regrafilo_util::log::{KindGroup4Logger, KindKey4Logger, Logger};

use crate::util::item_arena::ItemIndex;
use crate::util::kind_key::KindKey;
use crate::util::RefIndex;

pub(crate) type AttributeRefKey<ItemKindKey> =
    KindKey<ItemKindKey, KindKey<ItemIndex, AttributeKey>>;

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
        "{} for Form of {}",
        AttributeKey::kind_group(),
        item_kind.get_kind_string()
    )
}

/// key of Attribute
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum AttributeKey {
    Form,
    Group,
}

impl KindGroup4Logger for AttributeKey {
    fn kind_group() -> &'static str {
        "Attribute"
    }
}

/// value of Attribute. but user wouldn't use
#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum AttributeValue {
    String(String),
}

/// builder for reference of Attribute
pub(crate) struct AttributeRefIndexBuilder<ItemKindKey: Ord + Eq + Copy> {
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
    /// helper for setter of string attribute
    fn push_attribute_string(
        &mut self,
        key: AttributeKey,
        item_kind: ItemKindKey,
        value: &str,
        index: ItemIndex,
    ) -> Option<String> {
        let f = logger_format(item_kind);
        let result = self.ref_index.insert(
            create_ref_key(item_kind, key, index),
            AttributeValue::String(value.to_string()),
        );
        Logger::with_name_push_log(&f, value, index);
        result.map(|v| {
            if let AttributeValue::String(s) = v {
                Logger::override_log(&f, &s);
                return s;
            }
            Logger::inconsistent(&f, v);
            unreachable!();
        })
    }

    /// set form attribute
    pub fn push_form_name(
        &mut self,
        item_kind: ItemKindKey,
        name: &str,
        index: ItemIndex,
    ) -> Option<String> {
        self.push_attribute_string(AttributeKey::Form, item_kind, name, index)
    }

    /// set group attribute
    pub fn push_group_name(
        &mut self,
        item_kind: ItemKindKey,
        name: &str,
        index: ItemIndex,
    ) -> Option<String> {
        self.push_attribute_string(AttributeKey::Group, item_kind, name, index)
    }
}

/// reference of Attribute
pub(crate) struct AttributeRefIndex<ItemKindKey: Ord + Eq + Copy> {
    ref_index: RefIndex<AttributeRefKey<ItemKindKey>, AttributeValue>,
}

impl<ItemKindKey: Ord + Eq + Copy + KindKey4Logger> AttributeRefIndex<ItemKindKey> {
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

    /// get form attribute
    pub fn get_form_name(&mut self, item_kind: ItemKindKey, index: ItemIndex) -> Option<&str> {
        self.get_attribute_string(AttributeKey::Form, item_kind, index)
    }

    /// get group attribute
    pub fn get_group_name(&mut self, item_kind: ItemKindKey, index: ItemIndex) -> Option<&str> {
        self.get_attribute_string(AttributeKey::Group, item_kind, index)
    }
}

// TODO example and test
