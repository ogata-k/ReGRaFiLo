use std::env::current_exe;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use regrafilo_core::grafo::graph_item::edge::EdgeItemBuilder;
use regrafilo_core::grafo::graph_item::group::GroupItemBuilder;
use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::{GraphItemBase, GraphItemBuilderBase};
use regrafilo_core::grafo::{Grafo, NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
use regrafilo_core::util::alias::{GroupId, ItemId};
use regrafilo_core::util::item_base::ItemBase;
use regrafilo_core::util::kind::{GraphItemKind, HasGraphItemKind};
use regrafilo_core::util::name_type::NameType;

const INDENT_WIDTH: usize = 4;

fn main() {
    let graph = make_graph();
    let dotlang_graph = DotlangGraph::from(&graph);

    if cfg!(debug_assertions) {
        println!("{}", dotlang_graph);
    } else {
        //
        // assume that you install dot command(graphviz).
        // if you not installed dot command, set false to use_convert for no convert action.
        //
        let use_convert = true;

        // ../target/release/examples/example.dot
        let mut path = current_exe().unwrap();
        path.pop();
        path.push("example.dot");
        let mut file = OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path.clone())
            .expect("open file fail");
        file.write_all(format!("{}", dotlang_graph).as_bytes())
            .expect("write all fail");
        file.flush().expect("flush fail");
        println!("created dotLang file");
        if use_convert {
            println!("converting to png...");
            let mut converted_path = current_exe().unwrap();
            converted_path.pop();
            converted_path.push("example.png");

            Command::new("dot")
                .args(&[
                    "-Tpng",
                    "-o",
                    converted_path.to_str().unwrap(),
                    path.to_str().unwrap(),
                ])
                .status()
                .unwrap_or_else(|e| panic!(e));
            println!("finish convert");
        }
    }
}

type Graph = NameStrGrafo;
type GraphBuilder = NameStrGrafoBuilder;
type GraphError = NameStrGrafoError;

fn print_error(result: bool, errors: Vec<GraphError>) {
    if result {
        for e in errors {
            println!("warning:\t{}", e);
        }
    } else {
        for e in errors {
            eprintln!("error:\t{}", e);
        }
    }
}

fn to_item_name(kind: GraphItemKind, id: ItemId) -> impl NameType {
    if kind == GraphItemKind::Group {
        format!("cluster_{}_{}", kind, id)
    } else {
        format!("{}_{}", kind, id)
    }
}

