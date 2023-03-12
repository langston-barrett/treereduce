use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_c::language(),
        tree_sitter_c::NODE_TYPES,
        // ("parameter_declaration", &["int x"][..]),
        HashMap::from([
            ("compound_statement", &["{}"][..]),
            ("number_literal", &["0"][..]),
            ("parameter_list", &["()"][..]),
            ("primitive_type", &["int"][..]),
            ("return_statement", &["return;"][..]),
            ("string_literal", &["\"\""][..]),
            // Notes:
            //
            // - function_definition is optional wherever it appears, no need
            //   to replace
        ]),
    )
}
