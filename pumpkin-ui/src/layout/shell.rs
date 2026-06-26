use haven::*;

use crate::pages;
use crate::state::AppState;
use crate::theme;
use crate::widgets;

pub fn app_shell<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    stack(vec![
        rect(id!()).fill(theme::BACKGROUND).build(app).expand(),
        scroller(
            id!(),
            None,
            move |index, _, ctx| {
                if index != 0 {
                    return None;
                }

                Some(
                    row_spaced(
                        18.,
                        vec![
                            sidebar(state, ctx).width(320.),
                            pages::dashboard_body(state, ctx).expand(),
                        ],
                    )
                    .pad(20.)
                    .align(Align::Top),
                )
            },
            app,
        )
        .expand(),
    ])
}

fn sidebar<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    stack(vec![
        widgets::card(app),
        column_spaced(
            14.,
            vec![
                widgets::section_title("Pumpkin", Some("Minecraft server control panel"), app),
                status_summary(state, app),
                actions(state, app),
                config_summary(state, app),
            ],
        )
        .pad(16.),
    ])
}

fn status_summary<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    let snapshot = state.controller.status();
    column_spaced(
        10.,
        vec![
            widgets::status_badge(snapshot.status, None, app),
            text(id!(), state.status_message.as_str())
                .font_size(13)
                .fill(theme::MUTED_TEXT)
                .align(Alignment::Start)
                .build(app),
            if let Some(error) = state.error_message.as_deref() {
                text(id!(), error)
                    .font_size(12)
                    .fill(theme::DANGER)
                    .wrap()
                    .align(Alignment::Start)
                    .build(app)
            } else {
                empty()
            },
        ],
    )
}

fn actions<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    row_spaced(
        10.,
        vec![
            button(id!(), binding!(state.start_button))
                .text_label("Start")
                .on_click(|state, app| state.start_server(app))
                .build(app)
                .expand_x()
                .height(38.),
            button(id!(), binding!(state.stop_button))
                .text_label("Stop")
                .on_click(|state, app| state.stop_server(app))
                .build(app)
                .expand_x()
                .height(38.),
        ],
    )
}

fn config_summary<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    let config_path = state.controller.config_path();
    column_spaced(
        10.,
        vec![
            text(id!(), "Config file")
                .font_size(13)
                .font_weight(FontWeight::BOLD)
                .fill(theme::MUTED_TEXT)
                .align(Alignment::Start)
                .build(app),
            text(id!(), config_path.to_string_lossy().as_ref())
                .font_size(12)
                .fill(theme::TEXT)
                .wrap()
                .align(Alignment::Start)
                .build(app),
        ],
    )
}
