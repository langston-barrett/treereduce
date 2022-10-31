// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};
use std::io;
use std::sync::{Arc, RwLock};
use tree_sitter::{Language, Node, Tree};

use crate::alter::Alter;
use crate::check::Check;

// struct NodeHash {
//     hash: u64,
// }

// impl NodeHash {
//     fn new(node: &Node, source: &[u8]) -> Self {
//         let mut s = DefaultHasher::new();
//         node.utf8_text(source).unwrap().hash(&mut s); // TODO(lb): no unwrap
//         NodeHash { hash: s.finish() }
//     }
// }

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

fn try_delete<'a>(ctx: &'a Ctx, node: Node<'a>) -> io::Result<Vec<Node<'a>>> {
    match ctx.interesting(&Alter::new().omit(&node))? {
        // This tree was deleted, no need to recurse on children
        InterestingCheck::Yes => {
            eprintln!("Interesting deletion of {}", node.kind());
            // let mut r = ctx.lock.write().unwrap();
            // eprintln!("New tree: {}", node.kind());
            Ok(Vec::new())
        }
        InterestingCheck::TryAgain => Ok(vec![node]),
        InterestingCheck::No => {
            // can't use node.children() because it requires borrowing in the
            // lock
            let mut v = Vec::new();
            for i in 0..node.child_count() {
                v.push(node.child(i).expect("Counting error!"));
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

fn _ddmin<'a>(_ctx: &'a Ctx, _node: Node<'a>) -> Vec<Node<'a>> {
    Vec::new()
}

fn minimize<'a>(ctx: &'a Ctx, node: Node<'a>) -> io::Result<Vec<Node<'a>>> {
    eprintln!("Minimizing {} {}", node.id(), node.kind());
    // if is_optional(&node) {
    try_delete(ctx, node)
    // }
    // if is_list(&node) {
    //     ddmin(ctx, node);
    // }
    // Vec::new()
}

pub fn treedd(tree: Tree, source: Vec<u8>, check: &Check) -> io::Result<GenTree> {
    // TODO(lb): Queue needs to consist of node IDs - hashes of node text
    let mut queue: Vec<Node> = Vec::new();
    let lock = Arc::new(RwLock::new(GenTree::new(tree, source)));
    let ctx = Ctx { lock, check };
    while !queue.is_empty() {
        // TODO(lb): parallel
        let mut new_queue = Vec::new();
        new_queue.reserve(queue.len());
        for node in queue.into_iter() {
            new_queue.extend(minimize(&ctx, node)?)
        }
        queue = new_queue;
    }
    Ok(Arc::try_unwrap(ctx.lock)
        .expect("Only one reference should exist!")
        .into_inner()
        .expect("Corrupt lock!"))
}
