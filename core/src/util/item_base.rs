use std::error::Error;

use crate::grafo::GrafoError;
use crate::grafo::Resolver;

pub trait ItemBuilderBase {
    type Item;
    type ItemOption;
    type BuilderError: ItemBuilderErrorBase;
}

pub(crate) type ItemBuilderResult<Item, ItemOption> = (Option<(Item, ItemOption)>, Vec<GrafoError>);
pub(crate) trait HasItemBuilderMethod: ItemBuilderBase {
    fn build(self, resolver: &Resolver) -> ItemBuilderResult<Self::Item, Self::ItemOption>;
}

pub trait ItemBase {}
pub trait ItemBuilderErrorBase: Eq + Error + Into<GrafoError> {}
