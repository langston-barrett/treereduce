use tree_sitter::Node;

/// Newtype
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId {
    pub id: usize,
}

impl NodeId {
    pub fn new(node: &Node) -> Self {
        NodeId { id: node.id() }
    }
}
