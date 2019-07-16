extern crate serde;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

impl Command {
    pub fn get_key(&self) -> &str {
        unimplemented!();
    }

    pub fn serialize(&self) -> &str {
        unimplemented!();
    }

    pub fn deserialize(data: String) -> Option<Command> {
        unimplemented!();
    }

    pub fn get_value(&self) -> Option<String> {
        unimplemented!();
    }
}