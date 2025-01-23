use game::Game;

mod game;

fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    game.run();
}
