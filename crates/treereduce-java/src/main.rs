use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_java::language(),
        tree_sitter_java::NODE_TYPES,
        HashMap::new(),
    )
}
