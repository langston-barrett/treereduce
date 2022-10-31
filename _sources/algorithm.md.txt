# Algorithm

`treedd` brings together many improvements to the delta-debugging algorithm for
tree-structured inputs from the academic literature. Following the naming
schemes in the literature, we might call its algorithm "recursive hierarchical
delta debugging with hoisting".

See: HDDr Figure 5.

```python
def treedd():
    tree = parse(source_code)
    queue = Queue(tree.root_node())
    while not queue.empty():
        parallel for node in queue:
            while True:
                new_node = minimize(tree, node)
                with tree.lock():
                    if node not in tree:
                        break
                    if interesting_replacement(tree, node, new_node):
                        tree.replace(node, new_node)
                        break
            queue.extend(node.children)
    print(tree.render())

def interesting_replacement(tree, node, variant):
    return interesting(tree.replace(node, variant).render())

def minimize(tree, node):
    if node.is_optional():
        deleted = try_delete(tree, node)
        if new != None:
            return deleted
    if node.is_list():
        return pddmin2(tree, node)
    return hoist(tree, node)

def pddmin2(tree, node):
    if i > node.width():
        return node

    interesting_variant = None
    # TODO(lb): sort
    variants = {deltas(node, i) + nablas(node, i) for i in 2...}
    parallel for variant in variants:
        if interesting(tree.replace(node, variant).render()):
            interesting_variant = variant
            parallel return variant

def hoist(tree, node, i):
    pass
```

## Bibliography

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
  
To read:

- Groce, A., Alipour, M.A., Zhang, C., Chen, Y. and Regehr, J., 2016. Cause
  reduction: delta debugging, even without bugs. Software Testing, Verification
  and Reliability, 26(1), pp.40-68.
- Test Case Reduction: A Framework, Benchmark, and Comparative Study
- Hodován, R., Kiss, Á. and Gyimóthy, T., 2017, September. Coarse hierarchical
  delta debugging. In 2017 IEEE international conference on software maintenance
  and evolution (ICSME) (pp. 194-203). IEEE.
