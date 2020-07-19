use std::error::Error;

use crate::grafo::name_refindex::NameReference;
use crate::grafo::GrafoError;

pub trait ItemBuilderBase {
    type Item;
    type ItemOption;
    type BuilderError: ItemBuilderErrorBase;
}

pub(crate) type ItemBuilderResult<Item, Option> =
    Result<(Item, Option), Vec<GrafoError>>;
pub(crate) trait HasItemBuilderMethod: ItemBuilderBase {
    fn build(
        self,
        name_ref: &NameReference,
    ) -> ItemBuilderResult<Self::Item, Self::ItemOption>;
}

pub trait ItemBase {}
pub trait ItemBuilderErrorBase: Error + Into<GrafoError> {}
