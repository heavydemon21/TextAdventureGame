type RoomId = u32;

use crate::objects::Item;

pub(crate) enum Direction {
    North,
    South,
    East,
    West,
    None,
}

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
