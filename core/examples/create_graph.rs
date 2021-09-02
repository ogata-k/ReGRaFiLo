//! example for create graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::util::Identity;

fn main() {
    let can_multiple = true;
    let use_node_group = true;
    let config =
        GraphConfig::undirected_graph(can_multiple, use_node_group).to_replace_same_edge_mode();
    let mut graph: Graph<u8> = Graph::create_by_config(config);
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);

    // when not inserted node at the id, automatic insert node at the id
    graph.add_undirected_edge(1, 1, 100).unwrap();
    // replace edge at edge_id 1 because win insert after
    graph.add_undirected_edge(1, 1, 2).unwrap();
    graph.add_undirected_edge(2, 2, 3).unwrap();
    graph.add_undirected_edge(3, 3, 1).unwrap();
    graph.add_undirected_edge(4, 1, 1).unwrap();
    graph.add_undirected_edge(5, 2, 2).unwrap();
    graph.add_undirected_edge(6, 3, 3).unwrap();
    // When can_multiple = false,
    // with remove edge_id 6 because of same edge in replace same edge mode
    // or fail insert in not the mode.
    // When can_multiple = true, success insert.
    graph.add_undirected_edge(7, 3, 3).unwrap();

    graph.add_node_grouping(8, vec![1, 2]).unwrap();
    // Following is always fail in not replace same edge mode because of inserted same group
    graph.add_node_grouping(9, vec![1, 2]).unwrap();

    print_graph(&graph);
}

fn print_graph<Id: Identity>(graph: &Graph<Id>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
