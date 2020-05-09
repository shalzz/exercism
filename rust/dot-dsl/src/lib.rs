use std::collections::HashMap;

macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(key, value)| (key.to_string(),  value.to_string()))
                .collect();
            self
        }

        pub fn get_attr(&self, name: &str) -> Option<&str> {
            self.attrs.get(name).map(|s| s.as_str())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|&node| node.name == name)
    }

    impl_attrs!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_owned(),
            ..Self::default()
        }
    }

    impl_attrs!();
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Edge {
    from: String,
    to: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: from.to_owned(),
            to: to.to_owned(),
            ..Self::default()
        }
    }

    impl_attrs!();
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod node {
            pub use super::super::super::Node;
        }
        pub mod edge {
            pub use super::super::super::Edge;
        }
    }
}
