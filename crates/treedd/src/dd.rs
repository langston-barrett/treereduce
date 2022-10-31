use std::sync::{Arc, LockResult, RwLock};
use tree_sitter::{Node, Tree};

// use crate::alter::Alter;
use crate::check::Check;

// TODO(#15): Refine with access to node-types.json
fn is_list(_node: &Node) -> bool {
    false
}

// TODO(#15): Refine with access to node-types.json
fn is_optional(_node: &Node) -> bool {
    true
}

fn try_delete<'a>(_tree: Arc<RwLock<Tree>>, _node: &'a Node<'a>) -> Vec<Node<'a>> {
    Vec::new()
}

fn ddmin(_tree: Arc<RwLock<Tree>>, _node: Node) -> Vec<Node> {
    Vec::new()
}

fn minimize(tree: Arc<RwLock<Tree>>, node: Node) -> Vec<Node> {
    if is_optional(&node) {
        try_delete(Arc::clone(&tree), &node);
    }
    if is_list(&node) {
        ddmin(tree, node);
    }
    Vec::new()
}

pub fn treedd(tree: Tree, _check: &Check) -> LockResult<Tree> {
    let lock = Arc::new(RwLock::new(tree.clone()));
    let mut queue = vec![tree.root_node()];
    while !queue.is_empty() {
        // TODO(lb): parallel
        queue = queue
            .into_iter()
            .flat_map(|node| minimize(Arc::clone(&lock), node))
            .collect();
    }
    Ok(tree)
    // (*lock).into_inner()
    // let r = lock.write();
    // assert!(r.is_ok());
    // let x = r.unwrap();
    // x
}
