use crate::util::name_type::NameType;

pub(crate) struct EdgeItemOption<Name: NameType> {
    pub name: Option<Name>,
}
