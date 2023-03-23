// the CLI will accept 2 types of commands:
// 1) sql: create table, insert row
// 2) dot: .open <db>, .close

// SQLCommand

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
    pub fn new(command: String) -> SqlCommand {
        let first_word = command.trim().split(" ").collect::<Vec<&str>>()[0];
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

pub enum DotCommand {
    Exit,
    Help,
    NotYetImplemented(String),
    // Tables,
    // Save,
    // Open(String),
    // Database,
    Unknown(String),
}

impl DotCommand {
    pub fn new(command: String) -> DotCommand {
        let first_word = command.trim().split(" ").collect::<Vec<&str>>()[0];
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
    string
}

// CommandType

// pub enum CommandType {
//     DotCommand(DotCommand),
//     SqlCommand(SqlCommand),
// }
