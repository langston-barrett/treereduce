use std::cmp::Ordering;

use crate::id::NodeId;

// Someday, this might be able to store Nodes directly:
// https://github.com/tree-sitter/tree-sitter/issues/1241
#[derive(Debug, PartialEq, Eq)]
pub enum Task {
    // TODO(lb): Track parent kind and field name for more accurate optionality
    Explore(NodeId),
    Delete(NodeId),
    DeleteAll(Vec<NodeId>),
    // Hoist(NodeId, NodeId),
    // Delta(NodeId),
}

impl Task {
    pub fn show(&self) -> String {
        match self {
            Task::Explore(_) => "explore".to_string(),
            Task::Delete(_) => "delete".to_string(),
            Task::DeleteAll(_) => "delete_all".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrioritizedTask {
    pub task: Task,
    pub priority: usize,
}

impl Ord for PrioritizedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for PrioritizedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
