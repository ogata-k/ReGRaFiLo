//! example for create hyper graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::graph::helper::GraphItemExistedResultExt;
use regrafilo_core::util::Identity;

fn main() {
    let can_multiple = true;
    let config = GraphConfig::undirected_hyper_graph(can_multiple);
    let mut graph: Graph<u8> = Graph::create_by_config(config);

    // Create item action is failed when old item exist.
    // If catch as error, need convert to error.

    // ---
    // Node
    // ---

    graph
        .add_vertex_node(None, 1)
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(None, 2)
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(None, 3)
        .old_node_exist_to_error()
        .unwrap();

    // ---
    // Edge
    // ---

    // fail insert edge
    // graph
    //  .add_undirected_edge(1, 1, 2)
    //  .old_edge_exist_to_error()
    //  .unwrap();
    // graph
    //  .add_directed_edge(2, 2, 3)
    //  .old_edge_exist_to_error()
    //  .unwrap();

    graph
        .add_undirected_hyper_edge(3, vec![1, 2])
        .old_edge_exist_to_error()
        .unwrap();
    // cannot create empty hyper edge.
    // graph.add_undirected_hyper_edge(4, vec![])
    //      .old_edge_exist_to_error()
    //      .unwrap();
    graph
        .add_undirected_hyper_edge(5, vec![1, 1])
        .old_edge_exist_to_error()
        .unwrap();
    // hyper edge {1, 1} at the edge_id 5 is same to {1} at the edge_id 6.
    // success insert multiple edge.
    graph
        .add_undirected_hyper_edge(6, vec![1])
        .old_edge_exist_to_error()
        .unwrap();

    print_graph(&graph);
}

fn print_graph<Id: Identity>(graph: &Graph<Id>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
