//! example for create directed hyper graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::graph::helper::{EdgeExistedResultExt, NodeExistedResultExt};
use regrafilo_core::util::Identity;

fn main() {
    let config = GraphConfig::directed_hyper_graph().replace_same_edge();
    let mut graph: Graph<u8, u8> = Graph::create(config);

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
    // cannot create empty hyper edge.
    //  graph.add_directed_hyper_edge(4, vec![], vec![])
    //     .old_edge_exist_to_error()
    //   .unwrap();
    // graph.add_directed_hyper_edge(5, vec![1,3], vec![])
    //     .old_edge_exist_to_error()
    //    .unwrap();
    // graph.add_directed_hyper_edge(6, vec![], vec![3])
    //     .old_edge_exist_to_error()
    //     .unwrap();
    graph
        .add_directed_hyper_edge(7, vec![1, 1], vec![3])
        .old_edge_exist_to_error()
        .unwrap();
    // hyper edge {1, 1}->{3} at the edge_id 7 is same to {1}->{3} at the edge_id 8.
    // replace edge to specified later.
    graph
        .add_directed_hyper_edge(8, vec![1], vec![3])
        .old_edge_exist_to_error()
        .unwrap();
    // This edges at edge_id in 9, 10 is like a self loop.
    graph
        .add_directed_hyper_edge(9, vec![1, 2], vec![1])
        .old_edge_exist_to_error()
        .unwrap();
    graph
        .add_directed_hyper_edge(10, vec![1, 2], vec![1, 2])
        .old_edge_exist_to_error()
        .unwrap();

    print_graph(&graph);
}

fn print_graph<NodeId: Identity, EdgeId: Identity>(graph: &Graph<NodeId, EdgeId>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
