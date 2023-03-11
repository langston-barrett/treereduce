use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tracing::{debug, error, info, warn};
use tracing_subscriber::fmt::format::FmtSpan;
use tree_sitter::Tree;

use crate::check::{Check, CmdCheck};
use crate::edits::Edits;
use crate::original::Original;
use crate::reduce;
use crate::stats;

mod formatter;

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
            warn!(path, "Parse error in {}", path);
        }
        OnParseError::Error => {
            error!(path, "Parse error in {}", path);
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
// TODO(#6): stdout/stderr regex
// TODO(#20): --timeout flag
pub struct Args {
    /// Source code to consume; if empty, parse from stdin
    #[arg(
        short, long, default_value = None, value_name = "FILE")]
    pub source: Option<String>,
    // todo: pathbuf, default_value_os_t
    /// Behavior on parse errors
    #[arg(long, default_value_t = OnParseError::Warn, value_name = "CHOICE")]
    on_parse_error: OnParseError,

    /// Number of threads
    #[arg(short, long, default_value_t = num_cpus::get())]
    pub jobs: usize,

    /// Log messages in JSON format
    #[arg(long, default_value_t = false)]
    pub json: bool,

    /// File to output, use '-' for stdout
    #[arg(short, long, default_value = "treereduce.out")]
    pub output: String,

    /// Print statistics
    #[arg(long, default_value_t = false)]
    pub stats: bool,

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
        error!("Internal error: empty interestingness check!");
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
        error!("Initial test was not interesting. See the usage documentation for help: https://langston-barrett.github.io/treereduce/usage.html");
        std::process::exit(1);
    }
    Ok(())
}

fn edits(
    args: &Args,
    src_path: &str,
    src: &str,
    language: tree_sitter::Language,
    node_types_json_str: &'static str,
    conf: reduce::Config<CmdCheck>,
) -> Result<(Tree, Edits)> {
    debug_assert!(conf.min_reduction > 0);
    let tree = parse(language, src)?;
    handle_parse_errors(src_path, &tree, &args.on_parse_error);
    let node_types = crate::node_types::NodeTypes::new(node_types_json_str)?;
    let orig0 = Original::new(tree, src.as_bytes().to_vec());
    let (orig, edits) = crate::reduce::treereduce(node_types, orig0, conf)?;
    Ok((orig.tree, edits))
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

#[inline]
fn log_tracing_level(level: &log::Level) -> tracing::Level {
    match level {
        log::Level::Trace => tracing::Level::TRACE,
        log::Level::Debug => tracing::Level::DEBUG,
        log::Level::Info => tracing::Level::INFO,
        log::Level::Warn => tracing::Level::WARN,
        log::Level::Error => tracing::Level::ERROR,
    }
}

#[inline]
fn init_tracing(args: &Args) {
    // TODO(lb): Make this less verbose, drop time
    let builder = tracing_subscriber::fmt::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_target(false)
        .with_max_level(log_tracing_level(
            &args.verbose.log_level().unwrap_or(log::Level::Info),
        ));
    if args.json {
        builder.json().init();
    } else {
        builder.event_format(formatter::TerseFormatter).init();
    }
}

#[inline]
fn configure(args: &Args) -> reduce::Config<CmdCheck> {
    reduce::Config {
        check: check(args),
        jobs: args.jobs,
        min_reduction: min_reduction(args),
    }
}

pub fn main(language: tree_sitter::Language, node_types_json_str: &'static str) -> Result<()> {
    let args = Args::parse();
    debug_assert!(args.passes == DEFAULT_NUM_PASSES || !args.stable);
    debug_assert!(!(args.fast && args.slow));

    init_tracing(&args);
    make_temp_dir(&args.temp_dir)?;
    let conf = configure(&args);

    let (path, mut src) = if let Some(p) = &args.source {
        (p.to_string(), read_file(p)?)
    } else {
        ("<stdin>".to_string(), stdin_string()?)
    };

    let mut tree = parse(language, &src)?;
    handle_parse_errors(&path, &tree, &args.on_parse_error);
    if !args.no_verify {
        check_initial_input_is_interesting(&conf.check, &tree, src.as_bytes())?;
    }

    let mut stats = stats::Stats::new();
    stats.start_size = src.len();
    let reduce_start = Instant::now();
    let mut passes_done = 0;
    let max_passes = passes(&args);
    let mut es: Edits;
    while passes_done < max_passes.unwrap_or(std::usize::MAX) {
        let pass_start_size = src.len();
        info!(
            "Starting pass {} / {}",
            passes_done + 1,
            max_passes
                .map(|n| n.to_string())
                .unwrap_or_else(|| "?".to_string())
        );
        let pass_start = Instant::now();

        (tree, es) = edits(
            &args,
            &path,
            &src,
            language,
            node_types_json_str,
            conf.clone(),
        )?;
        let mut new_src = Vec::new();
        tree_sitter_edit::render(&mut new_src, &tree, src.as_bytes(), &es)?;
        src = std::str::from_utf8(&new_src)?.to_string();

        passes_done += 1;
        let pass_stats = stats::Pass {
            duration: pass_start.elapsed(),
            start_size: pass_start_size,
            end_size: src.len(),
        };
        debug!(
            "Pass {} duration: {}ms",
            passes_done,
            pass_stats.duration.as_millis()
        );
        stats.passes.push(pass_stats);

        if es.is_empty() {
            info!("Qutting after pass {} found no reductions", passes_done);
            break;
        }
    }
    stats.duration = reduce_start.elapsed();
    info!("Total time: {}ms", stats.duration.as_millis());
    stats.end_size = src.len();
    print_result(&args.output, &src)?;
    if args.stats {
        // https://nnethercote.github.io/perf-book/io.html#locking
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        stats.write_text(&mut lock)?;
    }
    Ok(())
}
