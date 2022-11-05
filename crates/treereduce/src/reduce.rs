// TODO(#22): Awareness of binding structure
// TODO(#23): Awareness of matched delimiters

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::io;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex, PoisonError, TryLockError};
use std::thread;
use std::time::Duration;

use thiserror::Error;
use tracing_mutex::stdsync::DebugRwLock;
use tree_sitter::{Language, Node, Tree};
use tree_sitter_edit::render;

use crate::check::Check;
use crate::edits::Edits;
use crate::id::NodeId;
use crate::node_types::NodeTypes;
use crate::original::Original;
use crate::versioned::Versioned;

fn node_size(node: &Node) -> usize {
    debug_assert!(node.start_byte() <= node.end_byte());
    node.end_byte() - node.start_byte()
}

#[derive(Debug, Error)]
pub enum ReductionError {
    #[error("Error setting ctrl-c handler")]
    Ctrlc(#[from] ctrlc::Error),
    #[error("I/O error")]
    Disconnect(#[from] io::Error),
    #[error("Lock poisoned")]
    LockError(String),
}

impl<T> From<PoisonError<T>> for ReductionError {
    fn from(e: PoisonError<T>) -> ReductionError {
        ReductionError::LockError(format!("{}", e))
    }
}

// Someday, this might be able to store Nodes directly:
// https://github.com/tree-sitter/tree-sitter/issues/1241
#[derive(Debug, PartialEq, Eq)]
enum Task {
    // TODO(lb): Track parent kind and field name for more accurate optionality
    Explore(NodeId),
    Delete(NodeId),
    DeleteAll(Vec<NodeId>),
    // Hoist(NodeId, NodeId),
    // Delta(NodeId),
}

impl Task {
    fn show(&self) -> String {
        match self {
            Task::Explore(_) => "explore".to_string(),
            Task::Delete(_) => "delete".to_string(),
            Task::DeleteAll(_) => "delete_all".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PrioritizedTask {
    task: Task,
    priority: usize,
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

#[derive(Debug)]
struct Tasks {
    heap: DebugRwLock<BinaryHeap<PrioritizedTask>>,
    push_signal: Condvar,
    push_signal_mutex: Mutex<bool>,
}

impl Tasks {
    fn new() -> Self {
        Tasks {
            heap: DebugRwLock::new(BinaryHeap::new()),
            push_signal: Condvar::new(),
            push_signal_mutex: Mutex::new(false),
        }
    }

    fn push(&self, pt: PrioritizedTask) -> Result<(), ReductionError> {
        {
            let mut w = self.heap.write()?;
            log::debug!(
                "Pushing {} task with priority {} onto heap of size {}",
                pt.task.show(),
                pt.priority,
                w.len()
            );
            w.push(pt);
        }
        // log::debug!("Heap size: {}", self.heap.read()?.len());
        self.push_signal.notify_one();
        Ok(())
    }

    fn pop(&self) -> Result<Option<PrioritizedTask>, ReductionError> {
        // log::debug!("Heap size: {}", self.heap.read()?.len());
        let task = self.heap.write()?.pop();
        // log::debug!("Popped task with priority {}", task.as_ref().map(|t| t.priority).unwrap_or(0));
        Ok(task)
    }

    fn wait_for_push(&self, dur: Duration) -> Result<(), ReductionError> {
        match self.push_signal_mutex.try_lock() {
            Err(TryLockError::WouldBlock) => Ok(()),
            Err(TryLockError::Poisoned(p)) => Err(p.into()),
            Ok(lock) => {
                let _l = self.push_signal.wait_timeout(lock, dur)?;
                Ok(())
            }
        }
    }

    fn _wait_pop(&self, dur: Duration) -> Result<Option<PrioritizedTask>, ReductionError> {
        if let Some(t) = self.heap.write()?.pop() {
            Ok(Some(t))
        } else {
            self.wait_for_push(dur)?;
            Ok(self.heap.write()?.pop())
        }
    }
}

#[derive(Debug)]
struct Idle {
    idle_threads: AtomicUsize,
    idle_signal: Condvar,
    idle_signal_mutex: Mutex<bool>,
}

impl Idle {
    fn new() -> Self {
        Idle {
            idle_threads: AtomicUsize::new(0),
            idle_signal: Condvar::new(),
            idle_signal_mutex: Mutex::new(false),
        }
    }

    fn count(&self) -> usize {
        self.idle_threads.load(atomic::Ordering::SeqCst)
    }

    fn dec(&self) -> usize {
        self.idle_threads.fetch_sub(1, atomic::Ordering::SeqCst)
    }

    fn inc(&self) -> usize {
        let n = self.idle_threads.fetch_add(1, atomic::Ordering::SeqCst);
        self.idle_signal.notify_all();
        n
    }

    fn wait(&self, dur: Duration) -> Result<(), ReductionError> {
        let lock = self.idle_signal_mutex.lock()?;
        let _l = self.idle_signal.wait_timeout(lock, dur)?;
        Ok(())
    }
}

#[derive(Debug)]
struct Ctx<T>
where
    T: Check + Send + Sync + 'static,
{
    node_types: NodeTypes,
    tasks: Tasks,
    edits: DebugRwLock<Versioned<Edits>>,
    orig: Original,
    idle: Idle,
    check: T,
    min_task_size: usize,
}

struct ThreadCtx<'a, T>
where
    T: Check + Send + Sync + 'static,
{
    ctx: &'a Arc<Ctx<T>>,
    node_ids: HashMap<NodeId, Node<'a>>,
}

impl<'a, T> ThreadCtx<'a, T>
where
    T: Check + Send + Sync + 'static,
{
    fn new(ctx: &'a Arc<Ctx<T>>) -> Self {
        let mut node_ids = HashMap::new();
        let mut queue = vec![ctx.orig.tree.root_node()];
        while let Some(node) = queue.pop() {
            node_ids.insert(NodeId::new(&node), node);
            queue.reserve(node.child_count());
            for child in node.children(&mut ctx.orig.tree.walk()) {
                queue.push(child);
            }
        }
        ThreadCtx { ctx, node_ids }
    }

    fn find(&self, id: &NodeId) -> Node<'a> {
        self.node_ids[id]
    }
}

impl<T> Ctx<T>
where
    T: Check + Send + Sync + 'static,
{
    fn render(&self, edits: &Edits) -> io::Result<(bool, Vec<u8>)> {
        let mut text: Vec<u8> = Vec::new();
        text.reserve(self.orig.text.len() / 2);
        let changed = render(&mut text, &self.orig.tree, &self.orig.text, edits)?;
        Ok((changed, text))
    }

    fn _language(&self) -> Language {
        self.orig.tree.language()
    }

    fn _parse(&self, src: &[u8]) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        // TODO(lb): Incremental re-parsing
        parser
            .set_language(self._language())
            .expect("Error loading language");
        parser.parse(src, None).expect("Failed to parse")
    }

    /// Pop the highest-priority task from the task heap.
    fn pop_task(&self) -> Result<Option<PrioritizedTask>, ReductionError>
    where
        T: Sync,
    {
        // TODO(lb): What's the problem?
        // let point_o_one_seconds = Duration::new(0, 10000000);
        // Ok(self.tasks.wait_pop(point_o_one_seconds)?.map(|pt| pt.task))
        let task = self.tasks.pop()?;
        debug_assert!(
            task.as_ref().map(|t| t.priority).unwrap_or(std::usize::MAX) >= self.min_task_size
        );
        Ok(task)
    }

    fn push_task(&self, node: &Node, task: Task) -> Result<(), ReductionError> {
        self.push_prioritized_task(node_size(node), task)
    }

    fn push_prioritized_task(&self, priority: usize, task: Task) -> Result<(), ReductionError> {
        if priority < self.min_task_size {
            return Ok(());
        }
        self.tasks.push(PrioritizedTask {
            task,
            // TODO(lb): Benchmark leaving this at 0
            priority,
        })
    }

    fn push_explore_children(&self, node: Node) -> Result<(), ReductionError>
    where
        T: Check,
    {
        // TODO(lb): Benchmark
        let mut w = self.tasks.heap.write()?;
        for child in node.children(&mut self.orig.tree.walk()) {
            let priority = node_size(&child);
            if priority < self.min_task_size {
                continue;
            }
            w.push(PrioritizedTask {
                task: Task::Explore(NodeId::new(&child)),
                priority,
            });
        }
        for _ in 0..node.child_count() {
            self.tasks.push_signal.notify_one();
        }
        Ok(())
    }

    fn add_task_edit(&self, task: &Task) -> Result<Versioned<Edits>, ReductionError> {
        match task {
            Task::Explore(_) => {
                debug_assert!(false);
                Ok(self.edits.read()?.clone())
            }
            Task::Delete(node_id) => Ok(self.edits.read()?.mutate_clone(|e| e.omit_id(*node_id))),
            Task::DeleteAll(node_ids) => {
                Ok(self.edits.read()?.mutate_clone(|e| e.omit_ids(node_ids)))
            }
        }
    }

    /// Check if the given edits yield an interesting tree. If so, and if the
    /// edits haven't been concurrently modified by another call to this
    /// function, replace the edits with the new ones.
    fn interesting(&self, task: &Task) -> Result<bool, ReductionError>
    where
        T: Check,
    {
        '_outer: loop {
            let edits = self.add_task_edit(task)?;
            // TODO(lb): Benchmark this:
            // if !self.edits.read()?.old_version(&edits) {
            //     return Ok(InterestingCheck::TryAgain);
            // }
            let (_changed, rendered) = self.render(edits.get())?;
            // TODO(lb): Don't introduce parse errors
            // let reparsed = self.parse(&rendered);
            // assert!({
            //     if reparsed.root_node().has_error() {
            //         self.orig.tree.root_node().has_error()
            //     } else {
            //         true
            //     }
            // });

            // Wait for the process to finish, exit early (try this reduction again)
            // if another thread beat us to it.

            let state = self.check.start(&rendered)?;

            // TODO(lb): Why is this slow?
            // while self.check.try_wait(&mut state)?.is_none() {
            //     // TODO(lb): Wait for 1/10 as long as the interestingness test takes
            //     // TODO(lb): Benchmark wait times
            //     // let point_o_o_one_seconds = Duration::new(0, 100000000);
            //     let point_o_one_seconds = Duration::new(0, 10000000);
            //     // let not_long = Duration::new(0, 1000);
            //     self.tasks.wait_for_push(point_o_one_seconds)?;
            //     match self.edits.try_read() {
            //         Err(_) => continue,
            //         Ok(l) => {
            //             if !l.old_version(&edits) {
            //                 self.check.cancel(state)?;
            //                 log::debug!("Canceled interestingness check");
            //                 continue 'outer;
            //             }
            //         }
            //     }
            // }

            if self.check.wait(state)? {
                match self.edits.try_write() {
                    Err(_) => continue,
                    Ok(mut w) => {
                        if !w.old_version(&edits) {
                            log::debug!("Cancel!");
                            continue;
                        }
                        *w = edits;
                        log::info!("Reduced to size: {}", rendered.len());
                        log::debug!(
                            "New minimal program:\n{}",
                            std::str::from_utf8(&rendered).unwrap_or("<not UTF-8>")
                        );
                        return Ok(true);
                    }
                }
            } else {
                log::debug!("Uninteresting.");
                return Ok(false);
            }
        }
    }
}

// TODO(#15): Refine with access to node-types.json
fn _is_list(_node: &Node) -> bool {
    false
}

fn explore<T: Check + Send + Sync + 'static>(
    tctx: &ThreadCtx<T>,
    node_id: NodeId,
) -> Result<(), ReductionError> {
    // TODO(lb): Include kind in explore task to avoid find
    let node = tctx.find(&node_id);
    log::debug!("Exploring {}...", tctx.find(&node_id).kind());
    if tctx.ctx.node_types.optional_node(&node) {
        tctx.ctx.push_task(&node, Task::Delete(node_id))?;
    } else {
        // If this node has some children/fields that can have multiple nodes,
        // try deleting all of them at once (by kind).
        let child_list_types = tctx.ctx.node_types.list_types(&node);
        if !child_list_types.is_empty() {
            // TODO(lb): Benchmark locking tasks and pushing all at once
            for node_kind in child_list_types {
                let mut batch = Vec::new();
                let mut batch_size = 0;
                for subkind in tctx.ctx.node_types.subtypes(&node_kind) {
                    for child in node.children(&mut tctx.ctx.orig.tree.walk()) {
                        if child.kind() == subkind {
                            batch.push(NodeId::new(&child));
                            batch_size += child.end_byte() - child.start_byte();
                        }
                    }
                }
                tctx.ctx
                    .push_prioritized_task(batch_size, Task::DeleteAll(batch))?;
            }
        }
        tctx.ctx.push_explore_children(node)?;
    }
    Ok(())
}

fn delete<T: Check + Send + Sync + 'static>(
    tctx: &ThreadCtx<T>,
    node_id: NodeId,
) -> Result<(), ReductionError> {
    // log::debug!("Deleting {}...", tctx.find(node_id).kind());
    if tctx.ctx.interesting(&Task::Delete(node_id))? {
        // This tree was deleted, no need to recurse on children
        // eprintln!("Interesting deletion of {}", node.kind());
        Ok(())
    } else {
        tctx.ctx.push_explore_children(tctx.find(&node_id))
    }
}

fn delete_all<T: Check + Send + Sync + 'static>(
    tctx: &ThreadCtx<T>,
    node_ids: Vec<NodeId>,
) -> Result<(), ReductionError> {
    // No need to check whether it was interesting, because the children will be
    // individually handled by `delete`.
    tctx.ctx.interesting(&Task::DeleteAll(node_ids))?;
    Ok(())
}

