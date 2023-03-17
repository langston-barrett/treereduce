use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::process;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{ArgGroup, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use regex::Regex;
use tracing::{error, warn};
use tracing_subscriber::fmt::format::FmtSpan;
use tree_sitter::Tree;

use crate::check::{Check, CmdCheck};
use crate::original::Original;
use crate::reduce;

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

#[allow(clippy::derivable_impls)]
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

    /// Regex to match interesting stdout
    #[arg(
        help_heading = "Interestingness check options",
        long,
        value_name = "REGEX"
    )]
    interesting_stdout: Option<String>,

    /// Regex to match interesting stderr
    #[arg(
        help_heading = "Interestingness check options",
        long,
        value_name = "REGEX"
    )]
    interesting_stderr: Option<String>,

    /// Regex to match *uninteresting* stdout, overrides interesting regex
    #[arg(
        help_heading = "Interestingness check options",
        long,
        value_name = "REGEX",
        requires = "interesting_stdout"
    )]
    uninteresting_stdout: Option<String>,

    /// Regex to match *uninteresting* stderr, overrides interesting regex
    #[arg(
        help_heading = "Interestingness check options",
        long,
        value_name = "REGEX",
        requires = "interesting_stderr"
    )]
    uninteresting_stderr: Option<String>,

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

    /// Timeout for the interestingness check (seconds)
    #[arg(
        help_heading = "Interestingness check options",
        long,
        default_value = None,
        value_name = "SECS"
    )]
    pub timeout: Option<u64>,

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

fn check(args: &Args) -> Result<CmdCheck> {
    if args.check.is_empty() {
        error!("Internal error: empty interestingness check!");
        std::process::exit(1);
    }
    let mut argv: Vec<_> = args.check.iter().collect();
    let cmd = argv[0];
    argv.remove(0);
    let stdout_regex = match &args.interesting_stdout {
        Some(r) => Some(Regex::new(r).context("Invalid interesting stdout regex")?),
        None => None,
    };
    let stderr_regex = match &args.interesting_stderr {
        Some(r) => Some(Regex::new(r).context("Invalid interesting stderr regex")?),
        None => None,
    };
    let un_stdout_regex = match &args.uninteresting_stdout {
        Some(r) => Some(Regex::new(r).context("Invalid uninteresting stdout regex")?),
        None => None,
    };
    let un_stderr_regex = match &args.uninteresting_stderr {
        Some(r) => Some(Regex::new(r).context("Invalid uninteresting stderr regex")?),
        None => None,
    };
    Ok(CmdCheck::new(
        cmd.to_string(),
        argv.iter().map(|s| s.to_string()).collect(),
        args.interesting_exit_code.clone(),
        args.temp_dir.clone(),
        stdout_regex,
        stderr_regex,
        un_stdout_regex,
        un_stderr_regex,
        args.inherit_stdout,
        args.inherit_stderr,
        args.timeout.map(Duration::from_secs),
    ))
}

fn check_initial_input_is_interesting(
    chk: &CmdCheck,
    tree: &Tree,
    src: &[u8],
    source: &Option<String>,
) -> Result<()> {
    let mut test: Vec<u8> = Vec::new();
    test.reserve(src.len());
    tree_sitter_edit::render(&mut test, tree, src, &crate::edits::Edits::new())?;
    if !chk
        .interesting(&test)
        .context("Failed to check that initial input was interesting")?
    {
        let (tmp_file, command_line) = if chk.needs_file {
            chk.args_with_file()?
        } else {
            (None, chk.args.clone())
        };
        let tmp_path = match tmp_file {
            Some(t) => String::from(t.path().to_string_lossy()),
            None => String::from("${tmp}/your-test-case"),
        };
        let mut args = command_line
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>();
        if !chk.needs_file {
            args.push("<".to_string());
            args.push(tmp_path.clone());
        }
        let s = format!(
            r#"Initial test was not interesting. Try the following:

    tmp="$(mktemp -d)"
    cp {} "{tmp_path}"
    cd "${{tmp}}"
    {} {}
    echo $?

The last line should print 0 (or any other code passed to `--interesting-exit-code`). See the usage documentation for help: https://langston-barrett.github.io/treereduce/usage.html"#,
            source
                .clone()
                .unwrap_or_else(|| String::from("your-test-case")),
            chk.cmd,
            args.join(" "),
            tmp_path = tmp_path,
        );
        eprintln!("{}", s);
        error!(s);
        std::process::exit(1);
    }
    Ok(())
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
fn configure(
    args: &Args,
    replacements: HashMap<&'static str, &'static [&'static str]>,
) -> Result<reduce::Config<CmdCheck>> {
    Ok(reduce::Config {
        check: check(args)?,
        jobs: args.jobs,
        min_reduction: min_reduction(args),
        replacements,
    })
}

pub fn main(
    language: tree_sitter::Language,
    node_types_json_str: &'static str,
    replacements: HashMap<&'static str, &'static [&'static str]>,
) -> Result<()> {
    let args = Args::parse();
    debug_assert!(args.passes == DEFAULT_NUM_PASSES || !args.stable);
    debug_assert!(!(args.fast && args.slow));
    debug_assert!(args.uninteresting_stdout.is_some() && args.interesting_stdout.is_none());
    debug_assert!(args.uninteresting_stderr.is_some() && args.interesting_stderr.is_none());

    init_tracing(&args);
    make_temp_dir(&args.temp_dir)?;
    let conf = configure(&args, replacements)?;

    let (path, src) = if let Some(p) = &args.source {
        (p.to_string(), read_file(p)?)
    } else {
        ("<stdin>".to_string(), stdin_string()?)
    };

    let tree = parse(language, &src)?;
    handle_parse_errors(&path, &tree, &args.on_parse_error);
    if !args.no_verify {
        check_initial_input_is_interesting(&conf.check, &tree, src.as_bytes(), &args.source)?;
    }

    let max_passes = passes(&args);
    let node_types = crate::node_types::NodeTypes::new(node_types_json_str)?;
    let orig = Original::new(tree, src.into_bytes());
    let (reduced, stats) =
        reduce::treereduce_multi_pass(language, node_types, orig, conf, max_passes)?;
    let text = std::str::from_utf8(&reduced.text)?.to_string();
    print_result(&args.output, &text)?;

    if args.stats {
        // https://nnethercote.github.io/perf-book/io.html#locking
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        stats.write_text(&mut lock)?;
    }
    Ok(())
}
