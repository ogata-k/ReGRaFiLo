use regrafilo_core::grafo::graph_item::edge::EdgeItemBuilder;
use regrafilo_core::grafo::graph_item::node::NodeItemBuilder;
use regrafilo_core::grafo::graph_item::GraphItemBuilderBase;
use regrafilo_core::grafo::{Grafo, NameStrGrafo, NameStrGrafoBuilder, NameStrGrafoError};
use regrafilo_core::util::alias::GroupId;
use regrafilo_core::util::item_base::ItemBase;
use regrafilo_core::util::kind::{GraphItemKind, HasGraphItemKind};
use regrafilo_core::util::name_type::NameType;

const INDENT_WIDTH: usize = 4;

fn main() {
    let graph = make_graph();
    let dotlang_graph = DotlangGraph::from(&graph);
    println!("{}", dotlang_graph);
}

type Graph = NameStrGrafo;
type GraphBuilder = NameStrGrafoBuilder;
type GraphError = NameStrGrafoError;

fn make_graph() -> Graph {
    // TODO 今は仮置きなのでちゃんとしたグラフに置き換える

    let mut graph: Graph = GraphBuilder::new().build_with_name_default_group("root group");

    let mut result = true;
    let mut errors: Vec<GraphError> = Vec::new();

    for i in 0..10 {
        let mut node_builder = NodeItemBuilder::new();
        // when not use following method, set root group automatically
        node_builder.set_belong_group("root group");
        if i % 2 == 0 {
            node_builder.set_name(format!("node {}", i));
        }
        let (_result, _errors) = graph.push_node(node_builder);
        result &= _result;
        errors.extend(_errors);
    }

    for i in 0..5 {
        let mut edge_builder = EdgeItemBuilder::new();
        // when not use following method, set root group automatically
        edge_builder.set_belong_group("root group");
        if i % 2 == 0 {
            edge_builder.set_name(format!("edge {}", i));
        }
        edge_builder.set_start_endpoint(GraphItemKind::Node, format!("node {}", (2 * i) % 5));
        edge_builder.set_end_endpoint(GraphItemKind::Node, format!("node {}", (2 * (i + 1)) % 5));
        let (_result, _errors) = graph.push_edge(edge_builder);
        result &= _result;
        errors.extend(_errors);
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
        let name_default = Name::default();
        let resolver = self.grafo.resolver();
        let root_id = self
            .grafo
            .get_root_group_item()
            .expect("root group is not initialized yet.")
            .get_item_id();
        writeln!(
            f,
            "digraph Grafo {{\n{}label = \"{}\";",
            " ".repeat(indent_count * INDENT_WIDTH),
            resolver
                .get_graph_item_name_by(GraphItemKind::Group, root_id, root_id)
                .expect("not found root group")
        )?;

        writeln!(f, "\n{}// Nodes", " ".repeat(indent_count * INDENT_WIDTH))?;
        for (item_id, item) in self.grafo.get_node_item_iter_limit_by_group_id(root_id) {
            writeln!(
                f,
                "{}{}_{}[label=\"{}\"];",
                " ".repeat(indent_count * INDENT_WIDTH),
                item.get_kind(),
                item_id,
                resolver
                    .get_graph_item_name_by_item(item)
                    .unwrap_or_else(|| &name_default)
            )?;
        }

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

        writeln!(f, "\n{}// Edges", " ".repeat(indent_count * INDENT_WIDTH))?;
        for (_, item) in self.grafo.get_edge_item_iter_limit_by_group_id(root_id) {
            let start = item.get_start_endpoint();
            let end = item.get_end_endpoint();
            writeln!(
                f,
                "{}{}_{} -> {}_{}[label=\"{}\"];",
                " ".repeat(indent_count * INDENT_WIDTH),
                start.get_kind(),
                start.get_item_id(),
                end.get_kind(),
                end.get_item_id(),
                resolver
                    .get_graph_item_name_by_item(item)
                    .unwrap_or_else(|| &name_default)
            )?;
        }
        write!(f, "}}")
    }

    fn write_subgraph(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        indent_count: usize,
        parent_id: GroupId,
        child_id: GroupId,
    ) -> std::fmt::Result {
        let name_default = Name::default();
        let resolver = self.grafo.resolver();
        writeln!(
            f,
            "{}subgraph subgraph_{} {{\n{}label=\"{}\";\n",
            " ".repeat(indent_count * INDENT_WIDTH),
            child_id,
            " ".repeat((indent_count + 1) * INDENT_WIDTH),
            resolver
                .get_graph_item_name_by(GraphItemKind::Group, parent_id, child_id)
                .unwrap_or_else(|| &name_default)
        )?;

        writeln!(
            f,
            "\n{}// Nodes",
            " ".repeat((indent_count + 1) * INDENT_WIDTH)
        )?;
        for (item_id, item) in self.grafo.get_node_item_iter_limit_by_group_id(child_id) {
            writeln!(
                f,
                "{}{}_{}[label=\"{}\"];",
                " ".repeat((indent_count + 1) * INDENT_WIDTH),
                item.get_kind(),
                item_id,
                resolver
                    .get_graph_item_name_by_item(item)
                    .unwrap_or_else(|| &name_default)
            )?;
        }
        writeln!(f)?;

        let child_ids = resolver.get_child_ids(child_id);
        if !child_ids.is_empty() {
            write!(
                f,
                "\n{}// sub graphs",
                " ".repeat((indent_count + 1) * INDENT_WIDTH)
            )?;
            for child_child_id in resolver.get_child_ids(child_id) {
                writeln!(f)?;
                self.write_subgraph(f, indent_count + 1, child_id, child_child_id)?;
            }
        }

        writeln!(
            f,
            "\n{}// Edges",
            " ".repeat((indent_count + 1) * INDENT_WIDTH)
        )?;
        for (_, item) in self.grafo.get_edge_item_iter_limit_by_group_id(child_id) {
            let start = item.get_start_endpoint();
            let end = item.get_end_endpoint();
            writeln!(
                f,
                "{}{}_{} -> {}_{}[label=\"{}\"];",
                " ".repeat((indent_count + 1) * INDENT_WIDTH),
                start.get_kind(),
                start.get_item_id(),
                end.get_kind(),
                end.get_item_id(),
                resolver
                    .get_graph_item_name_by_item(item)
                    .unwrap_or_else(|| &name_default)
            )?;
        }
        writeln!(f, "{}}};", " ".repeat(indent_count * INDENT_WIDTH))
    }
}
