use serde::Serialize;
use sqlite::State;
use std::collections::HashMap;

use crate::db::DB;
use crate::db::Error;
use super::Element;
use super::Role;

const ALL_CHARACTERS: &str = "
    SELECT c.id, c.name, c.portrait, c.element_id,
           e.name AS element_name, e.icon AS element_icon, e.color AS element_color
        FROM characters AS c
            JOIN elements AS e ON c.element_id = e.id";

const CHARACTER_ROLES: &str = "SELECT * FROM character_roles";

#[derive(Serialize)]
pub struct Character {
    pub id: i64,
    pub name: String,
    pub portrait: String,
    pub element: Element,
    pub roles: Vec<Role>
}

pub struct Characters {
    characters: Vec<Character>
}

impl Characters {
    pub fn all() -> Result<Self, Error> {
        let db = DB::open()?;

        let mut result = db.query(ALL_CHARACTERS)?;
        let mut characters: Vec<Character> = Vec::new();

        while let Ok(State::Row) = result.next() {
            let element = Element {
                id: result.read::<i64, _>("element_id").unwrap(),
                name: result.read::<String, _>("element_name").unwrap(),
                icon: result.read::<String, _>("element_icon").unwrap(),
                color: result.read::<String, _>("element_color").unwrap()
            };

            characters.push(
                Character {
                    id: result.read::<i64, _>("id").unwrap(),
                    name: result.read::<String, _>("name").unwrap(),
                    portrait: result.read::<String, _>("portrait").unwrap(),
                    element: element,
                    roles: Vec::new()
                }
            )
        }

        Ok(Self { characters })
    }

    pub fn with_roles(mut self) -> Result<Self, Error> {
        let roles = Role::all()?;
        let roles_by_id: HashMap<i64, Role> = roles.into_iter().map(|role| (role.id, role)).collect();
        let mut roles_by_character_id: HashMap<i64, Vec<&Role>> = HashMap::new();

        let db = DB::open()?;

        let mut result = db.query(CHARACTER_ROLES)?;

        while let Ok(State::Row) = result.next() {
            let character_id = result.read::<i64, _>("character_id").unwrap();
            let role_id = result.read::<i64, _>("role_id").unwrap();

            let character_roles = roles_by_character_id.entry(character_id).or_insert(Vec::new());

            if let Some(role) = roles_by_id.get(&role_id) {
                character_roles.push(role);
            }
        }

        self.characters.iter_mut().for_each(|character|{
            if let Some(character_roles) = roles_by_character_id.get(&character.id) {
                character.roles = character_roles.iter().map(|&role| role.clone()).collect();
            }
        });

        Ok(self)
    }

    pub fn characters(self) -> Vec<Character> {
        self.characters
    }
}

impl Character {

}