fn make_graph() -> Graph {
    let mut graph: Graph =
        GraphBuilder::new().build_with_name_default_group("root group", Some("example graph"));
    let mut result = true;

    for i in 1..=3 {
        let mut group_builder = GroupItemBuilder::new();
        // when not use following method, set root group automatically
        group_builder
            .set_belong_group("root group")
            .set_name(format!("V_{}", i))
            .set_label(format!("V_{}", i));
        let (_result, _errors) = graph.push_group(group_builder);
        result &= _result;
        print_error(_result, _errors);
    }

    for i in 1..=2 {
        let mut node_builder = NodeItemBuilder::new();
        // when not use following method, set root group automatically
        node_builder
            .set_belong_group("V_1")
            .set_name(format!("a_{}", i))
            .set_label(format!("a_{}", i));
        let (_result, _errors) = graph.push_node(node_builder);
        result &= _result;
        print_error(_result, _errors);
    }

    for i in 1..=2 {
        let mut node_builder = NodeItemBuilder::new();
        node_builder
            .set_belong_group("V_2")
            .set_name(format!("b_{}", i))
            .set_label(format!("b_{}", i));
        let (_result, _errors) = graph.push_node(node_builder);
        result &= _result;
        print_error(_result, _errors);
    }

    for i in 1..=3 {
        let mut node_builder = NodeItemBuilder::new();
        node_builder
            .set_belong_group("V_3")
            .set_name(format!("c_{}", i))
            .set_label(format!("c_{}", i));
        let (_result, _errors) = graph.push_node(node_builder);
        result &= _result;
        print_error(_result, _errors);
    }

    for i in 1..=2 {
        let mut edge_builder = EdgeItemBuilder::new();
        // when not use following method, set root group automatically
        edge_builder
            .set_start_endpoint(GraphItemKind::Group, format!("V_{}", i))
            .set_end_endpoint(GraphItemKind::Group, format!("V_{}", i + 1));
        let (_result, _errors) = graph.push_edge(edge_builder);
        result &= _result;
        print_error(_result, _errors);
    }

    let mut edge_builder = EdgeItemBuilder::new();
    edge_builder
        .set_belong_group("V_2")
        .set_start_endpoint(GraphItemKind::Node, "b_1")
        .set_end_endpoint(GraphItemKind::Node, "b_2");
    let (_result, _errors) = graph.push_edge(edge_builder);
    result &= _result;
    print_error(_result, _errors);

    let mut edge_builder = EdgeItemBuilder::new();
    edge_builder
        .set_belong_group("V_3")
        .set_start_endpoint(GraphItemKind::Node, "c_2")
        .set_end_endpoint(GraphItemKind::Node, "c_2");
    let (_result, _errors) = graph.push_edge(edge_builder);
    result &= _result;
    print_error(_result, _errors);

    for (is_reverse, i, j) in [(true, 1, 3), (false, 1, 2), (true, 2, 2), (false, 2, 3)].iter() {
        let mut edge_builder = EdgeItemBuilder::new();
        // when not use following method, set root group automatically
        edge_builder.set_belong_group("root group");
        if !*is_reverse {
            edge_builder
                .set_start_endpoint(GraphItemKind::Node, format!("a_{}", i))
                .set_end_endpoint(GraphItemKind::Group, format!("V_{}", j));
        } else {
            edge_builder
                .set_start_endpoint(GraphItemKind::Group, format!("V_{}", j))
                .set_end_endpoint(GraphItemKind::Node, format!("a_{}", i));
        }

        let (_result, _errors) = graph.push_edge(edge_builder);
        result &= _result;
        print_error(_result, _errors);
    }

    for (is_reverse, i, j) in [
        (false, 1, 1),
        (false, 1, 2),
        (true, 1, 3),
        (true, 2, 1),
        (true, 2, 2),
        (false, 2, 3),
    ]
    .iter()
    {
        let mut edge_builder = EdgeItemBuilder::new();
        edge_builder.set_belong_group("root group");
        if !*is_reverse {
            edge_builder
                .set_start_endpoint(GraphItemKind::Node, format!("b_{}", i))
                .set_end_endpoint(GraphItemKind::Node, format!("c_{}", j));
        } else {
            edge_builder
                .set_start_endpoint(GraphItemKind::Node, format!("c_{}", j))
                .set_end_endpoint(GraphItemKind::Node, format!("b_{}", i));
        }

        let (_result, _errors) = graph.push_edge(edge_builder);
        result &= _result;
        print_error(_result, _errors);
    }

    if result {
        println!("all item build success");
    } else {
        eprintln!("build fail item exist");
    }

    graph
}

#[derive(Debug, Clone)]
struct DotlangGraph<'a, Name: NameType> {
    grafo: &'a Grafo<Name>,
}

impl<'a, Name: NameType> From<&'a Grafo<Name>> for DotlangGraph<'a, Name> {
    fn from(graph: &'a Grafo<Name>) -> Self {
        Self { grafo: &graph }
    }
}

impl<'a, Name: NameType + Default> std::fmt::Display for DotlangGraph<'a, Name> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_graph(f)
    }
}

