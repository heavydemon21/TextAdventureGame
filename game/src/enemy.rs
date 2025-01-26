use std::fmt;

use rand::{thread_rng, Rng};

use crate::{console_output, objects::Item, room::RoomId};

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

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n Enemy Details:
--------------------
Name:           {}
Description:    {}
HP:             {}
Damage Range:   {} - {}
Room ID:        {}
Invisible Items: {}
--------------------\n",
            self.name,
            self.description,
            self.hp,
            self.minimum_damage,
            self.maximum_damage,
            self.id,
            if self.invisible_items.is_empty() {
                "None".to_string()
            } else {
                self.invisible_items
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        )
    }
}

impl Enemy {
    pub(crate) fn new(
        id: RoomId,
        name: &str,
        description: &str,
        hp: u32,
        min_dmg: u32,
        max_dmg: u32,
        items: &[Item],
    ) -> Self {
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

    pub(crate) fn attack(&self) -> u32 {
        if self.hp == 0 {
            return 0;
        }

        let mut rng = thread_rng();
        if rng.gen_bool(0.5) {
            let damage = rng.gen_range(self.minimum_damage..=self.maximum_damage);
            console_output!("{} hits the player for {}\n", self.name, damage);
            damage
        } else {
            console_output!("{} misses the player\n", self.name);
            0
        }
    }

    pub(crate) fn take_dmg(&mut self, dmg: u32) {
        if self.hp == 0 {
            console_output!("Enemy: {} is already dead\n", self.name);
            return;
        } else if dmg == 0 {
            return;
        }

        if self.hp > dmg {
            self.hp -= dmg;
            console_output!(
                "{} took {} damage, {} HP remaining.\n",
                self.name,
                dmg,
                self.hp
            );
        } else {
            self.hp = 0;
            console_output!("{} took {} damage and has died.\n", self.name, dmg);
        }
    }

    pub(crate) fn show(&self) {
        console_output!("{self}\n");
    }

    pub(crate) fn set_position(&mut self, new_room: RoomId) {
        if self.hp != 0 {
            self.id = new_room;
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn room_id(&self) -> RoomId {
        self.id
    }
}
