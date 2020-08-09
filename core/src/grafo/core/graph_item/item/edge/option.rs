use crate::util::name_type::NameType;

pub struct EdgeItemOption<Name: NameType> {
    pub name: Option<Name>,
}
