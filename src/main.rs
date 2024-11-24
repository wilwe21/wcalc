#![windows_subsystem = "windows"]
mod conf;
mod calc;
mod window;
mod game;
mod save;
mod legend;

fn main() {
    window::load();
}
