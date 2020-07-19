//! item pool

use std::collections::btree_map::{Iter, Range};
use std::collections::BTreeMap;
use std::ops::{Bound, RangeBounds};
use std::sync::{Arc, Mutex};

use crate::grafo::core::graph_item::{GraphBuilderErrorBase, GraphItemBase, GraphItemBuilderBase};
use crate::grafo::core::name_refindex::NameReference;
use crate::grafo::GrafoError;
use crate::util::alias::{GraphItemId, GroupId};
use crate::util::item_base::HasItemBuilderMethod;
use crate::util::kind::GraphItemKind;

/// item pool
#[derive(Debug, Clone)]
pub struct ItemArena<I> {
    pushed_index: Arc<Mutex<GraphItemId>>,
    /// (GroupId, ItemId) => Item
    arena: BTreeMap<(GroupId, GraphItemId), I>,
}

fn range_with_group(
    group_id: GroupId,
    bound: Bound<&GraphItemId>,
) -> Bound<(GroupId, GraphItemId)> {
    match bound {
        Bound::Included(item_id) => Bound::Included((group_id, *item_id)),
        Bound::Excluded(item_id) => Bound::Excluded((group_id, *item_id)),
        Bound::Unbounded => Bound::Unbounded,
    }
}

impl<I: GraphItemBase> ItemArena<I> {
    /// initialize
    pub fn new() -> Self {
        ItemArena::default()
    }

    //
    // helper
    //

    /// get the next index with increment as soon as possible
    fn get_push_index(&mut self) -> GraphItemId {
        match self.pushed_index.lock() {
            Ok(mut pushed_index) => {
                let next_index: GraphItemId = *pushed_index;
                *pushed_index += 1;
                next_index
            }
            Err(e) => {
                panic!("fail lock error: {}", e);
            }
        }
    }

    //
    // setter
    //

    /// push the item into arena with action for conclusion<br/>
    /// F: fn(item_kind, group_id, Result<(item_id, extension), err>)
    pub(crate) fn push<
        F,
        O,
        E: GraphBuilderErrorBase,
        B: GraphItemBuilderBase + HasItemBuilderMethod<Item = I, ItemOption = O, BuilderError = E>,
    >(
        &mut self,
        name_ref: &mut NameReference,
        item_builder: B,
        action: F,
    ) -> Option<Vec<GrafoError>>
    where
        F: FnOnce(
            &mut NameReference,
            GraphItemKind,
            GroupId,
            GraphItemId,
            B::ItemOption,
        ) -> Option<Vec<GrafoError>>,
    {
        let item_kind = B::kind();
        match item_builder.build(name_ref) {
            Ok((item, option)) => {
                let group_id = item.get_belong_group_id();
                let push_index = self.get_push_index();
                self.arena.insert((group_id, push_index), item);

                action(name_ref, item_kind, group_id, push_index, option)
            }
            Err(errors) => Some(errors),
        }
    }

    /// item getter
    pub fn get(&self, group_id: GroupId, index: GraphItemId) -> Option<&I> {
        self.arena.get(&(group_id, index))
    }

    /// item getter by range
    pub fn range<R: RangeBounds<GraphItemId>>(
        &self,
        group_id: GroupId,
        range: R,
    ) -> Range<(GroupId, GraphItemId), I> {
        let start = range_with_group(group_id, range.start_bound());
        let end = range_with_group(group_id, range.end_bound());
        self.arena.range((start, end))
    }

    /// iter by filtering group_id
    pub fn filter_by_group<'a>(&'a self, group_id: GroupId) -> impl Iterator + 'a {
        self.iter()
            .filter_map(move |((item_group_id, item_id), item)| {
                if item_group_id == &group_id {
                    Some(item)
                } else {
                    None
                }
            })
    }

    //
    // reference
    //

    /// count of item
    pub fn count(&self) -> usize {
        self.arena.len()
    }

    /// item pool is empty
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    //
    // iter or slice
    //

    /// to iterator
    pub fn iter(&self) -> Iter<(GroupId, GraphItemId), I> {
        self.arena.iter()
    }
}

