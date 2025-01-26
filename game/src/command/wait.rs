use crate::Game;

use super::Command;

pub struct WaitCommand {}

impl Command for WaitCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        world.enemies_attack();
        world.enemies_move();
    }
}
