use crate::util::name_type::{NameType, StoredNameType};
use std::marker::PhantomData;

pub struct EdgeItemOption<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    pub stored_name: PhantomData<StoredName>,
    pub name: Option<Name>,
}