impl<I> Default for ItemArena<I> {
    /// initialize without log
    fn default() -> Self {
        ItemArena {
            pushed_index: Default::default(),
            arena: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fmt::{Display, Formatter};

    use crate::grafo::core::graph_item::{
        GraphBuilderErrorBase, GraphItemBase, GraphItemBuilderBase, ItemArena,
    };
    use crate::grafo::core::name_refindex::{NameRefError, NameReference};
    use crate::grafo::GrafoError;
    use crate::util::alias::{GraphItemId, GroupId};
    use crate::util::item_base::{
        HasItemBuilderMethod, ItemBase, ItemBuilderBase, ItemBuilderErrorBase, ItemBuilderResult,
    };
    use crate::util::kind::test::graph_item_check_list;
    use crate::util::kind::{GraphItemKind, HasGraphItemKind};
    use std::error::Error;

    const ITERATE_COUNT: usize = 10;
    const TARGET_KIND: GraphItemKind = GraphItemKind::Node;

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct TargetItemBuilder {
        belong_group: Option<String>,
        name: Option<String>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct TargetItem {
        belong_group_id: GraphItemId,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    struct TargetItemOption {
        belong_group_id: GraphItemId,
        name: Option<String>,
    }

    #[derive(Debug)]
    enum TargetBuilderError {
        BuildFail,
        NotFindGroup,
    }

    impl Into<GrafoError> for TargetBuilderError {
        fn into(self) -> GrafoError {
            unimplemented!()
        }
    }

    impl HasGraphItemKind for TargetItem {
        fn kind() -> GraphItemKind {
            TARGET_KIND
        }
    }

    impl HasGraphItemKind for TargetItemBuilder {
        fn kind() -> GraphItemKind {
            TARGET_KIND
        }
    }

    impl HasGraphItemKind for TargetBuilderError {
        fn kind() -> GraphItemKind {
            TARGET_KIND
        }
    }

    impl ItemBuilderBase for TargetItemBuilder {
        type Item = TargetItem;
        type ItemOption = TargetItemOption;
        type BuilderError = TargetBuilderError;
    }

    impl GraphItemBuilderBase for TargetItemBuilder {
        fn set_belong_group<S: Into<String>>(&mut self, group: S) -> &mut Self {
            self.belong_group = Some(group.into());
            self
        }

        fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
            self.name = Some(name.into());
            self
        }
    }

    impl TargetItemBuilder {
        fn get_belong_group(
            &self,
            name_ref: &NameReference,
            errors: &mut Vec<GrafoError>,
            belong_group: Option<&str>,
        ) -> Option<GroupId> {
            match belong_group {
                None => Some(name_ref.get_root_group_id()),
                Some(belong_group_name) => {
                    let belong_group_result =
                        name_ref.get_item_id_pair(GraphItemKind::Group, &belong_group_name);
                    match belong_group_result {
                        Ok((_belong_group_id, item_id)) => Some(*item_id),
                        Err(err) => {
                            errors.push(err.into());
                            None
                        }
                    }
                }
            }
        }
    }

    impl HasItemBuilderMethod for TargetItemBuilder {
        fn build(
            self,
            name_ref: &NameReference,
        ) -> ItemBuilderResult<TargetItem, TargetItemOption> {
            assert_ne!(TARGET_KIND, GraphItemKind::Group);
            let mut errors: Vec<GrafoError> = Vec::new();

            let group_id =
                (&self).get_belong_group(&name_ref, &mut errors, self.belong_group.as_deref());
            if group_id.is_none() {
                errors.push(TargetBuilderError::NotFindGroup.into());
                return Err(errors);
            }
            let group_id = group_id.unwrap();

            let TargetItemBuilder {
                belong_group: _,
                name,
            } = self;
            if errors.is_empty() {
                Ok((
                    TargetItem {
                        belong_group_id: group_id,
                    },
                    TargetItemOption {
                        belong_group_id: group_id,
                        name,
                    },
                ))
            } else {
                Err(errors)
            }
        }
    }

    impl TargetItemBuilder {
        fn new() -> Self {
            TargetItemBuilder {
                belong_group: None,
                name: None,
            }
        }
    }

    impl ItemBase for TargetItem {}

    impl GraphItemBase for TargetItem {
        fn get_belong_group_id(&self) -> usize {
            self.belong_group_id
        }
    }

    impl Display for TargetBuilderError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            use TargetBuilderError::*;
            match &self {
                BuildFail => write!(f, "fail build item"),
                NotFindGroup => write!(f, "fail found belong group"),
            }
        }
    }

    impl Error for TargetBuilderError {}

    impl ItemBuilderErrorBase for TargetBuilderError {}

    impl GraphBuilderErrorBase for TargetBuilderError {}

    #[test]
    fn is_empty() {
        assert!(ItemArena::<TargetItem>::new().is_empty());
    }

    #[test]
    fn with_name_count() {
        let mut arena_mut = ItemArena::<TargetItem>::new();
        let mut reference = NameReference::default();
        reference.set_root_group_id(0);
        for i in 0..ITERATE_COUNT {
            let mut builder = TargetItemBuilder::new();
            builder.set_name(format!("{}", i));
            let push_result = arena_mut.push(
                &mut reference,
                builder,
                |name_ref, kind, group_id, item_id, option| {
                    if let TargetItemOption {
                        belong_group_id: _,
                        name: Some(name),
                    } = option
                    {
                        let mut errors: Vec<GrafoError> = Vec::new();
                        if let Err(err) = name_ref.push_item_name(kind, name, group_id, item_id) {
                            errors.push(err.into());
                        }
                        return if errors.is_empty() {
                            None
                        } else {
                            Some(errors)
                        };
                    }
                    None
                },
            );
            assert!(push_result.is_none());
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), ITERATE_COUNT);
        for target in graph_item_check_list() {
            assert_eq!(
                reference.item_name_count_by(target),
                if target == TARGET_KIND {
                    ITERATE_COUNT
                } else {
                    0
                }
            );
        }
    }

