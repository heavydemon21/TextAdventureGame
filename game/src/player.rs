use crate::objects::{Armor, Item, Weapon};

pub(crate) struct Player {
    name: String,
    hp: u32,
    gold: u32,
    attack_chance: u32,
    weapen: Option<Weapon>,
    armor: Option<Armor>,
    backpack: Vec<Item>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "TODO".to_string(),
            hp: 10,
            gold: 0,
            attack_chance: 40,
            weapen: None,
            armor: None,
            backpack: Vec::new(),
        }
    }
}
