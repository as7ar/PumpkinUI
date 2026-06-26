use haven::Color;
use pumpkin_core::ServerStatus;

pub const BACKGROUND: Color = Color::from_rgb8(24, 16, 10);
pub const SURFACE: Color = Color::from_rgb8(34, 24, 17);
pub const SURFACE_ALT: Color = Color::from_rgb8(44, 31, 21);
pub const BORDER: Color = Color::from_rgb8(86, 57, 36);
pub const TEXT: Color = Color::from_rgb8(247, 241, 234);
pub const MUTED_TEXT: Color = Color::from_rgb8(196, 170, 150);
pub const ACCENT: Color = Color::from_rgb8(245, 141, 56);
pub const SUCCESS: Color = Color::from_rgb8(113, 197, 121);
pub const WARNING: Color = Color::from_rgb8(245, 177, 74);
pub const DANGER: Color = Color::from_rgb8(241, 99, 71);

pub fn terminal_font_family() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "Consolas"
    }

    #[cfg(target_os = "macos")]
    {
        "Menlo"
    }

    #[cfg(target_os = "linux")]
    {
        "monospace"
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "monospace"
    }
}

pub fn status_color(status: ServerStatus) -> Color {
    match status {
        ServerStatus::Stopped => MUTED_TEXT,
        ServerStatus::Starting => WARNING,
        ServerStatus::Running => SUCCESS,
        ServerStatus::Stopping => WARNING,
        ServerStatus::Failed => DANGER,
    }
}
