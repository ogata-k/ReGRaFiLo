//! base of item and item builder

use crate::grafo::NameIdError;
use crate::util::alias::GroupId;
use crate::util::item_base::{ItemBase, ItemBuilderBase, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};

/// Item Builder's base set
pub trait GraphItemBuilderBase: ItemBuilderBase {
    fn set_belong_group<S: Into<String>>(&mut self, group: S) -> &mut Self;
    fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self;
}

/// Item's base set
pub trait GraphItemBase: ItemBase + HasGraphItemKind {
    fn get_belong_group_id(&self) -> GroupId;
}

// TODO ItemError事態に区別用にIdを持たせる。もちろんNameIdErrorからの変換時にIdを追加できるようにする。
pub trait GraphBuilderErrorBase:
    ItemErrorBase + HasGraphItemKind + From<NameIdError<GraphItemKind>>
{
}
