use std::io;
use std::io::Write;
use std::time::Duration;

use crate::reduce;

pub struct Pass {
    pub duration: Duration,
    pub start_size: usize,
    pub end_size: usize,
    pub reduction_stats: reduce::Stats,
}

impl Pass {
    pub fn write_text(&self, w: &mut impl Write) -> io::Result<()> {
        debug_assert!(self.end_size <= self.start_size);

        writeln!(w, "Duration   : {}s", self.duration.as_secs())?;
        writeln!(w, "Start size : {} bytes", self.start_size)?;
        writeln!(w, "End size   : {} bytes", self.end_size)?;
        writeln!(
            w,
            "Reduction  : {:.2}%",
            100.0 - ((self.end_size as f64 / self.start_size as f64) * 100.0)
        )?;
        writeln!(
            w,
            "Bytes/sec  : {:.2}",
            ((self.start_size - self.end_size) as f64 / self.duration.as_millis() as f64) * 1000.0
        )?;

        // TODO(lb): Make this prettier
        writeln!(w, "Tries:")?;
        for (strat, tries) in self.reduction_stats.tries.iter() {
            writeln!(w, "{} : {}", strat, tries)?;
        }
        writeln!(w, "Retries:")?;
        for (strat, retries) in self.reduction_stats.retries.iter() {
            writeln!(w, "{} : {}", strat, retries)?;
        }
        writeln!(w, "Success:")?;
        for (strat, success) in self.reduction_stats.successes.iter() {
            writeln!(w, "{} : {}", strat, success)?;
        }
        Ok(())
    }
}

pub struct Stats {
    pub duration: Duration,
    pub start_size: usize,
    pub end_size: usize,
    pub passes: Vec<Pass>,
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            duration: Duration::new(0, 0),
            start_size: 0,
            end_size: 0,
            passes: Vec::new(),
        }
    }

    pub fn write_text(&self, w: &mut impl Write) -> io::Result<()> {
        debug_assert!(self.end_size <= self.start_size);

        for (i, pass) in self.passes.iter().enumerate() {
            writeln!(w)?;
            writeln!(w, "Pass {} / {}", i + 1, self.passes.len())?;
            writeln!(w, "------------")?;
            pass.write_text(w)?;
        }

        writeln!(w)?;
        writeln!(w, "Total")?;
        writeln!(w, "-----")?;
        writeln!(w, "Duration   : {}s", self.duration.as_secs())?;
        writeln!(w, "Start size : {} bytes", self.start_size)?;
        writeln!(w, "End size   : {} bytes", self.end_size)?;
        writeln!(
            w,
            "Reduction  : {:.2}%",
            100.0 - ((self.end_size as f64 / self.start_size as f64) * 100.0)
        )?;
        Ok(())
    }
}
