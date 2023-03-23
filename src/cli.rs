use rustyline::{DefaultEditor};
use crate::commands::*;
use crate::processing::{process_dot_command, process_sql_command};

pub fn run_cli_loop(mut repl: DefaultEditor, db: &String) {
    let prompt = "sqlite-rust>> ";

    loop {
        let input = repl.readline(&prompt).unwrap();

        print_command_response(&input, &db);
    }
}

fn print_command_response(input: &String, db: &String) {
    let input = input.trim();

    if input != "" {
        let mut command_response: Result<String, String>;

        if input.starts_with(".") {
            let cmd = DotCommand::new(input.to_owned());
            command_response = process_dot_command(&cmd, &db);
        } else {
            let cmd = SqlCommand::new(input.to_owned());
            command_response = process_sql_command(&cmd, &db);
        }

        match command_response {
            Ok(response) => println!("{response}"),
            Err(response) => println!("{response}"),
        }
    }
}
