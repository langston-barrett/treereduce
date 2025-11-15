use std::io::Write;
use std::io::{self, Read};
#[cfg(target_family = "unix")]
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::Duration;

use regex::Regex;
use tempfile::NamedTempFile;
use tracing::debug;
use wait_timeout::ChildExt;

pub trait Check {
    type State;

    fn start(&self, stdin: &[u8]) -> io::Result<Self::State>;

    fn cancel(&self, state: Self::State) -> io::Result<()>;

    fn try_wait(&self, state: &mut Self::State) -> io::Result<Option<bool>>;

    fn wait(&self, state: Self::State) -> io::Result<bool>;

    fn interesting(&self, stdin: &[u8]) -> io::Result<bool> {
        self.wait(self.start(stdin)?)
    }
}

#[derive(Clone, Debug)]
pub struct CmdCheck {
    pub(crate) cmd: String,
    pub(crate) args: Vec<String>,
    exit_codes: Vec<i32>,
    interesting_stderr: Option<Regex>,
    interesting_stdout: Option<Regex>,
    uninteresting_stderr: Option<Regex>,
    uninteresting_stdout: Option<Regex>,
    temp_dir: PathBuf,
    pub(crate) needs_file: bool,
    inherit_stdout: bool,
    inherit_stderr: bool,
    timeout: Option<Duration>,
    // TODO(#6): stdout/stderr regex
    // Will interact poorly with try_wait...
}

#[derive(Debug)]
pub struct CmdCheckState {
    child: Child,
    temp_file: Option<NamedTempFile>,
}

fn is_marker(s: &str) -> bool {
    s.starts_with("@@")
}

