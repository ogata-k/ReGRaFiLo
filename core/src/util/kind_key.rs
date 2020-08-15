//! key with kind
use std::fmt::{Debug, Display, Formatter, Result};

/// key with kind
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct KeyWithKind<Kind: Copy, Key> {
    pub kind: Kind,
    pub key: Key,
}

impl<Kind: Copy, Key> KeyWithKind<Kind, Key> {
    /// initializer
    pub fn new(kind: Kind, key: Key) -> Self {
        KeyWithKind { kind, key }
    }
}

impl<Kind: Eq + Copy, Key> KeyWithKind<Kind, Key> {
    /// checker of kind
    pub fn is_kind(&self, kind: Kind) -> bool {
        self.kind == kind
    }
}

//
// impl Traits
//
impl<Kind: Display + Copy, Key: Display> Display for KeyWithKind<Kind, Key> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}:{}", self.kind, self.key)
    }
}
