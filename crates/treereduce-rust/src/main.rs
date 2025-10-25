use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_rust::LANGUAGE.into(),
        tree_sitter_rust::NODE_TYPES,
        HashMap::from([("type_identifier", &["()"][..])]),
    )
}
