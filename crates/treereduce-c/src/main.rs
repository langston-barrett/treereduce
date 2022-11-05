use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(tree_sitter_c::language(), tree_sitter_c::NODE_TYPES)
}
