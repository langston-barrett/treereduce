use tree_sitter::Node;

/// Newtype
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId {
    pub id: usize,
}

impl NodeId {
    pub fn new(node: &Node) -> Self {
        NodeId { id: node.id() }
    }

    pub fn get(&self) -> usize {
        self.id
    }
}
