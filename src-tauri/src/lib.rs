mod db;

use colored::Colorize;
use db::DB;
use tauri::AppHandle;

enum Severity {
    Debug,
    Info,
    Warning,
    Error,
    Fatal
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
        Severity::Info => "INFO:".blue(),
        Severity::Warning => "WARNING:".yellow(),
        Severity::Error => "ERROR:".red(),
        Severity::Fatal => "FATAL:".purple()
    };

    println!("{} {}", severity_text, message);
}

fn db_exists() -> Result<bool, db::Error> {
    let db_path = DB::path()?;

    print_message(
        format!("Checking path: {}...", db_path.display()),
        Severity::Debug
    );
    DB::exists()
}

fn prep_database() {
    print_message("Looking for an existing database file...".to_string(), Severity::Debug);

    match db_exists() {
        Ok(true) => {
            print_message("Database file already exists.".to_string(), Severity::Info)
        },
        Ok(false) => {
            print_message("No database file found. Creating a new one.".to_string(), Severity::Info);
        },
        Err(error) => {
            print_message(
                format!("Error checking for database file: {}", error),
                Severity::Fatal
            )
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    banner();

    prep_database();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![exit_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
