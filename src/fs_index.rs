use super::Result;
use super::LogPointer;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const FILE_NAME: &str = "index.txt";

pub struct FsIndex {
    path: PathBuf,
    map: HashMap<String, LogPointer>,
}

impl FsIndex {
    pub fn new(path: &Path) -> Result<FsIndex> {
        let log_path = path.join(FILE_NAME);
        let map = FsIndex::build_map_from_file(&log_path)?;

        Ok(FsIndex {
            path: log_path,
            map,
        })
    }

    fn build_map_from_file(path: &PathBuf) -> Result<HashMap<String, LogPointer>> {
        let mut map = HashMap::new();

        if path.exists() {
            let log_data = std::fs::read_to_string(path)?;
            let lines = log_data.split('\n');
            for line in lines.filter(|&l| l != "") {
                let mut vals = line.split_whitespace();
                let key = vals.next().unwrap();
                let pointer = vals.next().unwrap();

                map.insert(key.to_string(), pointer.parse()?);
            }
        }

        Ok(map)
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

    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> Result<()> {
        self.map.remove(key);

        self.sync_data()
    }

    fn sync_data(&mut self) -> Result<()> {
        let mut data = String::new();

        for (key, &pointer) in self.map.iter() {
            let line = format!("{} {}\n", key, pointer);
            data.push_str(&line);
        }

        let mut file = File::create(self.path.as_path())?;
        file.write_all(&data.into_bytes())?;

        Ok(())
    }
}