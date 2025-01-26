use crate::Game;

use super::Command;

pub struct GodmodeCommand {}

impl Command for GodmodeCommand {
    fn execute(&self, game: &mut Game) {}
}
