use crate::Game;

use super::Command;

pub struct GodmodeCommand {}

impl Command for GodmodeCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        let player = world.get_player_mut();
        player.toggle_godmode();
    }

}
