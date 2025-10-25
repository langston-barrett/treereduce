use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_smtlib2::language(),
        tree_sitter_smtlib2::NODE_TYPES,
        HashMap::new(),
    )
}
