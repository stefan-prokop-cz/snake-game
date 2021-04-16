mod snake;
mod command;
mod direction;
mod game;
mod point;

use crate::game::Game;
use std::io::stdout;

fn main() {
    Game::new(stdout(), 10, 10).run();
}
