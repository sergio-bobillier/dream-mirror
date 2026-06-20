use serde::Serialize;
use sqlite::State;

use crate::db::DB;
use crate::db::Error;

const ALL_ROLES: &str = "SELECT * FROM roles";

#[derive(Serialize, Clone)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub color: String
}

impl Role {
    pub fn all() -> Result<Vec<Role>, Error> {
        let db = DB::open()?;

        let mut result = db.query(ALL_ROLES)?;
        let mut roles: Vec<Role> = Vec::new();

        while let Ok(State::Row) = result.next() {
            roles.push(
                Role {
                    id: result.read::<i64, _>("id").unwrap(),
                    name: result.read::<String, _>("name").unwrap(),
                    icon: result.read::<String, _>("icon").unwrap(),
                    color: result.read::<String, _>("color").unwrap()
                }
            )
        }

        Ok(roles)
    }
}
