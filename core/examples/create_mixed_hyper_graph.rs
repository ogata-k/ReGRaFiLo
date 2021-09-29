//! example for create mixed hyper graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::graph::helper::GraphItemExistedResultExt;
use regrafilo_core::util::Identity;

fn main() {
    let can_multiple = false;
    let config = GraphConfig::mixed_hyper_graph(can_multiple);
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
        .add_directed_hyper_edge(3, vec![1, 2], vec![3])
        .old_edge_exist_to_error()
        .unwrap();
    // not same directed hyper edge {1, 2}->{3} != {3}->{1, 2}
    graph
        .add_directed_hyper_edge(4, vec![3], vec![1, 2])
        .old_edge_exist_to_error()
        .unwrap();
    // cannot insert same edge
    //graph
    //  .add_directed_hyper_edge(5, vec![1,2], vec![3])
    //.old_edge_exist_to_error()
    //  .unwrap();
    graph
        .add_undirected_hyper_edge(5, vec![1, 3])
        .old_edge_exist_to_error()
        .unwrap();
    graph
        .add_undirected_hyper_edge(6, vec![2])
        .old_edge_exist_to_error()
        .unwrap();

    print_graph(&graph);
}

fn print_graph<Id: Identity>(graph: &Graph<Id>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
