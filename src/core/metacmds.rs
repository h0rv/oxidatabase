use std::io::{self, Stdout, Write};

use crate::core::strings;

#[derive(Debug)]
pub enum Error {
    IGNORE,
    UNRECOGNIZED,
    GENERIC,
    EXIT,
}

pub fn print_prompt(stdout: &mut Stdout) -> Result<(), io::Error> {
    print!("{}", strings::PROMPT);
    stdout.flush()?;
    Ok(())
}

pub fn clear_screen(stdout: &mut Stdout) -> Result<(), io::Error> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    stdout.flush()?;
    Ok(())
}

pub fn print_help(stdout: &mut Stdout) -> Result<(), io::Error> {
    print!("{}", strings::HELP_MSG);
    stdout.flush()?;
    Ok(())
}

pub fn is_meta(input: &str) -> bool {
    if input.len() == 0 {
        return false;
    }
    return input.as_bytes()[0] == '.' as u8;
}

pub fn handler(input: &str, stdout: &mut Stdout) -> Result<(), Error> {
    match input {
        ".clear" => match clear_screen(stdout) {
            Ok(()) => return Ok(()),
            Err(_) => return Err(Error::IGNORE),
        },
        ".help" => match print_help(stdout) {
            Ok(()) => return Ok(()),
            Err(err) => {
                eprintln!(
                    "Failed to print help message due to the following error: {}",
                    err
                );
                return Err(Error::GENERIC);
            }
        },
        ".exit" => return Err(Error::EXIT),
        _ => return Err(Error::UNRECOGNIZED),
    }
}
