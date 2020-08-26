//! module for Edge item's option

use crate::util::name_type::NameType;

/// option for Edge item.
pub(crate) struct EdgeItemOption<Name: NameType> {
    /// item's name
    pub name: Option<Name>,
}
