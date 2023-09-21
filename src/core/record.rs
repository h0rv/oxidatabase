use std::{
    fmt::{self, Display},
    mem::size_of,
};

const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 255;

#[derive(Debug)]
pub struct Record {
    // Support different record schemas
    pub id: u32,
    pub username: [u8; USERNAME_SIZE],
    pub email: [u8; EMAIL_SIZE],
}

impl Record {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Record {
    fn default() -> Self {
        Self {
            id: 0,
            username: [0; USERNAME_SIZE],
            email: [0; EMAIL_SIZE],
        }
    }
}

fn username_to_string(arr: &[u8; USERNAME_SIZE]) -> String {
    String::from_utf8_lossy(arr).to_string()
}

fn email_to_string(arr: &[u8; EMAIL_SIZE]) -> String {
    String::from_utf8_lossy(arr).to_string()
}

impl Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.id,
            username_to_string(&self.username),
            email_to_string(&self.email)
        )
    }
}

pub const SIZE: usize = size_of::<Record>();

// pub fn size() -> usize {
//     let mut size = 0;
//     size += size_of::<Record::id>();
//     size += size_of::<Record::username>();
//     size += size_of::<Record::email>();
//     return size;
// }

// impl Serialize for Record {
//     fn serialize() -> Box<> {
//         // let box = Box::new(record_size());
//     }
