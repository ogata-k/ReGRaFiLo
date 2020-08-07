use crate::util::name_type::NameType;

pub struct NodeItemOption<Name: NameType> {
    pub name: Option<Name>,
}
