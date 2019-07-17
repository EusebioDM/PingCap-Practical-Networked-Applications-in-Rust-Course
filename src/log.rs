use super::Result;
use super::Command;
use super::LogPointer;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Seek, SeekFrom, BufRead, Write, Read};

const FILE_NAME: &str = "log.txt";

pub struct Log {
    path: PathBuf
}

impl Log {
    pub fn new(path: &Path) -> Log {
        Log {
            path: path.join(FILE_NAME).to_path_buf()
        }
    }

    pub fn get(&self, pointer: LogPointer) -> Result<Option<String>> {
        let file = File::open(self.path.as_path())?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(pointer))?;

        let mut line = String::new();
        reader.read_line(&mut line)?;

        let value = Command::deserialize(&line).and_then(|c| c.get_value());

        Ok(value)
    }

    pub fn append(&mut self, command: &Command) -> Result<LogPointer> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.path.as_path())?;

        let position = file.metadata()?.len();
        let line = command.serialize();
        file.write(line.as_bytes())?;

        Ok(position)
    }

    pub fn remove(&mut self, key: &str) -> Result<HashMap<String, LogPointer>> {
        let mut file = File::open(self.path.as_path())?;
        let file_length = file.metadata()?.len();

        let mut log_data: Vec<u8> = Vec::with_capacity(file_length as usize);
        file.read_to_end(&mut log_data)?;
        drop(file);

        let log_str = String::from_utf8(log_data)?;
        let log_lines = log_str.lines();
        let mut map = HashMap::new();
        let mut pointer: LogPointer = 0;
        let mut new_log_data = String::new();
        for line in log_lines {
            let command = Command::deserialize(line).unwrap();
            if command.get_key() == key { continue; }

            map.insert(command.get_key().to_string(), pointer);
            pointer += line.len() as u64;
            new_log_data.push_str(line);
        }
        if !new_log_data.is_empty() && !new_log_data.ends_with('\n') {
            new_log_data.push('\n');
        }

        file = File::create(self.path.as_path())?;
        file.write_all(&new_log_data.into_bytes())?;

        Ok(map)
    }
}