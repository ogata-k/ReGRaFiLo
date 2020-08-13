#[macro_use]
extern crate criterion;

use criterion::Criterion;
use regrafilo_core::grafo::graph_item::edge::EdgeItemBuilder;
use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
use regrafilo_core::util::kind::GraphItemKind;

pub type Graph = NameStrGrafo;
pub type GraphBuilder = NameStrGrafoBuilder;
pub type GraphError = NameStrGrafoError;

fn push_items(count: u32) {
    let mut graph: Graph = GraphBuilder::new()
        .build_with_no_name_default_group()
        .unwrap();
    let mut errors: Vec<GraphError> = Vec::new();
    push_nodes(&mut graph, &mut errors, count);
    push_edges(&mut graph, &mut errors, count);
}

fn push_nodes(graph: &mut Graph, errors: &mut Vec<GraphError>, count: u32) {
    for i in 0..count {
        let mut node_builder = NodeItemBuilder::new();
        if i % 2 == 0 {
            node_builder.set_name(format!("{}", i));
        }
        let (_result, _errors) = graph.push_node(node_builder);
        errors.extend(_errors);
    }
}

fn push_edges(graph: &mut Graph, errors: &mut Vec<GraphError>, count: u32) {
    for i in 0..count {
        let mut edge_builder = EdgeItemBuilder::new();
        if i % 2 == 0 {
            edge_builder.set_name(format!("{}", i));
        }
        edge_builder.set_start_endpoint(GraphItemKind::Node, format!("{}", (2 * i) % count));
        edge_builder.set_end_endpoint(GraphItemKind::Node, format!("{}", (2 * (i + 1)) % count));
        let (_result, _errors) = graph.push_edge(edge_builder);
        errors.extend(_errors);
    }
}

fn push_node_and_edge_100(c: &mut Criterion) {
    c.bench_function("push_node_and_edge_100", |b| b.iter(|| push_items(100)));
}

fn push_node_and_edge_1000(c: &mut Criterion) {
    c.bench_function("push_node_and_edge_1000", |b| b.iter(|| push_items(1000)));
}

fn push_node_and_edge_10000(c: &mut Criterion) {
    c.bench_function("push_node_and_edge_10000", |b| b.iter(|| push_items(10000)));
}

criterion_group!(
    benches,
    push_node_and_edge_100,
    push_node_and_edge_1000,
    push_node_and_edge_10000
);
criterion_main!(benches);