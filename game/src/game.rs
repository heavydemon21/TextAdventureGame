use std::io;

use crate::{
    command, objects,
    player::{self, Player},
    room::{self, Room},
    world::{self, World},
};

pub struct Game {
    world: World,
}

impl Game {
    pub fn new() -> Self {
        Game {
            world: World::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            println!("BEGIN LOOP");
            let input = self.get_player_input();
            println!("END LOOP");
        }
    }

    fn get_player_input(&self) -> Vec<String> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        input
            .split_whitespace()
            .map(|input| input.to_string())
            .collect()
    }
}
