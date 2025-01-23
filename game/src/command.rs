
use crate::room::Direction;

mod help;

enum Command {
    Help,
    Look,
    Search,
    Go(Direction),
    Take(String), // object
    Put(String), // object
    See(String), // object/enemy
    SeePlayer(String), 
    Hit(String), //enemy
    Wear(String), // object weapon/armor
    Wait,
    Consume, // object consumable
    Godmode,
    Quit,
    Unknown,
}


pub fn process_input(input: &[&str]) {
    let command = parse_input_to_command(input);
}

fn parse_input_to_command(input: &[&str]) -> Command {
    Command::Help
}
