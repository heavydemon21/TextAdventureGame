use crate::Game;

use super::Command;

pub(crate) struct TakeCommand {
    pub(crate) item: String,
}

impl Command for TakeCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        let current_room = world.get_current_room_mut();
        if let Some(new_item) = current_room.move_item(self.item.as_str()) {
            let player = world.get_player_mut();
            player.fill_backpack(new_item);
        }
    }
}
