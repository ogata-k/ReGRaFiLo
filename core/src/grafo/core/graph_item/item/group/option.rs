use crate::util::name_type::NameType;

pub struct GroupItemOption<Name: NameType> {
    pub name: Option<Name>,
}

impl<Name: NameType> Default for GroupItemOption<Name> {
    fn default() -> Self {
        Self { name: None }
    }
}
