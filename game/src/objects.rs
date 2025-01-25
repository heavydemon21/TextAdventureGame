use rand::Rng;

#[derive(Clone, Debug)]
pub(crate) struct GameObject {
    name: String,
    description: String,
}

#[derive(Clone, Debug)]
pub(crate) enum GameObjectType {
    Coin(u32),
    Weapon(u32, u32),
    Armor(u32),
    Consumable(u32),
}

#[derive(Clone, Debug)]
pub(crate) struct Item {
    base: GameObject,
    kind: GameObjectType,
}

#[derive(Debug)]
pub(crate) struct Coin {
    value: u32,
}

#[derive(Debug)]
pub(crate) struct Weapon {
    minimum_damage: u32,
    maximum_damage: u32,
}

#[derive(Debug)]
pub(crate) struct Armor {
    defense: u32,
}

#[derive(Debug)]
pub(crate) struct Consumable {
    heal_amount: u32,
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
                kind: GameObjectType::Consumable(rng.gen_range(value.0..=value.1)),
            },
            "ervaringsdrink" => Item {
                base,
                kind: GameObjectType::Consumable(rng.gen_range(value.0..=value.1)),
            },
            "levenselixer" => Item {
                base,
                kind: GameObjectType::Consumable(rng.gen_range(value.0..=value.1)),
            },
            "wapenrusting" => Item {
                base,
                kind: GameObjectType::Armor(value.2),
            },
            "wapen" => Item {
                base,
                kind: GameObjectType::Weapon(value.0, value.1),
            },
            "goudstukken" => Item {
                base,
                kind: GameObjectType::Coin(rng.gen_range(value.0..=value.1)),
            },
            _ => Item {
                base,
                kind: GameObjectType::Coin(value.0),
            },
        }
    }
}
