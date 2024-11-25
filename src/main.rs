#![windows_subsystem = "windows"]
mod conf;
mod calc;
mod window;
mod game;
mod save;
mod legend;
mod generate;
mod rpgmove;

fn main() {
    window::load();
}
