mod command;
mod fs_index;
mod log;
mod result;

use command::Command;
use fs_index::FsIndex;
use log::Log;
use std::path::Path;

pub type Result<T> = result::Result<T>;
type LogPointer = u64;

pub struct KvStore {
    index: FsIndex,
    log: Log,
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let index = FsIndex::new(path)?;
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
            None => Err(failure::err_msg("Corrupt data, command found has no value")),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        if self.index.contains(&key) {
            let new_index = self.log.remove(&key)?;
            self.index.rebuild(new_index)?;
        }
        let command = Command::Set { key, value };
        let pointer = self.log.append(&command)?;

        self.index.add(command.get_key(), pointer)?;
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let pointer = self.index.get(&key);

        match pointer {
            None => Err(failure::err_msg("Key not found")),
            Some(_) => {
                let new_command_pointers = self.log.remove(&key)?;

                self.index.rebuild(new_command_pointers)
            }
        }
    }
}
