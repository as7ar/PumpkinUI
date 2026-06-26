use haven::*;

use crate::theme;

pub fn log_row<'a, State: 'static>(line: &str, app: &mut PaneState) -> View<'a, State> {
    text(id!(), line)
        .font_size(13)
        .fill(theme::TEXT)
        .align(Alignment::Start)
        .build(app)
        .expand_x()
}
