use std::time::SystemTime;

use regrafilo_core::grafo::graph_item::edge::EdgeItemBuilder;
use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
use regrafilo_core::util::kind::GraphItemKind;

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
    let mut graph: Graph = GraphBuilder::new()
        .build_with_name_default_group("root group")
        .unwrap();
    let mut result = true;
    let mut errors: Vec<GraphError> = Vec::new();

    for i in 0..ITERATE_COUNT {
        let mut node_builder = NodeItemBuilder::new();
        // when not use following method, set root group automatically
        node_builder.set_belong_group("root group");
        if i % 2 == 0 {
            node_builder.set_name(format!("{}", i));
        }
        let (_result, _errors) = graph.push_node(node_builder);
        result &= _result;
        errors.extend(_errors);
    }

    for i in 0..ITERATE_COUNT {
        let mut edge_builder = EdgeItemBuilder::new();
        // when not use following method, set root group automatically
        edge_builder.set_belong_group("root group");
        if i % 2 == 0 {
            edge_builder.set_name(format!("{}", i));
        }
        edge_builder
            .set_start_endpoint(GraphItemKind::Node, format!("{}", (2 * i) % ITERATE_COUNT));
        edge_builder.set_end_endpoint(
            GraphItemKind::Node,
            format!("{}", (2 * (i + 1)) % ITERATE_COUNT),
        );
        let (_result, _errors) = graph.push_edge(edge_builder);
        result &= _result;
        errors.extend(_errors);
    }
    let end = SystemTime::now();

    println!(
        "diff: {:.3}ms, result: {}",
        end.duration_since(start).unwrap().as_micros() as f32 / 1000.0,
        result,
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
