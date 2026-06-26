use haven::*;

use crate::theme;

pub fn section_title<'a, State: 'static>(
    title: &'static str,
    subtitle: Option<&'static str>,
    app: &mut PaneState,
) -> View<'a, State> {
    let mut children = vec![
        text(id!(), title)
            .font_size(22)
            .font_weight(FontWeight::BOLD)
            .fill(theme::TEXT)
            .align(Alignment::Start)
            .build(app),
    ];

    if let Some(subtitle) = subtitle {
        children.push(
            text(id!(), subtitle)
                .font_size(13)
                .fill(theme::MUTED_TEXT)
                .align(Alignment::Start)
                .build(app),
        );
    }

    column_spaced(6., children)
}
