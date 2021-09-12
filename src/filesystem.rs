use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait FileSystem {
    async fn read_file(&self, path: Vec<String>) -> Option<String>;
}

#[derive(Debug)]
pub struct MockFileSystem {
    directories: Vec<HashMap<String, usize>>,
    files: Vec<HashMap<String, usize>>,
    sources: Vec<String>,
}

impl MockFileSystem {
    pub fn new() -> MockFileSystem {
        MockFileSystem {
            directories: vec![HashMap::new()],
            files: vec![HashMap::new()],
            sources: vec![],
        }
    }
}

#[async_trait]
impl FileSystem for MockFileSystem {
    async fn read_file(&self, path: Vec<String>) -> Option<String> {
        let mut index = 0;
        let last_index = path.len() - 1;
        for p in &path[..last_index] {
            match self.directories[index].get(&p.to_string()) {
                Some(&i) => index = i,
                None => return None,
            }
        }
        self.files[index]
            .get(&path[last_index].to_string())
            .map(|&i| self.sources[i].to_string())
    }
}

pub fn new_file(mut fs: MockFileSystem, path: Vec<&str>, source: &str) -> MockFileSystem {
    let mut index = 0;
    let last_index = path.len() - 1;
    for p in &path[..last_index] {
        let len = fs.directories.len();
        index = *fs.directories[index].entry(p.to_string()).or_insert(len);
        if index == len {
            fs.directories.push(HashMap::new());
            fs.files.push(HashMap::new());
        }
    }
    fs.files[index].insert(path[last_index].to_string(), fs.sources.len());
    fs.sources.push(source.to_string());
    fs
}
