use dirs::home_dir;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot locate the user's home directory.")]
    NoHomeDirectory
}

pub struct DB {}

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
}