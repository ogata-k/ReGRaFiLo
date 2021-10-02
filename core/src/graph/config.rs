//! Module for graph configuration

use std::fmt;

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
    /// check type is kind of Graph
    pub fn is_kind_of_graph(&self) -> bool {
        match self {
            GraphType::UndirectedGraph | GraphType::DirectedGraph | GraphType::MixedGraph => true,
            _ => false,
        }
    }

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

    /// check type is kind of HyperGraph
    pub fn is_kind_of_hyper_graph(&self) -> bool {
        match self {
            GraphType::UndirectedHyperGraph
            | GraphType::DirectedHyperGraph
            | GraphType::MixedHyperGraph => true,
            _ => false,
        }
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
    /// type of graph
    graph_type: GraphType,

    /// this option is a flag which we can group node
    use_group_node: bool,

    /// the optional option is a flag to do or not to do create edge's incidence node or child node for a group as vertex node if not exist
    create_not_exist_vertex_node: bool,

    /// this option is a flag which we can use check to make multiple edge
    multiple_edge: bool,

    /// this optional option is a flag remove same edge when insert.
    /// If set the mode and cannot use multiple edge, replace same edge.
    replace_same_edge: bool,
}

impl fmt::Display for GraphConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GraphType::*;

        match self.graph_type {
            UndirectedGraph => {
                f.write_str("{graph_kind: Graph, edge_spec: Undirected")?;
            },
            DirectedGraph => {
                f.write_str("{graph_kind: Graph, edge_spec: Directed")?;
            },
            MixedGraph => {
                f.write_str("{graph_kind: Graph, edge_spec: Mixed")?;
            },
            UndirectedHyperGraph => {
                f.write_str("{graph_kind: HyperGraph, edge_spec: Undirected")?;
            },
            DirectedHyperGraph => {
                f.write_str("{graph_kind: HyperGraph, edge_spec: Directed")?;
            },
            MixedHyperGraph => {
                f.write_str("{graph_kind: HyperGraph, edge_spec: Mixed")?;
            },
        }
        f.write_fmt(format_args!(
            ", node_group: {}, can_multiple: {}}}",
            self.use_group_node, self.multiple_edge
        ))
    }
}

impl GraphConfig {
    // ---
    // constructor
    // ---

    /// construct minimum config for Graph
    pub fn undirected_graph() -> Self {
        Self {
            graph_type: GraphType::UndirectedGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
        }
    }

    /// construct minimum config for Directed Graph
    pub fn directed_graph() -> Self {
        Self {
            graph_type: GraphType::DirectedGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
        }
    }

    /// construct minimum config for Mixed Graph
    pub fn mixed_graph() -> Self {
        Self {
            graph_type: GraphType::MixedGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
        }
    }

    /// construct minimum config for Hyper Graph
    pub fn undirected_hyper_graph() -> Self {
        Self {
            graph_type: GraphType::UndirectedHyperGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
        }
    }

    /// construct minimum config for Directed Hyper Graph
    pub fn directed_hyper_graph() -> Self {
        Self {
            graph_type: GraphType::DirectedHyperGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
        }
    }

    /// construct minimum config for Mixed Hyper Graph
    pub fn mixed_hyper_graph() -> Self {
        Self {
            graph_type: GraphType::MixedHyperGraph,
            use_group_node: false,
            create_not_exist_vertex_node: false,
            multiple_edge: false,
            replace_same_edge: false,
        }
    }

    // ---
    // getter
    // ---

    /// get graph type
    pub fn get_type(&self) -> GraphType {
        self.graph_type
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
    pub fn can_use_group_node(&self) -> bool {
        self.use_group_node
    }

    /// check the mode to do create edge's incidence node or child node for a group as vertex node if not exist
    pub fn can_create_not_exist_vertex_node(&self) -> bool {
        self.create_not_exist_vertex_node
    }

    /// check graph can multiple edge
    pub fn can_multiple_edge(&self) -> bool {
        self.multiple_edge
    }

    /// check can replace same edge when insert edge.
    /// If set the mode and cannot use multiple edge, replace same edge.
    pub fn can_replace_same_edge(&self) -> bool {
        self.replace_same_edge
    }

    /// check type is kind of Graph
    pub fn is_kind_of_graph(&self) -> bool {
        self.get_type().is_kind_of_graph()
    }

    /// check configure is for Graph
    pub fn is_undirected_graph(&self) -> bool {
        self.get_type().is_undirected_graph()
    }

    /// check configure is for Directed Graph
    pub fn is_directed_graph(&self) -> bool {
        self.get_type().is_directed_graph()
    }

    /// check configure is for Mixed Graph
    pub fn is_mixed_graph(&self) -> bool {
        self.get_type().is_mixed_graph()
    }

    /// check type is kind of HyperGraph
    pub fn is_kind_of_hyper_graph(&self) -> bool {
        self.get_type().is_kind_of_hyper_graph()
    }

    /// check configure is for Hyper Graph
    pub fn is_undirected_hyper_graph(&self) -> bool {
        self.get_type().is_undirected_hyper_graph()
    }

    /// check configure is for Directed Hyper Graph
    pub fn is_directed_hyper_graph(&self) -> bool {
        self.get_type().is_directed_hyper_graph()
    }

    /// check configure is for Mixed Hyper Graph
    pub fn is_mixed_hyper_graph(&self) -> bool {
        self.get_type().is_mixed_hyper_graph()
    }

    /// check graph can use undirected edge
    pub fn can_use_undirected_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_undirected_graph() || graph_type.is_mixed_graph()
    }

    /// check graph can use directed edge
    pub fn can_use_directed_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_directed_graph() || graph_type.is_mixed_graph()
    }

    /// check graph can use edge
    pub fn can_use_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_undirected_graph()
            || graph_type.is_directed_graph()
            || graph_type.is_mixed_graph()
    }

    /// check graph can use undirected hyper edge
    pub fn can_use_undirected_hyper_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_undirected_hyper_graph() || graph_type.is_mixed_hyper_graph()
    }

    /// check graph can use directed hyper edge
    pub fn can_use_directed_hyper_edge(&self) -> bool {
        let graph_type = self.get_type();

        graph_type.is_directed_hyper_graph() || graph_type.is_mixed_hyper_graph()
    }

    /// check graph can use hyper edge
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
