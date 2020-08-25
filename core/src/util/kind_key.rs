//! key with kind

/// key with kind
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct KeyWithKind<Kind: Copy, Key> {
    /// Key's Kind. Kind is grouping key.
    pub kind: Kind,
    /// main value group by kind.
    pub key: Key,
}

impl<Kind: Copy, Key> KeyWithKind<Kind, Key> {
    /// initializer
    pub fn new(kind: Kind, key: Key) -> Self {
        KeyWithKind { kind, key }
    }

    /// checker of kind
    pub fn is_kind(&self, kind: Kind) -> bool
    where
        Kind: Eq,
    {
        self.kind == kind
    }
}
