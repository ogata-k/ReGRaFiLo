//! module for reference and reference's error.<br/>
//! reference has kind as grouping key, name as referencable key and registered name, and value as referenced value and reverse referencable key.

use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;
use std::hash::Hash;

use crate::util::alias::{GroupId, ItemId};
use crate::util::iter::IterLimitedByOneGroup;
use crate::util::kind::{GraphItemKind, LayoutGraphItemKind};
use crate::util::name_type::NameType;
use crate::util::writer::DisplayAsJson;

/// key and value's type for NameRefIndex
pub trait NameRefKeyTrait: Eq + Copy + Hash + Ord {}

impl<T: Eq + Copy + Hash + Ord> NameRefKeyTrait for T {}

/// error for name's reference
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NameIdError<Name: NameType, Kind> {
    /// the name is referencable key already registered
    AlreadyExist(Kind, Name),
    /// override value of the key of already registered the name
    Override(Kind, Name),
    /// the specified name as key don't exist
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
    reference_index: HashMap<Kind, HashMap<Name, Value>>,
    rev_reference_index: HashMap<Kind, BTreeMap<Value, Name>>,
    no_name_reference: HashMap<Kind, BTreeSet<Value>>,
}

impl<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait> Default
    for NameRefIndex<Name, Kind, Value>
{
    fn default() -> Self {
        NameRefIndex {
            reference_index: Default::default(),
            rev_reference_index: Default::default(),
            no_name_reference: Default::default(),
        }
    }
}

impl<Name: NameType> DisplayAsJson for NameRefIndex<Name, GraphItemKind, (GroupId, ItemId)> {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"reference\": [")?;
        let mut is_first = true;
        // fix order
        for kind in [
            GraphItemKind::Group,
            GraphItemKind::Node,
            GraphItemKind::Edge,
        ]
        .iter()
        {
            for ((group_id, item_id), name) in self.iter_by_kind(*kind) {
                if is_first {
                    is_first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(
                    f,
                    "{{\"kind\": \"{}\", \"belong_group_id\": {}, \"item_id\": {}, \"name\": \"{}\"}}",
                    kind, group_id, item_id, name
                )?;
            }
        }
        write!(f, "]}}")
    }
}

