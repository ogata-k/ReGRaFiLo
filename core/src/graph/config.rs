//! Module for graph configuration

use std::fmt;

/// Kind of Graph. The kind is HyperGraph or not.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GraphKind {
    /// Undirected Graph or Directed Graph
    Graph,

    /// Undirected Hyper Graph or Directed Hyper Graph
    HyperGraph,
}

impl fmt::Display for GraphKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", &self))
    }
}

impl GraphKind {
    /// check graph is graph which has edge
    pub fn is_graph(&self) -> bool {
        self == &GraphKind::Graph
    }

    /// check graph is graph which has hyper edge
    pub fn is_hyper_graph(&self) -> bool {
        self == &GraphKind::HyperGraph
    }
}

/// Type of Graph. We use the type to check edge type.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GraphType {
    /// Type for Graph with undirected edge.
    UndirectedGraph,

    /// Type for Graph with Directed edge.
    DirectedGraph,

    /// Type for Graph with undirected edge or Directed edge.
    MixedGraph,

    /// Type for Graph with undirected Hyper edge.
    UndirectedHyperGraph,

    /// Type for Graph with Directed Hyper edge.
    DirectedHyperGraph,

    /// Config for Graph with undirected Hyper edge or Directed Hyper edge.
    MixedHyperGraph,
}

impl fmt::Display for GraphType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphType::*;

        match self {
            UndirectedGraph => f.write_str("Graph"),
            DirectedGraph => f.write_str("DirectedGraph"),
            MixedGraph => f.write_str("MixedGraph"),
            UndirectedHyperGraph => f.write_str("HyperGraph"),
            DirectedHyperGraph => f.write_str("DirectedHyperGraph"),
            MixedHyperGraph => f.write_str("MixedHyperGraph"),
        }
    }
}

impl GraphType {
    /// check type is for Graph
    pub fn is_undirected_graph(&self) -> bool {
        self == &GraphType::UndirectedGraph
    }

    /// check type is for Directed Graph
    pub fn is_directed_graph(&self) -> bool {
        self == &GraphType::DirectedGraph
    }

    /// check type is for Mixed Graph
    pub fn is_mixed_graph(&self) -> bool {
        self == &GraphType::MixedGraph
    }

    /// check type is for Hyper Graph
    pub fn is_undirected_hyper_graph(&self) -> bool {
        self == &GraphType::UndirectedHyperGraph
    }

    /// check type is for Directed Hyper Graph
    pub fn is_directed_hyper_graph(&self) -> bool {
        self == &GraphType::DirectedHyperGraph
    }

    /// check type is for Mixed Hyper Graph
    pub fn is_mixed_hyper_graph(&self) -> bool {
        self == &GraphType::MixedHyperGraph
    }
}

/// configuration for graph without layout
/// Not impl Copy because use reference and not create again and again.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GraphConfig {
    // ---
    // common
    // ---
    /// kind of graph
    kind: GraphKind,

    /// this option is a flag which we can group node
    use_group_node: bool,

    /// the optional option is a flag to do or not to do create edge's incidence node or child node for a group as vertex node if not exist
    create_not_exist_vertex_node: bool,

    /// this option is a flag which we can use check to make multiple edge
    multiple_edge: bool,

    /// this optional option is a flag remove same edge when insert.
    /// If set the mode and cannot use multiple edge, replace same edge.
    replace_same_edge: bool,

    // ---
    // Kind: Graph
    // usually use in graph or directed graph.
    // ---
    /// undirected edge
    undirected_edge: bool,
    /// directed edge
    directed_edge: bool,

    // ---
    // Kind: HyperGraph
    // usually use in hyper graph or directed hyper graph
    // ---
    /// undirected hyper edge
    undirected_hyper_edge: bool,
    /// directed hyper edge
    directed_hyper_edge: bool,
}

