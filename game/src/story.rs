use std::fs;

use quick_xml::{
    events::{attributes::Attribute, Event},
    Reader,
};
use rand::Rng;

use crate::{
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
}

impl Story {
    pub(crate) fn new(xml_path: &str) -> Self {
        let xml_bytes = fs::read(xml_path).expect("Failed to read xml file");
        let xml_content = String::from_utf8_lossy(&xml_bytes);
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

                        // Iterate over the attributes of the "locatie" tag
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
                Ok(Event::End(ref e)) => {
                    if e.name().0 == b"locatie" {
                        locaties.push(current_locatie.clone());
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    if e.name().0 == b"beschrijving" {
                        if let Ok(Event::Text(e)) = reader.read_event() {
                            current_locatie.beschrijving = e.unescape().unwrap().to_string();
                        }
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

        Self { locaties: locaties }
    }

    pub(crate) fn create_rooms(&self, rooms: &mut Vec<Room>) {
        let db = Database::new("../assets/kerkersendraken.db");
        let mut unique_name = NameGenerator::new();
        for story in &self.locaties {
            let mut visible_items: Vec<Item> = Vec::new();
            let mut invisible_items: Vec<Item> = Vec::new();

            if let Some(visible_str) = story.objectenzichtbaar.as_ref() {
                let vis_parts: Vec<&str> = visible_str.trim().split(';').collect();

                for item_name in vis_parts {
                    if let Some(obj) = db.get_object(item_name) {
                        let name = unique_name.generate_name(&obj.name);
                        let extra_parameters = (
                            obj.extra_parameters.0 as u32,
                            obj.extra_parameters.1 as u32,
                            obj.extra_parameters.2 as u32,
                        );
                        visible_items.push(ItemFactory::create_item(
                            &name,
                            &obj.description,
                            &obj.type_obj,
                            extra_parameters,
                        ));
                    } else {
                        // Handle the case where object is not found in the database
                        eprintln!("Object {} not found in database", item_name);
                    }
                }
            }

            // Handle invisible items
            if let Some(invisible_str) = story.objectenverborgen.as_ref() {
                let invis_parts: Vec<&str> = invisible_str.trim().split(';').collect();

                for item_name in invis_parts {
                    if let Some(obj) = db.get_object(item_name) {
                        let name = unique_name.generate_name(&obj.name);
                        let extra_parameters = (
                            obj.extra_parameters.0 as u32,
                            obj.extra_parameters.1 as u32,
                            obj.extra_parameters.2 as u32,
                        );

                        invisible_items.push(ItemFactory::create_item(
                            &name,
                            &obj.description,
                            &obj.type_obj,
                            extra_parameters,
                        ));
                    } else {
                        // Handle the case where object is not found in the database
                        eprintln!("Object {} not found in database", item_name);
                    }
                }
            }

            let name = unique_name.generate_name(&story.naam);
            rooms.push(Room::new(
                story.id,
                &name,
                &story.beschrijving,
                &visible_items,
                &invisible_items,
            ));
        }
    }

    pub(crate) fn iterate_locations(&self) {
        for locatie in &self.locaties {
            println!("Locatie ID: {:?}", locatie.id);
            println!("Naam: {:?}", locatie.naam.clone());
            println!("Beschrijving: {}", locatie.beschrijving);

            if let Some(enemies) = &locatie.vijand {
                println!("Vijanden: {}", enemies);
            }

            if let Some(hidden_objects) = &locatie.objectenverborgen {
                println!("Verborgen objecten: {}", hidden_objects);
            }

            if let Some(visible_objects) = &locatie.objectenzichtbaar {
                println!("Zichtbare objecten: {}", visible_objects);
            }

            println!("---");
        }
    }
}
