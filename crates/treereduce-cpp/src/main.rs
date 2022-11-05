use anyhow::Result;

fn main() -> Result<()> {
    treereduce::cli::main(tree_sitter_cpp::language(), tree_sitter_cpp::NODE_TYPES)
}
