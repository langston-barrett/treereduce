use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;

// struct Datum {
//     tool: Tool,
//     tool_version: String,
//     duration: usize,
// }

#[derive(clap::ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum Oracle {
    Clang,
    False,
    True,
}

const FALSE: &str = "/nix/store/i9q0jv6qnvg7zal98rqi7aq31k3p89hw-coreutils-9.0/bin/false";
const TRUE: &str = "/nix/store/i9q0jv6qnvg7zal98rqi7aq31k3p89hw-coreutils-9.0/bin/true";
const DEBUG: bool = true;

impl Oracle {
    fn get(&self, tool: &Tool) -> (String, Vec<String>) {
        match tool {
            Tool::Creduce => match self {
                Oracle::Clang => ("./scripts/clang-creduce.sh".to_string(), vec![]),
                Oracle::False => (FALSE.to_string(), vec![]),
                Oracle::True => (TRUE.to_string(), vec![]),
            },
            Tool::Halfempty => match self {
                Oracle::Clang => ("./scripts/clang.sh".to_string(), vec![]),
                Oracle::True => (TRUE.to_string(), vec![]),
                Oracle::False => (FALSE.to_string(), vec![]),
            },
            Tool::Picireny => match self {
                Oracle::Clang => ("./scripts/clang-picireny.sh".to_string(), vec![]),
                Oracle::False => (FALSE.to_string(), vec![]),
                Oracle::True => (TRUE.to_string(), vec![]),
            },
            Tool::Treereduce => match self {
                Oracle::Clang => (
                    "clang".to_string(),
                    vec![
                        "-o".to_string(),
                        "/dev/null".to_string(),
                        "@@.c".to_string(),
                    ],
                ),
                Oracle::True => (TRUE.to_string(), vec![]),
                Oracle::False => (FALSE.to_string(), vec![]),
            },
        }
    }
}

impl std::fmt::Display for Oracle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Oracle::Clang => write!(f, "clang"),
            Oracle::False => write!(f, "false"),
            Oracle::True => write!(f, "true"),
        }
    }
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum Config {
    Default,
    Fast,
    Slow,
}

impl Config {
    pub fn flags(&self, tool: &Tool) -> Vec<&'static str> {
        match tool {
            Tool::Creduce => match self {
                Config::Default => Vec::new(),
                Config::Fast => Vec::new(),
                Config::Slow => vec!["--sllooww"],
            },
            Tool::Halfempty => match self {
                Config::Default => Vec::new(),
                // 2x default values
                Config::Fast => vec![
                    "--bisect-skip-multiplier=0.0002",
                    "--zero-skip-multiplier=0.0002",
                ],
                Config::Slow => vec!["--stable"],
            },
            Tool::Picireny => match self {
                Config::Default => Vec::new(),
                Config::Fast => Vec::new(),
                Config::Slow => Vec::new(),
            },
            Tool::Treereduce => match self {
                Config::Default => Vec::new(),
                Config::Fast => vec!["--fast"],
                Config::Slow => vec!["--slow"],
            },
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Config::Default => write!(f, "default"),
            Config::Fast => write!(f, "fast"),
            Config::Slow => write!(f, "slow"),
        }
    }
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum Tool {
    Creduce,
    Halfempty,
    Picireny,
    Treereduce,
}

