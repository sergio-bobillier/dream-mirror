use thiserror::Error;
use std::io::stdout;

use super::DB;
use crate::console::Logger;
use crate::console::Severity;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unable to create table {0}: {1}")]
    TableCreationFailed(String, #[source] sqlite::Error)
}

pub fn load(db: &DB) -> Result<(), Error> {
    let tables = [
        (
            "elements",
            "CREATE TABLE {} (
                id INTEGER PRIMARY KEY,
                name TEXT,
                icon TEXT,
                color TEXT
            );"
        ),
        (
            "roles",
            "CREATE TABLE {} (
                id INTEGER PRIMARY KEY,
                name TEXT,
                icon TEXT,
                color TEXT
            );"
        ),
        (
            "characters",
            "CREATE TABLE {} (
                id INTEGER PRIMARY KEY,
                name TEXT,
                portrait TEXT,
                element_id INTEGER,

                FOREIGN KEY(element_id) REFERENCES elements(id)
            );"
        ),
        (
            "character_roles",
            "CREATE TABLE {} (
                character_id INTEGER,
                role_id INTEGER,

                FOREIGN KEY(character_id) REFERENCES characters(id)
                FOREIGN KEY(role_id) REFERENCES roles(id)
            );"
        )
    ];

    let mut logger = Logger::new(stdout());

    for (table_name, ddl_template) in tables {
        let ddl = ddl_template.replace("{}", table_name);

        logger.log_message(format!("Creating table {}...", table_name), Severity::Debug);

        if let Err(error) = db.connection.execute(ddl) {
            return Err(Error::TableCreationFailed(table_name.to_string(), error));
        }
    }

    Ok(())
}