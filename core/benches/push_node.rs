#[macro_use]
extern crate criterion;

use criterion::Criterion;
use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};

pub type Graph = NameStrGrafo;
pub type GraphBuilder = NameStrGrafoBuilder;
pub type GraphError = NameStrGrafoError;

fn push_node(count: u32) {
    let mut graph: Graph = GraphBuilder::new().build_with_default().unwrap();
    let mut errors: Vec<GraphError> = Vec::new();
    for i in 0..count {
        let mut node_builder = NodeItemBuilder::new();
        if i % 2 == 0 {
            node_builder.set_name(format!("{}", i));
        }
        let (_result, _errors) = graph.push_node(node_builder);
        errors.extend(_errors);
    }
}

fn push_node_100(c: &mut Criterion) {
    c.bench_function("push_node_100", |b| b.iter(|| push_node(100)));
}

fn push_node_1000(c: &mut Criterion) {
    c.bench_function("push_node_1000", |b| b.iter(|| push_node(1000)));
}

fn push_node_10000(c: &mut Criterion) {
    c.bench_function("push_node_10000", |b| b.iter(|| push_node(10000)));
}

criterion_group!(benches, push_node_100, push_node_1000, push_node_10000);
criterion_main!(benches);
