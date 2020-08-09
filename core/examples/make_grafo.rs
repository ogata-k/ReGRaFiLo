use std::time::SystemTime;

use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};

const ITERATE_COUNT: usize = 100;
const SHOW: bool = false;

// 下のNameXXXXGrafoYYYYのXXXXを適当なものにすればあとは簡単な修正で機能する。 ex NameStrGrafoBuilder
// なおGrafoBuilderを生のまま使うことも可能
type Graph = NameStrGrafo;
type GraphBuilder = NameStrGrafoBuilder;
type GraphError = NameStrGrafoError;

fn main() {
    println!("iter count: {}", ITERATE_COUNT);
    let start = SystemTime::now();
    let mut graph: Graph = GraphBuilder::new().build_with_default().unwrap();
    let mut result = true;
    let mut errors: Vec<GraphError> = Vec::new();
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
    println!(
        "diff: {:.3}ms",
        end.duration_since(start).unwrap().as_micros() as f32 / 1000.0
    );

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
