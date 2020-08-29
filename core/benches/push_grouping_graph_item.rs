#[macro_use]
extern crate criterion;

use criterion::Criterion;
use regrafilo_core::grafo::graph_item::edge::EdgeItemBuilder;
use regrafilo_core::grafo::graph_item::group::GroupItemBuilder;
use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
use regrafilo_core::util::kind::GraphItemKind;

pub type Graph = NameStrGrafo;
pub type GraphBuilder = NameStrGrafoBuilder;
pub type GraphError = NameStrGrafoError;

const GROUP_COUNT: u32 = 5;

fn create_base_graph() -> Graph {
    let mut graph: Graph = GraphBuilder::new()
        .build_with_name_default_group("group 0")
        .unwrap();

    for i in 0..GROUP_COUNT {
        let mut group = GroupItemBuilder::new();
        group.set_belong_group(format!("group {}", i));
        group.set_name(format!("group {}", i + 1));
        graph.push_group(group);
    }

    graph
}

fn push_items(count: u32) {
    let mut graph = create_base_graph();

    let mut errors: Vec<GraphError> = Vec::new();
    push_nodes(&mut graph, &mut errors, count);
    push_edges(&mut graph, &mut errors, count);
}

fn push_nodes(graph: &mut Graph, errors: &mut Vec<GraphError>, count: u32) {
    for i in 0..count {
        let mut node_builder = NodeItemBuilder::new();
        node_builder.set_belong_group(format!("group {}", i % GROUP_COUNT));
        node_builder.set_name(format!("{}", i));
        let (_result, _errors) = graph.push_node(node_builder);
        errors.extend(_errors);
    }
}

fn push_edges(graph: &mut Graph, errors: &mut Vec<GraphError>, count: u32) {
    for i in 0..count {
        let mut edge_builder = EdgeItemBuilder::new();
        edge_builder.set_belong_group(format!("group {}", i % GROUP_COUNT));
        edge_builder.set_name(format!("{}", i));
        edge_builder.set_start_endpoint(GraphItemKind::Node, format!("{}", i));
        edge_builder.set_end_endpoint(GraphItemKind::Node, format!("{}", (i + 1) % count));
        let (_result, _errors) = graph.push_edge(edge_builder);
        errors.extend(_errors);
    }
}

fn push_grouping_graph_item_100(c: &mut Criterion) {
    c.bench_function("push_grouping_graph_item_100", |b| {
        b.iter(|| push_items(100 / 2))
    });
}

fn push_grouping_graph_item_1000(c: &mut Criterion) {
    c.bench_function("push_grouping_graph_item_1000", |b| {
        b.iter(|| push_items(1000 / 2))
    });
}

fn push_grouping_graph_item_10000(c: &mut Criterion) {
    c.bench_function("push_grouping_graph_item_10000", |b| {
        b.iter(|| push_items(10000 / 2))
    });
}

criterion_group!(
    benches,
    push_grouping_graph_item_100,
    push_grouping_graph_item_1000,
    push_grouping_graph_item_10000
);
criterion_main!(benches);
