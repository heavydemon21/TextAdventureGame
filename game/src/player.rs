use std::os::unix::thread;

use rand::{thread_rng, Rng};

use crate::console_output;
use crate::objects::{Armor, GameObjectType, Item, Weapon};

use crate::console::console;

#[derive(Debug)]
pub(crate) struct Player {
    name: String,
    hp: u32,
    gold: u32,
    attack_chance: u32,
    weapon: Option<Item>,
    armor: Option<Item>,
    backpack: Vec<Item>,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Player Information:
--------------------
Name:           {}
HP:             {}
Gold:           {}
Attack Chance:  {}%
Weapon:         {}
Armor:          {}
Backpack:       {}
--------------------\n",
            self.name,
            self.hp,
            self.gold,
            self.attack_chance,
            match &self.weapon {
                Some(weapon) => format!("{:?}", weapon),
                None => "None".to_string(),
            },
            match &self.armor {
                Some(armor) => format!("{:?}", armor),
                None => "None".to_string(),
            },
            if self.backpack.is_empty() {
                "Empty".to_string()
            } else {
                self.backpack
                    .iter()
                    .map(|item| format!("{:?}", item))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        )
    }
}

impl Player {
    pub(crate) fn new(start_weapon: &Item) -> Self {
        console_output!("Enter player name\n");
        let player_name = console::read_input();
        Self {
            name: player_name.to_string(),
            hp: 10,
            gold: 0,
            attack_chance: 40,
            weapon: Some(start_weapon.clone()),
            armor: None,
            backpack: Vec::new(),
        }
    }

    pub(crate) fn show(&self) {
        console_output!("{self}");
    }

    pub(crate) fn hit(&self) -> u32 {
        let mut rng = thread_rng();

        let real_attack_chance = self.attack_chance as f64 / 100.0;
        if rng.gen_bool(real_attack_chance) {
            if let Some(weapon) = &self.weapon {
                let (min_dmg, max_dmg) = match &weapon.kind() {
                    GameObjectType::Weapon(min_dmg, max_dmg) => (*min_dmg, *max_dmg),
                    _ => (0, 0),
                };

                let damage = rng.gen_range(min_dmg..=max_dmg);

                console_output!("player does total damage {} with weapon {:?}\n", damage, weapon);

                return damage;
            }
        }
        console_output!("player misses\n");
        0
    }

    pub(crate) fn take_dmg(&mut self, damage: u32) {
        if damage != 0 {
            if let Some(armor) = &self.armor {
                let defense = match &armor.kind() {
                    GameObjectType::Armor(defense) => *defense,
                    _ => 0,
                };

                let actual_damage = if damage > defense {
                    damage - defense
                } else {
                    0
                };

                console_output!(
                    "You took {} damage after armor mitigation. Your current HP: {}\n",
                    actual_damage,
                    self.hp
                );

                self.hp = self.hp.saturating_sub(actual_damage);
            } else {
                self.hp = self.hp.saturating_sub(damage);
                console_output!("You took {} damage. Your current HP: {}\n", damage, self.hp);
            }
        }
    }

    pub(crate) fn fill_backpack(&mut self, item: Item) {
        self.backpack.push(item);
    }

    pub(crate) fn remove_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(weapon) = &self.weapon {
            if weapon.name() == item_name {
                let removed_item = self.weapon.take();
                console_output!("Weapon '{}' has been removed from your hand.\n", item_name);
                return removed_item;
            }
        }
        if let Some(armor) = &self.armor {
            if armor.name() == item_name {
                let removed_item = self.armor.take();
                console_output!("Armor '{}' has been removed from your body.\n", item_name);
                return removed_item;
            }
        }

        if let Some(position) = self
            .backpack
            .iter()
            .position(|item| item.name() == item_name)
        {
            let removed_item = self.backpack.remove(position);
            console_output!(
                "Item '{}' has been removed from your backpack.\n",
                item_name
            );
            return Some(removed_item);
        }

        console_output!(
            "Item '{}' not found in weapon, armor, or backpack.\n",
            item_name
        );
        None
    }

    pub(crate) fn equip_item(&mut self, item_name: &str) {
        if let Some(position) = self
            .backpack
            .iter()
            .position(|item| item.name() == item_name)
        {
            let item = self.backpack.remove(position);

            match item.kind() {
                GameObjectType::Weapon(min_damage, max_damage) => {
                    if let Some(current_weapon) = self.weapon.take() {
                        self.backpack.push(current_weapon);
                        console_output!("Switched out weapon.\n");
                    }

                    console_output!(
                        "Equipped weapon: {} (Damage: {}-{})\n",
                        item.name(),
                        min_damage,
                        max_damage
                    );

                    self.weapon = Some(item);
                }
                GameObjectType::Armor(defense) => {
                    if let Some(current_armor) = self.armor.take() {
                        self.backpack.push(current_armor);
                        console_output!("Switched out armor.\n");
                    }
                    console_output!("Equipped armor: {} (Defense: {})\n", item.name(), defense);
                    self.armor = Some(item);
                }
                _ => {
                    console_output!("Item '{}' is not equippable.\n", item_name);
                    self.backpack.push(item);
                }
            }
        } else {
            console_output!("Item '{}' not found in backpack.\n", item_name);
        }
    }

    pub(crate) fn consume_potion(&mut self, item_name: &str) {
        if let Some(position) = self
            .backpack
            .iter()
            .position(|item| item.name() == item_name)
        {
            let item = self.backpack.remove(position);

            match item.kind() {
                GameObjectType::Consumable(value) => {
                    self.hp += value;
                    console_output!(
                        "Consumed potion: {} (effectiveness: {})\n",
                        item.name(),
                        value,
                    );

                    console_output!("New health: {} \n", self.hp,);
                }
                _ => {
                    console_output!("Item '{}' is not equippable.\n", item_name);
                    self.backpack.push(item);
                }
            }
        } else {
            console_output!("Item '{}' not found in backpack.\n", item_name);
        }
    }
    pub(crate) fn hp(&self) -> u32 {
        self.hp
    }
}
