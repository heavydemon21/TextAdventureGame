use crate::Game;

use super::Command;

pub(crate) struct PutCommand {
    pub(crate) item: String,
}

impl Command for PutCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        let player = world.get_player_mut();

        if let Some(item) = player.remove_item(self.item.as_str()) {
            let current_room = world.get_current_room_mut();
            current_room.insert_item(item);
        }
    }
}
