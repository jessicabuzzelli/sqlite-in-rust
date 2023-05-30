use rustyline::{DefaultEditor};
use rustyline::error::ReadlineError;
use crate::commands::*;
use crate::handlers::{handle_dot_command, handle_sql_command};

pub fn run_cli_loop(mut repl: DefaultEditor, db: &String) {
    let prompt = "sqlite-rust> ";

    loop {
        let input = repl.readline(&prompt);

        match input {
            Ok(input) => {
                print_command_response(&input, &db);
            },
            Err(ReadlineError::Interrupted) => {
                println ! ("ctrl-c");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("ctrl-d");
                break
            },
            Err(_) => {
                println!("Uh oh! Something went wrong.");
                break
            }
        }
    }
}

fn print_command_response(input: &String, db: &String) {
    let input = input.trim();

    if input != "" {
        let command_response: Result<String, String>;

        if input.starts_with(".") {
            let cmd = DotCommand::new(input.to_owned());
            command_response = handle_dot_command(&cmd, &db);
        } else {
            let cmd = SqlCommand::new(input.to_owned());
            command_response = handle_sql_command(&cmd, &db);
        }

        match command_response {
            Ok(response) => println!("{response}"),
            Err(response) => println!("{response}"),
        }
    }
}
