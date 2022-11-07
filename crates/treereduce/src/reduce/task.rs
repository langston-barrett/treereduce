use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::id::NodeId;

/// Newtype
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskId {
    pub id: usize,
}

impl TaskId {
    pub fn get(&self) -> usize {
        self.id
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reduction {
    Delete(NodeId),
    DeleteAll(Vec<NodeId>),
    // Hoist(NodeId, NodeId),
    // Delta(NodeId),
}

// Someday, this might be able to store Nodes directly:
// https://github.com/tree-sitter/tree-sitter/issues/1241
//
// TODO(lb): Split into reduction task
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Task {
    // TODO(lb): Track parent kind and field name for more accurate optionality
    Explore(NodeId),
    Reduce(Reduction),
}

// TODO(lb): Show with priority, task ID
impl Task {
    pub fn kind(&self) -> String {
        match self {
            Task::Explore(_) => "explore".to_string(),
            Task::Reduce(Reduction::Delete(_)) => "delete".to_string(),
            Task::Reduce(Reduction::DeleteAll(_)) => "delete_all".to_string(),
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

impl Display for PrioritizedTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "task {} of kind {} and priority {}",
            self.id.get(),
            self.task.kind(),
            self.priority
        )
    }
}
