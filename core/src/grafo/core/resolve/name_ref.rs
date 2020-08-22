use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

use crate::util::alias::{GroupId, ItemId};
use crate::util::kind_key::KeyWithKind;
use crate::util::name_type::NameType;
use crate::util::writer::DisplayAsJson;

pub trait NameRefKeyTrait: Eq + Copy + Hash + Ord {}

impl<T: Eq + Copy + Hash + Ord> NameRefKeyTrait for T {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NameIdError<Name: NameType, Kind> {
    AlreadyExist(Kind, Name),
    Override(Kind, Name),
    NotExist(Kind, Name),
}

impl<Name: NameType, Kind: std::fmt::Display> std::fmt::Display for NameIdError<Name, Kind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NameIdError::AlreadyExist(kind, name) => write!(
                f,
                "{} \"{}\" already exist",
                kind.to_string().to_lowercase(),
                name
            ),
            NameIdError::Override(kind, name) => write!(
                f,
                "override \"{}\" as {} item",
                name,
                kind.to_string().to_lowercase()
            ),
            NameIdError::NotExist(kind, name) => write!(
                f,
                "{} \"{}\" not exist",
                kind.to_string().to_lowercase(),
                name
            ),
        }
    }
}

impl<Name: NameType, Kind: std::fmt::Debug + std::fmt::Display> Error for NameIdError<Name, Kind> {}

/// The value associated with the name is overwritten and registered.<br/>
/// However, the name can be restored from the registered value.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NameRefIndex<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait> {
    // @todo (A, B).borrow() == (&A, &B) とできるなら
    //        reference_indexの二重HashMapをHashMap<(Kind, Name), Value>に一重化する
    reference_index: HashMap<Kind, HashMap<Name, Value>>,
    rev_reference_index: HashMap<KeyWithKind<Kind, Value>, Name>,
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

impl<Name: NameType, Kind: NameRefKeyTrait + std::fmt::Display> DisplayAsJson
    for NameRefIndex<Name, Kind, (GroupId, ItemId)>
{
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"reference\": [")?;
        for (i, (kind, value, name)) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}, \"name\": \"{}\"}}",
                kind, value.0, value.1, name
            )?;
        }
        write!(f, "]}}")
    }
}

impl<Name: NameType, Kind: NameRefKeyTrait + std::fmt::Display> DisplayAsJson
    for NameRefIndex<Name, Kind, ItemId>
{
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"reference\": [")?;
        for (i, (kind, value, name)) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "{{\"kind\": \"{}\", \"item_id\": {}, \"name\": \"{}\"}}",
                kind, value, name
            )?;
        }
        write!(f, "]}}")
    }
}

impl<
        Name: NameType,
        Kind: NameRefKeyTrait + std::fmt::Display,
        Value: NameRefKeyTrait + std::fmt::Display,
    > std::fmt::Display for NameRefIndex<Name, Kind, Value>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reference{{\"reference\": [")?;
        for (i, (kind, value, name)) in self.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "{{\"kind\": \"{}\", \"value\": {}, \"name\": \"{}\"}}",
                kind, value, name
            )?;
        }
        write!(f, "]}}")
    }
}

impl<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait>
    NameRefIndex<Name, Kind, Value>
{
    /// initialize
    pub fn new() -> Self {
        NameRefIndex::default()
    }

    pub fn insert_value_or_override<S: Into<Name>>(
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

    /// helper for getter of string attribute
    pub fn get_value<S: ?Sized>(&self, kind: Kind, name: &S) -> Option<Value>
    where
        Name: Borrow<S>,
        S: Hash + Eq,
    {
        self.reference_index.get(&kind)?.get(name).copied()
    }

    pub fn get_name(&self, kind: Kind, value: Value) -> Option<&Name> {
        self.rev_reference_index.get(&KeyWithKind::new(kind, value))
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

    pub fn has_registered_name(&self, kind: Kind, value: Value) -> bool {
        self.rev_reference_index
            .contains_key(&KeyWithKind::new(kind, value))
    }

    pub fn count_usable_names_filtered_by<P>(&self, predicate: P) -> usize
    where
        P: Fn(&Kind) -> bool,
    {
        self.reference_index
            .iter()
            .filter(|(k, _)| predicate(*k))
            .fold(0, |acc, (_, map)| acc + map.iter().count())
    }

    pub fn count_usable_names_by(&self, kind: Kind) -> usize {
        self.count_usable_names_filtered_by(|k| k == &kind)
    }

    pub fn count_usable_names_all(&self) -> usize {
        self.count_usable_names_filtered_by(|_| true)
    }

    pub fn count_registered_names_filtered_by<P>(&self, predicate: P) -> usize
    where
        P: Fn(&Kind) -> bool,
    {
        self.rev_reference_index
            .keys()
            .filter(|k| predicate(&k.kind))
            .count()
    }

    pub fn count_registered_names_by(&self, kind: Kind) -> usize {
        self.count_registered_names_filtered_by(|k| k == &kind)
    }

    pub fn count_registered_names_all(&self) -> usize {
        self.count_registered_names_filtered_by(|_| true)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&Kind, &Value, &Name)> + 'a {
        self.rev_reference_index
            .iter()
            .map(|(KeyWithKind { kind, key: value }, name)| (kind, value, name))
    }
}

#[cfg(test)]
mod test {
    use crate::grafo::{NameIdError, NameRefIndex};
    use crate::util::alias::ItemId;
    use crate::util::kind::GraphItemKind;

    #[test]
    fn name_override() {
        let mut name_ref: NameRefIndex<String, GraphItemKind, ItemId> = NameRefIndex::new();
        assert_eq!(
            Ok(()),
            name_ref.insert_value_or_override(GraphItemKind::Node, "node".to_string(), 1)
        );
        assert_eq!(
            Err(NameIdError::Override(
                GraphItemKind::Node,
                "node".to_string(),
            )),
            name_ref.insert_value_or_override(GraphItemKind::Node, "node".to_string(), 2)
        );
        assert_eq!(Some(2), name_ref.get_value(GraphItemKind::Node, "node"));
    }

    #[test]
    fn name_not_override() {
        let mut name_ref: NameRefIndex<String, GraphItemKind, ItemId> = NameRefIndex::new();
        assert_eq!(
            Ok(()),
            name_ref.insert_value_or_override(GraphItemKind::Node, "item".to_string(), 1)
        );
        assert_eq!(
            Ok(()),
            name_ref.insert_value_or_override(GraphItemKind::Edge, "item".to_string(), 2)
        );
        assert_eq!(Some(1), name_ref.get_value(GraphItemKind::Node, "item"));
        assert_eq!(Some(2), name_ref.get_value(GraphItemKind::Edge, "item"));
    }
}
