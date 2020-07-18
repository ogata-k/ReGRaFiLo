use std::error::Error;

use crate::grafo::name_refindex::NameReference;
use crate::grafo::GrafoError;

pub trait ItemBuilderBase {
    type Item;
    type ItemOption;
    type BuildFailError: ItemErrorBase;
}

pub(crate) type ItemBuilderResult<Item, Option, BuildError> =
    Result<(Item, Option), Vec<BuildError>>;
pub(crate) trait HasItemBuilderMethod: ItemBuilderBase {
    fn build(
        self,
        name_ref: &NameReference,
    ) -> ItemBuilderResult<Self::Item, Self::ItemOption, Self::BuildFailError>;
}

pub trait ItemBase {}
pub trait ItemErrorBase: Error + Into<GrafoError> {}