impl fmt::Display for GraphConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let graph_type = self.get_type();
        let kind = self.get_kind();

        match &graph_type {
            GraphType::UndirectedGraph => f.write_fmt(format_args!(
                "{{kind: {}, node_group: {}, can_multiple: {}}}",
                kind, self.use_group_node, self.multiple_edge
            )),
            GraphType::DirectedGraph => f.write_fmt(format_args!(
                "{{kind: {}, node_group: {}, can_multiple: {}}}",
                kind, self.use_group_node, self.multiple_edge
            )),
            GraphType::MixedGraph => f.write_fmt(format_args!(
                "{{kind: {}, node_group: {}, can_multiple: {}}}",
                kind, self.use_group_node, self.multiple_edge
            )),
            GraphType::UndirectedHyperGraph => f.write_fmt(format_args!(
                "{{kind: {}, node_group: {}, can_multiple: {}}}",
                kind, self.use_group_node, self.multiple_edge
            )),
            GraphType::DirectedHyperGraph => f.write_fmt(format_args!(
                "{{kind: {}, node_group: {}, can_multiple: {}}}",
                kind, self.use_group_node, self.multiple_edge
            )),
            GraphType::MixedHyperGraph => f.write_fmt(format_args!(
                "{{kind: {}, node_group: {}, can_multiple: {}}}",
                kind, self.use_group_node, self.multiple_edge
            )),
        }
    }
}

impl GraphConfig {
    // ---
    // constructor
    // ---

    /// construct minimum config for Graph
    pub fn undirected_graph() -> Self {
        Self {
            kind: GraphKind::Graph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
            undirected_edge: true,
            directed_edge: false,
            undirected_hyper_edge: false,
            directed_hyper_edge: false,
        }
    }

    /// construct minimum config for Directed Graph
    pub fn directed_graph() -> Self {
        Self {
            kind: GraphKind::Graph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
            undirected_edge: false,
            directed_edge: true,
            undirected_hyper_edge: false,
            directed_hyper_edge: false,
        }
    }

    /// construct minimum config for Mixed Graph
    pub fn mixed_graph() -> Self {
        Self {
            kind: GraphKind::Graph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
            undirected_edge: true,
            directed_edge: true,
            undirected_hyper_edge: false,
            directed_hyper_edge: false,
        }
    }

    /// construct minimum config for Hyper Graph
    pub fn undirected_hyper_graph() -> Self {
        Self {
            kind: GraphKind::HyperGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
            undirected_edge: false,
            directed_edge: false,
            undirected_hyper_edge: true,
            directed_hyper_edge: false,
        }
    }

    /// construct minimum config for Directed Hyper Graph
    pub fn directed_hyper_graph() -> Self {
        Self {
            kind: GraphKind::HyperGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
            undirected_edge: false,
            directed_edge: false,
            undirected_hyper_edge: false,
            directed_hyper_edge: true,
        }
    }

    /// construct minimum config for Mixed Hyper Graph
    pub fn mixed_hyper_graph() -> Self {
        Self {
            kind: GraphKind::HyperGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
            undirected_edge: false,
            directed_edge: false,
            undirected_hyper_edge: true,
            directed_hyper_edge: true,
        }
    }

    // ---
    // getter
    // ---

    /// get graph kind
    pub fn get_kind(&self) -> GraphKind {
        self.kind
    }

    /// get graph type
    #[inline]
    pub fn get_type(&self) -> GraphType {
        match self {
            Self {
                kind: GraphKind::Graph,
                undirected_edge: is_undirected,
                directed_edge: is_directed,
                undirected_hyper_edge: false,
                directed_hyper_edge: false,
                ..
            } => match (is_undirected, is_directed) {
                (true, true) => GraphType::MixedGraph,
                (true, false) => GraphType::UndirectedGraph,
                (false, true) => GraphType::DirectedGraph,
                _ => panic!("Illegal config: {:?}", self),
            },
            Self {
                kind: GraphKind::HyperGraph,
                undirected_edge: false,
                directed_edge: false,
                undirected_hyper_edge: is_undirected_hyper,
                directed_hyper_edge: is_directed_hyper,
                ..
            } => match (is_undirected_hyper, is_directed_hyper) {
                (true, true) => GraphType::MixedHyperGraph,
                (true, false) => GraphType::UndirectedHyperGraph,
                (false, true) => GraphType::DirectedHyperGraph,
                _ => panic!("Illegal config: {:?}", self),
            },
            _ => panic!("Illegal config: {:?}", self),
        }
    }

