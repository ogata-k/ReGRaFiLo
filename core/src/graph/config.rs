//! Module for graph configuration

/// configuration for graph without layout
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GraphConfig {
    /// Config for Graph with undirected edge.
    /// multiple_edge option is a flag which can be used multiple edge.
    /// group option is a flag which can be used node grouping.
    UndirectedGraph { multiple_edge: bool, group: bool },

    /// Config for Graph with Directed edge.
    /// multiple_edge option is a flag which can be used multiple edge.
    /// group option is a flag which can be used node grouping.
    DirectedGraph { multiple_edge: bool, group: bool },

    /// Config for Graph with undirected edge or Directed edge.
    /// multiple_edge option is a flag which can be used multiple edge.
    /// group option is a flag which can be used node grouping.
    MixedGraph { multiple_edge: bool, group: bool },

    /// Config for Graph with undirected Hyper edge.
    /// multiple_hyper_edge option is a flag which can be used multiple edge. But usually false.
    /// If show graph with the option specified true, the order of a hierarchy of the group made by the edges is not specified.
    HyperGraph { multiple_hyper_edge: bool },

    /// Config for Graph with Directed Hyper edge.
    /// multiple_hyper_edge option is a flag which can be used multiple edge. But usually false.
    DirectedHyperGraph { multiple_hyper_edge: bool },

    /// Config for Graph with undirected Hyper edge or Directed Hyper edge.
    /// multiple_hyper_edge option is a flag which can be used multiple edge. But usually false.
    MixedHyperGraph { multiple_hyper_edge: bool },
}

impl GraphConfig {
    // ---
    // constructor
    // ---

    /// construtor for Graph
    pub fn undirected_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::UndirectedGraph {
            multiple_edge: can_multiple_edge,
            group: use_grouping,
        }
    }

    /// construtor for Directed Graph
    pub fn directed_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::DirectedGraph {
            multiple_edge: can_multiple_edge,
            group: use_grouping,
        }
    }

    /// construtor for Mixed Graph
    pub fn mixed_graph(can_multiple_edge: bool, use_grouping: bool) -> Self {
        Self::MixedGraph {
            multiple_edge: can_multiple_edge,
            group: use_grouping,
        }
    }

    /// construtor for Hyper Graph
    pub fn hyper_graph(can_multiple_edge: bool) -> Self {
        Self::HyperGraph {
            multiple_hyper_edge: can_multiple_edge,
        }
    }

    /// construtor for Directed Hyper Graph
    pub fn directed_hyper_graph(can_multiple_hyper_edge: bool) -> Self {
        Self::DirectedHyperGraph {
            multiple_hyper_edge: can_multiple_hyper_edge,
        }
    }

    /// construtor for Mixed Hyper Graph
    pub fn mixed_hyper_graph(can_multiple_hyper_edge: bool) -> Self {
        Self::MixedHyperGraph {
            multiple_hyper_edge: can_multiple_hyper_edge,
        }
    }

    // ---
    // getter
    // ---

    // ---
    // setter
    // ---

    // ---
    // checker
    // ---

    /// check configure is for Graph
    pub fn is_undirected_graph(&self) -> bool {
        match self {
            GraphConfig::UndirectedGraph {
                multiple_edge: _,
                group: _,
            } => true,
            _ => false,
        }
    }

    /// check configure is for Directed Graph
    pub fn is_directed_graph(&self) -> bool {
        match self {
            GraphConfig::DirectedGraph {
                multiple_edge: _,
                group: _,
            } => true,
            _ => false,
        }
    }

    /// check configure is for Mixed Graph
    pub fn is_mixed_graph(&self) -> bool {
        match self {
            GraphConfig::MixedGraph {
                multiple_edge: _,
                group: _,
            } => true,
            _ => false,
        }
    }

    /// check configure is for Hyper Graph
    pub fn is_hyper_graph(&self) -> bool {
        match self {
            GraphConfig::HyperGraph {
                multiple_hyper_edge: _,
            } => true,
            _ => false,
        }
    }

    /// check configure is for Directed Hyper Graph
    pub fn is_directed_hyper_graph(&self) -> bool {
        match self {
            GraphConfig::DirectedHyperGraph {
                multiple_hyper_edge: _,
            } => true,
            _ => false,
        }
    }

    /// check configure is for Mixed Hyper Graph
    pub fn is_mixed_hyper_graph(&self) -> bool {
        match self {
            GraphConfig::MixedHyperGraph {
                multiple_hyper_edge: _,
            } => true,
            _ => false,
        }
    }

    /// check graph can node grouping
    pub fn has_group(&self) -> bool {
        match self {
            GraphConfig::UndirectedGraph {
                multiple_edge: _,
                group: true,
            }
            | GraphConfig::DirectedGraph {
                multiple_edge: _,
                group: true,
            }
            | GraphConfig::MixedGraph {
                multiple_edge: _,
                group: true,
            }
            | GraphConfig::HyperGraph {
                multiple_hyper_edge: _,
            }
            | GraphConfig::MixedHyperGraph {
                multiple_hyper_edge: _,
            } => true,
            _ => false,
        }
    }

    /// check graph can multiple edge
    pub fn can_multiple_edge(&self) -> bool {
        match self {
            GraphConfig::UndirectedGraph {
                multiple_edge: true,
                group: _,
            }
            | GraphConfig::DirectedGraph {
                multiple_edge: true,
                group: _,
            }
            | GraphConfig::MixedGraph {
                multiple_edge: true,
                group: _,
            } => true,
            _ => false,
        }
    }

    /// check graph can multiple edge for hyper edge
    pub fn can_multiple_hyper_edge(&self) -> bool {
        match self {
            GraphConfig::HyperGraph {
                multiple_hyper_edge: true,
            }
            | GraphConfig::DirectedHyperGraph {
                multiple_hyper_edge: true,
            }
            | GraphConfig::MixedHyperGraph {
                multiple_hyper_edge: true,
            } => true,
            _ => false,
        }
    }

    // ---
    // delete
    // ---
}
