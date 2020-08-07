//! graph with the layout for a converter from an input to an output

mod core;
mod error;
mod layout_graph;

pub use crate::grafo::core::*;
pub use error::*;
pub use layout_graph::*;

//
// GrafoAlias
//

// grafo builder
/// alias of Grafo Builder: name String, stored String
pub type NameStrGrafoBuilder = GrafoBuilder<String>;
/// alias of Grafo Builder: name usize, stored usize
pub type NameUsizeGrafoBuilder = GrafoBuilder<usize>;
/// alias of Grafo Builder: name u16, stored u16
pub type NameU16GrafoBuilder = GrafoBuilder<u16>;
/// alias of Grafo Builder: name u32, stored u32
pub type NameU32GrafoBuilder = GrafoBuilder<u32>;

// grafo
/// alias of Grafo: name String, stored String
pub type NameStrGrafo = Grafo<String>;
/// alias of Grafo: name usize, stored usize
pub type NameUsizeGrafo = Grafo<usize>;
/// alias of Grafo: name u16, stored u16
pub type NameU16Grafo = Grafo<u16>;
/// alias of Grafo: name u32, stored u32
pub type NameU32Grafo = Grafo<u32>;

// grafo error
/// alias of Grafo Error: name String, stored String
pub type NameStrGrafoError = GrafoError<String>;
/// alias of Grafo Error: name usize, stored usize
pub type NameUsizeGrafoError = GrafoError<usize>;
/// alias of Grafo Error: name u16, stored u16
pub type NameU16GrafoError = GrafoError<u16>;
/// alias of Grafo Error: name u32, stored u32
pub type NameU32GrafoError = GrafoError<u32>;
