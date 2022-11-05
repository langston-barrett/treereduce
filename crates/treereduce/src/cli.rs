use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};
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

const DEFAULT_MIN_REDUCTION: usize = 2;
const FAST_MIN_REDUCTION: usize = 4;
const SLOW_MIN_REDUCTION: usize = 1;
const DEFAULT_NUM_PASSES: usize = 2;
const FAST_NUM_PASSES: usize = 1;

/// Minimize a program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None,
          group(ArgGroup::new("fast-xor-slow").arg("fast").arg("slow")),
          group(ArgGroup::new("passes-xor-stable").arg("passes").arg("stable")))]
// TODO(#17): --stats
// TODO(#6): stdout/stderr regex
// TODO(#20): --timeout flag
// TODO(#26): --verbosity flag
pub struct Args {
    /// Source code to consume; if empty, parse from stdin
    #[arg(
        short, long, default_value = None, value_name = "FILE")]
    pub source: Option<String>,

    /// Behavior on parse errors
    #[arg(long, default_value_t = OnParseError::Warn, value_name = "CHOICE")]
    on_parse_error: OnParseError,

    /// Number of threads
    #[arg(short, long, default_value_t = num_cpus::get())]
    pub jobs: usize,

    /// File to output, use '-' for stdout
    #[arg(short, long, default_value = "treereduce.out")]
    pub output: String,

    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,

    /// Exit code to consider interesting
    #[arg(help_heading = "Interestingness check options",
          long, default_values_t = vec![0], value_name = "CODE")]
    interesting_exit_code: Vec<i32>,

    /// Don't verify interestingness of the initial test case
    #[arg(
        help_heading = "Interestingness check options",
        long,
        default_value_t = false
    )]
    pub no_verify: bool,

    /// Don't silence the stdout of the interestingness check
    #[arg(
        help_heading = "Interestingness check options",
        long,
        default_value_t = false
    )]
    pub inherit_stdout: bool,

    /// Don't silence the stdout of the interestingness check
    #[arg(
        help_heading = "Interestingness check options",
        long,
        default_value_t = false
    )]
    pub inherit_stderr: bool,

    /// Directory in which to place temporary (@@) files
    #[arg(
        help_heading = "Interestingness check options",
        long, default_value = None, value_name = "DIR")]
    pub temp_dir: Option<String>,

    /// Same as --passes 1, --min-reduction 4
    #[arg(help_heading = "Reduction options", long, default_value_t = false)]
    pub fast: bool,

    /// Same as --stable, --min-reduction 1
    #[arg(help_heading = "Reduction options", long, default_value_t = false)]
    pub slow: bool,

    /// How many reduction passes to make
    #[arg(help_heading = "Reduction options", long, default_value_t = DEFAULT_NUM_PASSES)]
    pub passes: usize,

    /// Minimum size reduction to attempt
    #[arg(
        help_heading = "Reduction options",
        long,
        default_value_t = DEFAULT_MIN_REDUCTION,
        value_name = "BYTES"
    )]
    pub min_reduction: usize,

    /// Run passes until no progress is made - may be slow
    #[arg(help_heading = "Reduction options", long, default_value_t = false)]
    pub stable: bool,

    /// Interestingness check; fed test case on stdin or via '@@' file
    #[arg(value_name = "CMD", required = true, num_args = 1..)]
    pub check: Vec<String>,
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

#[inline]
fn stdin_string() -> Result<String> {
    let mut stdin_str: String = String::new();
    io::stdin().read_to_string(&mut stdin_str)?;
    Ok(stdin_str)
}

fn check(args: &Args) -> CmdCheck {
    if args.check.is_empty() {
        eprintln!("Internal error: empty interestingness check!");
        std::process::exit(1);
    }
    let mut argv: Vec<_> = args.check.iter().collect();
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
    let mut test: Vec<u8> = Vec::new();
    test.reserve(src.len());
    tree_sitter_edit::render(&mut test, tree, src, &crate::edits::Edits::new())?;
    if !chk
        .interesting(&test)
        .context("Failed to check that initial input was interesting")?
    {
        log::error!("Initial test was not interesting. See the usage documentation for help: https://langston-barrett.github.io/treereduce/usage.html");
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
    min_reduction: usize,
) -> Result<(Tree, Edits)> {
    debug_assert!(min_reduction > 0);
    let tree = parse(language, src)?;
    handle_parse_errors(src_path, &tree, &args.on_parse_error);
    let node_types = crate::node_types::NodeTypes::new(node_types_json_str)?;
    let tree2 = tree.clone();
    let orig = Original::new(tree, src.as_bytes().to_vec());
    let edits = crate::reduce::treereduce(args.jobs, node_types, orig, chk, min_reduction)?;
    Ok((tree2, edits))
}

#[inline]
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

#[inline]
fn min_reduction(args: &Args) -> usize {
    debug_assert!(!(args.fast && args.slow));
    if args.fast {
        return FAST_MIN_REDUCTION;
    }
    if args.slow {
        return SLOW_MIN_REDUCTION;
    }
    args.min_reduction
}

#[inline]
fn passes(args: &Args) -> Option<usize> {
    debug_assert!(!(args.fast && args.slow));
    debug_assert!(args.passes == DEFAULT_NUM_PASSES || !args.stable);
    if args.fast {
        return Some(FAST_NUM_PASSES);
    }
    if (args.slow || args.stable) && args.passes == DEFAULT_NUM_PASSES {
        return None;
    }
    Some(args.passes)
}

fn init_logger(args: &Args) {
    env_logger::builder()
        .format_module_path(false) // seems not to work?
        .format_timestamp(None)
        .filter_level(args.verbose.log_level_filter())
        .init();
}

pub fn main(language: tree_sitter::Language, node_types_json_str: &'static str) -> Result<()> {
    let args = Args::parse();
    debug_assert!(args.passes == DEFAULT_NUM_PASSES || !args.stable);
    debug_assert!(!(args.fast && args.slow));

    init_logger(&args);
    make_temp_dir(&args.temp_dir)?;
    let chk = check(&args);

    let (path, mut src) = if let Some(p) = &args.source {
        (p.to_string(), read_file(p)?)
    } else {
        ("<stdin>".to_string(), stdin_string()?)
    };

    let mut tree = parse(language, &src)?;
    handle_parse_errors(&path, &tree, &args.on_parse_error);
    if !args.no_verify {
        check_initial_input_is_interesting(&chk, &tree, src.as_bytes())?;
    }

    let reduce_start = Instant::now();
    let mut passes_done = 0;
    let max_passes = passes(&args);
    let mut es: Edits;
    while passes_done < max_passes.unwrap_or(std::usize::MAX) {
        log::info!(
            "Starting pass {} / {}",
            passes_done + 1,
            max_passes.map(|n| n.to_string()).unwrap_or("?".to_string())
        );
        let pass_start = Instant::now();
        (tree, es) = edits(
            &args,
            check(&args),
            &path,
            &src,
            language,
            node_types_json_str,
            min_reduction(&args),
        )?;
        let mut new_src = Vec::new();
        tree_sitter_edit::render(&mut new_src, &tree, src.as_bytes(), &es)?;
        src = std::str::from_utf8(&new_src)?.to_string();
        passes_done += 1;
        log::debug!(
            "Pass {} duration: {}ms",
            passes_done,
            pass_start.elapsed().as_millis()
        );
        if es.is_empty() {
            log::info!("Qutting after pass {} found no reductions", passes_done);
            break;
        }
    }
    log::info!("Total time: {}ms", reduce_start.elapsed().as_millis());
    print_result(&args.output, &src)?;
    Ok(())
}
