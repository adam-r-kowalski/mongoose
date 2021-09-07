use std::str;

use pretty_assertions::assert_eq;
use wasmer::{imports, Instance, Module, Store, Value};

use compiler::{codegen::codegen, parser::parse, tokenizer::tokenize, writer::write};

fn run(code: &str) -> Value {
    let store = Store::default();
    let module = Module::new(&store, code).unwrap();
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object).unwrap();
    let start = instance.exports.get_function("_start").unwrap();
    start.call(&[]).unwrap()[0].clone()
}

#[test]
fn test_codegen_int() {
    let tokens = tokenize("fn start() -> i64: 0");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 0))

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(0));
}

#[test]
fn test_codegen_add() {
    let tokens = tokenize("fn start() -> i64: 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.add)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(15));
}

#[test]
fn test_codegen_subtract() {
    let tokens = tokenize("fn start() -> i64: 5 - 10");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.sub)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(-5));
}

#[test]
fn test_codegen_multiply() {
    let tokens = tokenize("fn start() -> i64: 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 5)
    (i64.const 10)
    i64.mul)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(50));
}

#[test]
fn test_codegen_divide() {
    let tokens = tokenize("fn start() -> i64: 10 / 5");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 10)
    (i64.const 5)
    i64.div_s)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(2));
}

#[test]
fn test_codegen_modulo_signed() {
    let tokens = tokenize("fn start() -> i64: 7 % 5");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 7)
    (i64.const 5)
    i64.rem_s)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(2));
}

#[test]
fn test_codegen_and() {
    let tokens = tokenize("fn start() -> i64: 7 & 5");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 7)
    (i64.const 5)
    i64.and)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(5));
}

#[test]
fn test_codegen_or() {
    let tokens = tokenize("fn start() -> i64: 7 | 5");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 7)
    (i64.const 5)
    i64.or)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(7));
}

#[test]
fn test_codegen_xor() {
    let tokens = tokenize("fn start() -> i64: 7 ^ 5");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 7)
    (i64.const 5)
    i64.xor)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(2));
}

#[test]
fn test_codegen_shift_left() {
    let tokens = tokenize("fn start() -> i64: 2 << 1");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 2)
    (i64.const 1)
    i64.shl)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(4));
}

#[test]
fn test_codegen_shift_right_signed() {
    let tokens = tokenize("fn start() -> i64: 8 >> 1");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 8)
    (i64.const 1)
    i64.shr_s)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(4));
}

#[test]
fn test_codegen_equal() {
    let tokens = tokenize("fn start() -> i64: if 8 == 1: 1 else: 0");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 8)
    (i64.const 1)
    i64.eq
    if (result i64)
    (i64.const 1)
    else
    (i64.const 0)
    end)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(0));
}

#[test]
fn test_codegen_not_equal() {
    let tokens = tokenize("fn start() -> i64: if 8 != 1: 1 else: 0");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 8)
    (i64.const 1)
    i64.ne
    if (result i64)
    (i64.const 1)
    else
    (i64.const 0)
    end)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(1));
}

#[test]
fn test_codegen_less_than() {
    let tokens = tokenize("fn start() -> i64: if 8 < 1: 1 else: 0");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 8)
    (i64.const 1)
    i64.lt_s
    if (result i64)
    (i64.const 1)
    else
    (i64.const 0)
    end)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(0));
}

#[test]
fn test_codegen_less_than_equal() {
    let tokens = tokenize("fn start() -> i64: if 8 <= 1: 1 else: 0");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 8)
    (i64.const 1)
    i64.le_s
    if (result i64)
    (i64.const 1)
    else
    (i64.const 0)
    end)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(0));
}

#[test]
fn test_codegen_greater_than() {
    let tokens = tokenize("fn start() -> i64: if 8 > 1: 1 else: 0");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 8)
    (i64.const 1)
    i64.gt_s
    if (result i64)
    (i64.const 1)
    else
    (i64.const 0)
    end)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(1));
}

#[test]
fn test_codegen_greater_than_equal() {
    let tokens = tokenize("fn start() -> i64: if 8 >= 1: 1 else: 0");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 8)
    (i64.const 1)
    i64.ge_s
    if (result i64)
    (i64.const 1)
    else
    (i64.const 0)
    end)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(1));
}

#[test]
fn test_codegen_add_then_multiply() {
    let tokens = tokenize("fn start() -> i64: 3 + 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 3)
    (i64.const 5)
    (i64.const 10)
    i64.mul
    i64.add)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(53));
}

#[test]
fn test_codegen_multiply_then_add() {
    let tokens = tokenize("fn start() -> i64: 3 * 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (i64.const 3)
    (i64.const 5)
    i64.mul
    (i64.const 10)
    i64.add)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(25));
}

#[test]
fn test_codegen_local_variables() {
    let source = r#"
fn start() -> i64:
    x = 5
    y = 20
    x + y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

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

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(25));
}

#[test]
fn test_codegen_assignment() {
    let source = r#"
fn start() -> i64:
    x = 5
    x = x + 20
    x"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (local $x i64)
    (i64.const 5)
    (set_local $x)
    (get_local $x)
    (i64.const 20)
    i64.add
    (set_local $x)
    (get_local $x))

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(25));
}

#[test]
fn test_codegen_multiple_functions() {
    let source = r#"
fn square(x: i64) -> i64: x * x

fn sum_of_squares(x: i64, y: i64) -> i64:
    x2 = square(x)
    y2 = square(y)
    x2 + y2

fn start() -> i64: sum_of_squares(5, 3)"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

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

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(34));
}

#[test]
fn test_codegen_single_line_if() {
    let source = r#"
fn start() -> i64:
  x = 5
  y = 10
  if x < y: x else: y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (local $x i64)
    (local $y i64)
    (i64.const 5)
    (set_local $x)
    (i64.const 10)
    (set_local $y)
    (get_local $x)
    (get_local $y)
    i64.lt_s
    if (result i64)
    (get_local $x)
    else
    (get_local $y)
    end)

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(5));
}

#[test]
fn test_codegen_while() {
    let source = r#"
fn start() -> i64:
    i = 0
    while i < 10:
        i = i + 1
    i
"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(&ast);
    let code = write(&wasm);
    assert_eq!(
        code,
        r#"
(module

  (func $start (result i64)
    (local $i i64)
    (i64.const 0)
    (set_local $i)
    block $.label.0
    loop $.label.1
    (get_local $i)
    (i64.const 10)
    i64.lt_s
    i32.eqz
    br_if $.label.0
    (get_local $i)
    (i64.const 1)
    i64.add
    (set_local $i)
    br $.label.1
    end $.label.1
    end $.label.0
    (get_local $i))

  (export "_start" (func $start)))
"#
    );
    assert_eq!(run(&code), Value::I64(10));
}
