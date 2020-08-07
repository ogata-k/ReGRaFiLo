//! graph with the layout for a converter from an input to an output

mod core;
mod error;
mod layout_graph;

pub use crate::grafo::core::*;
pub use error::*;
pub use layout_graph::*;
use std::borrow::Cow;

//
// GrafoAlias
//

// grafo builder
/// alias of Grafo Builder: name String, stored Cow<'a, str>
pub type NameStrCowGrafoBuilder<'a> = GrafoBuilder<String, Cow<'a, str>>;
/// alias of Grafo Builder: name T, stored T
pub type NameTGrafoBuilder<T> = GrafoBuilder<T, T>;
/// alias of Grafo Builder: name String, stored String
pub type NameStrGrafoBuilder = NameTGrafoBuilder<String>;
/// alias of Grafo Builder: name usize, stored usize
pub type NameUsizeGrafoBuilder = NameTGrafoBuilder<usize>;
/// alias of Grafo Builder: name u16, stored u16
pub type NameU16GrafoBuilder = NameTGrafoBuilder<u16>;
/// alias of Grafo Builder: name u32, stored u32
pub type NameU32GrafoBuilder = NameTGrafoBuilder<u32>;

// grafo
/// alias of Grafo: name String, stored Cow<'a, str>
pub type NameStrCowGrafo<'a> = Grafo<String, Cow<'a, str>>;
/// alias of Grafo: name T, stored T
pub type NameTGrafo<T> = Grafo<T, T>;
/// alias of Grafo: name String, stored String
pub type NameStrGrafo = NameTGrafo<String>;
/// alias of Grafo: name usize, stored usize
pub type NameUsizeGrafo = NameTGrafo<usize>;
/// alias of Grafo: name u16, stored u16
pub type NameU16Grafo = NameTGrafo<u16>;
/// alias of Grafo: name u32, stored u32
pub type NameU32Grafo = NameTGrafo<u32>;

// grafo error
/// alias of Grafo Error: name String, stored Cow<'a, str>
pub type NameStrCowGrafoError<'a> = GrafoError<String, Cow<'a, str>>;
/// alias of Grafo Error: name T, stored T
pub type NameTGrafoError<T> = GrafoError<T, T>;
/// alias of Grafo Error: name String, stored String
pub type NameStrGrafoError = NameTGrafoError<String>;
/// alias of Grafo Error: name usize, stored usize
pub type NameUsizeGrafoError = NameTGrafoError<usize>;
/// alias of Grafo Error: name u16, stored u16
pub type NameU16GrafoError = NameTGrafoError<u16>;
/// alias of Grafo Error: name u32, stored u32
pub type NameU32GrafoError = NameTGrafoError<u32>;
