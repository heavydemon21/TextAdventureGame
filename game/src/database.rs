use sqlite::{Connection, State};


pub(crate) struct Object {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) type_obj: String,
    pub(crate) extra_parameters: (i64, i64, i64),
}

#[allow(dead_code)]
struct Location {
    name: String,
    description: String,
}

#[allow(dead_code)]
struct ObjectType {
    name: String,
}

pub(crate) struct Enemy {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) minimum_objects: i64,
    pub(crate) maximum_objects: i64,
    pub(crate) hp: i64,
    pub(crate) attack_chance: i64,
    pub(crate) minimum_damage: i64,
    pub(crate) maximum_damage: i64,
}

struct SqlData {
    objects: Vec<Object>,
    locations: Vec<Location>,
    object_types: Vec<ObjectType>,
    enemies: Vec<Enemy>,
}

pub(crate) struct Database {
    connection: Option<Connection>,
    data: SqlData,
}

impl Database {
    pub(crate) fn new(path: &str) -> Self {
        let mut db = Self {
            connection: None,
            data: SqlData {
                objects: Vec::new(),
                enemies: Vec::new(),
                locations: Vec::new(),
                object_types: Vec::new(),
            },
        };

        db.open_connection(path);
        db.load_data();

        db
    }

    fn open_connection(&mut self, path: &str) {
        match sqlite::open(path) {
            Ok(conn) => {
                self.connection = Some(conn);
            }
            Err(error) => eprintln!("Failed to open databse connection {}", error),
        };
    }
    fn load_data(&mut self) {
        self.load_object_types();
        self.load_objects();
        self.load_locations();
        self.load_enemies();
    }
    fn load_object_types(&mut self) {
        if let Some(connection) = &self.connection {
            let query = "SELECT * FROM Objecttypen";
            let mut statement = connection.prepare(query).unwrap();

            while let Ok(State::Row) = statement.next() {
                self.data.object_types.push(ObjectType {
                    name: statement.read::<String, _>("naam").unwrap(),
                })
            }
        }
    }

    fn load_locations(&mut self) {
        if let Some(connection) = &self.connection {
            let query = "SELECT * FROM Locaties";
            let mut statement = connection.prepare(query).unwrap();

            while let Ok(State::Row) = statement.next() {
                self.data.locations.push(Location {
                    name: statement.read::<String, _>("naam").unwrap(),
                    description: statement.read::<String, _>("beschrijving").unwrap(),
                })
            }
        }
    }

    fn load_objects(&mut self) {
        if let Some(connection) = &self.connection {
            let query = "SELECT * FROM Objecten";
            let mut statement = connection.prepare(query).unwrap();

            while let Ok(State::Row) = statement.next() {
                self.data.objects.push(Object {
                    name: statement.read::<String, _>("naam").unwrap(),
                    description: statement.read::<String, _>("omschrijving").unwrap(),
                    type_obj: statement.read::<String, _>("type").unwrap(),
                    extra_parameters: (
                        statement.read::<i64, _>("minimumwaarde").unwrap(),
                        statement.read::<i64, _>("maximumwaarde").unwrap(),
                        statement.read::<i64, _>("bescherming").unwrap(),
                    ),
                })
            }
        }
    }

    fn load_enemies(&mut self) {
        if let Some(connection) = &self.connection {
            let query = "SELECT * FROM Vijanden";
            let mut statement = connection.prepare(query).unwrap();

            while let Ok(State::Row) = statement.next() {
                self.data.enemies.push(Enemy {
                    name: statement.read::<String, _>("naam").unwrap(),
                    description: statement.read::<String, _>("omschrijving").unwrap(),
                    minimum_objects: statement.read::<i64, _>("minimumobjecten").unwrap(),
                    maximum_objects: statement.read::<i64, _>("maximumobjecten").unwrap(),
                    hp: statement.read::<i64, _>("levenspunten").unwrap(),
                    attack_chance: statement.read::<i64, _>("aanvalskans").unwrap(),
                    minimum_damage: statement.read::<i64, _>("minimumschade").unwrap(),
                    maximum_damage: statement.read::<i64, _>("maximumschade").unwrap(),
                })
            }
        }
    }

    pub(crate) fn get_object(&self, name: &str) -> Option<&Object> {
        for object in &self.data.objects {
            if object.name == name {
                return Some(object);
            }
        }
        None
    }

    pub(crate) fn get_enemy(&self, name: &str) -> Option<&Enemy> {
        for enemy in &self.data.enemies {
            if enemy.name == name {
                return Some(enemy);
            }
        }
        None
    }
    pub(crate) fn get_all_objects(&self) -> &Vec<Object> {
        &self.data.objects
    }
}
