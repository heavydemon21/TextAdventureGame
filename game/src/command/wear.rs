use crate::Game;

use super::Command;

pub(crate) struct WearCommand {
    pub(crate) item: String,
}

impl Command for WearCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        let player = world.get_player_mut();
        player.equip_item(self.item.as_str());
    }
}
