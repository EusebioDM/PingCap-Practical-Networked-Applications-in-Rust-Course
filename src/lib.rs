mod command;
mod result;
mod fs_index;
mod log;

use std::collections::HashMap;
use std::path::Path;
use command::Command;
use std::collections::btree_map::BTreeMap;
use fs_index::FsIndex;
use log::Log;


pub type Result<T> = result::Result<T>;
type LogPointer = u64;

pub struct KvStore {
    index: FsIndex,
    log: Log,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let index = FsIndex::new(path);
        let log = Log::new(path);

        Ok(KvStore { index, log })
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if !self.index.contains(&key) {
            return Ok(None);
        }
        let pointer = self.index.get(&key).unwrap();
        let value = self.log.get(pointer)?;

        match value {
            Some(value) => Ok(Some(value)),
            None => Err(failure::err_msg("Corrupt data, command found has no value"))
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let old_value_pointer = self.index.get(&key);
        if let Some(p) = old_value_pointer {
            self.log.remove(p)?;
        }
        let command = Command::Set {key, value};
        let pointer = self.log.append(&command)?;

        self.index.add(command.get_key(), pointer)?;
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let pointer = self.index.get(&key);

        match pointer {
            None => return Err(failure::err_msg("Key not found")),
            Some(p) => {
                let new_command_pointers = self.log.remove(p)?;

                self.index.rebuild(new_command_pointers)
            }
        }

    }
}