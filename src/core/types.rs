#[derive(Debug)]
pub enum StatementType {
    INSERT,
    SELECT,
}

const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 32;
#[derive(Debug)]
pub struct Record {
    // Support different record schemas
    pub id: u32,
    pub username: [char; USERNAME_SIZE],
    pub email: [char; EMAIL_SIZE],
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
pub struct Statement {
    pub stype: StatementType,
    pub record: Option<Record>,
}

#[derive(Debug)]
pub struct Query {
    pub str: String,
}
