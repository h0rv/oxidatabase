use crate::core::types::*;

#[derive(Debug)]
pub enum Error {
    EMPTY,
    SYNTAX,
    UNRECOGNIZED,
    GENERIC,
}

pub fn parse(input: &str) -> Result<Statement, Error> {
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
                            return Err(Error::SYNTAX);
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
                    return Err(Error::SYNTAX);
                }
            }
            index += 1;
        }

        if index != 4 {
            eprintln!("Not enough arguments provided");
            return Err(Error::SYNTAX);
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
        return Err(Error::EMPTY);
    }

    return Err(Error::UNRECOGNIZED);
}
