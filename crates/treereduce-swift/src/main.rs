use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
  treereduce::cli::main(
    tree_sitter_swift::language(),
    tree_sitter_swift::NODE_TYPES,
    HashMap::new(),
  )
}
