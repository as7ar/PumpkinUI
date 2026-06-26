use haven::*;

use crate::theme;

pub fn card<'a, State: 'static>(app: &mut PaneState) -> View<'a, State> {
    rect(id!())
        .fill(theme::SURFACE)
        .stroke(theme::BORDER, Stroke::new(1.))
        .corner_rounding(14.)
        .build(app)
}
