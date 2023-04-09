use std::sync::atomic;
use std::sync::atomic::AtomicUsize;

/// Keep track of idling threads
#[derive(Debug)]
pub struct Idle {
    idle_threads: AtomicUsize,
}

impl Idle {
    pub fn new() -> Self {
        Idle {
            idle_threads: AtomicUsize::new(0),
        }
    }

    pub fn count(&self) -> usize {
        self.idle_threads.load(atomic::Ordering::SeqCst)
    }

    pub fn dec(&self) -> usize {
        self.idle_threads.fetch_sub(1, atomic::Ordering::SeqCst)
    }

    pub fn inc(&self) -> usize {
        self.idle_threads.fetch_add(1, atomic::Ordering::SeqCst)
    }
}
