#![windows_subsystem = "windows"]
mod conf;
mod calc;
mod window;
mod tplayer;
mod game;
mod save;
mod legend;
mod generate;
mod rpgmove;
mod map;

fn main() {
    window::load();
}
