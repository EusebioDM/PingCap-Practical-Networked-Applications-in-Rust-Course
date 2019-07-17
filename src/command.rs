extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

impl Command {
    pub fn get_key(&self) -> &str {
        match self {
            Command::Get { key } => key,
            Command::Set { key, .. } => key,
            Command::Rm { key } => key,
        }
    }

    pub fn serialize(&self) -> String {
        match self {
            Command::Get { key } => format!("get {}\n", key),
            Command::Set { key, value } => format!("set {} {}\n", key, value),
            Command::Rm { key } => format!("rm {}\n", key),
        }
    }

    pub fn deserialize(data: &str) -> Option<Command> {
        let mut values = data.trim().split_whitespace();
        let command_name = values.next()?;
        let key = values.next();
        let value = values.next();

        match command_name {
            "get" => Some(Command::Get {
                key: key?.to_string(),
            }),
            "set" => Some(Command::Set {
                key: key?.to_string(),
                value: value?.to_string(),
            }),
            "rm" => Some(Command::Rm {
                key: key?.to_string(),
            }),
            _ => None,
        }
    }

    pub fn get_value(&self) -> Option<String> {
        match self {
            Command::Get { .. } => None,
            Command::Set { value, .. } => Some(value.to_string()),
            Command::Rm { .. } => None,
        }
    }
}
