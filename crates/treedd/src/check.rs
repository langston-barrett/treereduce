use std::io;
use std::io::Write;
use std::process::{Command, Output, Stdio};

pub struct Check {
    cmd: String,
    args: Vec<String>,
    exit_codes: Vec<i32>,
    // TODO(lb): stdout/stderr regex
}

impl Check {
    pub fn new(cmd: String, args: Vec<String>, exit_codes: Vec<i32>) -> Self {
        Check {
            cmd,
            args,
            exit_codes,
        }
    }

    fn exec(&self, stdin_bytes: &[u8]) -> io::Result<Output> {
        let mut child = Command::new(&self.cmd)
            .args(&self.args)
            .stdin(Stdio::piped())
            .spawn()?;
        {
            // New block to drop (i.e., close) stdin when done
            let mut sin = child.stdin.take().unwrap(); // TODO(lb): no unwrap
            sin.write_all(stdin_bytes)?;
        }
        child.wait_with_output()
    }

    fn is_interesting(&self, out: &Output) -> bool {
        let code = out.status.code();
        self.exit_codes.iter().any(|c| Some(*c) == code)
    }

    pub fn interesting(&self, stdin: &[u8]) -> io::Result<bool> {
        Ok(self.is_interesting(&self.exec(stdin)?))
    }
}
