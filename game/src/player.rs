use rand::{thread_rng, Rng};

use crate::console_output;
use crate::objects::{GameObjectType, Item};

#[derive(Debug)]
pub(crate) struct Player {
    name: String,
    hp: u32,
    gold: u32,
    attack_chance: u32,
    weapon: Option<Item>,
    armor: Option<Item>,
    backpack: Vec<Item>,
    godmode: bool,
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
    pub(crate) fn new(name: &str, start_weapon: &Item) -> Self {
        Self {
            name: name.to_string(),
            hp: 10,
            gold: 0,
            attack_chance: 40,
            weapon: Some(start_weapon.clone()),
            armor: None,
            backpack: Vec::new(),
            godmode: false,
        }
    }

    pub(crate) fn toggle_godmode(&mut self) {
        self.godmode = !self.godmode;
    }

    pub(crate) fn show(&self) {
        console_output!("{self}");
    }

    fn get_weapon_dmg(&self) -> u32 {
        if let Some(weapon) = &self.weapon {
            let damage = weapon.do_action();
            console_output!("player does total damage {} \n", damage);
            return damage;
        }
        console_output!("player has no weapon does 0 damage \n");
        0
    }

    pub(crate) fn hit(&self) -> u32 {
        if self.godmode {
            self.get_weapon_dmg()
        } else {
            let mut rng = thread_rng();

            let real_attack_chance = self.attack_chance as f64 / 100.0;
            if rng.gen_bool(real_attack_chance) {
                return self.get_weapon_dmg();
            }
            console_output!("player misses\n");
            0
        }
    }

    pub(crate) fn take_dmg(&mut self, damage: u32) {
        if self.godmode {
            console_output!("Player is in godmode. it cannot take damage in this state\n");
        } else {
            if damage != 0 {
                if let Some(armor) = &self.armor {
                    let defense = armor.do_action();
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
    }

    pub(crate) fn fill_backpack(&mut self, item: Item) {
        match &item.kind() {
            GameObjectType::Coin(_coin) => {
                let amount = item.do_action();
                self.gold += amount;
                console_output!("Coin pickup {} player has now {} \n", amount, self.gold,);
            }
            _ => self.backpack.push(item),
        }
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
                GameObjectType::Weapon(_weapon) => {
                    if let Some(current_weapon) = self.weapon.take() {
                        self.backpack.push(current_weapon);
                        console_output!("Switched out weapon.\n");
                    }
                    console_output!("Equipping new weapon.\n");
                    item.show();
                    self.weapon = Some(item);
                }
                GameObjectType::Armor(_armor) => {
                    if let Some(current_armor) = self.armor.take() {
                        self.backpack.push(current_armor);
                        console_output!("Switched out armor.\n");
                    }
                    console_output!("Equipped new armor \n");
                    item.show();
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
                GameObjectType::Consumable(_consumable) => {
                    self.hp += item.do_action();
                    console_output!("Potion is consumed:\n");
                    item.show();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ItemFactory;

    #[test]
    fn test_player_creation() {
        let weapon = ItemFactory::create_item("SWORD", "SWORD", "wapen", (10, 10, 10));
        let player = Player::new("NAME", &weapon);
        assert_eq!(player.hp(), 10);
    }

    #[test]
    fn test_take_damage() {
        let weapon = ItemFactory::create_item("SWORD", "SWORD", "wapen", (10, 10, 10));
        let mut player = Player::new("NAME", &weapon);
        player.take_dmg(5);
        assert_eq!(player.hp(), 5);
    }

    #[test]
    fn test_godmode_prevents_damage() {
        let weapon = ItemFactory::create_item("SWORD", "SWORD", "wapen", (10, 10, 10));
        let mut player = Player::new("NAME", &weapon);
        player.toggle_godmode();
        player.take_dmg(100);
        assert_eq!(player.hp(), 10);
    }

    #[test]
    fn test_fill_backpack() {
        let weapon = ItemFactory::create_item("SWORD", "SWORD", "wapen", (10, 10, 10));
        let potion = ItemFactory::create_item("HEALTH_POTION", "Potion", "levenselixer", (5, 5, 0));
        let mut player = Player::new("NAME", &weapon);
        player.fill_backpack(potion);
        assert_eq!(player.backpack.len(), 1);
    }

    #[test]
    fn test_remove_item() {
        let weapon = ItemFactory::create_item("SWORD", "SWORD", "wapen", (10, 10, 10));
        let mut player = Player::new("NAME", &weapon);
        let armor = ItemFactory::create_item("ARMOR", "Shield", "wapenrusting", (0, 5, 0));
        player.fill_backpack(armor.clone());
        let removed_item = player.remove_item("ARMOR");
        assert!(removed_item.is_some());
        assert_eq!(player.backpack.len(), 0);
    }

    #[test]
    fn test_equip_item() {
        let weapon = ItemFactory::create_item("SWORD", "SWORD", "wapen", (10, 10, 10));
        let mut player = Player::new("NAME", &weapon);
        let new_weapon = ItemFactory::create_item("AXE", "AXE", "wapen", (15, 15, 15));
        player.fill_backpack(new_weapon.clone());
        player.equip_item("AXE");
        assert!(player.weapon.is_some());
        assert_eq!(player.weapon.unwrap().name(), "AXE");
    }

    #[test]
    fn test_consume_potion() {
        let weapon = ItemFactory::create_item("SWORD", "SWORD", "wapen", (10, 10, 10));
        let potion = ItemFactory::create_item("HEALTH_POTION", "Potion", "levenselixer", (5, 5, 0));
        let mut player = Player::new("NAME", &weapon);
        player.fill_backpack(potion);
        player.take_dmg(5);
        player.consume_potion("HEALTH_POTION");
        assert_eq!(player.hp(), 10);
    }
}
