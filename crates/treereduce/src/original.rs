use tree_sitter::Tree;

/// The original parse tree and program source code provided by the user
#[derive(Clone, Debug)]
pub struct Original {
    pub tree: Tree,
    pub text: Vec<u8>,
}

impl Original {
    pub fn new(tree: Tree, text: Vec<u8>) -> Original {
        Original { tree, text }
    }
}
