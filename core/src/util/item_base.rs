use std::error::Error;

use crate::grafo::GrafoError;
use crate::grafo::Resolver;

pub trait ItemBuilderBase {
    type Item: ItemBase;
    type ItemError: ItemErrorBase;
}

pub(crate) type ItemBuilderResult<Item, ItemOption> = (Option<(Item, ItemOption)>, Vec<GrafoError>);
pub(crate) trait HasItemBuilderMethod: ItemBuilderBase {
    type ItemOption;
    fn build(self, resolver: &Resolver) -> ItemBuilderResult<Self::Item, Self::ItemOption>;
}

pub trait ItemBase {}
pub trait ItemErrorBase: Eq + Error + Into<GrafoError> {}
