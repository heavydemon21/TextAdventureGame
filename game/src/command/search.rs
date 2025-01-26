use crate::Game;

use super::Command;

pub struct SearchCommand {}

impl Command for SearchCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        world.show();
    }
}
