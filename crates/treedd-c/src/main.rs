use anyhow::Result;

fn main() -> Result<()> {
    treedd::cli::main(tree_sitter_c::language())
}
