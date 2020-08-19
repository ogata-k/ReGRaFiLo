use crate::util::name_type::NameType;

pub(crate) struct NodeItemOption<Name: NameType> {
    pub name: Option<Name>,
}
