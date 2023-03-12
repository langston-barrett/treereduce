use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_souffle::language(),
        tree_sitter_souffle::NODE_TYPES,
        HashMap::from([
            // ("atom", &["0 = 0"][..]),
            // ("disjunction", &[""][..]),
            // ("conjunction", &[""][..]),
            // ("ident", &["0", "\"\""][..]),
            ("string", &["\"\""][..]),
            ("number", &["0"][..]),
        ]),
    )
}
