use std::time::SystemTime;

use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{GrafoBuilder, GrafoError};

const ITERATE_COUNT: u32 = 100;
const SHOW: bool = false;

fn main() {
    println!("iter count: {}", ITERATE_COUNT);
    let start = SystemTime::now();
    let mut graph = GrafoBuilder::new().build_with_default();
    let mut result = true;
    let mut errors: Vec<GrafoError> = Vec::new();
    for i in 0..ITERATE_COUNT {
        let mut node_builder = NodeItemBuilder::new();
        if i % 2 == 0 {
            node_builder.set_name(format!("{}", i));
        }
        let (_result, _errors) = graph.push_node(node_builder);
        result &= _result;
        errors.extend(_errors);
    }
    let end = SystemTime::now();
    println!("diff: {}ms", end.duration_since(start).unwrap().as_millis());

    if SHOW {
        if result {
            println!("{:?}", graph);
        } else {
            for error in errors {
                println!("{}", error);
            }
        }
    }
}
