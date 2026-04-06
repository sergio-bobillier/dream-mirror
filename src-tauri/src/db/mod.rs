use dirs::home_dir;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot locate the user's home directory.")]
    NoHomeDirectory,
    #[error("Cannot open the database file {0}: {1}")]
    CannotOpenDatabase(PathBuf, #[source] sqlite::Error),
    #[error("Cannot load the database schema: {0}")]
    SchemaLoadError(#[from] sqlite::Error)
}

pub struct DB {
    connection: sqlite::Connection
}

impl DB {
    pub fn path() -> Result<PathBuf, Error> {
        let result = home_dir();

        match result {
            None => Err(Error::NoHomeDirectory),
            Some(home_directory) => {
                Ok(home_directory.join(".dream-mirror.sqlite3"))
            }
        }
    }

    pub fn exists() -> Result<bool, Error> {
        let path = Self::path()?;
        Ok(path.exists())
    }

    pub fn new() -> Result<Self, Error> {
        let path = Self::path()?;
        let result = sqlite::open(&path);

        match result {
            Ok(connection) => {
                let db = Self { connection };
                db.load_schema()?;
                Ok(db)
            },
            Err(error) => Err(Error::CannotOpenDatabase(path, error))
        }
    }

    fn load_schema(&self) -> Result<(), Error> {
        Ok(())
    }
}