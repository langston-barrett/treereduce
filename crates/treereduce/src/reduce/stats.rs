use serde::{Deserialize, Serialize};

use super::task::PrioritizedTask;

// TODO(#29): Canceled/stale
// TODO(lb): Output times of each message

#[derive(Debug)]
pub struct StatCollector {
    collect: bool,
}

#[derive(Serialize, Deserialize)]
enum Event {
    Push {
        #[serde(rename = "task")]
        ptask: PrioritizedTask,
    },
    Pop {
        #[serde(rename = "task")]
        ptask: PrioritizedTask,
    },
    //
    Try {
        #[serde(rename = "task")]
        ptask: PrioritizedTask,
    },
    Retry {
        #[serde(rename = "task")]
        ptask: PrioritizedTask,
    },
    Interesting {
        #[serde(rename = "task")]
        ptask: PrioritizedTask,
    },
    Uninteresting {
        #[serde(rename = "task")]
        ptask: PrioritizedTask,
    },
}

impl StatCollector {
    pub fn new(collect: bool) -> Self {
        StatCollector { collect }
    }

    pub fn push(&self, ptask: &PrioritizedTask) -> Result<(), serde_json::Error> {
        if !self.collect {
            return Ok(());
        }
        eprintln!(
            "{}",
            serde_json::to_string(&Event::Push {
                ptask: ptask.clone()
            })?
        );
        Ok(())
    }

    pub fn pop(&self, ptask: &PrioritizedTask) -> Result<(), serde_json::Error> {
        if !self.collect {
            return Ok(());
        }
        eprintln!(
            "{}",
            serde_json::to_string(&Event::Pop {
                ptask: ptask.clone()
            })?
        );
        Ok(())
    }

    pub fn try_(&self, ptask: &PrioritizedTask) -> Result<(), serde_json::Error> {
        if !self.collect {
            return Ok(());
        }
        eprintln!(
            "{}",
            serde_json::to_string(&Event::Try {
                ptask: ptask.clone()
            })?
        );
        Ok(())
    }

    pub fn retry(&self, ptask: &PrioritizedTask) -> Result<(), serde_json::Error> {
        if !self.collect {
            return Ok(());
        }
        eprintln!(
            "{}",
            serde_json::to_string(&Event::Retry {
                ptask: ptask.clone()
            })?
        );
        Ok(())
    }

    pub fn interesting(&self, ptask: &PrioritizedTask) -> Result<(), serde_json::Error> {
        if !self.collect {
            return Ok(());
        }
        eprintln!(
            "{}",
            serde_json::to_string(&Event::Interesting {
                ptask: ptask.clone()
            })?
        );
        Ok(())
    }

    pub fn uninteresting(&self, ptask: &PrioritizedTask) -> Result<(), serde_json::Error> {
        if !self.collect {
            return Ok(());
        }
        eprintln!(
            "{}",
            serde_json::to_string(&Event::Uninteresting {
                ptask: ptask.clone()
            })?
        );
        Ok(())
    }
}
