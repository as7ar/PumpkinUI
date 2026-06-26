use haven::*;

use crate::layout;
use crate::state::AppState;
use crate::theme;
use crate::widgets;

pub fn dashboard<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    layout::app_shell(state, app)
}

pub fn dashboard_body<'a>(state: &'a AppState, app: &'a mut PaneState) -> View<'a, AppState> {
    let content = column_spaced(
        18.,
        vec![
            widgets::card(app),
            column_spaced(
                16.,
                vec![
                    header(state, app),
                    config_section(state, app),
                    runtime_section(state, app),
                    log_section(state, app),
                ],
            )
            .pad(16.),
        ],
    );

    scroller(
        id!(),
        None,
        move |index, _, _ctx| {
            if index != 0 {
                return None;
            }

            Some(content.clone().expand())
        },
        app,
    )
}

fn header<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    let snapshot = state.controller.status();
    row_spaced(
        12.,
        vec![
            column_spaced(
                6.,
                vec![
                    text(id!(), "Pumpkin Server UI")
                        .font_size(28)
                        .font_weight(FontWeight::BOLD)
                        .fill(theme::TEXT)
                        .align(Alignment::Start)
                        .build(app),
                    text(id!(), "Control the server process")
                        .font_size(13)
                        .fill(theme::MUTED_TEXT)
                        .align(Alignment::Start)
                        .build(app),
                ],
            )
            .expand_x(),
            widgets::status_badge(
                snapshot.status,
                snapshot.process_id.map(|pid| format!("pid {}", pid)),
                app,
            ),
        ],
    )
}

fn config_section<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    column_spaced(
        12.,
        vec![
            widgets::section_title(
                "Configuration",
                Some("Edit the pumpkin-ui.toml values used by the core"),
                app,
            ),
            row_spaced(
                12.,
                vec![
                    widgets::field_card(
                        "Server directory",
                        text_field(id!(), binding!(state.server_directory))
                            .hint_text("Server working directory")
                            .align(Alignment::Start)
                            .build(app)
                            .height(42.),
                        app,
                    )
                    .expand_x(),
                    widgets::field_card(
                        "Executable",
                        text_field(id!(), binding!(state.executable))
                            .hint_text(default_executable_hint())
                            .align(Alignment::Start)
                            .build(app)
                            .height(42.),
                        app,
                    )
                    .expand_x(),
                ],
            ),
            widgets::field_card(
                "Arguments",
                text_field(id!(), binding!(state.arguments))
                    .hint_text("Optional launch arguments")
                    .align(Alignment::Start)
                    .build(app)
                    .height(42.),
                app,
            ),
            row_spaced(
                10.,
                vec![
                    button(id!(), binding!(state.save_button))
                        .text_label("Save config")
                        .on_click(|state, app| state.save_config(app))
                        .build(app)
                        .height(38.)
                        .width(132.),
                    button(id!(), binding!(state.refresh_button))
                        .text_label("Refresh")
                        .on_click(|state, app| state.reload_logs(app))
                        .build(app)
                        .height(38.)
                        .width(102.),
                ],
            ),
        ],
    )
}

fn runtime_section<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    let snapshot = state.controller.status();
    column_spaced(
        12.,
        vec![
            widgets::section_title("Runtime", Some("Current process state"), app),
            row_spaced(
                12.,
                vec![
                    widgets::status_badge(snapshot.status, None, app).width(160.),
                    text(id!(), state.status_message.as_str())
                        .font_size(13)
                        .fill(theme::TEXT)
                        .align(Alignment::Start)
                        .build(app)
                        .expand_x(),
                ],
            ),
            if let Some(error) = state.error_message.as_deref() {
                text(id!(), error)
                    .font_size(13)
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

fn log_section<'a>(state: &'a AppState, app: &mut PaneState) -> View<'a, AppState> {
    let logs = state.logs.clone();
    column_spaced(
        12.,
        vec![
            widgets::section_title("Logs", Some("stdout and stderr from the process"), app),
            stack(vec![
                rect(id!())
                    .fill(theme::SURFACE_ALT)
                    .stroke(theme::BORDER, Stroke::new(1.))
                    .corner_rounding(10.)
                    .build(app),
                scroller(
                    id!(),
                    None,
                    move |index, _, ctx| logs.get(index).map(|line| widgets::log_row(line, ctx)),
                    app,
                )
                .pad(12.),
            ])
            .height(320.),
        ],
    )
}

fn default_executable_hint() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "pumpkin-X64-Windows.exe"
    }

    #[cfg(target_os = "macos")]
    {
        "pumpkin-X64-macOS"
    }

    #[cfg(target_os = "linux")]
    {
        "pumpkin-X64-Linux"
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "pumpkin-X64-Windows.exe"
    }
}