impl<'a, Name: NameType + Default> DotlangGraph<'a, Name> {
    fn write_graph(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indent_count: usize = 1;
        let resolver = self.grafo.resolver();
        let root_id = self
            .grafo
            .get_root_group_item()
            .expect("root group is not initialized yet.")
            .get_item_id();
        let root_group = self
            .grafo
            .get_root_group_item()
            .expect("get root group fail");

        writeln!(
            f,
            "digraph Grafo{{\n{}graph[layout=fdp, compound=true, label=\"{}\", sep=0.3];",
            " ".repeat(indent_count * INDENT_WIDTH),
            root_group.get_label().unwrap_or_else(|| "")
        )?;

        let child_ids = resolver.get_child_ids(root_id);
        if !child_ids.is_empty() {
            write!(
                f,
                "\n{}// SubGraphs",
                " ".repeat(indent_count * INDENT_WIDTH)
            )?;
            for child_id in resolver.get_child_ids(root_id) {
                writeln!(f)?;
                self.write_subgraph(f, 1, root_id, child_id)?;
            }
        }

        writeln!(f, "\n{}// Nodes", " ".repeat(indent_count * INDENT_WIDTH))?;
        for (item_id, item) in self.grafo.get_node_item_iter_limit_by_group_id(root_id) {
            writeln!(
                f,
                "{}{}[label=\"{}\"];",
                " ".repeat(indent_count * INDENT_WIDTH),
                to_item_name(item.get_kind(), *item_id),
                item.get_label().unwrap_or_else(|| "")
            )?;
        }

        writeln!(f, "\n{}// Edges", " ".repeat(indent_count * INDENT_WIDTH))?;
        for (_, item) in self.grafo.get_edge_item_iter_limit_by_group_id(root_id) {
            let start = item.get_start_endpoint();
            let end = item.get_end_endpoint();
            writeln!(
                f,
                "{}{} -> {}[label=\"{}\"];",
                " ".repeat(indent_count * INDENT_WIDTH),
                to_item_name(start.get_kind(), start.get_item_id()),
                to_item_name(end.get_kind(), end.get_item_id()),
                item.get_label().unwrap_or_else(|| "")
            )?;
        }
        write!(f, "}}")
    }

    fn write_subgraph(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent_count: usize,
        parent_id: GroupId,
        self_id: GroupId,
    ) -> std::fmt::Result {
        let resolver = self.grafo.resolver();
        let group = self
            .grafo
            .get_group_item(parent_id, self_id)
            .expect("get specified group fail");

        writeln!(
            f,
            "{}subgraph {} {{\n{}graph[label=\"{}\"];\n{}node [style=filled];\n{}color=black;",
            " ".repeat(indent_count * INDENT_WIDTH),
            to_item_name(GraphItemKind::Group, self_id),
            " ".repeat((indent_count + 1) * INDENT_WIDTH),
            group.get_label().unwrap_or_else(|| ""),
            " ".repeat((indent_count + 1) * INDENT_WIDTH),
            " ".repeat((indent_count + 1) * INDENT_WIDTH)
        )?;

        writeln!(
            f,
            "\n{}// Nodes",
            " ".repeat((indent_count + 1) * INDENT_WIDTH)
        )?;
        for (item_id, item) in self.grafo.get_node_item_iter_limit_by_group_id(self_id) {
            writeln!(
                f,
                "{}{}[label=\"{}\"];",
                " ".repeat((indent_count + 1) * INDENT_WIDTH),
                to_item_name(item.get_kind(), *item_id),
                item.get_label().unwrap_or_else(|| "")
            )?;
        }

        let child_ids = resolver.get_child_ids(self_id);
        if !child_ids.is_empty() {
            write!(
                f,
                "\n{}// sub graphs",
                " ".repeat((indent_count + 1) * INDENT_WIDTH)
            )?;
            for child_child_id in resolver.get_child_ids(self_id) {
                writeln!(f)?;
                self.write_subgraph(f, indent_count + 1, self_id, child_child_id)?;
            }
        }

        writeln!(
            f,
            "\n{}// Edges",
            " ".repeat((indent_count + 1) * INDENT_WIDTH)
        )?;
        for (_, item) in self.grafo.get_edge_item_iter_limit_by_group_id(self_id) {
            let start = item.get_start_endpoint();
            let end = item.get_end_endpoint();
            writeln!(
                f,
                "{}{} -> {}[label=\"{}\"];",
                " ".repeat((indent_count + 1) * INDENT_WIDTH),
                to_item_name(start.get_kind(), start.get_item_id()),
                to_item_name(end.get_kind(), end.get_item_id()),
                item.get_label().unwrap_or_else(|| "")
            )?;
        }
        writeln!(f, "{}}};", " ".repeat(indent_count * INDENT_WIDTH))
    }
}
