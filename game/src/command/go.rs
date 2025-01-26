use crate::{console_output, room::Direction, Game};

use super::Command;

pub(crate) struct GoCommand {
    pub(crate) direction: Direction,
}

impl Command for GoCommand {
    fn execute(&self, game: &mut Game) {
        let current_room = game.get_current_room();
        let new_room = current_room.check_direction(&self.direction);

        if new_room != 0 {
            let world = game.get_world();
            world.goto_next_room(new_room);
            console_output!("Going to {new_room}\n");
        } else {
            console_output!("Wrong goto direction\n");
        }

        let world = game.get_world();
        world.enemies_attack();
    }
}
