use crate::core::{
    record::{self, Record},
    statement::{self, Statement},
};

#[derive(Debug)]
pub enum Error {
    EMPTY,
    SYNTAX,
    UNRECOGNIZED,
    GENERIC,
}

pub fn parse(input: &str) -> Result<Statement, Error> {
    if input.starts_with("insert") {
        let mut record = Record::new();

        let mut index = 0;
        for s in input.split(" ") {
            // println!("{}", s);
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
                    for (i, b) in s.as_bytes().iter().enumerate() {
                        if i >= record.username.len() {
                            break;
                        }
                        record.username[i] = *b;
                    }
                }
                3 => {
                    for (i, b) in s.as_bytes().iter().enumerate() {
                        if i >= record.email.len() {
                            break;
                        }
                        record.email[i] = *b;
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

        println!("Parsed input into {:?}", record);
        println!("Record size: {}", record::SIZE);

        return Ok(Statement {
            stype: statement::Type::INSERT,
            record: Some(record),
        });
    }
    if input.starts_with("select") {
        return Ok(Statement {
            stype: statement::Type::SELECT,
            record: None,
        });
    }

    if input == "" {
        return Err(Error::EMPTY);
    }

    return Err(Error::UNRECOGNIZED);
}
