use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io;
use std::sync::{Arc, RwLock};
use tree_sitter::{Language, Node, Tree};

use crate::alter::Alter;
use crate::check::Check;
use crate::id::NodeId;

#[derive(PartialEq, Eq, Hash)]
struct NodeHash {
    hash: u64,
}

impl NodeHash {
    fn new(node: &Node, source: &[u8]) -> Self {
        let mut s = DefaultHasher::new();
        node.utf8_text(source).unwrap().hash(&mut s); // TODO(lb): no unwrap
        NodeHash { hash: s.finish() }
    }
}

// fn find_node<'a>(tree: &'a Tree, source: &[u8], node: &'a Node, hash: &'a NodeHash) -> Option<Node> {
//     if NodeHash::new(node, ())
// }

// TODO(lb)!
// fn find<'a>(_tree: &'a Tree, _source: &[u8], _hash: &'a NodeHash) -> Option<Node<'a>> {
//     unimplemented!()
// }

fn find<'a>(_tree: &'a Tree, _hash: &'a NodeHash) -> Option<Node<'a>> {
    unimplemented!()
}

enum InterestingCheck {
    Yes,
    No,
    TryAgain, // tree was concurrently modified
}

#[derive(Debug)]
pub struct GenTree {
    gen: usize, // "generation"
    pub tree: Tree,
    pub source: Vec<u8>,
}

impl GenTree {
    fn new(tree: Tree, source: Vec<u8>) -> Self {
        GenTree {
            gen: 0,
            tree,
            source,
        }
    }

    fn next(&self, tree: Tree, source: Vec<u8>) -> Self {
        GenTree {
            gen: self.gen + 1,
            tree,
            source,
        }
    }
}

struct Ctx<'a> {
    lock: Arc<RwLock<GenTree>>,
    check: &'a Check,
}

impl<'a> Ctx<'a> {
    fn render(&self, alter: &Alter) -> io::Result<(bool, usize, Vec<u8>)> {
        let mut text: Vec<u8> = Vec::new();
        let r = self.lock.read().unwrap();
        text.reserve(r.source.len() / 2);
        let changed = crate::render::render(&mut text, &r.tree, &r.source, alter)?;
        Ok((changed, r.gen, text))
    }

    fn language(&self) -> Language {
        self.lock.read().unwrap().tree.language()
    }

    fn parse(&self, src: &[u8]) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        // TODO(lb): Incremental re-parsing
        parser
            .set_language(self.language())
            .expect("Error loading language");
        parser.parse(src, None).expect("Failed to parse")
    }

    /// Check if a given alteration yields an interesting tree. If so, and if
    /// the tree hasn't been concurrently modified by another call to this
    /// function, replace the tree with the altered version.
    fn interesting(&self, alter: &Alter) -> io::Result<InterestingCheck> {
        let (_changed, gen, rendered) = self.render(alter)?;
        // TODO(lb)
        // if !changed {
        //     return Ok(InterestingCheck::TryAgain);
        // }
        eprintln!(
            "Rendered with {:?}: {}",
            alter,
            std::str::from_utf8(&rendered).unwrap()
        );
        let reparsed = self.parse(&rendered);
        // TODO(lb): Don't introduce parse errors
        // assert!({
        //     if reparsed.root_node().has_error() {
        //         self.lock.read().unwrap().root_node().has_error()
        //     } else {
        //         true
        //     }
        // });
        if self.check.interesting(&rendered)? {
            let mut w = self.lock.write().unwrap();
            if w.gen != gen {
                return Ok(InterestingCheck::TryAgain);
            }
            *w = w.next(reparsed, rendered);
            eprintln!("New source: {}", std::str::from_utf8(&w.source).unwrap());
            Ok(InterestingCheck::Yes)
        } else {
            Ok(InterestingCheck::No)
        }
    }
}

// TODO(#15): Refine with access to node-types.json
fn _is_list(_node: &Node) -> bool {
    false
}

// TODO(#15): Refine with access to node-types.json
fn _is_optional(_node: &Node) -> bool {
    true
}

fn try_delete(ctx: &Ctx, node_id: NodeId, node_hash: NodeHash) -> io::Result<Vec<NodeHash>> {
    match ctx.interesting(&Alter::new().omit_id(node_id))? {
        // This tree was deleted, no need to recurse on children
        InterestingCheck::Yes => {
            // eprintln!("Interesting deletion of {}", node.kind());
            // let mut r = ctx.lock.write().unwrap();
            // eprintln!("New tree: {}", node.kind());
            Ok(Vec::new())
        }
        InterestingCheck::TryAgain => Ok(vec![node_hash]),
        InterestingCheck::No => {
            // TODO(lb): maybe use node.children() with walk()
            let r = ctx.lock.read().unwrap();
            let node = match find(&r.tree, &node_hash) {
                Some(n) => n,
                None => return Ok(Vec::new()),
            };
            let mut v = Vec::new();
            for i in 0..node.child_count() {
                v.push(NodeHash::new(
                    &node.child(i).expect("Counting error!"),
                    &r.source,
                ));
            }
            Ok(v)
        }
    }
    //
    // } else {
    //     // node.children(&mut ctx.tree.walk())
    //     todo!()
    // }
}

fn minimize(ctx: &Ctx, node_hash: NodeHash) -> io::Result<Vec<NodeHash>> {
    let (id, hash) = {
        let r = ctx.lock.read().unwrap();
        let node = match find(&r.tree, &node_hash) {
            Some(n) => n,
            None => return Ok(Vec::new()),
        };
        (NodeId::new(&node), NodeHash::new(&node, &r.source))
    };
    // if is_optional(&node) {
    try_delete(ctx, id, hash)
    // }
    // if is_list(&node) {
    //     ddmin(ctx, node);
    // }
    // Vec::new()
}

pub fn treedd(tree: Tree, source: Vec<u8>, check: &Check) -> io::Result<GenTree> {
    // TODO(lb): Queue needs to consist of node IDs - hashes of node text
    let mut queue: Vec<NodeHash> = vec![NodeHash::new(&tree.root_node(), &source)];
    let lock = Arc::new(RwLock::new(GenTree::new(tree, source)));
    let ctx = Ctx { lock, check };
    while !queue.is_empty() {
        let mut new_queue = Vec::new();
        new_queue.reserve(queue.len());
        // TODO(lb): parallel
        for hash in queue.into_iter() {
            new_queue.extend(minimize(&ctx, hash)?)
        }
        queue = new_queue;
    }
    Ok(Arc::try_unwrap(ctx.lock)
        .expect("Only one reference should exist!")
        .into_inner()
        .expect("Corrupt lock!"))
}
