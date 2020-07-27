use crate::grafo::GrafoError;
use crate::util::kind::{GraphItemKind, LayoutItemKind, NameKind};
use crate::util::kind_key::KeyWithKind;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;

/// helper for make reference key for layout
fn create_layout_key<'a, Kind: Eq + Copy + Hash, S: Into<Cow<'a, str>>>(
    kind: Kind,
    name_kind: NameKind,
    name: S,
) -> KeyWithKind<(Kind, NameKind), Cow<'a, str>> {
    KeyWithKind::new((kind, name_kind), name.into())
}

fn key_to_str<'a, 'b: 'a, Kind: Eq + Copy + Hash>(
    key: &'b KeyWithKind<Kind, Cow<'a, str>>,
) -> &'b str {
    key.key.deref()
}

/// references indexes
type RefIndex<K, V> = HashMap<K, V>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NameIdError<Kind> {
    AlreadyExist(Kind, NameKind, String),
    Override(Kind, NameKind, String),
    NotExist(Kind, NameKind, String),
}

impl<Kind: Display> Display for NameIdError<Kind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Kind: Debug + Display> Error for NameIdError<Kind> {}

#[derive(Debug, Clone)]
pub struct NameRefIndex<'a, Kind: Eq + Copy + Hash, Value: Eq + Copy + Hash> {
    reference_index: RefIndex<KeyWithKind<(Kind, NameKind), Cow<'a, str>>, Value>,
}

impl<'a, Kind: Eq + Copy + Hash, Value: Eq + Copy + Hash> NameRefIndex<'a, Kind, Value> {
    /// initialize
    pub fn new() -> Self {
        NameRefIndex::default()
    }
}

impl<'a, Kind: Debug + Display + Eq + Copy + Hash, Value: Eq + Copy + Hash>
    NameRefIndex<'a, Kind, Value>
{
    //
    // helper
    //

    /// helper for a setter of string attribute
    pub fn push_value<S: Into<Cow<'a, str>> + Clone>(
        &mut self,
        kind: Kind,
        name_kind: NameKind,
        name: S,
        value: Value,
    ) -> Result<(), NameIdError<Kind>> {
        let key = create_layout_key(kind, name_kind, name);
        if self.reference_index.contains_key(&key) {
            let s = key_to_str(&key).to_string();
            self.reference_index.insert(key, value);
            return Err(NameIdError::Override(kind, name_kind, s));
        }
        self.reference_index.insert(key, value);
        Ok(())
    }
}

impl<'a, Kind: Eq + Copy + Hash, Value: Eq + Copy + Hash> NameRefIndex<'a, Kind, Value> {
    /// helper for getter of string attribute
    pub fn get_value<'b: 'a>(
        &'a self,
        kind: Kind,
        name_kind: NameKind,
        name: &'b str,
    ) -> Result<&'a Value, NameIdError<Kind>> {
        self.reference_index
            .get(&create_layout_key(kind, name_kind, name))
            .ok_or_else(|| NameIdError::NotExist(kind, name_kind, name.to_string()))
    }

    pub fn contains_key(&self, kind: Kind, name_kind: NameKind, name: &str) -> bool {
        self.reference_index
            .contains_key(&create_layout_key(kind, name_kind, name))
    }

    /// helper for count by kind
    pub fn count_by(&self, kind: Kind, name_kind: NameKind) -> usize {
        self.reference_index
            .keys()
            .filter(|k| k.is_kind((kind, name_kind)))
            .count()
    }
}

impl<'a, Kind: Eq + Copy + Hash, Value: Eq + Copy + Hash> Default
    for NameRefIndex<'a, Kind, Value>
{
    /// initialize without log
    fn default() -> Self {
        NameRefIndex {
            reference_index: RefIndex::default(),
        }
    }
}
