use tokio::runtime::Runtime;

use compiler::filesystem::{new_file, FileSystem, MockFileSystem};

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_read_file() {
    let fs = MockFileSystem::new();
    let fs = new_file(fs, vec!["a"], "a contents");
    let fs = new_file(fs, vec!["b"], "b contents");
    let fs = new_file(fs, vec!["c"], "c contents");
    let fs = new_file(fs, vec!["c", "d"], "c.d contents");
    let fs = new_file(fs, vec!["c", "d", "e"], "c.d.e contents");
    Runtime::new().unwrap().block_on(async {
        assert_eq!(
            fs.read_file(strings(&["a"])).await,
            Some("a contents".to_string())
        );
        assert_eq!(
            fs.read_file(strings(&["b"])).await,
            Some("b contents".to_string())
        );
        assert_eq!(
            fs.read_file(strings(&["c"])).await,
            Some("c contents".to_string())
        );
        assert_eq!(
            fs.read_file(strings(&["c", "d"])).await,
            Some("c.d contents".to_string())
        );
        assert_eq!(
            fs.read_file(strings(&["c", "d", "e"])).await,
            Some("c.d.e contents".to_string())
        );
    });
}
