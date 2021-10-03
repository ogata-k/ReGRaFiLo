//! example for create graph without layout

use regrafilo_core::graph::{EdgeExistedResultExt, Graph, GraphConfig, NodeExistedResultExt};
use regrafilo_core::util::Identity;

fn main() {
    let config = GraphConfig::undirected_graph()
        .use_group_node()
        .use_multiple_edge()
        .replace_same_edge()
        .create_not_exist_vertex_node();
    let mut graph: Graph<u8, u8> = Graph::create(config);

    // Create item action is failed when old item exist.
    // If catch as error, need convert to error.

    // ---
    // Node
    // ---

    graph
        .add_group_node(None, 1, vec![])
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_group_node(Some(1), 2, vec![4])
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(Some(1), 3)
        .old_node_exist_to_error()
        .unwrap();
    // already created when create the group node at node_id 2.
    // graph.add_vertex_node(Some(2), 4).old_node_exist_to_error().unwrap();
    graph
        .add_group_node(None, 5, vec![])
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(Some(5), 6)
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(None, 7)
        .old_node_exist_to_error()
        .unwrap();
    // can make group with nodes whose parent is None or specified parent
    graph
        .add_group_node(Some(5), 8, vec![6, 7])
        .old_node_exist_to_error()
        .unwrap();

    // ---
    // Edge
    // ---

    graph
        .add_undirected_edge(1, 1, 1)
        .old_edge_exist_to_error()
        .unwrap();
    graph
        .add_undirected_edge(2, 2, 3)
        .old_edge_exist_to_error()
        .unwrap();
    // cannot create edge between a group and the group's child
    // graph.add_undirected_edge(3, 2, 4).unwrap();
    // graph.add_undirected_edge(4, 4, 2).unwrap();
    graph
        .add_undirected_edge(5, 2, 5)
        .old_edge_exist_to_error()
        .unwrap();
    graph
        .add_undirected_edge(6, 6, 2)
        .old_edge_exist_to_error()
        .unwrap();
    // When use replace same edge mode, insert edge successfully with replace same edge.
    // This edge is same to the edge with edge_id 2.
    graph
        .add_undirected_edge(7, 3, 2)
        .old_edge_exist_to_error()
        .unwrap();

    print_graph(&graph);
}

fn print_graph<NodeId: Identity, EdgeId: Identity>(graph: &Graph<NodeId, EdgeId>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
