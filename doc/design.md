# Design

(algorithm-goals)=
## Goals

The core reduction algorithm has three goals:

1. Go fast
2. Produce small test cases
3. Produce readable test cases

Unlike more theoretical work in this space, the algorithm does *not* attempt to
minimize the number of "oracle queries", that is, invocations of the
user-provided "interestingness test".

## Design Statement

The aims of `treereduce` are more pragmatic than academic. It is based on
similar principles to those discussed in the [AFL whitepaper][afl]:

> [treereduce] does its best not to focus on any singular principle of operation
> and not be a proof-of-concept for any specific theory. [...] The only true
> governing principles are speed, reliability, and ease of use.

(algorithm-assumptions)=
## Assumptions

These assumptions strongly inform the algorithm design:

1. The interestingness will be comparatively slow---it will generally involve
   spinning up a new process, disk I/O, parsing, type-checking, etc.
2. Smaller inputs lead to faster interestingness tests.

These assumptions hold for the use-case of reducing programs that cause compiler
crashes.

## High-Level Design

Due to [Assumption (1)](algorithm-assumptions), it's essential that `treereduce`
execute several interestingness tests in parallel. Luckily, for the same reason,
lock contention is unlikely to be a major issue---so long as executing the
interestingness test doesn't require holding a lock, most threads will spend the
majority of their time executing the interestingness test, rather than waiting
for locks on shared data. (This claim has been validated by {ref}`profiling
<profiling>` several {doc}`benchmarks <benchmarks>`.)

The recent paper "[PARDIS][pardis] : Priority Aware Test Case Reduction"
highlights the importance of *prioritization* of reductions. Greedy removal of
the *largest* subtrees leads to greatly increased performance due to faster
interestingness tests ([Assumption (2)](algorithm-assumptions)).

With these ideas in mind, the overall algorithm design involves spinning up some
number of threads, which share three pieces of data:

1. The original text and syntax tree of the target (read-only)
2. A prioritized max-heap of [*reduction tasks*](reduction-strategies)
   (read-write)
3. A set of *edits* to the syntax tree (read-write)

where an edit is either the deletion of a subtree, or a replacement of one
subtree by another. Each thread executes the following loop:

1. Pop a reduction task off the heap.
2. Create a local copy of the edits. Add edits according to the current task.
3. Execute the interestingness test on the reduced program, i.e., the target
   as altered by the local set of edits.
4. If the reduced program was still interesting, try replacing the global edits.
   If the global edits were changed by another thread, try this task again with
   the new edits, that is, go to (2).
5. Push any new tasks onto the task queue.
6. Go to (1).

If lock contention does become an issue, it may be beneficial for each thread to
maintain a local task heap in addition to the global one, or even to attempt
multiple tasks before replacing the global edits.

(reduction-strategies)=
## Reduction Strategies

`treereduce` uses several strategies during program minimization:

- *Deletion*: When a child is optional, `treereduce` attempts to delete it. For
  example, `treereduce` might delete the `const` in `const int x;`.