    // ---
    // setter
    // ---

    /// to replace same edge mode when insert edge
    /// If set the mode and cannot use multiple edge, replace same edge.
    pub fn replace_same_edge(mut self) -> Self {
        self.replace_same_edge = true;
        self
    }

    /// set to be able to use node group.
    pub fn use_group_node(mut self) -> Self {
        self.use_group_node = true;
        self
    }

    /// set to be able to use multiple edge.
    /// If in replace same edge mode, this option is not work.
    pub fn use_multiple_edge(mut self) -> Self {
        self.multiple_edge = true;
        self
    }

    /// set to the mode to do create edge's incidence node or child node for a group as vertex node if not exist
    pub fn create_not_exist_vertex_node(mut self) -> Self {
        self.create_not_exist_vertex_node = true;
        self
    }

    // ---
    // checker
    // ---

    /// check graph can create node grouping
    #[inline]
    pub fn can_use_group_node(&self) -> bool {
        self.use_group_node
    }

    /// check the mode to do create edge's incidence node or child node for a group as vertex node if not exist
    #[inline]
    pub fn can_create_not_exist_vertex_node(&self) -> bool {
        self.create_not_exist_vertex_node
    }

    /// check graph can multiple edge
    #[inline]
    pub fn can_multiple_edge(&self) -> bool {
        self.multiple_edge
    }

    /// check can replace same edge when insert edge.
    /// If set the mode and cannot use multiple edge, replace same edge.
    #[inline]
    pub fn can_replace_same_edge(&self) -> bool {
        self.replace_same_edge
    }

    /// check configure is for Graph
    #[inline]
    pub fn is_undirected_graph(&self) -> bool {
        self.get_type().is_undirected_graph()
    }

    /// check configure is for Directed Graph
    #[inline]
    pub fn is_directed_graph(&self) -> bool {
        self.get_type().is_directed_graph()
    }

    /// check configure is for Mixed Graph
    #[inline]
    pub fn is_mixed_graph(&self) -> bool {
        self.get_type().is_mixed_graph()
    }

    /// check configure is for Hyper Graph
    #[inline]
    pub fn is_undirected_hyper_graph(&self) -> bool {
        self.get_type().is_undirected_hyper_graph()
    }

    /// check configure is for Directed Hyper Graph
    #[inline]
    pub fn is_directed_hyper_graph(&self) -> bool {
        self.get_type().is_directed_hyper_graph()
    }

    /// check configure is for Mixed Hyper Graph
    #[inline]
    pub fn is_mixed_hyper_graph(&self) -> bool {
        self.get_type().is_mixed_hyper_graph()
    }

    /// check graph can use undirected edge
    #[inline]
    pub fn can_use_undirected_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_undirected_graph() || graph_type.is_mixed_graph()
    }

    /// check graph can use directed edge
    #[inline]
    pub fn can_use_directed_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_directed_graph() || graph_type.is_mixed_graph()
    }

    /// check graph can use edge
    #[inline]
    pub fn can_use_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_undirected_graph()
            || graph_type.is_directed_graph()
            || graph_type.is_mixed_graph()
    }

    /// check graph can use undirected hyper edge
    #[inline]
    pub fn can_use_undirected_hyper_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_undirected_hyper_graph() || graph_type.is_mixed_hyper_graph()
    }

    /// check graph can use directed hyper edge
    #[inline]
    pub fn can_use_directed_hyper_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_directed_hyper_graph() || graph_type.is_mixed_hyper_graph()
    }

    /// check graph can use hyper edge
    #[inline]
    pub fn can_use_hyper_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_undirected_hyper_graph()
            || graph_type.is_directed_hyper_graph()
            || graph_type.is_mixed_hyper_graph()
    }

    // ---
    // delete
    // ---
}