fn dispatch<T: Check + Send + Sync + 'static>(
    tctx: &ThreadCtx<T>,
    task: Task,
) -> Result<(), ReductionError> {
    match task {
        Task::Explore(node_id) => explore(tctx, node_id),
        Task::Delete(node_id) => delete(tctx, node_id),
        Task::DeleteAll(node_ids) => delete_all(tctx, node_ids),
    }
}

/// Main function for each thread
fn work<T: Check + Send + Sync + 'static>(
    ctx: Arc<Ctx<T>>,
    num_threads: usize,
) -> Result<(), ReductionError> {
    let tctx = ThreadCtx::new(&ctx);
    let mut idle = false;
    // Quit if all threads are idle and there are no remaining tasks
    while ctx.idle.count() < num_threads {
        if idle {
            // TODO(lb): Integrate waiting into pop?
            // TODO(lb): Benchmark the duration
            // let point_o_one_seconds = Duration::new(0, 10000000);
            let not_long = Duration::new(0, 100000);
            tctx.ctx.tasks.wait_for_push(not_long)?;
            tctx.ctx.idle.dec();
        }
        while let Some(task) = tctx.ctx.pop_task()? {
            log::debug!(
                "Popped {} task with priority {}",
                task.task.show(),
                task.priority
            );
            // TODO(lb): Benchmark, but seems like a win
            if task.priority < ctx.min_task_size {
                continue;
            }
            dispatch(&tctx, task.task)?;
        }
        let num_idle = tctx.ctx.idle.inc();
        log::debug!("Idling {} / {}...", num_idle + 1, num_threads);
        idle = true;
    }
    Ok(())
}

