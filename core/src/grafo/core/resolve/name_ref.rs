use crate::util::kind_key::KeyWithKind;
use std::borrow::{Borrow, Cow};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;

pub trait NameRefKeyTrait: Eq + Copy + Hash + Ord {}
impl<T: Eq + Copy + Hash + Ord> NameRefKeyTrait for T {}

/// helper for make reference key for layout
fn create_key<'a, Kind: NameRefKeyTrait, S: Into<Cow<'a, str>>>(
    kind: Kind,
    name: S,
) -> KeyWithKind<Kind, Cow<'a, str>> {
    KeyWithKind::new(kind, name.into())
}

/// helper for make reference key for layout
fn create_rev_key<Kind: NameRefKeyTrait, Value: NameRefKeyTrait>(
    kind: Kind,
    value: Value,
) -> KeyWithKind<Kind, Value> {
    KeyWithKind::new(kind, value)
}

fn key_to_str<'a, 'b: 'a, Kind: Eq + Copy + Hash>(
    key: &'b KeyWithKind<Kind, Cow<'a, str>>,
) -> &'b str {
    key.key.deref()
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NameIdError<Kind> {
    AlreadyExist(Kind, String),
    Override(Kind, String),
    NotExist(Kind, String),
}

impl<Kind: Display> Display for NameIdError<Kind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Kind: Debug + Display> Error for NameIdError<Kind> {}

#[derive(Debug, Clone)]
pub struct NameRefIndex<'a, Kind: NameRefKeyTrait, Value: NameRefKeyTrait> {
    reference_index: HashMap<KeyWithKind<Kind, Cow<'a, str>>, Value>,
    rev_reference_index: BTreeMap<KeyWithKind<Kind, Value>, Cow<'a, str>>,
}

impl<'a, Kind: NameRefKeyTrait, Value: NameRefKeyTrait> NameRefIndex<'a, Kind, Value> {
    /// initialize
    pub fn new() -> Self {
        NameRefIndex::default()
    }

    /// helper for getter of string attribute
    pub fn get_value<'b: 'a>(&'a self, kind: Kind, name: &'b str) -> Option<&'a Value> {
        self.reference_index.get(&create_key(kind, name))
    }

    pub fn get_name(&self, kind: Kind, value: Value) -> Option<&str> {
        self.rev_reference_index
            .get(&create_rev_key(kind, value))
            .map(|cow_str| cow_str.borrow())
    }

    pub fn contains_value(&self, kind: Kind, value: Value) -> bool {
        self.rev_reference_index
            .contains_key(&create_rev_key(kind, value))
    }

    pub fn contains_name(&self, kind: Kind, name: &str) -> bool {
        self.reference_index.contains_key(&create_key(kind, name))
    }

    pub fn count_names_by(&self, kind: Kind) -> usize {
        self.reference_index
            .keys()
            .filter(|k| k.is_kind(kind))
            .count()
    }

    pub fn count_name_all(&self) -> usize {
        self.reference_index.keys().count()
    }

    pub fn count_values_by(&self, kind: Kind) -> usize {
        self.rev_reference_index
            .keys()
            .filter(|k| k.is_kind(kind))
            .count()
    }

    pub fn count_value_all(&self) -> usize {
        self.rev_reference_index.keys().count()
    }
}

impl<'a, Kind: Debug + Display + NameRefKeyTrait, Value: NameRefKeyTrait>
    NameRefIndex<'a, Kind, Value>
{
    pub fn push_value<S: Into<Cow<'a, str>> + Clone>(
        &mut self,
        kind: Kind,
        name: S,
        value: Value,
    ) -> Result<(), NameIdError<Kind>> {
        let item_name = name.clone().into();
        let key = create_key(kind, name);
        let rev_key = create_rev_key(kind, value);
        if self.reference_index.contains_key(&key) {
            let s = item_name.clone().to_string();
            self.reference_index.insert(key, value);
            self.rev_reference_index.insert(rev_key, item_name);
            return Err(NameIdError::Override(kind, s));
        }
        self.reference_index.insert(key, value);
        self.rev_reference_index.insert(rev_key, item_name);
        Ok(())
    }
}

impl<'a, Kind: NameRefKeyTrait, Value: NameRefKeyTrait> Default for NameRefIndex<'a, Kind, Value> {
    fn default() -> Self {
        NameRefIndex {
            reference_index: Default::default(),
            rev_reference_index: Default::default(),
        }
    }
}
