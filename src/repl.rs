use std::io::{self};
use std::io::{BufRead, StdinLock, StdoutLock, Write};

const PROMPT: &str = " oxi(db) ❯ ";
// const PROMPT: &str = " OxI(DB) ⚛ ";
// const PROMPT: &str = " oxidb ⚙ ";

const HELP_MSG: &str = "
Available commands:

    .clear - Clear screen
    .help  - Print this help message
    .exit  - Exit the REPL

";

#[derive(Debug)]
enum MetaCommandError {
    IGNORE,
    UNRECOGNIZED,
    GENERIC,
    EXIT,
}

#[derive(Debug)]
enum ParserError {
    EMPTY,
    SYNTAX,
    UNRECOGNIZED,
    GENERIC,
}

#[derive(Debug)]
enum StatementType {
    INSERT,
    SELECT,
}

const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 32;
#[derive(Debug)]
struct Record {
    // Support different record schemas
    id: u32,
    username: [char; USERNAME_SIZE],
    email: [char; EMAIL_SIZE],
}

impl Default for Record {
    fn default() -> Self {
        Self {
            id: 0,
            username: ['0' as char; USERNAME_SIZE],
            email: ['0' as char; EMAIL_SIZE],
        }
    }
}

#[derive(Debug)]
struct Statement {
    stype: StatementType,
    record: Option<Record>,
}

#[derive(Debug)]
struct Query {
    str: String,
}

fn is_meta_command(input: &str) -> bool {
    if input.len() == 0 {
        return false;
    }
    return input.as_bytes()[0] == '.' as u8;
}

fn handle_meta_command(input: &str, stdout_lock: &mut StdoutLock) -> Result<(), MetaCommandError> {
    match input {
        ".clear" => match clear_screen(stdout_lock) {
            Ok(()) => return Ok(()),
            Err(_) => return Err(MetaCommandError::IGNORE),
        },
        ".help" => match print_help(stdout_lock) {
            Ok(()) => return Ok(()),
            Err(err) => {
                eprintln!(
                    "Failed to print help message due to the following error: {}",
                    err
                );
                return Err(MetaCommandError::GENERIC);
            }
        },
        ".exit" => return Err(MetaCommandError::EXIT),
        _ => return Err(MetaCommandError::UNRECOGNIZED),
    }
}

fn parse_input(input: &str) -> Result<Statement, ParserError> {
    if input.starts_with("insert") {
        let mut record = Record::default();

        let mut index = 0;
        for s in input.split(" ") {
            println!("{}", s);
            match index {
                0 => {
                    // skip insert
                    ();
                }
                1 => {
                    record.id = match s.parse() {
                        Ok(id) => id,
                        Err(err) => {
                            eprintln!("Error parsing {} into the record id: {}", s, err);
                            return Err(ParserError::SYNTAX);
                        }
                    };
                }
                2 => {
                    for (i, c) in s.chars().enumerate() {
                        if i >= record.username.len() {
                            break;
                        }
                        record.username[i] = c;
                    }
                }
                3 => {
                    for (i, c) in s.chars().enumerate() {
                        if i >= record.email.len() {
                            break;
                        }
                        record.email[i] = c;
                    }
                }
                // TODO: Currently fixed length for record ([0] == insert)
                _ => {
                    eprintln!("Too many arguments for insert");
                    return Err(ParserError::SYNTAX);
                }
            }
            index += 1;
        }

        if index == 0 {
            eprintln!("No arguments given for insert");
            return Err(ParserError::SYNTAX);
        }

        return Ok(Statement {
            stype: StatementType::INSERT,
            record: Some(record),
        });
    }
    if input.starts_with("select") {
        return Ok(Statement {
            stype: StatementType::SELECT,
            record: None,
        });
    }

    if input == "" {
        return Err(ParserError::EMPTY);
    }

    return Err(ParserError::UNRECOGNIZED);
}

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

fn print_prompt(stdout_lock: &mut StdoutLock) -> Result<(), io::Error> {
    print!("{}", PROMPT);
    stdout_lock.flush()?;
    Ok(())
}

fn clear_screen(stdout_lock: &mut StdoutLock) -> Result<(), io::Error> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    stdout_lock.flush()?;
    Ok(())
}

fn print_help(stdout_lock: &mut StdoutLock) -> Result<(), io::Error> {
    print!("{}", HELP_MSG);
    stdout_lock.flush()?;
    Ok(())
}

fn read_input(stdin_lock: &mut StdinLock) -> Result<String, io::Error> {
    let mut input = String::new();

    stdin_lock.read_line(&mut input)?;
    // Remove new line at end of string
    input.pop();

    Ok(input)
}

pub fn start() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        if let Err(err) = print_prompt(&mut stdout.lock()) {
            eprintln!("Error printing prompt: {}", err);
            continue;
        }
        let input = match read_input(&mut stdin.lock()) {
            Ok(input) => input,
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                continue;
            }
        };

        let input_ref = input.as_ref();

        if is_meta_command(input_ref) {
            match handle_meta_command(input_ref, &mut stdout.lock()) {
                Ok(()) => continue,
                Err(MetaCommandError::IGNORE) => continue,
                Err(MetaCommandError::UNRECOGNIZED) => {
                    println!("Unrecognized command: {}", input);
                    continue;
                }
                Err(MetaCommandError::GENERIC) => {
                    println!("Error handling command. Exiting...");
                    return;
                }
                Err(MetaCommandError::EXIT) => return,
            }
        }

        let statement = match parse_input(input_ref) {
            Ok(statement) => statement,
            Err(ParserError::EMPTY) => continue,
            Err(ParserError::SYNTAX) => {
                println!("Invalid syntax: {}", input);
                continue;
            }
            Err(ParserError::UNRECOGNIZED) => {
                println!("Unrecognized statement: {}", input);
                continue;
            }
            Err(ParserError::GENERIC) => {
                println!("Error handling command. Exiting...");
                return;
            }
        };

        query_engine(&statement);
    }
}
