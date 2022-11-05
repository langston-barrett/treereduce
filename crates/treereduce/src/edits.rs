use std::collections::HashSet;
use tree_sitter::{Node, Tree};
use tree_sitter_edit::Editor;

use crate::id::NodeId;

#[derive(Clone, Debug)]
pub struct Edits {
    omit: HashSet<NodeId>,
}

impl Edits {
    pub fn new() -> Edits {
        Edits {
            omit: HashSet::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.omit.is_empty()
    }

    pub fn omit(mut self, node: &Node) -> Self {
        self.omit.insert(NodeId::new(node));
        self
    }

    pub fn omit_id(mut self, node_id: NodeId) -> Self {
        self.omit.insert(node_id);
        self
    }

    pub fn omit_ids(mut self, node_ids: &[NodeId]) -> Self {
        for node_id in node_ids {
            self.omit.insert(*node_id);
        }
        self
    }

    pub fn should_omit(&self, node: &Node) -> bool {
        self.omit.contains(&NodeId::new(node))
    }
}

impl Default for Edits {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor for Edits {
    fn has_edit(&self, _tree: &Tree, node: &Node) -> bool {
        self.should_omit(node)
    }

    fn edit(&self, _source: &[u8], tree: &Tree, node: &Node) -> Vec<u8> {
        debug_assert!(self.has_edit(tree, node));
        Vec::new()
    }
}
