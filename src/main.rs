extern crate clap;

mod commands;
mod processing;
mod cli;

use crate::cli::run_cli_loop;
use std::process::exit;
use clap::Command;

fn main() {

    //  init CLI
    let _matches = Command::new("sqlite-in-rust")
        .get_matches();

    //  init REPL editor
    let repl = rustyline::DefaultEditor::new();

    // let mut db = Database::new("tempdb".to_string());
    let db = "temp.db".to_string();

    match repl {
        Ok(repl) => {
            println!(
                "{}\n{}",
                "sqlite-in-rust",
                "Status: Connected \n",
            );

            run_cli_loop(repl, &db);
        },
        Err(_) => {
            println!(
                "{}\n{}",
                "sqlite-in-rust",
                "Status: Connection failed! \n",
            );

            exit(1)
        }
    }

}
