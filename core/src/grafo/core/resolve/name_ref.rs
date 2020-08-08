use crate::util::kind_key::KeyWithKind;
use crate::util::name_type::NameType;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

pub trait NameRefKeyTrait: Eq + Copy + Hash + Ord {}
impl<T: Eq + Copy + Hash + Ord> NameRefKeyTrait for T {}

/// helper for make reference key for layout
fn create_key<N, Kind: NameRefKeyTrait>(kind: Kind, name: N) -> KeyWithKind<Kind, N> {
    KeyWithKind::new(kind, name)
}

/// helper for make reference key for layout
fn create_rev_key<Kind: NameRefKeyTrait, Value>(
    kind: Kind,
    value: Value,
) -> KeyWithKind<Kind, Value> {
    KeyWithKind::new(kind, value)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NameIdError<Name: NameType, Kind> {
    AlreadyExist(Kind, Name),
    Override(Kind, Name),
    NotExist(Kind, Name),
}

impl<Name: NameType, Kind> Display for NameIdError<Name, Kind> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl<Name: NameType, Kind: Debug + Display> Error for NameIdError<Name, Kind> {}

#[derive(Debug, Clone)]
pub struct NameRefIndex<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait> {
    reference_index: HashMap<KeyWithKind<Kind, Name>, Value>,
    rev_reference_index: BTreeMap<KeyWithKind<Kind, Value>, Name>,
}

impl<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait>
    NameRefIndex<Name, Kind, Value>
{
    /// initialize
    pub fn new() -> Self {
        NameRefIndex::default()
    }

    /// helper for getter of string attribute
    pub fn get_value<S: Into<Name>>(&self, kind: Kind, name: S) -> Option<&Value> {
        // @fixme push以外は&Sと参照を受け取るようにしたい
        self.reference_index.get(&create_key(kind, name.into()))
    }

    pub fn get_name(&self, kind: Kind, value: Value) -> Option<&Name> {
        self.rev_reference_index.get(&create_rev_key(kind, value))
    }

    pub fn contains_value(&self, kind: Kind, value: Value) -> bool {
        self.rev_reference_index
            .contains_key(&create_rev_key(kind, value))
    }

    pub fn contains_name<S: Into<Name>>(&self, kind: Kind, name: S) -> bool {
        // @fixme push以外は&Sと参照を受け取るようにしたい
        self.reference_index
            .contains_key(&create_key(kind, name.into()))
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

impl<Name: NameType, Kind: Debug + Display + NameRefKeyTrait, Value: NameRefKeyTrait>
    NameRefIndex<Name, Kind, Value>
{
    pub fn push_value<S: Into<Name>>(
        &mut self,
        kind: Kind,
        name: S,
        value: Value,
    ) -> Result<(), NameIdError<Name, Kind>> {
        let item_name = name.into();
        let key = create_key(kind, item_name.clone());
        let rev_key = create_rev_key(kind, value);
        if self.reference_index.contains_key(&key) {
            let s = item_name.clone();
            self.reference_index.insert(key, value);
            self.rev_reference_index.insert(rev_key, item_name);
            return Err(NameIdError::Override(kind, s));
        }
        self.reference_index.insert(key, value);
        self.rev_reference_index.insert(rev_key, item_name);
        Ok(())
    }
}

impl<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait> Default
    for NameRefIndex<Name, Kind, Value>
{
    fn default() -> Self {
        NameRefIndex {
            reference_index: Default::default(),
            rev_reference_index: Default::default(),
        }
    }
}
