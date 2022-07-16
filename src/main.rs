use game::Game;

mod cell;
mod board;
mod snake;
mod screen;
mod game;
mod food;

fn main() {

  let mut game: Game = Game::new(32, 16, 32);

  game.start();
}
