use super::Result;
use super::LogPointer;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

pub struct FsIndex {
    path: PathBuf,
    map: HashMap<String, LogPointer>,
}

impl FsIndex {
    pub fn new(path: &Path) -> FsIndex {
        FsIndex {
            path: path.to_path_buf(),
            map: HashMap::new(),
        }
    }

    pub fn rebuild(&mut self, map: HashMap<String, LogPointer>) -> Result<()> {
        self.map = map;

        self.sync_data()
    }

    pub fn add(&mut self, key: &str, pointer: LogPointer) -> Result<()> {
        self.map.insert(key.to_string(), pointer);

        self.sync_data()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<LogPointer> {
        self.map.get(key)
            .map(|p| *p)
    }

    pub fn remove(&mut self, key: &str) -> Result<()> {
        self.map.remove(key);

        self.sync_data()
    }

    fn sync_data(&mut self) -> Result<()> {
        let mut data = String::new();

        for (key, &pointer) in self.map.iter() {
            let line = format!("{} {}", key, pointer);
            data.push_str(&line);
        }

        let mut file = File::create(self.path.as_path())?;
        file.write_all(&data.into_bytes())?;

        Ok(())
    }
}