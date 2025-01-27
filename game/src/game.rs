use crate::{
    command::{parse_input_to_command, Command}, console::console::read_input, console_output, room::Room, world::World
};

pub struct Game {
    world: World,
    running: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            world: World::new(),
            running: true,
        }
    }

    pub fn run(&mut self) {
        console_output!("Welcome to kerkers en draken. A simple text adventure game made in rust for a school project learning rust \n");
        console_output!("Everytime the prompt shows '>' insert a command. if player does not know any commands insert Help");
        while self.running {
            let input = self.get_player_input();
            let command = parse_input_to_command(input.as_str());
            command.execute(self);

            self.check_player_hp();
        }
    }

    fn check_player_hp(&mut self) {
        if self.world.get_player().hp() <= 0 {
            self.running = false;
        }
    }

    pub(crate) fn quit_game(&mut self) {
        self.running = false;
    }

    pub(crate) fn get_current_room(&self) -> &Room {
        self.world.get_current_room()
    }

    pub(crate) fn get_world(&mut self) -> &mut World {
        &mut self.world
    }

    fn get_player_input(&self) -> String {
        read_input()
    }
}
