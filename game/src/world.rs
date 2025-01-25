use crate::enemy::Enemy;
use crate::name_generator::NameGenerator;
use crate::player::*;
use crate::room::*;
use crate::story::Story;

pub(crate) struct World {
    rooms: Vec<Room>,
    enemies: Vec<Enemy>,
    player: Player,
}

impl World {
    pub fn new() -> Self {
        let xml_parser = Story::new("../assets/kasteelruine.xml");
        let mut unique_name_generator = NameGenerator::new();
        let created_rooms = Self::create_rooms(&xml_parser, &mut unique_name_generator);
        let created_enemies = Self::create_enemies(&xml_parser, &mut unique_name_generator);

        println!("{:?}", created_enemies);
        Self {
            player: Player::new(),
            rooms: created_rooms,
            enemies: created_enemies,
        }
    }

    fn create_rooms(parser: &Story, gen: &mut NameGenerator) -> Vec<Room> {
        parser.create_rooms(gen)
    }

    fn create_enemies(parser: &Story, gen: &mut NameGenerator) -> Vec<Enemy> {
        parser.create_enemies(gen)
    }
}
