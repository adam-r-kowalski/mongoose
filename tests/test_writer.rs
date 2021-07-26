use pretty_assertions::assert_eq;
use std::str;

use ra::{codegen::codegen, parser::parse, tokenizer::tokenize, writer::write};

#[test]
fn test_codegen() {
    let tokens = tokenize("start() -> i64: 0");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i32)
    (i32.const 0))

  (export "_start" (func $start)))"#
    );
}