impl CmdCheck {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cmd: String,
        args: Vec<String>,
        exit_codes: Vec<i32>,
        temp_dir: Option<String>,
        interesting_stdout: Option<Regex>,
        interesting_stderr: Option<Regex>,
        uninteresting_stdout: Option<Regex>,
        uninteresting_stderr: Option<Regex>,
        inherit_stdout: bool,
        inherit_stderr: bool,
        timeout: Option<Duration>,
    ) -> Self {
        let temp_dir_path: Option<PathBuf> = temp_dir.as_ref().map(From::from);
        CmdCheck {
            needs_file: args.iter().any(|s| is_marker(s)),
            temp_dir: temp_dir_path.unwrap_or_else(std::env::temp_dir),
            cmd,
            args,
            exit_codes,
            interesting_stdout,
            interesting_stderr,
            uninteresting_stdout,
            uninteresting_stderr,
            inherit_stdout,
            inherit_stderr,
            timeout,
        }
    }

    fn temp_file(&self, marker: &str) -> io::Result<NamedTempFile> {
        debug_assert!(is_marker(marker));
        let mut builder = tempfile::Builder::new();
        if marker.len() > "@@".len() {
            let mut chars = marker.chars();
            let one = chars.next();
            debug_assert!(one == Some('@'));
            let two = chars.next();
            debug_assert!(two == Some('@'));
            let rest: String = chars.collect();
            builder
                .prefix("treereduce-tmp-")
                .suffix(&rest)
                .tempfile_in(&self.temp_dir)
        } else {
            builder.tempfile_in(&self.temp_dir)
        }
    }

    /// Replace `@@` with the path to a temporary file
    pub(crate) fn args_with_file(&self) -> io::Result<(Option<NamedTempFile>, Vec<String>)> {
        debug_assert!(self.needs_file);
        let mut found = false;
        let mut temp_file = None;
        let mut args = Vec::new();
        for arg in &self.args {
            if is_marker(arg) {
                debug_assert!(!found);
                found = true;
                let f = self.temp_file(arg)?;
                // TODO(lb): No expect
                args.push(f.path().to_str().expect("Path not valid UTF-8").to_string());
                temp_file = Some(f);
            } else {
                args.push(arg.clone());
            }
        }
        debug_assert!(found);
        Ok((temp_file, args))
    }

    fn exec(&self, stdin_bytes: &[u8]) -> io::Result<CmdCheckState> {
        let (mut temp_file, args) = if self.needs_file {
            self.args_with_file()?
        } else {
            (None, self.args.clone())
        };

        let child = if let Some(tf) = &mut temp_file {
            tf.write_all(stdin_bytes)?;
            Command::new(&self.cmd)
                .args(args)
                .stdin(Stdio::piped())
                .stdout(if self.inherit_stdout {
                    Stdio::inherit()
                } else if self.interesting_stdout.is_none() {
                    Stdio::null()
                } else {
                    Stdio::piped()
                })
                .stderr(if self.inherit_stderr {
                    Stdio::inherit()
                } else if self.interesting_stderr.is_some() {
                    Stdio::piped()
                } else {
                    Stdio::null()
                })
                .spawn()?
        } else {
            let mut child = Command::new(&self.cmd)
                .args(args)
                .stdin(Stdio::piped())
                .stdout(if self.inherit_stdout {
                    Stdio::inherit()
                } else if self.interesting_stdout.is_some() {
                    Stdio::piped()
                } else {
                    Stdio::null()
                })
                .stderr(if self.inherit_stderr {
                    Stdio::inherit()
                } else if self.interesting_stderr.is_some() {
                    Stdio::piped()
                } else {
                    Stdio::null()
                })
                .spawn()?;
            {
                // New block to drop (i.e., close) stdin when done
                let mut sin = child.stdin.take().unwrap(); // TODO(lb): no unwrap
                sin.write_all(stdin_bytes)?;
            }
            child
        };
        Ok(CmdCheckState { child, temp_file })
    }

    fn is_interesting(
        &self,
        status: ExitStatus,
        stdout: Option<impl Read>,
        stderr: Option<impl Read>,
    ) -> (bool, Vec<u8>, Vec<u8>) {
        #[cfg(not(target_family = "unix"))]
        let code = status.code();
        #[cfg(target_family = "unix")]
        let code = status.code().or_else(|| status.signal().map(|c| c + 128));
        let mut stdout_bytes = Vec::new();
        let mut stderr_bytes = Vec::new();
        if let Some(mut out) = stdout {
            out.read_to_end(&mut stdout_bytes).unwrap();
        }
        if let Some(mut err) = stderr {
            err.read_to_end(&mut stderr_bytes).unwrap();
        }
        let out_str = String::from_utf8_lossy(&stdout_bytes);
        let err_str = String::from_utf8_lossy(&stderr_bytes);
        let interesting_code =
            !self.exit_codes.is_empty() && self.exit_codes.iter().any(|c| Some(*c) == code);
        let stdout_match = self
            .interesting_stdout
            .as_ref()
            .is_some_and(|rx| rx.is_match(&out_str));
        let stderr_match = self
            .interesting_stderr
            .as_ref()
            .is_some_and(|rx| rx.is_match(&err_str));
        let stdout_unmatch = self
            .uninteresting_stdout
            .as_ref()
            .is_some_and(|rx| rx.is_match(&out_str));
        let stderr_unmatch = self
            .uninteresting_stderr
            .as_ref()
            .is_some_and(|rx| rx.is_match(&err_str));
        let is_interesting = (interesting_code || stdout_match || stderr_match)
            && !stdout_unmatch
            && !stderr_unmatch;
        debug!(
            interesting_code,
            stdout_match,
            stderr_match,
            stdout_unmatch,
            stderr_unmatch,
            is_interesting,
            "Interesting? {}",
            is_interesting
        );
        (is_interesting, stdout_bytes, stderr_bytes)
    }

    #[allow(clippy::type_complexity)]
    pub fn wait_with_output(
        &self,
        mut state: CmdCheckState,
    ) -> io::Result<(bool, Option<ExitStatus>, Vec<u8>, Vec<u8>)> {
        let status = if let Some(to) = self.timeout {
            if let Some(s) = state.child.wait_timeout(to)? {
                s
            } else {
                state.child.kill()?;
                return Ok((false, None, Vec::new(), Vec::new())); // timeout
            }
        } else {
            state.child.wait()?
        };
        let (b, o, e) = self.is_interesting(status, state.child.stdout, state.child.stderr);
        Ok((b, Some(status), o, e))
    }
}

impl Check for CmdCheck {
    type State = CmdCheckState;

    fn start(&self, stdin: &[u8]) -> io::Result<Self::State> {
        self.exec(stdin)
    }

    fn cancel(&self, mut state: Self::State) -> io::Result<()> {
        state.child.kill()?;
        if let Some(tf) = state.temp_file {
            tf.close()?;
        }
        Ok(())
    }

    fn try_wait(&self, state: &mut Self::State) -> io::Result<Option<bool>> {
        let mut stdout_bytes = Vec::new();
        let mut stderr_bytes = Vec::new();
        if let Some(ref mut out) = &mut state.child.stdout {
            out.read_to_end(&mut stdout_bytes)?;
        }
        if let Some(ref mut err) = &mut state.child.stderr {
            err.read_to_end(&mut stderr_bytes)?;
        }
        Ok(state.child.try_wait()?.map(|s| {
            let (b, _, _) = self.is_interesting(
                s,
                Some(stdout_bytes.as_slice()),
                Some(stderr_bytes.as_slice()),
            );
            b
        }))
    }

    fn wait(&self, state: Self::State) -> io::Result<bool> {
        Ok(self.wait_with_output(state)?.0)
    }
}
