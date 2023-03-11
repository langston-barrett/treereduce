use std::collections::{HashMap, HashSet};
use tree_sitter::{Node, Tree};
use tree_sitter_edit::Editor;

use crate::id::NodeId;

#[derive(Clone, Debug)]
pub struct Edits {
    omit: HashSet<NodeId>,
    replace: HashMap<NodeId, String>,
}

impl Edits {
    pub fn new() -> Edits {
        Edits {
            omit: HashSet::new(),
            replace: HashMap::new(),
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

    pub fn replace(mut self, node: &Node, s: String) -> Self {
        self.replace.insert(NodeId::new(node), s);
        self
    }

    pub fn replace_id(mut self, node_id: NodeId, s: String) -> Self {
        self.replace.insert(node_id, s);
        self
    }

    pub fn should_omit(&self, node: &Node) -> bool {
        self.omit.contains(&NodeId::new(node))
    }

    pub fn should_omit_id(&self, node_id: &NodeId) -> bool {
        self.omit.contains(node_id)
    }

    pub fn should_replace(&self, node: &Node) -> bool {
        self.replace.contains_key(&NodeId::new(node))
    }

    pub fn should_replace_id(&self, node_id: &NodeId) -> bool {
        self.replace.contains_key(node_id)
    }
}

impl Default for Edits {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor for Edits {
    fn has_edit(&self, _tree: &Tree, node: &Node) -> bool {
        self.should_omit(node) || self.should_replace(node)
    }

    fn edit(&self, _source: &[u8], tree: &Tree, node: &Node) -> Vec<u8> {
        debug_assert!(self.has_edit(tree, node));
        if self.should_omit(node) {
            Vec::new()
        } else {
            self.replace
                .get(&NodeId::new(node))
                .unwrap()
                .clone()
                .into_bytes()
        }
    }
}
