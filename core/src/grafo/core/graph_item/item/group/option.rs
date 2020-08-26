//! module for Group item's option

use crate::util::name_type::NameType;

/// option for Group item.
pub(crate) struct GroupItemOption<Name: NameType> {
    /// item's name
    pub name: Option<Name>,
}

impl<Name: NameType> Default for GroupItemOption<Name> {
    fn default() -> Self {
        Self { name: None }
    }
}
