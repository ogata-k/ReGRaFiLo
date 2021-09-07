//! example for create directed hyper graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::util::Identity;

fn main() {
    let can_multiple = true;
    let config = GraphConfig::directed_hyper_graph(can_multiple).to_replace_same_edge_mode();
    let mut graph: Graph<u8> = Graph::create_by_config(config);
    graph.add_vertex_node(1);
    graph.add_vertex_node(2);
    graph.add_vertex_node(3);

    // fail insert edge
    // graph.add_undirected_edge(1, 1, 2).unwrap();
    // graph.add_directed_edge(2, 2, 3).unwrap();

    // when not inserted node at the id, automatic insert node at the id
    graph
        .add_directed_hyper_edge(3, vec![1, 2], vec![100])
        .unwrap();
    // this method is alias to add_directed_hyper_edge()
    graph
        .add_directed_hyper_edge(3, vec![1, 2], vec![3])
        .unwrap();
    // not same hyper edge {1, 2}->{3} != {3}->{1, 2}
    graph
        .add_directed_hyper_edge(4, vec![3], vec![1, 2])
        .unwrap();
    // fail insert because empty hyper edge is illegal
    // graph.add_directed_hyper_edge(5, vec![], vec![]).unwrap();
    // graph.add_directed_hyper_edge(6, vec![1], vec![]).unwrap();
    // graph.add_directed_hyper_edge(7, vec![], vec![1]).unwrap();
    // hyper edge {1}->{1} is loop hyper edge, but not edge
    graph.add_directed_hyper_edge(8, vec![1], vec![1]).unwrap();
    // replace same edge_id 8 because of same edge replace mode if can_multiple = false
    graph.add_directed_hyper_edge(9, vec![1], vec![1]).unwrap();
    graph
        .add_directed_hyper_edge(10, vec![1, 2], vec![1])
        .unwrap();
    // success insert because can_multiple = true, so not replace
    graph
        .add_directed_hyper_edge(11, vec![1, 2], vec![1])
        .unwrap();

    print_graph(&graph);
}

fn print_graph<Id: Identity>(graph: &Graph<Id>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
