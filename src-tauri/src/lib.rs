use std::fs::remove_file;
use std::io::stdout;
use std::path::PathBuf;
use tauri::AppHandle;
use thiserror::Error;

mod console;
mod db;

use console::Logger;
use console::Severity;
use db::DB;
use db::objects::Character;
use db::objects::Characters;

#[derive(Error, Debug)]
enum Error {
    #[error("Database error: {0}")]
    Database(#[from] db::Error),
    #[error("Schema load error: {0}")]
    SchemaLoad(#[from] db::schema::Error),
    #[error("Database seeding error: {0}")]
    Seeding(#[from] db::seeder::Error),
    #[error("Database removal error. Could not delete database at: {0}")]
    DatabaseRemoval(PathBuf, #[source] std::io::Error)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn exit_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn fetch_characters() -> Result<Vec<Character>, String> {
    let message: &str;

    match Characters::all() {
        Ok(characters) => {
            match characters.with_roles() {
                Ok(characters_with_roles) => {
                    return Ok(characters_with_roles.characters());
                }
                Err(error) => {
                    message = "Failed to fetch character roles";
                    log_db_error(message, error);
                }
            }
        },
        Err(error) => {
            message = "Failed to fetch characters";
            log_db_error(message, error)
        }
    }

    Err(message.to_string())
}

fn banner() {
    println!(
        r#"

         ____                             __  __ _
        / __ \________  ____ _____ ___   /  |/  (_)_____________  _____
       / / / / ___/ _ \/ __ `/ __ `__ \ / /|_/ / / ___/ ___/ __ \/ ___/
      / /_/ / /  /  __/ /_/ / / / / / // /  / / / /  / /  / /_/ / /
     /_____/_/   \___/\__,_/_/ /_/ /_//_/  /_/_/_/  /_/   \____/_/

        "#
    );
}

fn log_db_error(message: &str, error: db::Error) {
    let mut logger = Logger::new(stdout());
    logger.log_message(format!("{}: {}", message, error), Severity::Error);
}

fn check_db_exists() -> Result<(PathBuf, bool), db::Error> {
    let db_path = DB::path()?;
    let mut logger = Logger::new(stdout());

    logger.log_message(
        format!("Checking path: {}...", db_path.display()),
        Severity::Debug
    );

    Ok((db_path, DB::exists()?))
}

fn delete_db() -> Result<(), Error> {
    match DB::path() {
        Ok(path) => {
            if let Err(error) = remove_file(&path) {
                return Err(Error::DatabaseRemoval(path, error));
            }

            Ok(())
        }
        Err(error) => {
            return Err(Error::Database(error));
        }
    }
}

fn create_db() -> Result<DB, Error> {
    let mut logger = Logger::new(stdout());

    match DB::new() {
        Ok(db) => Ok(db),
        Err(error) => {
            logger.log_message(
                format!("Error creating database: {}", error),
                Severity::Fatal
            );
            Err(Error::Database(error))
        }
    }
}

fn load_schema(db: &DB) -> Result<(), Error> {
    match db::schema::load(db) {
        Ok(result) => {
            Ok(result)
        }
        Err(error) => {
            Err(Error::SchemaLoad(error))
        }
    }
}

fn seed_database(db: &DB) -> Result<(), Error> {
    match db::seed(db) {
        Ok(result) => {
            Ok(result)
        }
        Err(error) => {
            Err(Error::Seeding(error))
        }
    }
}

fn prep_database() -> Result<(), Error> {
    let mut logger = Logger::new(stdout());

    logger.log_message("Looking for an existing database file...".to_string(), Severity::Debug);

    match check_db_exists() {
        Ok((path, exists)) => {
            if exists {
                logger.log_message("Database file already exists.".to_string(), Severity::Info);
                Ok(())
            } else {
                logger.log_message("No database file found.".to_string(), Severity::Info);
                logger.log_message("Creating a new database...".to_string(), Severity::Info);
                logger.log_message(format!("Database location: {}...", path.display()), Severity::Debug);

                let db = create_db()?;

                logger.log_message("Database created successfully.".to_string(), Severity::Info);
                logger.log_message("Loading schema...".to_string(), Severity::Info);

                load_schema(&db)?;

                logger.log_message("Schema loaded successfully.".to_string(), Severity::Info);

                logger.log_message("Seeding database...".to_string(), Severity::Info);

                seed_database(&db)?;

                Ok(())
            }
        },
        Err(error) => {
            logger.log_message(
                format!("Error checking for database file: {}", error),
                Severity::Fatal
            );
            Err(Error::Database(error))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut logger = Logger::new(stdout());

    logger.log_message("Hello".to_string(), Severity::Info);

    banner();

    match prep_database() {
        Ok(_) => {
            logger.log_message("Database is ready.".to_string(), Severity::Debug);

            tauri::Builder::default()
                .plugin(tauri_plugin_opener::init())
                .invoke_handler(tauri::generate_handler![exit_app, fetch_characters])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        },
        Err(error) => {
            logger.log_message(
                format!("Database initialization failed!\n\t{}", error),
                Severity::Fatal
            );

            if let Err(error) = delete_db() {
                logger.log_message(
                    format!("{}", error),
                    Severity::Fatal
                );
            }
        }
    }
}