    #[test]
    fn with_name_each_eq() {
        let mut arena_mut = ItemArena::<TargetItem>::new();
        let mut reference = NameReference::default();
        reference.set_root_group_id(0);

        for i in 0..ITERATE_COUNT {
            let mut builder = TargetItemBuilder::new();
            builder.set_name(format!("{}", i));
            let push_result = arena_mut.push(
                &mut reference,
                builder,
                |name_ref, kind, group_id, item_id, option| {
                    if let TargetItemOption {
                        belong_group_id: _,
                        name: Some(name),
                    } = option
                    {
                        let mut errors: Vec<GrafoError> = Vec::new();
                        if let Err(err) = name_ref.push_item_name(kind, name, group_id, item_id) {
                            errors.push(err.into());
                        }
                        return if errors.is_empty() {
                            None
                        } else {
                            Some(errors)
                        };
                    }
                    None
                },
            );
            assert!(push_result.is_none());
        }
        let arena = arena_mut;
        for (index, item) in (&arena).iter().enumerate() {
            let result: (usize, usize) = (0, index);
            assert_eq!(result, *item.0);
            for kind in graph_item_check_list() {
                let name = format!("{}", index);
                let ref_result = reference.get_item_id_pair(kind, &name);
                if let Ok(success) = ref_result {
                    assert_eq!(success, &result);
                } else {
                    let err = ref_result.err();
                    assert!(err.is_some());
                    assert_eq!(
                        err.unwrap(),
                        NameRefError::NotExist(kind, format!("{}", index))
                    );
                }
            }
        }
    }

    #[test]
    fn mixed_count() {
        let mut arena_mut = ItemArena::<TargetItem>::new();
        let mut reference = NameReference::default();
        reference.set_root_group_id(0);
        for i in 0..2 * ITERATE_COUNT {
            let mut builder = TargetItemBuilder::new();
            if i < ITERATE_COUNT {
                builder.set_name(format!("{}", i));
            }
            let push_result = arena_mut.push(
                &mut reference,
                builder,
                |name_ref, kind, group_id, item_id, option| {
                    if let TargetItemOption {
                        belong_group_id: _,
                        name: Some(name),
                    } = option
                    {
                        let mut errors: Vec<GrafoError> = Vec::new();
                        if let Err(err) = name_ref.push_item_name(kind, name, group_id, item_id) {
                            errors.push(err.into());
                        }
                        return if errors.is_empty() {
                            None
                        } else {
                            Some(errors)
                        };
                    }
                    None
                },
            );
            assert!(push_result.is_none());
        }
        let arena = arena_mut;
        assert_eq!(arena.count(), 2 * ITERATE_COUNT);
        for target in graph_item_check_list() {
            assert_eq!(
                reference.item_name_count_by(target),
                if target == TARGET_KIND {
                    ITERATE_COUNT
                } else {
                    0
                }
            );
        }
    }

    #[test]
    fn mixed_each_eq() {
        let mut arena_mut = ItemArena::<TargetItem>::new();
        let mut reference = NameReference::default();
        reference.set_root_group_id(0);
        for i in 0..2 * ITERATE_COUNT {
            let mut builder = TargetItemBuilder::new();
            if i < ITERATE_COUNT {
                builder.set_name(format!("{}", i));
            }
            let push_result = arena_mut.push(
                &mut reference,
                builder,
                |name_ref, kind, group_id, item_id, option| {
                    if let TargetItemOption {
                        belong_group_id: _,
                        name: Some(name),
                    } = option
                    {
                        let mut errors: Vec<GrafoError> = Vec::new();
                        if let Err(err) = name_ref.push_item_name(kind, name, group_id, item_id) {
                            errors.push(err.into());
                        }
                        return if errors.is_empty() {
                            None
                        } else {
                            Some(errors)
                        };
                    }
                    None
                },
            );
            assert!(push_result.is_none());
        }
        let arena = arena_mut;
        for (index, item) in (&arena).iter().enumerate() {
            let result: (usize, usize) = (0, index);
            assert_eq!(result, *item.0);
            for kind in graph_item_check_list() {
                let name = format!("{}", index);
                let ref_result = reference.get_item_id_pair(kind, &name);
                if index < ITERATE_COUNT && kind == TARGET_KIND {
                    if let Ok(success) = &ref_result {
                        assert_eq!(success, &&result);
                    } else {
                        unreachable!("over count and not exist the name \"{}\"", name)
                    }
                } else {
                    let err = &ref_result.err();
                    assert!(err.is_some());
                    assert_eq!(
                        err.clone().unwrap(),
                        NameRefError::NotExist(kind, format!("{}", index))
                    );
                }
            }
        }
    }
}
