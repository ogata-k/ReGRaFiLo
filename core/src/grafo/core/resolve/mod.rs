//! module of resolver which is reference for name and group hierarchy

mod id_tree;
mod name_ref;
mod resolver;

pub(crate) use id_tree::*;
pub(crate) use name_ref::*;
pub use resolver::*;
