use super::Result;
use super::Command;
use super::LogPointer;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Seek, SeekFrom, BufRead, Write, Read};

pub struct Log {
    path: PathBuf
}

impl Log {
    pub fn new(path: &Path) -> Log {
        Log { path: path.to_path_buf() }
    }

    pub fn get(&self, pointer: LogPointer) -> Result<Option<String>> {
        let file = File::open(self.path.as_path())?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(pointer))?;

        let mut line = String::new();
        reader.read_line(&mut line);

        let value = Command::deserialize(line).and_then(|c| c.get_value());

        Ok(value)
    }

    pub fn append(&mut self, command: &Command) -> Result<LogPointer> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.path.as_path())?;

        let position = file.metadata()?.len();
        let line = command.serialize();
        file.write(line.as_bytes())?;

        Ok(position)
    }

    pub fn remove(&mut self, pointer: LogPointer) -> Result<HashMap<String, LogPointer>> {
        let mut file = File::open(self.path.as_path())?;
        let file_length = file.metadata()?.len();

        let mut log_data: Vec<u8> = Vec::with_capacity(file_length as usize);
        file.read_to_end(&mut log_data);
        drop(file);

        let log_lines = String::from_utf8(log_data)?.split(' ');
        let mut map = HashMap::new();
        llog_lines.iter().filter(|line| lin)

        unimplemented!()
    }
}