use std::io;

use crate::{
    command, database::{self, Database}, objects, player::{self, Player}, room::{self, Room}
};

pub struct Game {
    world: World,
    db: Database,
}

pub(crate) struct World {
    rooms: Vec<Room>,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World {
                rooms: Vec::new(),
                player: Player::new(),
            },
            db: Database::new("../assets/kerkersendraken.db")
        }
    }

    pub fn run(&mut self) {
        loop {
            let input = self.get_player_input();
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
