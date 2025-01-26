use crate::Game;

use super::Command;

pub(crate) struct ConsumeCommand {
    pub(crate) item: String,
}

impl Command for ConsumeCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        let player = world.get_player_mut();
        player.consume_potion(self.item.as_str());
    }
}
