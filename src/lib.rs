mod command;

extern crate failure;

use std::collections::HashMap;
use failure::Error;
use std::path::{Path};
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use command::Command;

pub type Result<T> = std::result::Result<T, Error>;

pub struct KvStore {
    map: HashMap<String, String>,
    log: File,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let mut log = fs::OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(path.join("log.txt"))?;
        let map = get_log_data(&mut log)?;
        let store = KvStore { map, log };

        Ok(store)
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key.clone(), value.clone());
        let command = Command::Set { key, value };
        self.log_command(&command)?;

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let val = self.map.get(&key)
            .and_then(|value| Some(value.to_string()));

        Ok(val)
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let removed = self.map.remove(&key);

        if removed.is_none() {
            return Err(failure::err_msg("Key not found"));
        }
        self.log_command(&Command::Rm { key })?;

        Ok(())
    }

    fn log_command(&mut self, command: &Command) -> Result<()> {
        let serialized = serde_json::to_string(&command).unwrap();
        self.log.write_all(&serialized.into_bytes())?;
        self.log.write_all(&[b'\n'])?;

        Ok(())
    }
}

fn get_log_data(log: &mut File) -> Result<HashMap<String, String>> {
    let mut map = HashMap::new();

    for line in BufReader::new(log).lines() {
        let command: Command = serde_json::from_str(&line?)?;
        match command {
            Command::Set { key, value } => map.insert(key, value),
            Command::Rm { key } => map.remove(&key),
            _ => None
        };
    }

    Ok(map)
}