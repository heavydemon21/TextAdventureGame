use std::io;

use game::command::process_input;

pub struct Game {

}

impl Game {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn run(&mut self) {
        loop {
            let input = self.get_player_input();
            process_input(input);
        }
    }

    fn get_player_input(&self) -> Vec<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Failed to read line");
        input.split_whitespace().map(|input| input.to_string()).collect()
    }
}
