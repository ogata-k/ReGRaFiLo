//! module for set of layout item's stores

use crate::util::writer::DisplayAsJson;

/// set of store for layout items
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Layout {
    // TODO layout item store
}

impl Default for Layout {
    fn default() -> Self {
        Self {}
    }
}

impl DisplayAsJson for Layout {
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        // TODO
        write!(f, "}}")
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Layout")?;
        self.fmt_as_json(f)
    }
}

impl Layout {
    // TODO
}
