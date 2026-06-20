use dirs::home_dir;
use std::path::PathBuf;
use sqlite::Statement;
use thiserror::Error;

pub mod objects;
pub mod schema;
pub mod seeder;
pub use seeder::seed;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot locate the user's home directory.")]
    NoHomeDirectory,
    #[error("Database already exists")]
    DatabaseAlreadyExists,
    #[error("Cannot open the database file {0}: {1}")]
    ConnectionFailed(PathBuf, #[source] sqlite::Error),
    #[error("Failed to prepare statement from query: '{0}': {1}")]
    PrepareFailed(String, #[source] sqlite::Error),
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

        if path.exists() {
            return Err(Error::DatabaseAlreadyExists);
        }

        Self::connect(path)
    }

    pub fn open() -> Result<Self, Error> {
        let path = Self::path()?;
        Self::connect(path)
    }

    pub fn query(&self, query: &str) -> Result<Statement, Error> {
        match self.connection.prepare(query) {
            Ok(statement) => Ok(statement),
            Err(error) => Err(Error::PrepareFailed(query.to_string(), error))
        }
    }

    fn connect(path: PathBuf) -> Result<Self, Error> {
        let result = sqlite::open(&path);

        match result {
            Ok(connection) => {
                Ok(Self { connection })
            },
            Err(error) => Err(Error::ConnectionFailed(path, error))
        }
    }
}