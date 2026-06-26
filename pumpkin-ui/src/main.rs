#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod layout;
mod pages;
mod state;
mod theme;
mod widgets;

fn main() {
    app::run();
}
