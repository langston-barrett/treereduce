use tree_sitter::Node;

use serde::{Deserialize, Serialize};

/// Newtype
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId {
    pub id: usize,
}

impl NodeId {
    pub fn new(node: &Node<'_>) -> Self {
        NodeId { id: node.id() }
    }

    pub fn get(self) -> usize {
        self.id
    }
}
