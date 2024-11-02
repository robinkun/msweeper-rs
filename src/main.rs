mod config;
mod game;
mod msweeper;
mod point;
mod terminal;

use config::Config;
use game::Game;
use msweeper::Msweeper;
use std::env;
use std::process;
use terminal::Terminal;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = match Config::new(&args) {
        Ok(c) => c,
        Err(_) => process::exit(1),
    };

    let mut game = Game::construct(&cfg);

    game.main_loop();
}
