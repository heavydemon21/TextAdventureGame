use crate::{console_output, Game};

use super::Command;

pub struct UnknownCommand {}

impl Command for UnknownCommand {
    fn execute(&self, _game: &mut Game) {
        console_output!("Not a valid command\n");
    }
}
