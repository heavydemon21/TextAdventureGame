use crate::{console_output, Game};

use super::Command;

pub struct HelpCommand {}

const COMMANDS: [&str; 15] = [
    "Help",
    "Look",
    "Search",
    "Go <Direction>",
    "Take <Object>",
    "Put <Object>",
    "See <Object>",
    "SeePlayer",
    "Hit <Enemy>",
    "Wear <Object>",
    "Wait",
    "Consume <Object>",
    "Godmode",
    "Quit",
    "Unknown",
];

impl Command for HelpCommand {
    fn execute(&self, _game: &mut Game) {
        console_output!("{:?}\n", COMMANDS);
    }
}