fn ctrlc_handler() -> Result<mpsc::Receiver<()>, ReductionError> {
    let (send_ctrl_c, recv_ctrl_c) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        eprintln!("HERE!");
        send_ctrl_c
            .send(())
            .expect("Could not send signal on channel.")
    })?;
    Ok(recv_ctrl_c)
}

pub fn treereduce<T: Check + Debug + Send + Sync + 'static>(
    mut jobs: usize,
    node_types: NodeTypes,
    orig: Original,
    check: T,
    // TODO(lb): Maybe per-pass, benchmark
    mut min_reduction: usize,
) -> Result<Edits, ReductionError> {
    log::info!("Original size: {}", orig.text.len());
    // TODO(#25): SIGHUP handler to save intermediate progress
    jobs = std::cmp::max(1, jobs);
    min_reduction = std::cmp::max(1, min_reduction);
    let tasks = Tasks::new();
    let root = orig.tree.root_node();
    let root_id = NodeId::new(&root);
    tasks.push(PrioritizedTask {
        task: Task::Explore(root_id),
        priority: node_size(&root),
    })?;
    let ctx = Arc::new(Ctx {
        node_types,
        tasks,
        edits: DebugRwLock::new(Versioned::new(Edits::new())),
        orig,
        idle: Idle::new(),
        check,
        min_task_size: min_reduction,
    });

    let mut thread_handles = Vec::new();
    thread_handles.reserve(jobs);
    for _ in 0..jobs {
        let actx = Arc::clone(&ctx);
        thread_handles.push(thread::spawn(move || work(actx, jobs)));
    }

    // TODO(lb): This doesn't really work...
    //
    // https://github.com/Detegr/rust-ctrlc/issues/30
    //
    // Maybe try signal_hook crate?
    let recv_ctrl_c = ctrlc_handler()?;

    while let Some(t) = thread_handles.pop() {
        if recv_ctrl_c.try_recv().is_ok() {
            eprintln!("CTRL-C!");
            log::info!("Got ctrl-c, writing reduced test case...");
            ctrlc::set_handler(move || {
                log::info!("Got second ctrl-c, exiting without saving!");
                std::process::exit(0);
            })?;
            return Ok(ctx.edits.read()?.get().clone());
        }
        if t.is_finished() {
            t.join().expect("Thread panic'd!")?; // TODO(lb): don't expect
        } else {
            thread_handles.push(t);
            // TODO(lb): Benchmark the duration
            // let point_o_one_seconds = Duration::new(0, 10000000);
            let not_long = Duration::new(0, 1000);
            // TODO(lb): This is the wrong condition to wait on - wait for
            // threads to actually quit!
            ctx.idle.wait(not_long)?;
        }
    }
    // Arc::try_unwrap is not needed, but is nice just to assert that this is
    // the only reference.
    let ctx = Arc::try_unwrap(ctx).expect("Multiple references!");
    debug_assert!(ctx.tasks.heap.read()?.is_empty());
    let edits = ctx.edits.read()?.clone();
    Ok(edits.extract())
}
