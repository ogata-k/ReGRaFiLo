use crate::grafo::core::refindex::error::NameRefWarning;
use crate::util::alias::RefIndex;
use crate::util::kind_key::KeyWithKind;
use std::borrow::Cow;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Deref;

/// helper for make reference key for layout
fn create_layout_key<'a, Kind: Eq + Copy + Hash, S: Into<Cow<'a, str>>>(
    kind: Kind,
    name: S,
) -> KeyWithKind<Kind, Cow<'a, str>> {
    KeyWithKind::new(kind, name.into())
}

fn key_to_str<'a, 'b: 'a, Kind: Eq + Copy + Hash>(
    key: &'b KeyWithKind<Kind, Cow<'a, str>>,
) -> &'b str {
    key.key.deref()
}

#[derive(Debug, Clone)]
pub struct NameRefIndex<'a, Kind: Eq + Copy + Hash, Value: Eq + Copy> {
    reference_index: RefIndex<KeyWithKind<Kind, Cow<'a, str>>, Value>,
}

impl<'a, Kind: Eq + Copy + Hash, Value: Eq + Copy> NameRefIndex<'a, Kind, Value> {
    /// initialize
    pub fn new() -> Self {
        NameRefIndex::default()
    }
}

impl<'a, Kind: Debug + Display + Eq + Copy + Hash, Value: Eq + Copy> NameRefIndex<'a, Kind, Value> {
    //
    // helper
    //

    /// helper for a setter of string attribute
    pub fn push_name<S: Into<Cow<'a, str>> + Clone>(
        &mut self,
        kind: Kind,
        name: S,
        item_id: Value,
    ) -> Result<(), NameRefWarning<Kind>> {
        let key = create_layout_key(kind, name);
        if self.reference_index.contains_key(&key) {
            let s = key_to_str(&key).to_string();
            self.reference_index.insert(key, item_id);
            return Err(NameRefWarning::Override(kind, s));
        }
        self.reference_index.insert(key, item_id);
        Ok(())
    }
}

impl<'a, Kind: Eq + Copy + Hash, Value: Eq + Copy> NameRefIndex<'a, Kind, Value> {
    /// helper for getter of string attribute
    pub fn get_value<S: Into<Cow<'a, str>>>(&self, kind: Kind, name: S) -> Option<&Value> {
        self.reference_index
            .get(&create_layout_key(kind, name.into()))
    }

    /// helper for count by kind
    pub fn count_by(&self, kind: Kind) -> usize {
        self.reference_index
            .iter()
            .filter(|(k, _)| k.is_kind(kind))
            .count()
    }
}

impl<'a, Kind: Eq + Copy + Hash, Value: Eq + Copy> Default for NameRefIndex<'a, Kind, Value> {
    /// initialize without log
    fn default() -> Self {
        NameRefIndex {
            reference_index: RefIndex::default(),
        }
    }
}

// TODO Test
