use std::sync::atomic;
use std::sync::atomic::AtomicUsize;
use std::sync::{Condvar, Mutex};
use std::time::Duration;

use super::error::ReductionError;

/// Keep track of idling threads
#[derive(Debug)]
pub struct Idle {
    idle_threads: AtomicUsize,
    idle_signal: Condvar,
    idle_signal_mutex: Mutex<bool>,
}

impl Idle {
    pub fn new() -> Self {
        Idle {
            idle_threads: AtomicUsize::new(0),
            idle_signal: Condvar::new(),
            idle_signal_mutex: Mutex::new(false),
        }
    }

    pub fn count(&self) -> usize {
        self.idle_threads.load(atomic::Ordering::SeqCst)
    }

    pub fn dec(&self) -> usize {
        self.idle_threads.fetch_sub(1, atomic::Ordering::SeqCst)
    }

    pub fn inc(&self) -> usize {
        let n = self.idle_threads.fetch_add(1, atomic::Ordering::SeqCst);
        self.idle_signal.notify_all();
        n
    }

    pub fn wait(&self, dur: Duration) -> Result<(), ReductionError> {
        let lock = self.idle_signal_mutex.lock()?;
        let _l = self.idle_signal.wait_timeout(lock, dur)?;
        Ok(())
    }
}
