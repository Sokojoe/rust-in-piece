extern crate rand;
extern crate yansi;

mod game;
mod game_interface;
mod console_interface;
mod entity;

fn main() {
    game::play_game();
}
