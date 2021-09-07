use std::collections::HashMap;

use compiler::codegen_next::{codegen, FileSystem};

#[derive(Debug)]
pub struct MockFileSystem {
    directories: Vec<HashMap<String, usize>>,
    files: Vec<HashMap<String, usize>>,
    sources: Vec<String>,
}

impl MockFileSystem {
    fn new() -> MockFileSystem {
        MockFileSystem {
            directories: vec![HashMap::new()],
            files: vec![HashMap::new()],
            sources: vec![],
        }
    }
}

impl FileSystem for MockFileSystem {
    fn read_file<'a>(&self, path: Vec<&str>) -> Option<String> {
        let mut index = 0;
        let last_index = path.len() - 1;
        for p in &path[..last_index] {
            match self.directories[index].get(&p.to_string()) {
                Some(&i) => index = i,
                None => return None,
            }
        }
        self.files[index]
            .get(path[last_index])
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

#[test]
fn test_read_file() {
    let fs = MockFileSystem::new();
    let fs = new_file(fs, vec!["a"], "a contents");
    let fs = new_file(fs, vec!["b"], "b contents");
    let fs = new_file(fs, vec!["c"], "c contents");
    let fs = new_file(fs, vec!["c", "d"], "c.d contents");
    let fs = new_file(fs, vec!["c", "d", "e"], "c.d.e contents");
    assert_eq!(fs.read_file(vec!["a"]), Some("a contents".to_string()));
    assert_eq!(fs.read_file(vec!["b"]), Some("b contents".to_string()));
    assert_eq!(fs.read_file(vec!["c"]), Some("c contents".to_string()));
    assert_eq!(
        fs.read_file(vec!["c", "d"]),
        Some("c.d contents".to_string())
    );
    assert_eq!(
        fs.read_file(vec!["c", "d", "e"]),
        Some("c.d.e contents".to_string())
    );
}

#[test]
fn test_lower_import() {
    let fs = MockFileSystem::new();
    let fs = new_file(
        fs,
        vec!["foo"],
        r#"
import bar: baz

fn start() -> i64: baz()
"#,
    );
    let fs = new_file(fs, vec!["bar"], "fn baz() -> i64: 5");
    println!("\n\n\n{:?}\n\n\n", fs);
    // let wasm = codegen(fs, "foo");
    // println!("{:?}", wasm);
}
