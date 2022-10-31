use std::fs;
use std::io;
use std::io::Read;
use std::process;

use anyhow::{Context, Result};
use clap::Parser;
use tree_sitter::Tree;

use crate::check::Check;

#[derive(clap::ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum OnParseError {
    Ignore,
    Warn,
    Error,
}

impl std::fmt::Display for OnParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OnParseError::Ignore => write!(f, "ignore"),
            OnParseError::Warn => write!(f, "warn"),
            OnParseError::Error => write!(f, "error"),
        }
    }
}

impl Default for OnParseError {
    fn default() -> Self {
        OnParseError::Warn
    }
}

fn handle_parse_errors(path: &str, tree: &Tree, on_parse_error: &OnParseError) {
    let node = tree.root_node();
    match on_parse_error {
        OnParseError::Ignore => (),
        OnParseError::Warn if !node.has_error() => (),
        OnParseError::Error if !node.has_error() => (),
        OnParseError::Warn => {
            eprintln!("[warn] Parse error in {}", path);
        }
        OnParseError::Error => {
            eprintln!("[error] Parse error in {}", path);
            process::exit(1);
        }
    }
}

/// Minimize a program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// TODO(#5): --output flag
// #[arg(short, long, default_value = None)]
// pub output: Option<String>,
// TODO(#6): stdout/stderr regex
// TODO(#7): --jobs flag
// TODO(#8): --verbosity flag
pub struct Args {
    /// Behavior on parse errors
    #[arg(long, default_value_t = OnParseError::Warn, value_name = "CHOICE")]
    on_parse_error: OnParseError,

    /// Exit code to consider interesting
    #[arg(long, default_values_t = vec![0], value_name = "EXIT_CODE")]
    interesting_exit_code: Vec<i32>,

    /// Source code to consume; if empty, parse from stdin
    #[arg(short, long, default_value = None, value_name = "SRC_FILE")]
    pub source: Option<String>,

    /// Interestingness check
    #[arg(value_name = "CMD")]
    pub check: String,
}

fn read_file(file: &str) -> Result<String> {
    fs::read_to_string(file).with_context(|| format!("Failed to read file {}", file))
}

fn parse(language: tree_sitter::Language, code: &str) -> Result<tree_sitter::Tree> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(language)
        .context("Failed to set tree-sitter parser language")?;
    parser.parse(code, None).context("Failed to parse code")
}

fn stdin_string() -> Result<String> {
    let mut stdin_str: String = String::new();
    io::stdin().read_to_string(&mut stdin_str)?;
    Ok(stdin_str)
}

fn check(args: &Args) -> Check {
    let mut argv: Vec<_> = args.check.split_whitespace().collect();
    match argv.pop() {
        None => {
            eprintln!("Empty check!");
            std::process::exit(1);
        }
        Some(cmd) => Check::new(
            cmd.to_string(),
            argv.iter().map(|s| s.to_string()).collect(),
            args.interesting_exit_code.clone(),
        ),
    }
}

pub fn main(language: tree_sitter::Language) -> Result<()> {
    let args = Args::parse();
    let chk = check(&args);
    let (path, src) = if let Some(path) = args.source {
        (path.to_string(), read_file(&path)?)
    } else {
        ("<stdin>".to_string(), stdin_string()?)
    };
    let tree = parse(language, &src)?;
    handle_parse_errors(&path, &tree, &args.on_parse_error);

    let mut test: Vec<u8> = Vec::new(); // TODO(lb): reserve
    crate::render::render(
        &mut test,
        &tree,
        src.as_bytes(),
        &crate::alter::Alter::new(),
    )?;
    if !chk
        .interesting(&test)
        .context("Failed to check that initial input was interesting")?
    {
        eprintln!("Initial test was not interesting. See the usage documentation for help: https://langston-barrett.github.io/treedd/usage.html");
        std::process::exit(1);
    }

    let gen_tree = crate::dd::treedd(tree, src.as_bytes().to_vec(), &chk)?;
    // TODO(#4): Default to outputting to treedd.out
    crate::render::show_stdout(&gen_tree.tree, &gen_tree.source)?;
    Ok(())
}
