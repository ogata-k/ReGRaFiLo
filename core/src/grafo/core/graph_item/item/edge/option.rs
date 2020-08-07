use crate::util::name_type::NameType;
use std::marker::PhantomData;

pub struct EdgeItemOption<Name: NameType> {
    pub name: Option<Name>,
}
