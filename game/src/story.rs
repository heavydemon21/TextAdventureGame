use std::fs;

use quick_xml::{
    events::{attributes::Attribute, Event},
    Reader,
};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    enemy::Enemy,
    name_generator::{self, NameGenerator},
    objects::ItemFactory,
};

use crate::{
    database::Database,
    objects::Item,
    room::{self, Room},
};

#[derive(Debug, Clone)]
pub(crate) struct Locatie {
    id: u32,
    noord: Option<u32>,
    oost: Option<u32>,
    zuid: Option<u32>,
    west: Option<u32>,
    vijand: Option<String>,
    objectenverborgen: Option<String>,
    objectenzichtbaar: Option<String>,
    naam: String,
    beschrijving: String,
}

pub(crate) struct Story {
    locaties: Vec<Locatie>,
    db: Database,
}

impl Story {
    pub(crate) fn new(xml_path: &str) -> Self {
        let db = Database::new("../assets/kerkersendraken.db");
        let xml_bytes = fs::read(xml_path).expect("Failed to read xml file");
        let xml_content = String::from_utf8(xml_bytes).unwrap();
        let mut reader = Reader::from_str(&xml_content);

        let mut locaties: Vec<Locatie> = Vec::new();
        let mut current_locatie = Locatie {
            id: 0,
            noord: None,
            oost: None,
            zuid: None,
            west: None,
            vijand: None,
            objectenverborgen: None,
            objectenzichtbaar: None,
            naam: String::new(),
            beschrijving: String::new(),
        };

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    if e.name().0 == b"locatie" {
                        // Reset the current_locatie
                        current_locatie = Locatie {
                            id: 0,
                            noord: None,
                            oost: None,
                            zuid: None,
                            west: None,
                            vijand: None,
                            objectenverborgen: None,
                            objectenzichtbaar: None,
                            naam: String::new(),
                            beschrijving: String::new(),
                        };

                        for attr in e.attributes() {
                            match attr {
                                Ok(Attribute { key, value }) => {
                                    let value_str = String::from_utf8_lossy(&value).into_owned();
                                    match key.0 {
                                        b"id" => {
                                            current_locatie.id = value_str.parse().unwrap_or(0)
                                        }
                                        b"noord" => {
                                            current_locatie.noord =
                                                Some(value_str.parse().unwrap_or(0))
                                        }
                                        b"oost" => {
                                            current_locatie.oost =
                                                Some(value_str.parse().unwrap_or(0))
                                        }
                                        b"zuid" => {
                                            current_locatie.zuid =
                                                Some(value_str.parse().unwrap_or(0))
                                        }
                                        b"west" => {
                                            current_locatie.west =
                                                Some(value_str.parse().unwrap_or(0))
                                        }
                                        b"vijand" => current_locatie.vijand = Some(value_str),
                                        b"objectenverborgen" => {
                                            current_locatie.objectenverborgen = Some(value_str)
                                        }
                                        b"objectenzichtbaar" => {
                                            current_locatie.objectenzichtbaar = Some(value_str)
                                        }
                                        b"naam" => current_locatie.naam = value_str,
                                        _ => (),
                                    }
                                }
                                Err(e) => eprintln!("Error reading attribute: {}", e),
                            }
                        }
                    }
                }
                Ok(Event::Text(ref e)) => {
                    if e.len() > 30 {
                        current_locatie.beschrijving = e.unescape().unwrap().to_string();
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().0 == b"locatie" {
                        locaties.push(current_locatie.clone());
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    eprintln!("Error reading XML: {}", e);
                    break;
                }
                _ => (), // Ignore other events
            }
        }

        Self { locaties, db }
    }

    fn fill_items(&self, objects: &Option<String>, unique_name: &mut NameGenerator) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();
        if let Some(visible_str) = objects.as_ref() {
            let vis_parts: Vec<&str> = visible_str.trim().split(';').collect();

            for item_name in vis_parts {
                if let Some(obj) = self.db.get_object(item_name) {
                    let name = unique_name.generate_name(&obj.name);
                    let extra_parameters = (
                        obj.extra_parameters.0 as u32,
                        obj.extra_parameters.1 as u32,
                        obj.extra_parameters.2 as u32,
                    );
                    items.push(ItemFactory::create_item(
                        &name,
                        &obj.description,
                        &obj.type_obj,
                        extra_parameters,
                    ));
                } else {
                    eprintln!("Object {} not found in database", item_name);
                }
            }
        }
        items
    }

    pub(crate) fn create_rooms(&self, unique_name: &mut NameGenerator) -> Vec<Room> {
        let mut rooms: Vec<Room> = Vec::new();
        for story in &self.locaties {
            let visible_items: Vec<Item> = self.fill_items(&story.objectenzichtbaar, unique_name);
            let invisible_items: Vec<Item> = self.fill_items(&story.objectenverborgen, unique_name);
            let name = unique_name.generate_name(&story.naam);

            rooms.push(Room::new(
                story.id,
                &name,
                &story.beschrijving,
                &visible_items,
                &invisible_items,
            ));
        }
        rooms
    }

    pub(crate) fn create_enemies(&self, unique_name: &mut NameGenerator) -> Vec<Enemy> {
        let mut enemies: Vec<Enemy> = Vec::new();

        for story in &self.locaties {
            if let Some(enemy) = story.vijand.as_ref() {
                let multiple_enemies: Vec<&str> = enemy.split(";").collect();
                for single_enemy in multiple_enemies {
                    if let Some(found_enemy) = self.db.get_enemy(single_enemy) {
                        let name = unique_name.generate_name(&found_enemy.name);
                        let hp = found_enemy.hp as u32;
                        let min_dmg = found_enemy.minimum_damage as u32;
                        let max_dmg = found_enemy.maximum_damage as u32;
                        let items = self.generate_random_items(
                            unique_name,
                            &(
                                found_enemy.minimum_objects as u32,
                                found_enemy.maximum_objects as u32,
                            ),
                        );
                        enemies.push(Enemy::new(
                            story.id,
                            &name,
                            &found_enemy.description,
                            hp,
                            min_dmg,
                            max_dmg,
                            &items,
                        ));
                    } else {
                        eprintln!("enemy not found: {}", single_enemy);
                    }
                }
            }
        }
        enemies
    }

    pub(crate) fn generate_random_items(
        &self,
        unique_name: &mut NameGenerator,
        range: &(u32, u32),
    ) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();
        let mut rng = thread_rng();
        let max_len = rng.gen_range(range.0..=range.1);
        let mut idx = 0;
        let objects = self.db.get_all_objects();

        while idx != max_len {
            let enemy_obj = objects.choose(&mut rng).unwrap();
            let name = unique_name.generate_name(&enemy_obj.name);
            let parameters: (u32, u32, u32) = (
                enemy_obj.extra_parameters.0 as u32,
                enemy_obj.extra_parameters.1 as u32,
                enemy_obj.extra_parameters.2 as u32,
            );
            items.push(ItemFactory::create_item(
                &name,
                &enemy_obj.description,
                &enemy_obj.type_obj,
                parameters,
            ));
            idx += 1;
        }

        items
    }
}
