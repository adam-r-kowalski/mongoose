use std::collections::HashMap;

use compiler::lower::{lower, FileSystem};

enum Kind {
    File,
}

pub struct MockFileSystem {
    indices: Vec<usize>,
    kinds: Vec<Kind>,
    file_names: HashMap<String, usize>,
    sources: Vec<String>,
}

impl MockFileSystem {
    fn new() -> MockFileSystem {
        MockFileSystem {
            indices: vec![],
            kinds: vec![],
            sources: vec![],
            file_names: HashMap::new(),
        }
    }
}

impl FileSystem for MockFileSystem {
    fn read_file<'a>(&'a self, file_name: &str) -> Option<&'a str> {
        self.file_names
            .get(file_name)
            .map(|&index| self.sources[index].as_str())
    }
}

pub fn new_file(mut fs: MockFileSystem, file_name: &str, source: &str) -> (MockFileSystem, usize) {
    let index = fs.sources.len();
    fs.kinds.push(Kind::File);
    fs.indices.push(index);
    fs.sources.push(source.to_string());
    fs.file_names.insert(file_name.to_string(), index);
    (fs, index)
}

#[test]
fn test_lower_import() {
    let fs = MockFileSystem::new();
    let (fs, _) = new_file(
        fs,
        "start.mon",
        r#"
import foo: bar

fn start(): bar()
"#,
    );
    let (fs, _) = new_file(fs, "foo.mon", "fn bar(): 5");
    let ir = lower(fs, "start.mon");
    println!("{:?}", ir);
}
