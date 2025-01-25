use crate::{objects::Item, room::RoomId};



#[derive(Debug)]
pub(crate) struct Enemy {
    id: RoomId,
    name: String, 
    description: String,
    hp: u32,
    minimum_damage: u32,
    maximum_damage: u32,
    invisible_items: Vec<Item>,
}


impl Enemy {
    pub(crate) fn new(id: RoomId, name: &str, description: &str, hp: u32, min_dmg: u32, max_dmg:u32, items: &[Item]) -> Self {
        Enemy {
            id,
            name: name.to_string(),
            description: description.to_string(), 
            hp,
            minimum_damage: min_dmg,
            maximum_damage: max_dmg, 
            invisible_items: items.to_vec(),
        }
    }
}
