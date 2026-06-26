use haven::*;
use pumpkin_core::{ServerLogLine, ServerStream};

use crate::theme;

pub fn log_row<'a, State: 'static>(line: &ServerLogLine, app: &mut PaneState) -> View<'a, State> {
    let (level, message) = split_log_level(&line.line);
    let stream_color = match line.stream {
        ServerStream::Stdout => theme::MUTED_TEXT,
        ServerStream::Stderr => theme::DANGER,
    };
    let level_color = match level {
        Some("[ERROR]") => theme::DANGER,
        Some("[WARN]") => theme::WARNING,
        Some("[INFO]") => theme::ACCENT,
        Some(_) | None => theme::TEXT,
    };

    let mut spans = vec![
        span(match line.stream {
            ServerStream::Stdout => "[stdout]",
            ServerStream::Stderr => "[stderr]",
        })
        .color(stream_color)
        .weight(FontWeight::BOLD),
        span(" ").color(theme::MUTED_TEXT),
    ];

    if let Some(level) = level {
        spans.push(span(level).color(level_color).weight(FontWeight::BOLD));
        if !message.is_empty() {
            spans.push(span(" ").color(theme::MUTED_TEXT));
            spans.push(span(message).color(theme::TEXT));
        }
    } else {
        spans.push(span(&line.line).color(theme::TEXT));
    }

    rich_text(id!(), spans)
        .font_family(theme::terminal_font_family())
        .font_size(13)
        .align(Alignment::Start)
        .build(app)
        .expand_x()
}

fn split_log_level(line: &str) -> (Option<&str>, &str) {
    for level in ["[ERROR]", "[WARN]", "[INFO]", "[DEBUG]", "[TRACE]"] {
        if let Some(rest) = line.strip_prefix(level) {
            return (Some(level), rest.trim_start());
        }
    }

    (None, line)
}
