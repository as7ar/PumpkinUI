use crate::pages;
use crate::state::AppState;
use haven::PaneBuilder;
use haven::winit::WinitApp;

pub fn run() {
    WinitApp::new(AppState::new())
        .pane(
            PaneBuilder::new("main", pages::dashboard)
                .title("Pumpkin Server Manager")
                .inner_size(1280, 860),
        )
        .run();
}
