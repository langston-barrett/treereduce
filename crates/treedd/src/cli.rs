use std::fs;
use std::io;
use std::io::Read;
use std::process;

use anyhow::{Context, Result};
use clap::Parser;
use tree_sitter::Tree;

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
// TODO(lb): Output file
// #[arg(short, long, default_value = None)]
// pub output: Option<String>,
// TODO(lb): Interestingness test
pub struct Args {
    /// Behavior on parse errors
    #[arg(long, default_value_t = OnParseError::Warn, value_name = "CHOICE")]
    on_parse_error: OnParseError,

    /// Source code to consume; if empty, parse from stdin
    #[arg(short, long, default_value = None, value_name = "SRC_FILE")]
    pub source: Option<String>,
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

pub fn main(language: tree_sitter::Language) -> Result<()> {
    let args = Args::parse();
    let (path, src) = if let Some(path) = args.source {
        (path.to_string(), read_file(&path)?)
    } else {
        ("<stdin>".to_string(), stdin_string()?)
    };
    let tree = parse(language, &src)?;
    handle_parse_errors(&path, &tree, &args.on_parse_error);
    // https://nnethercote.github.io/perf-book/io.html#locking
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    crate::render::render(
        &mut lock,
        &tree,
        src.as_bytes(),
        &crate::alter::Alter::new(),
    )?;
    Ok(())
}
