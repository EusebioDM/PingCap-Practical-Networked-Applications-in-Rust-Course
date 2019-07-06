mod command;

extern crate failure;

use std::collections::HashMap;
use failure::Error;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Seek, SeekFrom};
use command::Command;
use std::collections::btree_map::BTreeMap;

pub type Result<T> = std::result::Result<T, Error>;
type Key = String;
type LogPointer = u64;

pub struct KvStore {
    map: HashMap<Key, LogPointer>,
    indexes: BTreeMap<LogPointer, (Key, u64)>,
    log: File,
    index_file: File,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let log = fs::OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(path.join("log.txt"))?;

        let mut index_file = fs::OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(path.join("index.txt"))?;
        let (map, indexes) = initialize_from_file(&mut index_file)?;
        let store = KvStore { map, log, index_file, indexes };

        Ok(store)
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let (pointer, length) = self.save_to_log(key.clone(), value)?;

        self.map.insert(key.clone(), pointer);
        self.indexes.insert(pointer, (key, length));

        Ok(())
    }

    fn save_to_log(&mut self, key: String, value: String) -> Result<(LogPointer, u64)> {
        let command = Command::Set { key: key.clone(), value };
        let serialized = self.serialize(&command);
        let length = serialized.len();
        let command_pointer = self.get_next_pointer();
        let command_index_serialized = format!("{} {} {}\n", command_pointer, key, length).into_bytes();

        self.index_file.write_all(&command_index_serialized)?;
        self.log.write_all(&serialized)?;

        Ok((command_pointer, length as u64))
    }

    fn get_next_pointer(&self) -> LogPointer {
        let last = self.indexes.iter().next_back();

        match last {
            Some((pointer, (_, length))) => *pointer + *length,
            None => 0 // First pointer
        }
    }

    fn serialize(&self, command: &Command) -> Vec<u8> {
        let mut serialized = serde_json::to_string(&command).unwrap();
        serialized.push('\n');

        serialized.into_bytes()
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let pointer = self.map.get(&key);
        if pointer.is_none() { return Ok(None); }

        let command = self.get_command_at_pointer(*pointer.unwrap())?;
        match command {
            Command::Set { key: command_key, value } => {
                if command_key == key {
                    Ok(Some(value))
                } else {
                    Err(failure::err_msg("Corrupted data, actual key different from given"))
                }
            }
            _ => Err(failure::err_msg("Corrupted data, not a set command"))
        }
    }

    fn get_command_at_pointer(&mut self, pointer: LogPointer) -> Result<Command> {
        let mut reader = BufReader::new(&self.log);
        reader.seek(SeekFrom::Start(pointer))?;

        let mut line = String::new();
        reader.read_line(&mut line)?;


        match serde_json::from_str(&line) {
            Ok(command) => Ok(command),
            Err(_e) => Err(failure::err_msg("Corrupted data, command deserialization failed"))
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let removed = self.map.remove(&key);

        if removed.is_none() {
            return Err(failure::err_msg("Key not found"));
        }
        //self.log_command(&Command::Rm { key })?;

        Ok(())
    }
}

fn initialize_from_file(index_file: &mut File) -> Result<(HashMap<Key, LogPointer>, BTreeMap<LogPointer, (Key, u64)>)> {
    let mut map = HashMap::new();
    let mut indexes = BTreeMap::new();

    for line in BufReader::new(index_file).lines() {
        let (pointer, (key, length)) = deserialize_index_pair(line?)?;
        map.insert(key.clone(), pointer);
        indexes.insert(pointer, (key, length));
    }

    Ok((map, indexes))
}

fn deserialize_index_pair(line: String) -> Result<(LogPointer, (Key, u64))> {
    let mut split = line.split(' ');

    let pointer = split.next();
    let length = split.next_back();
    let key = split.next_back();

    if pointer.is_none() || key.is_none() || length.is_none() { return Err(failure::err_msg("Corrupted data, invalid index")); }

    Ok((pointer.unwrap().parse().unwrap(), (key.unwrap().to_string(), length.unwrap().parse().unwrap())))
}
