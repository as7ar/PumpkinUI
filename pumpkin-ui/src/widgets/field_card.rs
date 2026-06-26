use haven::*;

use crate::theme;
use crate::widgets::card;

pub fn field_card<'a, State: 'static>(
    title: &'static str,
    field: View<'a, State>,
    app: &mut PaneState,
) -> View<'a, State> {
    stack(vec![
        card(app),
        column_spaced(
            10.,
            vec![
                text(id!(), title)
                    .font_size(13)
                    .font_weight(FontWeight::BOLD)
                    .fill(theme::MUTED_TEXT)
                    .align(Alignment::Start)
                    .build(app),
                field,
            ],
        )
        .pad(14.),
    ])
}
