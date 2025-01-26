use core::panic;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::console_output;
use crate::enemy::Enemy;
use crate::name_generator::NameGenerator;
use crate::player::*;
use crate::room::*;
use crate::story::Story;

pub(crate) struct World {
    rooms: Vec<Room>,
    enemies: Vec<Enemy>,
    player: Player,
    current_room: RoomId,
}

impl World {
    pub(crate) fn new() -> Self {
        let story_parser = Story::new("../assets/kasteelruine.xml");
        let mut unique_name_generator = NameGenerator::new();
        let created_rooms = story_parser.create_rooms(&mut unique_name_generator);
        let curr_room = created_rooms[0].get_id();
        let created_enemies = story_parser.create_enemies(&mut unique_name_generator);
        let player_start_weapon =
            story_parser.get_player_starting_weapen(&mut unique_name_generator);

        Self {
            rooms: created_rooms,
            enemies: created_enemies,
            player: Player::new(&player_start_weapon),
            current_room: curr_room,
        }
    }

    pub(crate) fn get_current_room(&self) -> &Room {
        for room in &self.rooms {
            if room.get_id() == self.current_room {
                return &room;
            }
        }
        panic!("Could not find room");
    }

    pub(crate) fn get_current_room_mut(&mut self) -> &mut Room {
        for room in self.rooms.iter_mut() {
            if room.get_id() == self.current_room {
                return room;
            }
        }
        panic!("Could not find room");
    }

    pub(crate) fn get_player(&self) -> &Player {
        &self.player
    }

    pub(crate) fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
    pub(crate) fn goto_next_room(&mut self, new_room: RoomId) {
        self.current_room = new_room;
    }

    pub(crate) fn show(&self) {
        let idx = (self.current_room - 1) as usize;
        let room = self.rooms.get(idx).unwrap();
        room.show();

        for enemy in &self.enemies {
            if enemy.room_id() == self.current_room {
                enemy.show();
            }
        }
    }

    pub(crate) fn enemies_move(&mut self) {
        let mut rng = thread_rng();

        for enemy in self.enemies.iter_mut() {
            let id = enemy.room_id();
            let room_idx = (id - 1) as usize;
            let room = self.rooms.get(room_idx).unwrap();
            let chosen_exit = room.get_exits().choose(&mut rng).unwrap();
            enemy.set_position(chosen_exit.id());
        }
    }

    pub(crate) fn enemies_attack(&mut self) {
        for enemy in self.enemies.iter_mut() {
            if enemy.room_id() == self.current_room {
                let enemy_damage = enemy.attack();
                self.player.take_dmg(enemy_damage);
            }
        }
    }

    pub(crate) fn player_attack_enemy(&mut self, player_dmg: u32, name: &str) {
        for enemy in self.enemies.iter_mut() {
            if (enemy.room_id() == self.current_room) && enemy.name() == name {
                enemy.take_dmg(player_dmg);
                return;
            }
        }
        console_output!("No enemies found with that name: {}\n", name);
    }
}
