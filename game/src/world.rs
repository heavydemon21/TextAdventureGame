use crate::player::*;
use crate::room::*;
use crate::story::Story;

pub(crate) struct World {
    rooms: Vec<Room>,
    player: Player,
}

impl World {
    pub fn new() -> Self {
        let created_rooms = Self::create_rooms();
        Self {
            player: Player::new(),
            rooms: created_rooms,
        }
    }

    fn create_rooms() -> Vec<Room> {
        let mut rooms: Vec<Room> = Vec::new();
        let xml_parser = Story::new("../assets/kasteelruine.xml");
        xml_parser.create_rooms(&mut rooms);
        rooms
    }
}
