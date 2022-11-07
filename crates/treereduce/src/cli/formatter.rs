use std::fmt;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::{
    format::{self, FormatEvent, FormatFields},
    FmtContext,
};

use nu_ansi_term::{Color, Style};
use tracing_subscriber::registry::LookupSpan;

pub struct TerseFormatter;

fn style_for(level: &Level) -> Style {
    match *level {
        Level::TRACE => Style::new().fg(Color::Purple),
        Level::DEBUG => Style::new().fg(Color::Blue),
        Level::INFO => Style::new().fg(Color::Green),
        Level::WARN => Style::new().fg(Color::Yellow),
        Level::ERROR => Style::new().fg(Color::Red),
    }
}

impl<S, N> FormatEvent<S, N> for TerseFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: format::Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let metadata = event.metadata();
        let level = metadata.level();
        let style = style_for(level);
        for field in event.fields() {
            // Only print events with human-readable messages
            if field.name() == "message" {
                // TODO(lb): Pad level to 5 places
                // TODO(lb): Don't print all the danged fields
                write!(&mut writer, "[{}] ", style.paint(format!("{}", level)))?;
                ctx.field_format().format_fields(writer.by_ref(), event)?;
            }
        }
        writeln!(writer)
    }
}
