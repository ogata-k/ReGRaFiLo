//! example for create mixed graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::util::Identity;

fn main() {
    let can_multiple = false;
    let use_node_group = true;
    let config = GraphConfig::mixed_graph(can_multiple, use_node_group);
    let mut graph: Graph<u8> = Graph::create_by_config(config);
    graph.add_vertex_node(1);
    graph.add_vertex_node(2);
    graph.add_vertex_node(3);

    // when not inserted node at the id, automatic insert node at the id
    graph.add_directed_edge(1, 1, 100).unwrap();
    // replace edge at edge_id 1 because win insert after
    graph.add_undirected_edge(1, 1, 2).unwrap();
    graph.add_directed_edge(2, 2, 3).unwrap();
    // not multiple edge at edge id 2, 3 because of 2->3 != 3->2
    graph.add_directed_edge(3, 3, 2).unwrap();
    // not multiple edge at edge id 3, 4 because of 2->3 != 3--2
    graph.add_undirected_edge(4, 3, 2).unwrap();
    // error because of multiple edge at edge id 3, 4 because of 2--3 == 3--2
    // graph.add_undirected_edge(5, 2, 3).unwrap();
    graph.add_directed_edge(6, 3, 3).unwrap();

    graph.add_node_grouping(7, vec![3]).unwrap();
    graph.add_node_grouping(8, vec![1, 2]).unwrap();
    // Following is always fail so not replace same edge mode
    // graph.add_node_grouping(9, vec![1, 2]).unwrap();
    // Cannot create grouping have intersection (i.e. edge{1, 2} /\ {1} != empty set) but not same
    // graph.add_node_grouping(10, vec![1]).unwrap();

    print_graph(&graph);
}

fn print_graph<Id: Identity>(graph: &Graph<Id>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
