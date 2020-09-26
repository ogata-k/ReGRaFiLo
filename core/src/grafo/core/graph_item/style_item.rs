//! module of style's item for graph item

/// item wrapper for Style
#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
pub struct StyleItem<Base: Default + Eq + Clone> {
    item_base: Option<Base>,
}

impl<Base: Default + Eq + Clone> Default for StyleItem<Base> {
    fn default() -> Self {
        Self { item_base: None }
    }
}

impl<Base: Default + Eq + Clone> StyleItem<Base> {
    /// initializer for StyleItem
    pub fn new() -> Self {
        Self { item_base: None }
    }

    /// initializer for StyleItem with value
    pub fn new_with(base: Base) -> Self {
        Self {
            item_base: Some(base),
        }
    }

    /// update inner value
    pub fn update(&mut self, base: Base) {
        self.item_base = Some(base);
    }

    /// get inner value
    pub fn get(&self) -> Option<&Base> {
        self.item_base.as_ref()
    }

    /// get inner value or function's result value
    pub fn get_or(&self, default: Base) -> &Base {
        // TODO どうにかして参照を返すようにできないか （get_or(default: &Base)?????）
        &self.item_base.unwrap_or(default)
    }

    /// get inner value or default value
    pub fn get_or_default(&self) -> &Base {
        // TODO どうにかして参照を返すようにできないか
        self.get_or(Base::default())
    }

    /// check that self inner value is already specified
    pub fn is_specified(&self) -> bool {
        // TODO remove
        self.get_or_default();

        self.item_base.is_some()
    }
}
