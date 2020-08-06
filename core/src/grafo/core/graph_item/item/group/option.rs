use crate::util::name_type::{NameType, StoredNameType};
use std::marker::PhantomData;

pub struct GroupItemOption<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> {
    pub stored_name: PhantomData<StoredName>,
    pub name: Option<Name>,
}

impl<Name: NameType<StoredName>, StoredName: StoredNameType<Name>> Default
    for GroupItemOption<Name, StoredName>
{
    fn default() -> Self {
        Self {
            stored_name: PhantomData,
            name: None,
        }
    }
}
