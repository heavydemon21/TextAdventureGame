pub type RoomId = u32;

use crate::{console_output, objects::Item};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
    None,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Exit {
    direction: Direction,
    destination: RoomId,
}

impl Exit {
    pub(crate) fn id(&self) -> RoomId {
       self.destination 
    }
}


#[derive(Debug)]
pub(crate) struct Room {
    id: RoomId,
    name: String,
    description: String,
    exits: Vec<Exit>,
    visible_items: Vec<Item>,
    invisible_items: Vec<Item>,
}

impl Room {
    pub(crate) fn new(
        id: RoomId,
        name: &str,
        description: &str,
        vis_items: &[Item],
        invis_items: &[Item],
        exits_in: &[(u32, String)],
    ) -> Self {
        let mut exits: Vec<Exit> = Vec::new();

        for exit_input in exits_in.iter() {
            if exit_input.0 != 0 {
                exits.push(Exit {
                    direction: Direction::from_str(exit_input.1.as_str()),
                    destination: exit_input.0,
                })
            }
        }

        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            exits,
            visible_items: vis_items.to_vec(),
            invisible_items: invis_items.to_vec(),
        }
    }

    pub(crate) fn show(&self) {
        let name = &self.name;
        let des = &self.description;
        console_output!("{name}\n");
        console_output!("{des}\n");
        console_output!("\nVisible items: ");
        for item in &self.visible_items {
            console_output!(" {item:?} ");
        }

        console_output!("\nAvailable exits: ");
        for exit in &self.exits {
            console_output!(" {exit:?} ");
        }
    }

    pub(crate) fn get_id(&self) -> RoomId {
        self.id
    }

    pub(crate) fn get_exits(&self) -> &Vec<Exit> {
        &self.exits
    }

    pub(crate) fn move_invis_to_visible_items(&mut self) {
        self.visible_items.append(&mut self.invisible_items);
    }

    pub(crate) fn check_direction(&self, new_direction: &Direction) -> u32 {
        let new_direction = *new_direction;
        if new_direction == Direction::None {
            return 0;
        }

        let mut is_good_direction = 0;
        for exit in &self.exits {
            if exit.direction == new_direction {
                is_good_direction = exit.destination;
                break;
            }
        }

        is_good_direction
    }

    pub(crate) fn insert_item(&mut self, item: Item) {
        self.visible_items.push(item);
    }

    pub(crate) fn move_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(position) = self
            .visible_items
            .iter()
            .position(|item| item.name() == item_name)
        {
            let item = self.visible_items.remove(position);
            console_output!("Item '{}' has been moved to your backpack.\n", item_name);
            Some(item)
        } else {
            console_output!(
                "Item '{}' not found in the room's visible items.\n",
                item_name
            );
            None
        }
    }
}

impl Direction {
    pub(crate) fn from_str(input: &str) -> Self {
        match input {
            "North" => Direction::North,
            "East" => Direction::East,
            "West" => Direction::West,
            "South" => Direction::South,
            _ => Direction::None,
        }
    }
}
