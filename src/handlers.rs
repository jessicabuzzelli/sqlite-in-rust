use crate::commands::{DotCommand, get_help_text, SqlCommand};
use std::process::exit;
use crate::helpers::get_first_word;
use crate::sql::parser::parse_sql_command;

pub fn handle_sql_command(command: &SqlCommand, _db: &String) -> Result<String, String> {
    match command {
        SqlCommand::CreateTable(args) => {
            Ok(format!("Not yet implemented: {}", get_first_word(&args)))
        }
        SqlCommand::NotYetImplemented(args) => {
            Ok(format!("Command not yet implemented: {}", get_first_word(&args)))
        }
        SqlCommand::Unknown(args) => Err(format!(
            "Unknown command: {}", get_first_word(&args))
        )
    }
}

pub fn handle_dot_command(command: &DotCommand, _db: &String) -> Result<String, String> {
    match command {
        DotCommand::Exit => exit(0),
        DotCommand::Help => Ok(get_help_text()),
        DotCommand::NotYetImplemented(args) => Ok(format!(
            "Command not yet implemented: {}", get_first_word(&args))
        ),
        DotCommand::Unknown(args) => Err(format!(
            "Unknown command: {}", get_first_word(&args))
        ),
    }
}
