use std::str;

use pretty_assertions::assert_eq;
use wasmer::{imports, Instance, Module, Store, Value};

use ra::{codegen::codegen, parser::parse, tokenizer::tokenize, writer::write};

fn run(code: &[u8]) -> Value {
    let store = Store::default();
    let module = Module::new(&store, code).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();
    let start = instance.exports.get_function("_start").unwrap();
    start.call(&[]).unwrap()[0].clone()
}

#[test]
fn test_write_int() {
    let tokens = tokenize("def start(): 0");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
        r#"(module

  (func $start (result i64)
    (i64.const 0))

  (export "_start" (func $start)))"#
    );
    assert_eq!(run(&code), Value::I64(0));
}

#[test]
fn test_write_add() {
    let tokens = tokenize("def start(): 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
        r#"(module

  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.add)

  (export "_start" (func $start)))"#
    );
    assert_eq!(run(&code), Value::I64(15));
}

#[test]
fn test_write_subtract() {
    let tokens = tokenize("def start(): 5 - 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
        r#"(module

  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.sub)

  (export "_start" (func $start)))"#
    );
    assert_eq!(run(&code), Value::I64(-5));
}

#[test]
fn test_write_multiply() {
    let tokens = tokenize("def start(): 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
        r#"(module

  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.mul)

  (export "_start" (func $start)))"#
    );
    assert_eq!(run(&code), Value::I64(50));
}

#[test]
fn test_write_divide() {
    let tokens = tokenize("def start(): 10 / 5");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
        r#"(module

  (func $start (result i64)
    (i64.const 10)
    (i64.const 5)
    i64.div_s)

  (export "_start" (func $start)))"#
    );
    assert_eq!(run(&code), Value::I64(2));
}

#[test]
fn test_write_add_then_multiply() {
    let tokens = tokenize("def start(): 3 + 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
        r#"(module

  (func $start (result i64)
    (i64.const 3)
    (i64.const 5)
    (i64.const 10)
    i64.mul
    i64.add)

  (export "_start" (func $start)))"#
    );
    assert_eq!(run(&code), Value::I64(53));
}

#[test]
fn test_write_multiply_then_add() {
    let tokens = tokenize("def start(): 3 * 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
        r#"(module

  (func $start (result i64)
    (i64.const 3)
    (i64.const 5)
    i64.mul
    (i64.const 10)
    i64.add)

  (export "_start" (func $start)))"#
    );
    assert_eq!(run(&code), Value::I64(25));
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
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
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
    assert_eq!(run(&code), Value::I64(25));
}

#[test]
fn test_write_multiple_functions() {
    let source = r#"
def square(x): x * x

def sum_of_squares(x, y):
    x2 = square(x)
    y2 = square(y)
    x2 + y2

def start(): sum_of_squares(5, 3)"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(ast);
    let code = write(Vec::<u8>::new(), wasm).unwrap();
    assert_eq!(
        str::from_utf8(&code).unwrap(),
        r#"(module

  (func $start (result i64)
    (i64.const 5)
    (i64.const 3)
    (call $sum_of_squares))

  (func $sum_of_squares (param $x i64) (param $y i64) (result i64)
    (local $x2 i64)
    (local $y2 i64)
    (get_local $x)
    (call $square)
    (set_local $x2)
    (get_local $y)
    (call $square)
    (set_local $y2)
    (get_local $x2)
    (get_local $y2)
    i64.add)

  (func $square (param $x i64) (result i64)
    (get_local $x)
    (get_local $x)
    i64.mul)

  (export "_start" (func $start)))"#
    );
    assert_eq!(run(&code), Value::I64(34));
}
