type RoomId = u32;

use crate::objects::Item;

#[derive(Clone, Copy)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
    None,
}
#[derive(Clone, Copy)]
pub(crate) struct Exit {
    direction: Direction,
    destination: RoomId,
}

pub(crate) struct Room {
    id: RoomId,
    name: String,
    description: String,
    exits: [Exit; 4],
    visible_items: Vec<Item>,
    invisible_items: Vec<Item>,
}

impl Room {
    pub(crate) fn new(id: RoomId, name: &str, description: &str, vis_items: &[Item], invis_items: &[Item]) -> Self {
        let empty_exit = 
        Exit{
            direction: Direction::None, 
            destination: 0
        };

        let empty: [Exit; 4] = [empty_exit, empty_exit, empty_exit, empty_exit];

        Self {
            id: id,
            name: name.to_string(),
            description: description.to_string(),
            exits: empty,
            visible_items: vis_items.to_vec(),
            invisible_items: invis_items.to_vec(),
        }
    }
}
