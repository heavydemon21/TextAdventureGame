use crate::Game;

use super::Command;

pub struct SeePlayerCommand {}

impl Command for SeePlayerCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        let player = world.get_player();
        player.show();
    }
}
