use std::error::Error;

use crate::grafo::resolve::Resolver;
use crate::grafo::GrafoError;

pub trait ItemBuilderBase {
    type Item;
    type ItemOption;
    type BuilderError: ItemBuilderErrorBase;
}

pub(crate) type ItemBuilderResult<Item, Option> = Result<(Item, Option), Vec<GrafoError>>;
pub(crate) trait HasItemBuilderMethod: ItemBuilderBase {
    fn build(self, resolver: &Resolver) -> ItemBuilderResult<Self::Item, Self::ItemOption>;
}

pub trait ItemBase {}
pub trait ItemBuilderErrorBase: Error + Into<GrafoError> {}