// TODO: Ability to print commands before executing them
impl Tool {
    fn run(
        &self,
        config: &Config,
        jobs: usize,
        in_file: &Path,
        test_bin: &str,
        test_args: Vec<String>,
    ) -> Result<Output> {
        let mut args = config.flags(self);
        let j = &format!("{}", jobs);
        match self {
            Tool::Creduce => {
                assert!(test_args.is_empty());
                args.extend(vec![
                    "--n", j, "--tidy", test_bin,
                    OUT_FILE, // creduce outputs to the input file
                ]);
                Command::new("creduce")
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .context("Failed to spawn c-reduce")?
                    .wait_with_output()
                    .context("Failed to spawn c-reduce")
            }
            Tool::Halfempty => {
                assert!(test_args.is_empty());
                let path = &in_file.to_string_lossy();
                args.extend(vec![
                    "--noverify",
                    "--num-threads",
                    j,
                    "--output",
                    OUT_FILE,
                    test_bin,
                    path,
                ]);
                Command::new("halfempty")
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .context("Failed to spawn halfempty")?
                    .wait_with_output()
                    .context("Failed to spawn halfempty")
            }
            Tool::Picireny => {
                assert!(test_args.is_empty());
                let path = &in_file.to_string_lossy();
                args.extend(vec![
                    "--grammar",
                    "crates/treereduce/examples/assets/C.g4",
                    "--start",
                    "compilationUnit",
                    "--jobs",
                    j,
                    "--output",
                    OUT_FILE,
                    "--test",
                    test_bin,
                    "--input",
                    path,
                ]);
                if jobs > 1 {
                    args.push("--parallel");
                }
                Command::new("picireny")
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .context("Failed to spawn picireny")?
                    .wait_with_output()
                    .context("Failed to spawn picireny")
            }
            Tool::Treereduce => {
                let path = &in_file.to_string_lossy();
                args.extend(vec![
                    "--no-verify",
                    "--jobs",
                    j,
                    "--output",
                    OUT_FILE,
                    "-s",
                    path,
                    &test_bin,
                ]);
                args.extend::<Vec<&str>>(test_args.iter().map(|s| s.as_ref()).collect::<Vec<_>>());
                Command::new("treereduce-c")
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .context("Failed to spawn treereduce-c")?
                    .wait_with_output()
                    .context("Failed to spawn treereduce-c")
            }
        }
    }
}

impl std::fmt::Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tool::Creduce => write!(f, "creduce"),
            Tool::Halfempty => write!(f, "halfempty"),
            Tool::Picireny => write!(f, "picireny"),
            Tool::Treereduce => write!(f, "treereduce"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = Oracle::False)]
    pub oracle: Oracle,

    #[arg(short, long, default_value_t = 1)]
    pub jobs: usize,

    #[arg(long, default_values_t = vec![Tool::Treereduce], value_name = "TOOL")]
    pub tool: Vec<Tool>,

    #[arg(long, default_value_t = String::from("<unknown>"))]
    pub tool_version: String,

    #[arg(long, default_values_t = vec![Config::Default], value_name = "CONF")]
    pub config: Vec<Config>,

    #[arg(long, default_value_t = 1)]
    pub trials: usize,

    #[arg(value_name = "SRC_FILE")]
    pub files: Vec<PathBuf>,
}

// Must have .c name for Clang+C-reduce
const OUT_FILE: &str = "bench.c";

fn run_tool_on_file(args: &Args, conf: &Config, tool: &Tool, file: &Path) -> Result<()> {
    let path_str = file.to_string_lossy();
    let (test_bin, test_args) = args.oracle.get(tool);
    std::fs::copy(file, OUT_FILE)
        .with_context(|| format!("Failed to copy input file {} to {}", path_str, OUT_FILE))?;
    let src = std::fs::read_to_string(file)
        .with_context(|| format!("Failed to read input file {}", path_str))?;
    let start_size = src.len();

    if DEBUG {
        eprintln!("Start:\n{}", src);
    }

    let start = Instant::now();
    let out = tool.run(conf, args.jobs, file, &test_bin, test_args.clone())?;
    if !out.status.success() {
        eprintln!(
            "Tool failed\nstdout: {}\nstderr: {}",
            std::str::from_utf8(&out.stdout).unwrap_or("<not UTF-8>"),
            std::str::from_utf8(&out.stderr).unwrap_or("<not UTF-8>"),
        );
        return Ok(());
    }
    let duration = start.elapsed();
    let result = std::fs::read_to_string(OUT_FILE)
        .with_context(|| format!("Failed to read output file {}", "out"))?;
    if DEBUG {
        eprintln!("Result:\n{}", result);
    }
    std::fs::remove_file(OUT_FILE)?;
    let end_size = result.len();
    eprintln!(
        "{},{},{},{},{},{},{},{}",
        tool,
        args.tool_version,
        args.oracle,
        conf,
        file.file_name().map(|s| s.to_str().unwrap()).unwrap(),
        start_size,
        end_size,
        duration.as_millis()
    );
    Ok(())
}

fn main() -> Result<()> {
    // TODO(lb): error if out file already exists
    let args = Args::parse();
    if args.jobs == 0 {
        eprintln!("Jobs must be greater than 0.");
        return Ok(());
    }
    for conf in &args.config {
        for tool in &args.tool {
            for file in &args.files {
                for _ in 0..args.trials {
                    run_tool_on_file(&args, conf, tool, file)?;
                }
            }
        }
    }
    Ok(())
}
