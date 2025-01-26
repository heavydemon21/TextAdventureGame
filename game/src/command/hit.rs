use crate::Game;

use super::Command;

pub(crate) struct HitCommand {
    pub(crate) name: String,
}

impl Command for HitCommand {
    fn execute(&self, game: &mut Game) {
        let world = game.get_world();
        let player = world.get_player();
        let player_dmg = player.hit();
        world.player_attack_enemy(player_dmg, self.name.as_str());

        world.enemies_attack();
        world.enemies_move();
    }
}
