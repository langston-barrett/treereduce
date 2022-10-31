use std::io;
use std::io::Write;

use tree_sitter::{Node, Tree};

use crate::alter::Alter;

// TODO(lb): Assert bounds checks
fn node_bytes_from<'a>(node: &'a Node, source: &'a [u8], from: usize) -> &'a [u8] {
    &source[from..node.end_byte()]
}

// TODO(lb): Assert bounds checks
fn node_bytes_until<'a>(node: &'a Node, source: &'a [u8], until: usize) -> &'a [u8] {
    &source[node.start_byte()..until]
}

fn node_bytes<'a>(node: &'a Node, source: &'a [u8]) -> &'a [u8] {
    node_bytes_until(node, source, node.end_byte())
}

// TODO(lb): Memoize? Probably not a hot spot.
fn has_alter(tree: &Tree, node: &Node, alter: &Alter) -> bool {
    if alter.should_omit(node) {
        true
    } else {
        node.children(&mut tree.walk())
            .any(|c| has_alter(tree, &c, alter))
    }
}

#[inline(always)] // no extra stack frame
fn render_with_children(
    w: &mut impl Write,
    tree: &Tree,
    node: &Node,
    source: &[u8],
    alter: &Alter,
) -> Result<(), io::Error> {
    debug_assert!(node.child_count() != 0);
    let first = node.child(0).unwrap();
    debug_assert!(first.start_byte() > node.start_byte());
    w.write_all(node_bytes_until(node, source, first.start_byte()))?;
    let mut last = node.end_byte();
    for child in node.children(&mut tree.walk()) {
        last = child.end_byte();
        render_node(w, tree, &child, source, alter)?;
    }
    w.write_all(node_bytes_from(node, source, last))?;
    Ok(())
}

fn render_node(
    w: &mut impl Write,
    tree: &Tree,
    node: &Node,
    source: &[u8],
    alter: &Alter,
) -> Result<(), io::Error> {
    if !has_alter(tree, node, alter) {
        w.write_all(node_bytes(node, source))?;
    } else if alter.should_omit(node) {
        return Ok(());
    } else {
        render_with_children(w, tree, node, source, alter)?;
    }
    Ok(())
}

pub fn render(
    w: &mut impl Write,
    tree: &Tree,
    source: &[u8],
    alter: &Alter,
) -> Result<(), io::Error> {
    render_node(w, tree, &tree.root_node(), source, alter)
}

pub fn show(w: &mut impl Write, tree: &Tree, source: &[u8]) -> Result<(), io::Error> {
    render(w, tree, source, &Alter::new())
}

pub fn show_stdout(tree: &Tree, source: &[u8]) -> Result<(), io::Error> {
    // https://nnethercote.github.io/perf-book/io.html#locking
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    crate::render::render(&mut lock, tree, source, &Alter::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_c::language())
            .expect("Error loading C grammar");
        parser.parse(src, None).expect("Failed to parse test")
    }

    fn do_render(tree: &Tree, src: &str, alter: &Alter) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
        render(&mut v, tree, src.as_bytes(), alter).expect("I/O error on a vector?");
        v
    }

    fn parse_then_render(src: &str, alter: &Alter) -> Vec<u8> {
        do_render(&parse(src), src, alter)
    }

    #[test]
    fn parse_then_render_nil() {
        let src = r#""#;
        let r = parse_then_render(src, &Alter::new());
        assert!(src.as_bytes() == r)
    }

    #[test]
    fn parse_then_render_main_id() {
        let src = r#"int main(int argc, char *argv[]) { return 0; }"#;
        let r = parse_then_render(src, &Alter::new());
        assert!(src.as_bytes() == r)
    }

    #[test]
    fn parse_then_render_main_omit() {
        let src = r#"int main(int argc, char *argv[]) { return 0; }"#;
        let tree = parse(src);
        let mut alter = Alter::new();
        alter.omit(&tree.root_node());
        let r = do_render(&tree, src, &alter);
        assert!("".as_bytes() == r)
    }
}
