#![windows_subsystem = "windows"]
mod conf;
mod calc;
mod window;
mod game;
mod save;

fn main() {
    window::load();
}
