use pretty_assertions::assert_eq;

use std::str;

use ra::{codegen::codegen, parser::parse, tokenizer::tokenize, writer::write};

#[test]
fn test_write_int() {
    let tokens = tokenize("def start(): 0");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i64)
    (i64.const 0))

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_add() {
    let tokens = tokenize("def start(): 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.add)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_subtract() {
    let tokens = tokenize("def start(): 5 - 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.sub)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_multiply() {
    let tokens = tokenize("def start(): 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.mul)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_divide() {
    let tokens = tokenize("def start(): 10 / 5");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i64)
    (i64.const 10)
    (i64.const 5)
    i64.div_s)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_add_then_multiply() {
    let tokens = tokenize("def start(): 3 + 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i64)
    (i64.const 3)
    (i64.const 5)
    (i64.const 10)
    i64.mul
    i64.add)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_multiply_then_add() {
    let tokens = tokenize("def start(): 3 * 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i64)
    (i64.const 3)
    (i64.const 5)
    i64.mul
    (i64.const 10)
    i64.add)

  (export "_start" (func $start)))"#
    );
}

#[test]
fn test_write_local_variables() {
    let source = r#"
def start():
    x = 5
    y = 20
    x + y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let buffer = Vec::<u8>::new();
    let buffer = write(buffer, wasm).unwrap();
    assert_eq!(
        str::from_utf8(&buffer).unwrap(),
        r#"(module
  (func $start (result i64)
    (local $x i64)
    (local $y i64)
    (i64.const 5)
    (set_local $x)
    (i64.const 20)
    (set_local $y)
    (get_local $x)
    (get_local $y)
    i64.add)

  (export "_start" (func $start)))"#
    );
}
