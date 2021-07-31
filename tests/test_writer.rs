use std::str;

use ra::{codegen::codegen, parser::parse, tokenizer::tokenize, writer::write};

#[test]
fn test_write_int() {
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

#[test]
fn test_write_add() {
    let tokens = tokenize("start() -> i64: 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i32)
    (i32.const 5)
    (i32.const 10)
    i32.add)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_multiply() {
    let tokens = tokenize("start() -> i64: 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i32)
    (i32.const 5)
    (i32.const 10)
    i32.mul)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_add_then_multiply() {
    let tokens = tokenize("start() -> i64: 3 + 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i32)
    (i32.const 3)
    (i32.const 5)
    (i32.const 10)
    i32.mul
    i32.add)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_multiply_then_add() {
    let tokens = tokenize("start() -> i64: 3 * 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i32)
    (i32.const 3)
    (i32.const 5)
    i32.mul
    (i32.const 10)
    i32.add)

  (export "_start" (func $start)))"#
    );
}
