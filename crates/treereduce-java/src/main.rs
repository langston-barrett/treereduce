use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_java::LANGUAGE.into(),
        tree_sitter_java::NODE_TYPES,
        HashMap::new(),
    )
}
