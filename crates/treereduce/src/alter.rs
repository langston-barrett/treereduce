use std::collections::HashSet;
use tree_sitter::Node;

use crate::id::NodeId;

#[derive(Debug)]
pub struct Alter {
    omit: HashSet<NodeId>,
}

impl Alter {
    pub fn new() -> Alter {
        Alter {
            omit: HashSet::new(),
        }
    }

    pub fn omit(mut self, node: &Node) -> Self {
        self.omit.insert(NodeId::new(node));
        self
    }

    pub fn omit_id(mut self, node_id: NodeId) -> Self {
        self.omit.insert(node_id);
        self
    }

    pub fn should_omit(&self, node: &Node) -> bool {
        self.omit.contains(&NodeId::new(node))
    }
}

impl Default for Alter {
    fn default() -> Self {
        Self::new()
    }
}
