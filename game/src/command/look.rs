use crate::Game;

use super::Command;

pub struct LookCommand {}

impl Command for LookCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        world.show();
    }
}
