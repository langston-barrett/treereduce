use std::collections::HashSet;
use tree_sitter::Node;

use crate::id::NodeId;

pub struct Alter {
    omit: HashSet<NodeId>,
}

impl Alter {
    pub fn new() -> Alter {
        Alter {
            omit: HashSet::new(),
        }
    }

    pub fn omit(&mut self, node: &Node) -> bool {
        self.omit.insert(NodeId::new(node))
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
