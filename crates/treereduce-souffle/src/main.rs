use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(
        tree_sitter_souffle::language(),
        tree_sitter_souffle::NODE_TYPES,
    )
}
