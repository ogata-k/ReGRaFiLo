#[macro_use]
extern crate criterion;

use criterion::Criterion;
use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};

pub type Graph = NameStrGrafo;
pub type GraphBuilder = NameStrGrafoBuilder;
pub type GraphError = NameStrGrafoError;

fn push_items(count: u32) {
    let mut graph: Graph = GraphBuilder::new().build_with_no_name_default_group(Some("root group"));

    let mut errors: Vec<GraphError> = Vec::new();
    push_nodes(&mut graph, &mut errors, count);
}

fn push_nodes(graph: &mut Graph, errors: &mut Vec<GraphError>, count: u32) {
    for i in 0..count {
        let mut node_builder = NodeItemBuilder::new();
        node_builder
            .set_name(format!("node {}", i))
            .set_label(format!("node {}", i));
        let (_result, _errors) = graph.push_node(node_builder);
        errors.extend(_errors);
    }
}

fn push_node_100(c: &mut Criterion) {
    c.bench_function("push_node_100", |b| b.iter(|| push_items(100)));
}

fn push_node_1000(c: &mut Criterion) {
    c.bench_function("push_node_1000", |b| b.iter(|| push_items(1000)));
}

fn push_node_10000(c: &mut Criterion) {
    c.bench_function("push_node_10000", |b| b.iter(|| push_items(10000)));
}

criterion_group!(benches, push_node_100, push_node_1000, push_node_10000);
criterion_main!(benches);
