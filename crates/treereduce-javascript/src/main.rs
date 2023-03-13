use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_javascript::language(),
        tree_sitter_javascript::NODE_TYPES,
        HashMap::new(),
    )
}
