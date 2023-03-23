use crate::commands::{DotCommand, get_help_text, SqlCommand};
use std::process::exit;

pub fn process_sql_command(command: &SqlCommand, _db: &String) -> Result<String, String> {
    match command {
        SqlCommand::CreateTable(args) => {
            Ok(format!("Valid input: {}", args))
        }
        SqlCommand::NotYetImplemented(args) => {
            Ok(format!("Not yet implemented: {}", args))
        }
        SqlCommand::Unknown(args) => Err(format!(
                "Unknown command or invalid arguments: {}", args)
        )
    }
}

pub fn process_dot_command(command: &DotCommand, _db: &String) -> Result<String, String> {
    match command {
        DotCommand::Exit => exit(0),
        DotCommand::Help => Ok(get_help_text()),
        DotCommand::NotYetImplemented(args) => Ok(format!(
            "Command not yet implemented: {}", args)
        ),
        DotCommand::Unknown(args) => Err(format!(
            "Unknown command or invalid arguments: {}", args)
        ),
    }
}
