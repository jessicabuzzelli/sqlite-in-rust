use crate::helpers::get_first_word;

// the CLI will accept 2 types of commands:
// 1) sql: create table, insert row
// 2) dot: .open <db>, .close

// SQLCommand

// ANSI-SQL command types
pub enum SqlCommand {
    CreateTable(String),
    // InsertRows(String),
    // DeleteTable(String),
    // UpdateTable(String),
    // Select(String),
    NotYetImplemented(String),
    Unknown(String),
}

impl SqlCommand {

    // generates a new SQLCommand based on the first word of the command
    pub fn new(command: String) -> SqlCommand {
        let first_word = get_first_word(&command);
        match first_word {
            "create" => SqlCommand::CreateTable(command),
            "insert" => SqlCommand::NotYetImplemented(command),
            "update" => SqlCommand::NotYetImplemented(command),
            "delete" => SqlCommand::NotYetImplemented(command),
            "select" => SqlCommand::NotYetImplemented(command),
            _ => SqlCommand::Unknown(command),
        }
    }
}

// DotCommand

// non-SQL, adminstrative CLI commands
pub enum DotCommand {
    Exit,
    Help,
    // Tables,
    // Save,
    // Open(String),
    // Database,
    NotYetImplemented(String),
    Unknown(String),
}

impl DotCommand {

    // generates a new DotCommand based on the first word of the command
    pub fn new(command: String) -> DotCommand {
        let first_word = get_first_word(&command);
        match first_word {
            ".exit" => DotCommand::Exit,
            ".quit" => DotCommand::Exit,
            ".help" => DotCommand::Help,
            ".open" => DotCommand::NotYetImplemented(command),
            ".tables" => DotCommand::NotYetImplemented(command),
            ".database" => DotCommand::NotYetImplemented(command),
            ".save" => DotCommand::NotYetImplemented(command),
            _ => DotCommand::Unknown(command),
        }
    }

}

// returns a string listing possible DotCommands and their usage
pub fn get_help_text() -> String {
    let string = format!(
        "{}{}{}{}{}{}{}",
        "Dot commands:\n",
        ".exit                   - Exit this prompt",
        ".help                   - Display command options and usage\n",
        ".open <database.db>     - Close existing database and open <database.db>\n",
        ".save <database.db>     - Write current database into <database.db>\n",
        ".tables                 - List names of tables in the current database\n",
        ".database               - Prints name of current database\n",
    );

    return string
}
