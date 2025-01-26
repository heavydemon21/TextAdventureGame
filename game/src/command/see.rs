use crate::Game;

use super::Command;

pub struct SeeCommand {}

impl Command for SeeCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        let current_room = world.get_current_room_mut();
        current_room.move_invis_to_visible_items();

        world.enemies_attack();
    }
}
