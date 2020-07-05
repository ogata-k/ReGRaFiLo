use regrafilo_core::util::item_arena::{ItemArenaBuilder, ItemBase, ItemIndex};
use regrafilo_util::log::Logger;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Item {
    id: ItemIndex,
}

impl Item {
    fn new() -> Self {
        Item { id: 0 }
    }
}

impl ItemBase for Item {
    fn kind_string() -> &'static str {
        "example"
    }

    fn set_item_id(&mut self, index: usize) {
        self.id = index;
    }

    fn get_item_id(&self) -> usize {
        self.id
    }
}

fn main() {
    Logger::init(false);
    let split: usize = 10;
    let finish: usize = 20;
    let mut builder = ItemArenaBuilder::<Item>::new();
    for _ in 0..split {
        builder.push(Item::new());
    }
    for i in split..finish {
        builder.push_with_name(&format!("{}", i), Item::new());
    }
    let (arena, names) = builder.build();
    println!("arena: {:?}", arena);
    println!("names: {:?}", names);
}
