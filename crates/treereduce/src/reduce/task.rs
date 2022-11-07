use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::id::NodeId;

/// Newtype
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId {
    pub id: usize,
}

impl TaskId {
    pub fn _get(&self) -> usize {
        self.id
    }
}

// Someday, this might be able to store Nodes directly:
// https://github.com/tree-sitter/tree-sitter/issues/1241
//
// TODO(lb): Split into reduction task
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Task {
    // TODO(lb): Track parent kind and field name for more accurate optionality
    Explore(NodeId),
    Delete(NodeId),
    DeleteAll(Vec<NodeId>),
    // Hoist(NodeId, NodeId),
    // Delta(NodeId),
}

// TODO(lb): Show with priority, task ID
impl Task {
    pub fn show(&self) -> String {
        match self {
            Task::Explore(_) => "explore".to_string(),
            Task::Delete(_) => "delete".to_string(),
            Task::DeleteAll(_) => "delete_all".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrioritizedTask {
    #[serde(flatten)]
    pub task: Task,
    pub id: TaskId,
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
