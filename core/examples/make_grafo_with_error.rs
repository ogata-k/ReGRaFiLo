use std::time::SystemTime;

use regrafilo_core::grafo::graph_item::edge::EdgeItemBuilder;
use regrafilo_core::grafo::graph_item::group::GroupItemBuilder;
use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{NameUsizeGrafo, NameUsizeGrafoBuilder, NameUsizeGrafoError};
use regrafilo_core::util::kind::GraphItemKind;

const ITERATE_COUNT: usize = 5;

// 下のNameXXXXGrafoYYYYのXXXXを適当なものにすればあとは簡単な修正で機能する。 ex NameStrGrafoBuilder
// なおGrafoBuilderを生のまま使うことも可能
type Graph = NameUsizeGrafo;
type GraphBuilder = NameUsizeGrafoBuilder;
type GraphError = NameUsizeGrafoError;

fn select_kind(index: usize) -> GraphItemKind {
    match index % 5 {
        0 => GraphItemKind::Group,
        1 | 2 | 3 => GraphItemKind::Node,
        4 => GraphItemKind::Edge,
        _ => unreachable!(),
    }
}

fn print_errors(errors: Vec<GraphError>) {
    for e in errors.iter() {
        eprintln!("{}", e);
    }
}

fn main() {
    println!("iter count: {}", ITERATE_COUNT);

    let start = SystemTime::now();
    let mut graph: Graph = GraphBuilder::new().build_with_name_default_group(0_usize);

    let mut result = true;

    for i in 0..ITERATE_COUNT {
        let mut group_builder = GroupItemBuilder::new();
        group_builder.set_belong_group(i);
        group_builder.set_name(i + 1);
        let (_result, _errors) = graph.push_group(group_builder);
        result &= _result;
        print_errors(_errors);
    }

    for i in 0..ITERATE_COUNT {
        let mut node_builder = NodeItemBuilder::new();
        node_builder.set_belong_group(i + 2);
        node_builder.set_name(i + 1);
        let (_result, _errors) = graph.push_node(node_builder);
        result &= _result;
        print_errors(_errors);
    }

    for i in 0..ITERATE_COUNT {
        let mut edge_builder = EdgeItemBuilder::new();
        edge_builder.set_belong_group(i);
        // Edge does not have name
        edge_builder.set_start_endpoint(select_kind(i), i);
        edge_builder.set_end_endpoint(select_kind(i.saturating_sub(1)), i.saturating_sub(1));
        let (_result, _errors) = graph.push_edge(edge_builder);
        result &= _result;
        print_errors(_errors);
    }
    let end = SystemTime::now();

    print!(
        "diff: {:.3}ms",
        end.duration_since(start).unwrap().as_micros() as f32 / 1000.0,
    );
    if result {
        println!();
    } else {
        println!(", build item fail exist");
    }

    println!("\n{}", graph);
}
