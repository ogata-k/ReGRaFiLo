use crate::util::kind_key::KeyWithKind;
use crate::util::name_type::NameType;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

pub trait NameRefKeyTrait: Eq + Copy + Hash + Ord {}

impl<T: Eq + Copy + Hash + Ord> NameRefKeyTrait for T {}

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

/// The value associated with the name is overwritten and registered.<br/>
/// However, the name can be restored from the registered value.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameRefIndex<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait> {
    // @todo (A, B).borrow() == (&A, &B) とできるなら
    //        reference_indexの二重HashMapをHashMap<(Kind, Name), Value>に一重化する
    reference_index: HashMap<Kind, HashMap<Name, Value>>,
    rev_reference_index: HashMap<KeyWithKind<Kind, Value>, Name>,
}

// TODO 分割したい
impl<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait>
    NameRefIndex<Name, Kind, Value>
{
    /// initialize
    pub fn new() -> Self {
        NameRefIndex::default()
    }

    /// helper for getter of string attribute
    pub fn get_value<S: ?Sized>(&self, kind: Kind, name: &S) -> Option<&Value>
    where
        Name: Borrow<S>,
        S: Hash + Eq,
    {
        self.reference_index.get(&kind)?.get(name)
    }

    pub fn get_name(&self, kind: Kind, value: Value) -> Option<&Name> {
        self.rev_reference_index.get(&KeyWithKind::new(kind, value))
    }

    pub fn has_registered_name(&self, kind: Kind, value: Value) -> bool {
        self.rev_reference_index
            .contains_key(&KeyWithKind::new(kind, value))
    }

    pub fn is_usable_name<S: ?Sized>(&self, kind: Kind, name: &S) -> bool
    where
        Name: Borrow<S>,
        S: Hash + Eq,
    {
        match self.reference_index.get(&kind) {
            None => false,
            Some(map) => map.contains_key(&name),
        }
    }

    pub fn count_usable_names_by(&self, kind: Kind) -> usize {
        self.reference_index
            .iter()
            .filter(|(k, _)| *k == &kind)
            .fold(0, |acc, (_, map)| acc + map.iter().count())
    }

    pub fn count_registered_names_by(&self, kind: Kind) -> usize {
        self.rev_reference_index
            .keys()
            .filter(|k| k.is_kind(kind))
            .count()
    }

    pub fn count_usable_names_all(&self) -> usize {
        self.reference_index
            .iter()
            .fold(0, |acc, (_, map)| acc + map.iter().count())
    }

    pub fn count_registered_names_all(&self) -> usize {
        self.rev_reference_index.iter().count()
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
        let rev_key = KeyWithKind::new(kind, value);
        let result = if self.is_usable_name(kind, &item_name) {
            Err(NameIdError::Override(kind, item_name.clone()))
        } else {
            Ok(())
        };
        self.reference_index
            .entry(kind)
            .or_insert_with(HashMap::new)
            .insert(item_name.clone(), value);
        self.rev_reference_index.insert(rev_key, item_name);
        result
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
