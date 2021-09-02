//! example for create hyper graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::util::Identity;

fn main() {
    let can_multiple = true;
    let config = GraphConfig::undirected_hyper_graph(can_multiple).to_replace_same_edge_mode();
    let mut graph: Graph<u8> = Graph::create_by_config(config);
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);

    // fail insert edge
    // graph.add_undirected_edge(1, 1, 2).unwrap();
    // graph.add_directed_edge(2, 2, 3).unwrap();

    // when not inserted node at the id, automatic insert node at the id
    graph.add_undirected_hyper_edge(3, vec![1, 100]).unwrap();
    // this method is alias to add_undirected_hyper_edge()
    graph.add_node_grouping(4, vec![1, 2, 3]).unwrap();
    // fail insert because empty hyper edge is illegal
    // graph.add_undirected_hyper_edge(5, vec![]).unwrap();
    // hyper edge {2,2} is same to {2}
    graph.add_undirected_hyper_edge(6, vec![3, 3]).unwrap();
    graph.add_undirected_hyper_edge(7, vec![1]).unwrap();
    // replace same edge_id 7
    graph.add_undirected_hyper_edge(7, vec![2]).unwrap();
    graph.add_undirected_hyper_edge(8, vec![1, 2]).unwrap();
    // success insert because can_multiple = true, so not replace
    graph.add_undirected_hyper_edge(9, vec![1, 2]).unwrap();

    print_graph(&graph);
}

fn print_graph<Id: Identity>(graph: &Graph<Id>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
