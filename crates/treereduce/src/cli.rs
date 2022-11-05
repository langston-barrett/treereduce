use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;
use tree_sitter::Tree;

use crate::check::{Check, CmdCheck};
use crate::edits::Edits;
use crate::original::Original;

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
            log::warn!("Parse error in {}", path);
        }
        OnParseError::Error => {
            log::error!("Parse error in {}", path);
            process::exit(1);
        }
    }
}

/// Minimize a program
#[derive(Parser, Debug)]
// TODO(lb): Think about the ordering... use clap argument groups?
#[command(author, version, about, long_about = None)]
// TODO(#6): stdout/stderr regex
// TODO(lb): --fast flag
// TODO(lb): --inherit-std{out,err} flag, silence by default
// TODO(#7): --jobs flag
// TODO(lb): --slow flag
// TODO(lb): --timeout flag
// TODO(#8): --verbosity flag
pub struct Args {
    /// Behavior on parse errors
    #[arg(long, default_value_t = OnParseError::Warn, value_name = "CHOICE")]
    on_parse_error: OnParseError,

    /// Exit code to consider interesting
    #[arg(long, default_values_t = vec![0], value_name = "EXIT_CODE")]
    interesting_exit_code: Vec<i32>,

    /// Number of threads
    #[arg(short, long, default_value_t = 0)]
    pub jobs: usize,

    /// File to output, use '-' for stdout
    #[arg(short, long, default_value = "treereduce.out")]
    pub output: String,

    /// Don't verify interestingness of the initial test case
    #[arg(long, default_value_t = false)]
    pub no_verify: bool,

    /// Don't silence the stdout of the interestingness check
    #[arg(long, default_value_t = false)]
    pub inherit_stdout: bool,

    /// Don't silence the stdout of the interestingness check
    #[arg(long, default_value_t = false)]
    pub inherit_stderr: bool,

    /// Directory in which to place temporary files
    #[arg(long, default_value = None)]
    pub temp_dir: Option<String>,

    /// Source code to consume; if empty, parse from stdin
    #[arg(short, long, default_value = None, value_name = "SRC_FILE")]
    pub source: Option<String>,

    /// How many reduction passes to make
    #[arg(long, default_value_t = 2)]
    pub passes: usize,

    /// Run until no progress is made - this may be very slow
    #[arg(long, default_value_t = false)]
    pub stable: bool,

    /// Interestingness check
    #[arg(value_name = "CMD")]
    pub check: String,
}

fn read_file(file: &str) -> Result<String> {
    fs::read_to_string(file).with_context(|| format!("Failed to read file {}", file))
}

fn make_temp_dir(dir: &Option<String>) -> Result<()> {
    if let Some(d) = dir {
        // Just best-effort, to error out early
        std::fs::create_dir_all(d)
            .with_context(|| format!("Failed to access or create temporary directory {}", d))?;
    }
    Ok(())
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

fn check(args: &Args) -> CmdCheck {
    let mut argv: Vec<_> = args.check.split_whitespace().collect();
    if argv.is_empty() {
        eprintln!("Empty check!");
        std::process::exit(1);
    }
    let cmd = argv[0];
    argv.remove(0);
    CmdCheck::new(
        cmd.to_string(),
        argv.iter().map(|s| s.to_string()).collect(),
        args.interesting_exit_code.clone(),
        args.temp_dir.clone(),
        args.inherit_stdout,
        args.inherit_stderr,
    )
}

fn check_initial_input_is_interesting(chk: &CmdCheck, tree: &Tree, src: &[u8]) -> Result<()> {
    let mut test: Vec<u8> = Vec::new(); // TODO(lb): reserve
    tree_sitter_edit::render(&mut test, tree, src, &crate::edits::Edits::new())?;
    if !chk
        .interesting(&test)
        .context("Failed to check that initial input was interesting")?
    {
        eprintln!("Initial test was not interesting. See the usage documentation for help: https://langston-barrett.github.io/treereduce/usage.html");
        std::process::exit(1);
    }
    Ok(())
}

fn edits(
    args: &Args,
    chk: CmdCheck,
    src_path: &str,
    src: &str,
    language: tree_sitter::Language,
    node_types_json_str: &'static str,
) -> Result<(Tree, Edits)> {
    let tree = parse(language, src)?;
    handle_parse_errors(src_path, &tree, &args.on_parse_error);
    let node_types = crate::node_types::NodeTypes::new(node_types_json_str)?;
    let tree2 = tree.clone();
    let orig = Original::new(tree, src.as_bytes().to_vec());
    let edits = crate::reduce::treereduce(args.jobs, node_types, orig, chk)?;
    Ok((tree2, edits))
}

fn print_result(output: &str, src: &str) -> Result<()> {
    if output == "-" {
        // https://nnethercote.github.io/perf-book/io.html#locking
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        lock.write_all(src.as_bytes())?;
    } else {
        let mut file = File::create(output)?;
        file.write_all(src.as_bytes())?;
    }
    Ok(())
}

pub fn main(language: tree_sitter::Language, node_types_json_str: &'static str) -> Result<()> {
    env_logger::builder()
        .format_module_path(false)
        .format_timestamp(None)
        .init();

    let args = Args::parse();
    make_temp_dir(&args.temp_dir)?;

    let chk = check(&args);
    let (path, mut src) = if let Some(p) = &args.source {
        (p.to_string(), read_file(p)?)
    } else {
        ("<stdin>".to_string(), stdin_string()?)
    };

    let mut tree = parse(language, &src)?;
    if !args.no_verify {
        check_initial_input_is_interesting(&chk, &tree, src.as_bytes())?;
    }

    let reduce_start = Instant::now();
    let mut passes = 1;
    let mut es = Edits::new();
    while passes <= args.passes || (args.stable && !es.is_empty()) {
        log::info!(
            "Starting pass {} / {}",
            passes,
            if args.stable {
                "?".to_string()
            } else {
                format!("{}", args.passes)
            }
        );
        let pass_start = Instant::now();
        (tree, es) = edits(
            &args,
            check(&args),
            &path,
            &src,
            language,
            node_types_json_str,
        )?;
        let mut new_src = Vec::new();
        tree_sitter_edit::render(&mut new_src, &tree, src.as_bytes(), &es)?;
        src = std::str::from_utf8(&new_src)?.to_string();
        passes += 1;
        log::debug!(
            "Pass {} duration: {}ms",
            passes,
            pass_start.elapsed().as_millis()
        );
    }
    log::info!("Total time: {}ms", reduce_start.elapsed().as_millis());
    print_result(&args.output, &src)?;
    Ok(())
}
