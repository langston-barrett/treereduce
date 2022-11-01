# Algorithm

(algorithm-goals)=
## Goals

The core reduction algorithm has three goals:

1. Go fast
2. Produce small test cases
3. Produce readable test cases

Unlike more theoretical work in this space, the algorithm does *not* attempt to
minimize the number of "oracle queries", that is, invocations of the
user-provided "interestingness test".

(algorithm-assumptions)=
## Assumptions

These assumptions strongly inform the algorithm design:

1. The interestingness will be comparatively slow---it will generally involve
   spinning up a new process, disk I/O, parsing, type-checking, etc.
2. Smaller inputs lead to faster interestingness tests.

These assumptions hold for the use-case of reducing programs that cause compiler
crashes.

## High-Level Design

Due to [Assumption (1)](algorithm-assumptions), it's essential that `treedd`
execute several interestingness tests in parallel. Luckily, for the same reason,
lock contention is unlikely to be a major issue---so long as executing the
interestingness test doesn't require holding a lock, most threads will spend the
majority of their time executing the interestingness test, rather than waiting
for locks on shared data.

The recent paper "[PARDIS][pardis] : Priority Aware Test Case Reduction"
highlights the importance of *prioritization* of reductions. Greedy removal of
the *largest* subtrees leads to greatly increased performance due to faster
interestingness tests ([Assumption (2)](algorithm-assumptions)).

With these ideas in mind, the overall algorithm design involves spinning up some
number of threads, which share two pieces of mutable (locked) data: the target
program being minimized, and a prioritized max-heap of reductions to attempt.
Each thread executes the following loop:

- Pop a reduction task off the heap
- Execute the interestingness test with the reduced program
- If the reduced program was still interesting, try replacing the global target:

  * If the target was replaced by another thread, try this reduction again
  * Otherwise, replace the target with the reduced version

- Push any new tasks onto the task queue

If lock contention does become an issue, it may be beneficial for each thread to
maintain a local prioritized heap in addition to the global one.

## Reduction Strategies

`treedd` uses several strategies during program minimization:

- *Deletion* (TODO([#1][#1])): When a child is optional, `treedd` attempts to
  delete it. For example, `treedd` might delete the `const` in `const int x;`.
- *Delta debugging* (TODO([#2][#2])): When a node has a list of children,
  `treedd` uses *delta debugging* to delete as many as possible in an efficient
  way.
- *Hoisting* (TODO([#3][#3])): Nodes with a recursive structure may be replaced
  by their descendants, e.g. replacing `5 + (3 * y)` with just `y`.

## Pseudocode

```python
class Tree:
    def render(self) -> str:
        ...

    def replace(self, old_node, new_node) -> Tree:
        ...

class Node:
    ...  # defined in tree-sitter

enum Task:
    Explore(Node)
    Delete(Node)
    Hoist(Node, Node)
    Delta(Node)

class PrioritizedTask:
    priority: int
    task: Task

def treedd():
    tree = parse(source_code)
    heap = Heap(PrioritizedTask(Explore(tree.root_node()), priority=0))
    threads = AtomicUsize(0)
    idle_threads = AtomicUsize(0)
    fork(spawn, tree, heap, threads, idle_threads)

    # Wait for all threads to finish and exit:
    while tree.count_references() > 1:
        wait()

    return tree.extract()

def spawn(tree, heap, threads, idle_threads):
    pass

def interesting_replacement(tree, node, variant):
    return interesting(tree.replace(node, variant).render())
```

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

[#1]: https://github.com/langston-barrett/treedd/issues/1
[#2]: https://github.com/langston-barrett/treedd/issues/2
[#3]: https://github.com/langston-barrett/treedd/issues/3
[#16]: https://github.com/langston-barrett/treedd/issues/16
[pardis]: https://github.com/golnazgh/PARDIS
