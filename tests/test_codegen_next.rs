use compiler::{
    codegen_next::codegen,
    filesystem::{new_file, MockFileSystem},
};

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
    let _wasm = codegen(&fs, "foo");
}