- *Delta debugging* (TODO([#2][#2])): When a node has a list of children,
  `treereduce` uses *delta debugging* to delete as many as possible in an efficient
  way.
- *Hoisting* (TODO([#3][#3])): Nodes with a recursive structure may be replaced
  by their descendants, e.g. replacing `5 + (3 * y)` with just `y`.

## Pseudocode

A few notes:

- In practice, `weight` is simply the number of bytes in the source text of the
  node.

TODO(lb): This is not current/complete.

```python
class NodeId:
    ...

class Node:
    ...  # defined in tree-sitter

    def is_list(self) -> bool:
        ...

    def is_optional(self) -> bool:
        ...

class Heap:
    ...

class Tree:
    def find(self, node_id: NodeId) -> Node:
        ...

    def render(self) -> str:
        ...

    def replace(self, old_node, new_node) -> Tree:
        ...

    def root_node(self) -> Node:
        ...

enum Task:
    Explore(NodeId)
    Delete(NodeId)
    Hoist(NodeId, NodeId)
    Delta(NodeId)

class PrioritizedTask:
    task: Task
    priority: int

class Target:
    tree: Tree
    text: Text

class Ctx:
    target: Target
    heap: RwLock[Heap]
    edits: RwLock[Edits]
    # TODO(lb): Condition variables?
    threads: AtomicUsize
    idle_threads: AtomicUsize

def treereduce(source_code: str) -> str:
    tree = parse(source_code)
    target = Target(tree, source_code)
    ctx = Ctx(target, RwLock(Heap()), Edits())
    root = tree.root_node()
    task = Explore(NodeId(root))
    ctx.heap.push(PrioritizedTask(task, priority=weight(root)))
    threads = AtomicUsize(0)
    idle_threads = AtomicUsize(0)
    fork(spawn, tree, heap, threads, idle_threads)

    # Wait for all threads to finish and exit:
    while tree.count_references() > 1:
        wait()

    return tree.extract().render()

# -----------------------------------------------------------
# Parallel structure of the computation

def spawn(tree: Tree, heap: Heap, threads: AtomicUsize, idle_threads: AtomicUsize) -> None:
    threads += 1
    idle = False
    while True:
        if idle:
            idle_threads -= 1
            idle = False
        heap.lock()
        match heap.pop_max():
            case None:
                heap.unlock()
                idle = idle_logic(idle_threads)
            case task:
                heap.unlock()
                dispatch(tree, heap, task)

def idle_logic(idle_threads: AtomicUSize) -> None:
    idle_threads += 1
    if idle_threads == NUM_THREADS:
        exit_thread()
    sleep()  # some kind of backoff
    return True

# -----------------------------------------------------------
# Reduction logic

def dispatch(tree: Tree, heap: Heap, task: Task) -> None:
    match task:
        case Explore(node_id):
            explore(tree, heap, node_id)
        case Delete(node_id):
            delete(tree, heap, node_id)
        case Hoist(node_id, node_id):
            assert False, "Unimplemented" # TODO(lb)
        case Delta(node_id):
            assert False, "Unimplemented" # TODO(lb)

def explore(tree: Tree, heap: Heap, node_id: NodeId) -> None:
    node = tree.find(node_id)
    with heap.lock():
        if node.is_optional():
            heap.push(PrioritizedTask(Delete(node, priority=weight(node))))
        else:
            for child in node.children():
                heap.push(PrioritizedTask(Explore(child, priority=weight(child))))
        # TODO(lb): Other tasks

def delete(tree: Tree, heap: Heap, node_id: NodeId) -> None:
    pass

# -----------------------------------------------------------
# Helpers

def interesting_replacement(tree, node, variant):
    return interesting(tree.replace(node, variant).render())

def weight(node):
    ...

def parse(source_code: str) -> Tree:
    ...

def exit_thread():
    ...
```

(bib)=
## Bibliography

TODO(#16): BibTeX

- Gharachorlu, G. and Sumner, N., 2019, April. : Priority Aware Test Case
  Reduction. In International Conference on Fundamental Approaches to Software
  Engineering (pp. 409-426). Springer, Cham.
- Sun, C., Li, Y., Zhang, Q., Gu, T. and Su, Z., 2018, May. Perses:
  Syntax-guided program reduction. In Proceedings of the 40th International
  Conference on Software Engineering (pp. 361-371).
- Hodován, R. and Kiss, Á., 2016, July. Practical Improvements to the Minimizing
  Delta Debugging Algorithm. In ICSOFT-EA (pp. 241-248).
- Hodován, R. and Kiss, Á., 2016, November. Modernizing hierarchical delta
  debugging. In Proceedings of the 7th International Workshop on Automating Test
  Case Design, Selection, and Evaluation (pp. 31-37).
- Vince, D., Hodován, R., Bársony, D. and Kiss, Á., 2021, May. Extending
  Hierarchical Delta Debugging with Hoisting. In 2021 IEEE/ACM International
  Conference on Automation of Software Test (AST) (pp. 60-69). IEEE.
- Kiss, Á., Hodován, R. and Gyimóthy, T., 2018, November. HDDr: a recursive
  variant of the hierarchical delta debugging algorithm. In Proceedings of the
  9th ACM SIGSOFT International Workshop on Automating TEST Case Design,
  Selection, and Evaluation (pp. 16-22).
- Hodován, R., Kiss, Á. and Gyimóthy, T., 2017, September. Coarse hierarchical
  delta debugging. In 2017 IEEE international conference on software maintenance
  and evolution (ICSME) (pp. 194-203). IEEE.
- https://blog.sigplan.org/2021/03/30/an-overview-of-test-case-reduction/
- https://blog.trailofbits.com/2019/11/11/test-case-reduction/
- https://www.drmaciver.com/2017/06/adaptive-delta-debugging/
- https://www.drmaciver.com/2019/01/notes-on-test-case-reduction/

[#1]: https://github.com/langston-barrett/treereduce/issues/1
[#2]: https://github.com/langston-barrett/treereduce/issues/2
[#3]: https://github.com/langston-barrett/treereduce/issues/3
[#16]: https://github.com/langston-barrett/treereduce/issues/16
[afl]: https://lcamtuf.coredump.cx/afl/technical_details.txt
[pardis]: https://github.com/golnazgh/PARDIS
