use rand::Rng;

use crate::console_output;

#[derive(Clone, Debug)]
pub(crate) struct GameObject {
    name: String,
    description: String,
}

#[derive(Clone, Debug)]
pub(crate) enum GameObjectType {
    Coin(Coin),
    Weapon(Weapon),
    Armor(Armor),
    Consumable(Consumable),
}

#[derive(Clone, Debug)]
pub(crate) struct Item {
    base: GameObject,
    kind: GameObjectType,
}

#[derive(Clone, Debug)]
pub(crate) struct Coin {
    value: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct Weapon {
    minimum_damage: u32,
    maximum_damage: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct Armor {
    defense: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct Consumable {
    heal_amount: u32,
}

impl Item {
    pub(crate) fn name(&self) -> &str {
        &self.base.name
    }

    pub(crate) fn kind(&self) -> &GameObjectType {
        &self.kind
    }

    pub(crate) fn do_action(&self) -> u32 {
        self.kind.do_action()
    }

    pub(crate) fn show(&self) {
        console_output!(
            "Item Details:
Name: {}
Description: {}
Details: ",
            self.base.name,
            self.base.description
        );
        self.kind.show();
    }
}
pub(crate) trait Actionable {
    fn do_action(&self) -> u32;
    fn show(&self);
}

impl Actionable for GameObjectType {
    fn do_action(&self) -> u32 {
        match self {
            GameObjectType::Coin(Coin { value }) => *value,
            GameObjectType::Weapon(Weapon {
                minimum_damage,
                maximum_damage,
            }) => {
                let mut rng = rand::thread_rng();
                rng.gen_range(*minimum_damage..=*maximum_damage)
            }
            GameObjectType::Armor(Armor { defense }) => *defense,
            GameObjectType::Consumable(Consumable { heal_amount }) => *heal_amount,
        }
    }
    fn show(&self) {
        match self {
            GameObjectType::Coin(Coin { value }) => {
                console_output!("pouch has amount {}\n", value);
            }
            GameObjectType::Weapon(Weapon {
                minimum_damage,
                maximum_damage,
            }) => {
                console_output!(
                    "Weapon has damage range {}-{}\n",
                    minimum_damage,
                    maximum_damage
                );
            }
            GameObjectType::Armor(Armor { defense }) => {
                console_output!("Armor has defense of {}\n", defense);
            }
            GameObjectType::Consumable(Consumable { heal_amount }) => {
                console_output!("Potion has heals {}\n", heal_amount);
            }
        }
    }
}

pub(crate) struct ItemFactory;
impl ItemFactory {
    pub(crate) fn create_item(
        name: &str,
        description: &str,
        obj_type: &str,
        value: (u32, u32, u32),
    ) -> Item {
        let base = GameObject {
            name: name.to_string(),
            description: description.to_string(),
        };

        let mut rng = rand::thread_rng();

        match obj_type {
            "teleportatiedrank" => Item {
                base,
                kind: GameObjectType::Consumable(Consumable {
                    heal_amount: rng.gen_range(value.0..=value.1),
                }),
            },
            "ervaringsdrink" => Item {
                base,
                kind: GameObjectType::Consumable(Consumable {
                    heal_amount: rng.gen_range(value.0..=value.1),
                }),
            },
            "levenselixer" => Item {
                base,
                kind: GameObjectType::Consumable(Consumable {
                    heal_amount: rng.gen_range(value.0..=value.1),
                }),
            },
            "wapenrusting" => Item {
                base,
                kind: GameObjectType::Armor(Armor { defense: value.2 }),
            },
            "wapen" => Item {
                base,
                kind: GameObjectType::Weapon(Weapon {
                    minimum_damage: value.0,
                    maximum_damage: value.1,
                }),
            },
            "goudstukken" => Item {
                base,
                kind: GameObjectType::Coin(Coin {
                    value: rng.gen_range(value.0..=value.1),
                }),
            },
            _ => Item {
                base,
                kind: GameObjectType::Coin(Coin { value: value.0 }),
            },
        }
    }
}
