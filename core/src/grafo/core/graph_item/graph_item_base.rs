//! base of graph item and graph item's builder

use crate::grafo::{NameIdError, ResolverError};
use crate::util::alias::{GroupId, ItemId};
use crate::util::item_base::{FromWithItemId, ItemBase, ItemBuilderBase, ItemErrorBase};
use crate::util::kind::{GraphItemKind, HasGraphItemKind};
use crate::util::name_type::NameType;

/// Item Builder's base set
pub trait GraphItemBuilderBase<Name: NameType>: ItemBuilderBase<Name> {
    /// setter for belong group
    fn set_belong_group<S: Into<Name>>(&mut self, group: S) -> &mut Self;
    /// setter for graph item's name. You can use the name for specified item.
    /// However the name is not the item's label. If you want label, use label on item's parameter.
    fn set_name<S: Into<Name>>(&mut self, name: S) -> &mut Self;

    /// setter for graph item's label
    fn set_label<S: Into<String>>(&mut self, label: S) -> &mut Self;
}

/// Item's base set
pub trait GraphItemBase: ItemBase + HasGraphItemKind {
    /// getter for item's belong group
    fn get_belong_group_id(&self) -> GroupId;

    /// getter for item's label
    fn get_label(&self) -> Option<&str>;
}

/// Item's mutability for Default initializer
pub(crate) trait WithMutable: GraphItemBase + Default {
    fn set_label<S: Into<String>>(&mut self, label: Option<S>)->&mut Self;
}

/// base of build result's error for graph item's builder
pub trait GraphBuilderErrorBase<Name: NameType>:
    ItemErrorBase<Name>
    + HasGraphItemKind
    + FromWithItemId<NameIdError<Name, GraphItemKind>, Name>
    + FromWithItemId<ResolverError, Name>
{
    /// get item id for graph item whose builder is error source
    fn get_item_id(&self) -> &ItemId;
    /// get item name for graph item whose builder is error source
    fn get_item_name(&self) -> &Option<Name>;
    /// write error's header
    fn fmt_header(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.get_item_name() {
            write!(
                f,
                "{} {} with name \"{}\": ",
                self.get_kind(),
                self.get_item_id(),
                name
            )
        } else {
            write!(f, "{} {}: ", self.get_kind(), self.get_item_id())
        }
    }
}
