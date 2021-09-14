use tokio::runtime::Runtime;

use compiler::filesystem::{FileSystem, MockFileSystem};

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_read_file() {
    let mut fs = MockFileSystem::new();
    fs.new_file(vec!["a"], "a contents");
    fs.new_file(vec!["b"], "b contents");
    fs.new_file(vec!["c"], "c contents");
    fs.new_file(vec!["c", "d"], "c.d contents");
    fs.new_file(vec!["c", "d", "e"], "c.d.e contents");
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
