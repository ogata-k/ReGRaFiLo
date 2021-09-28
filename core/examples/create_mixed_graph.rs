//! example for create mixed graph without layout

use regrafilo_core::graph::{Graph, GraphConfig};
use regrafilo_core::graph::helper::GraphItemExistedResultExt;
use regrafilo_core::util::Identity;

fn main() {
    let can_multiple = false;
    let use_node_group = false;
    let config =
        GraphConfig::mixed_graph(can_multiple, use_node_group).to_create_not_exist_vertex_node();
    let mut graph: Graph<u8> = Graph::create_by_config(config);

    // Create item action is failed when old item exist.
    // If catch as error, need convert to error.

    // ---
    // Node
    // ---

    // Cannot create group. Because using config not allow.
    // graph
    //     .add_group_node(None, 1, vec![])
    //     .old_node_exist_to_error()
    //     .unwrap();
    // graph
    //     .add_group_node(Some(1), 2, vec![])
    //     .old_node_exist_to_error()
    //     .unwrap();
    // graph
    //     .add_vertex_node(Some(1), 3)
    //     .old_node_exist_to_error()
    //     .unwrap();

    graph
        .add_vertex_node(None, 4)
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(None, 5)
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(None, 6)
        .old_node_exist_to_error()
        .unwrap();
    graph
        .add_vertex_node(None, 7)
        .old_node_exist_to_error()
        .unwrap();

    // ---
    // Edge
    //
    // In this example,
    //
    // create directed edge when source and target is even or odd if source <= target.
    // create undirected edge when source is even(odd) and target is odd(even) if source <= target.
    // ---
    let mut loop_count = 0; // use as edge_id
    let start = 4;
    let end = 8; // The node with node_id 8 create when it used at first.
    for source_id in start..=end {
        for target_id in start..=end {
            loop_count += 1;
            let mod_source_id = source_id % 2;
            let mod_target_id = target_id % 2;
            if source_id > target_id {
                continue;
            }
            if mod_source_id == mod_target_id {
                graph
                    .add_directed_edge(loop_count, source_id, target_id)
                    .old_edge_exist_to_error()
                    .unwrap();
                continue;
            } else {
                graph
                    .add_undirected_edge(loop_count, source_id, target_id)
                    .old_edge_exist_to_error()
                    .unwrap();
                continue;
            }
        }
    }

    print_graph(&graph);
}

fn print_graph<Id: Identity>(graph: &Graph<Id>) {
    println!("\nDebug:\n\t{:?}", graph);
    println!("\nDisplay:\n\t{}", graph);
}
