pub(crate) struct GameObject {
    name: String,
    description: String,
}

pub(crate) enum GameObjectType {
    Coin(u32),
    Weapon(u32, u32),
    Armor(u32),
    Consumable(u32),
}

pub(crate) struct Item {
    base: GameObject,
    kind: GameObjectType,
}

/*
impl GameObject {
    pub fn describe(&self) {
        //match self {
            //SpelObject::Coin(coin) => coin.describe(),
            //SpelObject::Weapon(weapon) => weapon.describe(),
            //SpelObject::Armor(armor) => armor.describe(),
            //SpelObject::Consumable(consumable) => consumable.describe(),
        }
    }
}
*/

pub(crate) struct Coin {
    value: u32,
}
pub(crate) struct Weapon {
    minimum_damage: u32,
    maximum_damage: u32,
}
pub(crate) struct Armor {
    defense: u32,
}
pub(crate) struct Consumable {
    heal_amount: u32,
}
