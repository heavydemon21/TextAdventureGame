use crate::Game;

use super::Command;

pub(crate) struct SeeCommand {
    pub(crate) enemy_name: String,
}

impl Command for SeeCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        world.move_enemy_items_to_current_room(self.enemy_name.as_str());
    }
}
