use std::collections::HashMap;
use std::sync::PoisonError;

use tracing_mutex::stdsync::{DebugMutex, TracingMutexGuard};

use super::task::Task;

// TODO(#29): Canceled/stale

#[derive(Debug)]
pub struct Stats {
    pub tries: HashMap<String, usize>,
    pub retries: HashMap<String, usize>,
    pub successes: HashMap<String, usize>,
}

#[derive(Debug)]
pub struct StatCollector {
    collect: bool,
    tries: DebugMutex<HashMap<String, usize>>,
    retries: DebugMutex<HashMap<String, usize>>,
    successes: DebugMutex<HashMap<String, usize>>,
}

impl StatCollector {
    pub fn new(collect: bool) -> Self {
        StatCollector {
            collect,
            tries: DebugMutex::new(HashMap::new()),
            retries: DebugMutex::new(HashMap::new()),
            successes: DebugMutex::new(HashMap::new()),
        }
    }

    pub fn try_(
        &self,
        task: &Task,
    ) -> Result<(), PoisonError<TracingMutexGuard<'_, HashMap<String, usize>>>> {
        if !self.collect {
            return Ok(());
        }
        *self.tries.lock()?.entry(task.show()).or_insert(0) += 1;
        Ok(())
    }

    pub fn retry(
        &self,
        task: &Task,
    ) -> Result<(), PoisonError<TracingMutexGuard<'_, HashMap<String, usize>>>> {
        if !self.collect {
            return Ok(());
        }
        *self.retries.lock()?.entry(task.show()).or_insert(0) += 1;
        Ok(())
    }

    pub fn success(
        &self,
        task: &Task,
    ) -> Result<(), PoisonError<TracingMutexGuard<'_, HashMap<String, usize>>>> {
        if !self.collect {
            return Ok(());
        }
        *self.successes.lock()?.entry(task.show()).or_insert(0) += 1;
        Ok(())
    }

    pub fn done(self) -> Result<Stats, PoisonError<HashMap<String, usize>>> {
        Ok(Stats {
            tries: self.tries.into_inner()?,
            retries: self.retries.into_inner()?,
            successes: self.successes.into_inner()?,
        })
    }
}
