use rusqlite::{Connection, Result};

pub struct TableData {
    name: String,
    content: Vec<Vec<String>>
}

impl TableData {
    pub fn new(name: &String) -> Self {
        TableData {
            name: String::new(),
            content: Vec::new(),
        }
    }
}
