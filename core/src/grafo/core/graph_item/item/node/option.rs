//! module for Node item's option

use crate::util::name_type::NameType;

/// option for Node item.
pub(crate) struct NodeItemOption<Name: NameType> {
    /// item's name
    pub name: Option<Name>,
}
