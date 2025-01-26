use crate::{console_output, Game};

use super::Command;

pub struct QuitCommand {}

impl Command for QuitCommand {
    fn execute(&self, game: &mut Game) {
        game.quit_game();
        console_output!("End Game\n");
    }
}
