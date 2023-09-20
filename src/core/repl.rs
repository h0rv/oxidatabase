use crate::core::types::*;
use crate::core::{metacmds, parser};

use std::io::{self};
use std::io::{Stdin};

fn query_engine(statement: &Statement) {
    match statement.stype {
        StatementType::INSERT => {
            println!("Executing insert");
            return;
        }
        StatementType::SELECT => {
            println!("Executing select");
            return;
        }
    }
}

fn read_input(stdin: &mut Stdin) -> Result<String, io::Error> {
    let mut input = String::new();

    stdin.read_line(&mut input)?;
    // Remove new line at end of string
    input.pop();

    Ok(input)
}

pub fn start() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        if let Err(err) = metacmds::print_prompt(&mut stdout) {
            eprintln!("Error printing prompt: {}", err);
            continue;
        }
        let input = match read_input(&mut stdin) {
            Ok(input) => input,
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                continue;
            }
        };

        let input_ref = input.as_ref();

        if metacmds::is_meta(input_ref) {
            match metacmds::handler(input_ref, &mut stdout) {
                Ok(()) => continue,
                Err(metacmds::Error::IGNORE) => continue,
                Err(metacmds::Error::UNRECOGNIZED) => {
                    println!("Unrecognized command: {}", input);
                    continue;
                }
                Err(metacmds::Error::GENERIC) => {
                    println!("Error handling command. Exiting...");
                    return;
                }
                Err(metacmds::Error::EXIT) => return,
            }
        }

        let statement = match parser::parse(input_ref) {
            Ok(statement) => statement,
            Err(parser::Error::EMPTY) => continue,
            Err(parser::Error::SYNTAX) => {
                println!("Invalid syntax: {}", input);
                continue;
            }
            Err(parser::Error::UNRECOGNIZED) => {
                println!("Unrecognized statement: {}", input);
                continue;
            }
            Err(parser::Error::GENERIC) => {
                println!("Error handling command. Exiting...");
                return;
            }
        };

        query_engine(&statement);
    }
}
