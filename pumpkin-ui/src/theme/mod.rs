use haven::Color;
use pumpkin_core::ServerStatus;

pub const BACKGROUND: Color = Color::from_rgb8(18, 20, 26);
pub const SURFACE: Color = Color::from_rgb8(26, 28, 36);
pub const SURFACE_ALT: Color = Color::from_rgb8(32, 35, 44);
pub const BORDER: Color = Color::from_rgb8(52, 56, 68);
pub const TEXT: Color = Color::from_rgb8(235, 238, 244);
pub const MUTED_TEXT: Color = Color::from_rgb8(160, 167, 179);
pub const ACCENT: Color = Color::from_rgb8(110, 146, 255);
pub const SUCCESS: Color = Color::from_rgb8(88, 196, 126);
pub const WARNING: Color = Color::from_rgb8(230, 178, 74);
pub const DANGER: Color = Color::from_rgb8(240, 90, 90);

pub fn status_color(status: ServerStatus) -> Color {
    match status {
        ServerStatus::Stopped => MUTED_TEXT,
        ServerStatus::Starting => WARNING,
        ServerStatus::Running => SUCCESS,
        ServerStatus::Stopping => WARNING,
        ServerStatus::Failed => DANGER,
    }
}
