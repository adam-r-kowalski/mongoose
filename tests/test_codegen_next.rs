use compiler::{codegen_next::codegen, filesystem::MockFileSystem};

#[test]
fn test_lower_import() {
    let mut fs = MockFileSystem::new();
    fs.new_file(
        vec!["foo"],
        r#"
import bar: baz

fn start() -> i64: baz()
"#,
    );
    fs.new_file(vec!["bar"], "fn baz() -> i64: 5");
    let _wasm = codegen(&fs, "foo");
}
