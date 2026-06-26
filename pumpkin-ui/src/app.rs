use crate::pages;
use crate::state::AppState;
use haven::PaneBuilder;
use haven::winit::WinitApp;

pub fn run() {
    WinitApp::new(AppState::new())
        .pane(
            PaneBuilder::new("main", pages::dashboard)
                .title("Pumpkin Server UI")
                .on_start(|state, app| state.on_start(app))
                .on_wake(|state, app| state.on_wake(app))
                .inner_size(1280, 860),
        )
        .run();
}
