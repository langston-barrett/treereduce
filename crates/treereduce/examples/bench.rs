use std::fmt::Debug;
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

impl Oracle {
    fn get(&self) -> (String, Vec<String>) {
        match self {
            Oracle::Clang => ("./scripts/clang.sh".to_string(), vec![]),
            Oracle::True => ("true".to_string(), vec![]),
            Oracle::False => (
                "/nix/store/i9q0jv6qnvg7zal98rqi7aq31k3p89hw-coreutils-9.0/bin/false".to_string(),
                vec![],
            ),
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
pub enum Tool {
    Creduce,
    Halfempty,
    Treereduce,
}

impl Tool {
    fn run(
        &self,
        jobs: usize,
        in_file: &str,
        test_bin: &str,
        test_args: Vec<String>,
    ) -> Result<Output> {
        match self {
            Tool::Creduce => {
                assert!(test_args.is_empty());
                let j = &format!("{}", jobs);
                let args = vec![
                    "--n", j, "--tidy", test_bin,
                    OUT_FILE, // creduce outputs to the input file
                ];
                Command::new("creduce")
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .env("CREDUCE", in_file)
                    .spawn()
                    .context("Failed to spawn c-reduce")?
                    .wait_with_output()
                    .context("Failed to spawn c-reduce")
            }
            Tool::Halfempty => {
                assert!(test_args.is_empty());
                let j = &format!("{}", jobs);
                let args = vec![
                    "--noverify",
                    "--num-threads",
                    j,
                    "--output",
                    OUT_FILE,
                    test_bin,
                    in_file,
                ];
                Command::new("halfempty")
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .env("HALFEMPTY", "1")
                    .spawn()
                    .context("Failed to spawn halfempty")?
                    .wait_with_output()
                    .context("Failed to spawn halfempty")
            }
            Tool::Treereduce => {
                let j = &format!("{}", jobs);
                let mut args = vec![
                    "--no-verify",
                    "--jobs",
                    j,
                    "--output",
                    OUT_FILE,
                    "-s",
                    in_file,
                    &test_bin,
                ];
                args.extend::<Vec<&str>>(test_args.iter().map(|s| s.as_ref()).collect::<Vec<_>>());
                Command::new("treereduce-c")
                    .args(args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .env("TREEREDUCE", "1")
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
            Tool::Treereduce => write!(f, "treereduce"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = Oracle::False)]
    pub oracle: Oracle,

    #[arg(short, long, default_value_t = Tool::Treereduce)]
    pub tool: Tool,

    #[arg(short, long, default_value_t = 1)]
    pub jobs: usize,

    #[arg(long, default_value_t = String::from("<unknown>"))]
    pub tool_version: String,

    #[arg(long, default_value_t = 1)]
    pub trials: usize,

    #[arg(value_name = "SRC_FILE")]
    pub files: Vec<String>,
}

const OUT_FILE: &str = "bench.out";

fn main() -> Result<()> {
    // TODO(lb): error if out file already exists
    let args = Args::parse();
    if args.jobs == 0 {
        eprintln!("Jobs must be greater than 0.");
        return Ok(());
    }
    let (test_bin, test_args) = args.oracle.get();
    for file in args.files {
        for _ in 0..args.trials {
            std::fs::copy(&file, OUT_FILE)
                .with_context(|| format!("Failed to copy input file {} to {}", &file, OUT_FILE))?;
            let src = std::fs::read_to_string(&file)
                .with_context(|| format!("Failed to read input file {}", &file))?;
            let start_size = src.len();
            let start = Instant::now();
            let out = args
                .tool
                .run(args.jobs, &file, &test_bin, test_args.clone())?;
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
            std::fs::remove_file(OUT_FILE)?;
            let end_size = result.len();
            eprintln!(
                "{},{},{},{},{},{}",
                args.tool,
                args.tool_version,
                args.oracle,
                start_size,
                end_size,
                duration.as_millis()
            )
        }
    }
    Ok(())
}
