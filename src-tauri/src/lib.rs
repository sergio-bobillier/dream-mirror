mod db;

use colored::Colorize;
use db::DB;
use std::path::PathBuf;
use tauri::AppHandle;
use thiserror::Error;

enum Severity {
    Debug,
    Info,
    Warning,
    Error,
    Fatal
}

#[derive(Error, Debug)]
enum Error {
    #[error("Database error: {0}")]
    Database(#[from] db::Error)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn exit_app(app: AppHandle) {
    app.exit(0);
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

fn print_message(message: String, severity: Severity) -> () {
    let severity_text = match severity {
        Severity::Debug => "DEBUG:".green(),
        Severity::Info => "INFO: ".blue(),
        Severity::Warning => "WARN: ".yellow(),
        Severity::Error => "ERROR:".red(),
        Severity::Fatal => "FATAL:".purple()
    };

    println!("{} {}", severity_text, message);
}

fn check_db_exists() -> Result<(PathBuf, bool), db::Error> {
    let db_path = DB::path()?;

    print_message(
        format!("Checking path: {}...", db_path.display()),
        Severity::Debug
    );

    Ok((db_path, DB::exists()?))
}

fn create_db() -> Result<DB, Error> {
    match DB::new() {
        Ok(db) => Ok(db),
        Err(error) => {
            print_message(
                format!("Error creating database: {}", error),
                Severity::Fatal
            );
            Err(Error::Database(error))
        }
    }
}

fn prep_database() -> Result<(), Error> {
    print_message("Looking for an existing database file...".to_string(), Severity::Debug);

    match check_db_exists() {
        Ok((path, exists)) => {
            if exists {
                print_message("Database file already exists.".to_string(), Severity::Info);
                Ok(())
            } else {
                print_message("No database file found.".to_string(), Severity::Info);
                print_message("Creating a new database...".to_string(), Severity::Info);
                print_message(format!("Loading schema to: {}...", path.display()), Severity::Debug);

                create_db()?;

                print_message("Database created successfully.".to_string(), Severity::Info);
                Ok(())
            }
        },
        Err(error) => {
            print_message(
                format!("Error checking for database file: {}", error),
                Severity::Fatal
            );
            Err(Error::Database(error))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    banner();

    match prep_database() {
        Ok(_) => {
            print_message("Database is ready.".to_string(), Severity::Debug);

            tauri::Builder::default()
                .plugin(tauri_plugin_opener::init())
                .invoke_handler(tauri::generate_handler![exit_app])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        },
        Err(error) => print_message(
            format!("Database initialization failed!\n\t{}", error),
            Severity::Fatal
        )
    }
}
