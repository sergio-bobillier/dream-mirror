use colored::Colorize;
use std::io::Write;

pub enum Severity {
    Debug,
    Info,
    Warning,
    Error,
    Fatal
}

pub struct Logger<T: Write> {
    writer: T,
}

impl<T: Write> Logger<T> {
    pub fn new(writer: T) -> Self {
        Logger { writer }
    }

    pub fn log_message(&mut self, message: String, severity: Severity) -> () {
        let severity_text = match severity {
            Severity::Debug => "DEBUG:".green(),
            Severity::Info => "INFO: ".blue(),
            Severity::Warning => "WARN: ".yellow(),
            Severity::Error => "ERROR:".red(),
            Severity::Fatal => "FATAL:".purple()
        };

        let result = self.writer.write(format!("{} {}\n", severity_text, message).as_bytes());

        if result.is_ok() {
            let _ = self.writer.flush();
        }
    }

    pub fn debug(&mut self, message: &str) -> () {
        self.log_message(message.to_string(), Severity::Debug);
    }
}