impl<Name: NameType> DisplayAsJson for NameRefIndex<Name, LayoutGraphItemKind, ItemId> {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"reference\": [")?;
        let mut is_first = true;
        // fix order
        for kind in [
            LayoutGraphItemKind::Group,
            LayoutGraphItemKind::Node,
            LayoutGraphItemKind::Edge,
        ]
        .iter()
        {
            for (item_id, name) in self.iter_by_kind(*kind) {
                if is_first {
                    is_first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(
                    f,
                    "{{\"kind\": \"{}\", \"item_id\": {}, \"name\": \"{}\"}}",
                    kind, item_id, name
                )?;
            }
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
        let mut is_first = true;
        for (kind, map) in self.rev_reference_index.iter() {
            for (value, name) in map.iter() {
                if is_first {
                    is_first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(
                    f,
                    "{{\"kind\": \"{}\", \"value\": {}, \"name\": \"{}\"}}",
                    kind, value, name
                )?;
            }
        }
        write!(f, "]}}")
    }
}

impl<Name: NameType, Kind: NameRefKeyTrait, Value: NameRefKeyTrait>
    NameRefIndex<Name, Kind, Value>
{
    /// initializer
    pub fn new() -> Self {
        NameRefIndex::default()
    }

    /// initializer with no name item's store is smallest store.
    pub fn initialize_without_no_name() -> Self {
        NameRefIndex {
            reference_index: Default::default(),
            rev_reference_index: Default::default(),
            no_name_reference: HashMap::with_capacity(0),
        }
    }

    /// insert value by reference name. If name already exist, override name's value.
    pub fn insert_value_or_override<S: Into<Name>>(
        &mut self,
        kind: Kind,
        name: Option<S>,
        value: Value,
    ) -> Result<(), NameIdError<Name, Kind>> {
        match name {
            Some(ref_name) => {
                let item_name = ref_name.into();
                let result = if self.is_usable_name(kind, &item_name) {
                    Err(NameIdError::Override(kind, item_name.clone()))
                } else {
                    Ok(())
                };
                self.reference_index
                    .entry(kind)
                    .or_default()
                    .insert(item_name.clone(), value);
                self.rev_reference_index
                    .entry(kind)
                    .or_default()
                    .insert(value, item_name);
                result
            }
            None => {
                self.no_name_reference
                    .entry(kind)
                    .or_default()
                    .insert(value);
                Ok(())
            }
        }
    }

    /// helper for getter of string attribute
    pub fn get_value<S: ?Sized>(&self, kind: Kind, name: &S) -> Option<Value>
    where
        Name: Borrow<S>,
        S: Hash + Eq,
    {
        self.reference_index.get(&kind)?.get(name).copied()
    }

    /// get registered name
    pub fn get_name(&self, kind: Kind, value: Value) -> Option<&Name> {
        match self.rev_reference_index.get(&kind) {
            Some(map) => map.get(&value),
            None => None,
        }
    }

    /// check value grouped by kind is registered
    pub fn is_already_registered(&self, kind: Kind, value: Value) -> bool {
        match self.rev_reference_index.get(&kind) {
            Some(map) => map.contains_key(&value),
            None => match self.no_name_reference.get(&kind) {
                Some(map) => map.contains(&value),
                None => false,
            },
        }
    }

    /// check specified name is referencable
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

    /// check the name is already registered
    pub fn has_registered_name(&self, kind: Kind, value: Value) -> bool {
        match self.rev_reference_index.get(&kind) {
            Some(map) => map.contains_key(&value),
            None => false,
        }
    }

    /// count all registered items by the kind
    pub fn count_all_registered_by(&self, kind: Kind) -> usize {
        let with_name_count = match self.rev_reference_index.get(&kind) {
            Some(map) => map.len(),
            None => 0,
        };
        let no_name_count = match self.no_name_reference.get(&kind) {
            Some(map) => map.len(),
            None => 0,
        };
        with_name_count + no_name_count
    }

    /// count all referencable names
    pub fn count_all_registered(&self) -> usize {
        let with_name_count = self
            .rev_reference_index
            .iter()
            .fold(0, |acc, map| acc + (map.1).len());
        let no_name_count = self
            .no_name_reference
            .iter()
            .fold(0, |acc, map| acc + (map.1).len());
        with_name_count + no_name_count
    }

    /// count names which is referencable filtering by the kind
    pub fn count_usable_names_by(&self, kind: Kind) -> usize {
        match self.reference_index.get(&kind) {
            Some(map) => map.len(),
            None => 0,
        }
    }

    /// count all referencable names
    pub fn count_usable_names_all(&self) -> usize {
        self.reference_index
            .iter()
            .fold(0, |acc, map| acc + (map.1).len())
    }

    /// count names which is already registered filtering by the kind
    pub fn count_registered_names_by(&self, kind: Kind) -> usize {
        match self.rev_reference_index.get(&kind) {
            Some(map) => map.len(),
            None => 0,
        }
    }

    /// count all registered names
    pub fn count_registered_names_all(&self) -> usize {
        self.rev_reference_index
            .iter()
            .fold(0, |acc, map| acc + (map.1).len())
    }

    /// get iter grouped by the kind
    pub fn iter_by_kind(&self, kind: Kind) -> IterLimitedByOneGroup<Kind, Value, Name> {
        IterLimitedByOneGroup::from_hash_map(&kind, &self.rev_reference_index)
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
            name_ref.insert_value_or_override(GraphItemKind::Node, Some("node".to_string()), 1)
        );
        assert_eq!(
            Err(NameIdError::Override(
                GraphItemKind::Node,
                "node".to_string(),
            )),
            name_ref.insert_value_or_override(GraphItemKind::Node, Some("node".to_string()), 2)
        );
        assert_eq!(Some(2), name_ref.get_value(GraphItemKind::Node, "node"));
    }

    #[test]
    fn name_not_override() {
        let mut name_ref: NameRefIndex<String, GraphItemKind, ItemId> = NameRefIndex::new();
        assert_eq!(
            Ok(()),
            name_ref.insert_value_or_override(GraphItemKind::Node, Some("item".to_string()), 1)
        );
        assert_eq!(
            Ok(()),
            name_ref.insert_value_or_override(GraphItemKind::Edge, Some("item".to_string()), 2)
        );
        assert_eq!(Some(1), name_ref.get_value(GraphItemKind::Node, "item"));
        assert_eq!(Some(2), name_ref.get_value(GraphItemKind::Edge, "item"));
    }
}
