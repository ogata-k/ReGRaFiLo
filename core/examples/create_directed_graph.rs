//! example for create directed graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::graph::helper::{EdgeExistedResultExt, NodeExistedResultExt};
use regrafilo_core::util::Identity;

fn main() {
    let config = GraphConfig::directed_graph().use_group_node();
    let mut graph: Graph<String, String> = Graph::create(config);

    // Create item action is failed when old item exist.
    // If catch as error, need convert to error.

    // ---
    // Node
    // ---

    graph
        .add_group_node(None, 1.to_string(), vec![])
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_group_node(Some(1.to_string()), 2.to_string(), vec![])
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(Some(1.to_string()), 3.to_string())
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(Some(2.to_string()), 4.to_string())
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_group_node(None, 5.to_string(), vec![])
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(Some(5.to_string()), 6.to_string())
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(None, 7.to_string())
        .old_node_exist_to_error()
        .unwrap();
    // can make group with nodes whose parent is None or specified parent
    graph
        .add_group_node(
            Some(5.to_string()),
            8.to_string(),
            vec![6.to_string(), 7.to_string()],
        )
        .old_node_exist_to_error()
        .unwrap();

    // ---
    // Edge
    // ---

    graph
        .add_directed_edge(1.to_string(), 1.to_string(), 1.to_string())
        .old_edge_exist_to_error()
        .unwrap();
    graph
        .add_directed_edge(2.to_string(), 2.to_string(), 3.to_string())
        .old_edge_exist_to_error()
        .unwrap();
    // cannot create edge between a group and the group's child
    // graph.add_directed_edge(3.to_string(), 2.to_string(), 4.to_string()).unwrap();
    // graph.add_directed_edge(4.to_string(), 4.to_string(), 2.to_string()).unwrap();
    graph
        .add_directed_edge(5.to_string(), 2.to_string(), 5.to_string())
        .old_edge_exist_to_error()
        .unwrap();
    graph
        .add_directed_edge(6.to_string(), 6.to_string(), 2.to_string())
        .old_edge_exist_to_error()
        .unwrap();
    // If replace same edge mode, then success insert with replace same edge.
    // This edge is not same to the edge with edge_id 2.Because fail assert 2->3 != 3->2.
    graph
        .add_directed_edge(7.to_string(), 3.to_string(), 2.to_string())
        .old_edge_exist_to_error()
        .unwrap();

    print_graph(&graph);
}

fn print_graph<NodeId: Identity, EdgeId: Identity>(graph: &Graph<NodeId, EdgeId>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
