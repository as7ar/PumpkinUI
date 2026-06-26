use haven::*;
use pumpkin_core::ServerStatus;

use crate::theme;

pub fn status_badge<'a, State: 'static>(
    status: ServerStatus,
    extra: Option<String>,
    app: &mut PaneState,
) -> View<'a, State> {
    let label = match extra {
        Some(extra) => format!("{} · {}", status.label(), extra),
        None => status.label().to_string(),
    };

    stack(vec![
        rect(id!())
            .fill(theme::status_color(status).with_alpha(0.16))
            .stroke(theme::status_color(status), Stroke::new(1.))
            .corner_rounding(999.)
            .build(app),
        text(id!(), label)
            .font_size(13)
            .font_weight(FontWeight::BOLD)
            .fill(theme::status_color(status))
            .build(app)
            .pad_x(14.)
            .pad_y(7.),
    ])
    .height(30.)
}
