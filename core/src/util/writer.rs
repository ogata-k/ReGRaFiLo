//! helper's for writer method.

/// helper for Display Trait. Write as Json.<br/>
/// Since only Json needs to be converted, Serialize crate is not used.
pub trait DisplayAsJson {
    /// this method is same to fmt method in Display Trait.
    fn fmt_as_json(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
