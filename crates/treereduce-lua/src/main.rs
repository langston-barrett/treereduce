use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_lua::LANGUAGE.into(),
        tree_sitter_lua::NODE_TYPES,
        HashMap::new(),
    )
}
