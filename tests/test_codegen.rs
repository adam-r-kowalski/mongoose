use std::{collections::HashMap, iter::FromIterator};

use pretty_assertions::assert_eq;

use smith::{
    codegen::{codegen, Function, Instruction, OperandKind, Wasm},
    parser::parse,
    tokenizer::tokenize,
};
use test_utilities::strings;

#[test]
fn test_codegen_int() {
    let tokens = tokenize("def start(): 0");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![Instruction::I64Const],
                operand_kinds: vec![vec![OperandKind::IntLiteral]],
                operands: vec![vec![0]],
                locals: vec![],
                name_to_local: HashMap::new(),
                symbols: strings(["start"]),
                ints: strings(["0"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}

#[test]
fn test_codegen_add() {
    let tokens = tokenize("def start(): 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![
                    Instruction::I64Const,
                    Instruction::I64Const,
                    Instruction::I64Add
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![]
                ],
                operands: vec![vec![0], vec![1], vec![]],
                locals: vec![],
                name_to_local: HashMap::new(),
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}

#[test]
fn test_codegen_subtract() {
    let tokens = tokenize("def start(): 5 - 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![
                    Instruction::I64Const,
                    Instruction::I64Const,
                    Instruction::I64Sub
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![]
                ],
                operands: vec![vec![0], vec![1], vec![]],
                locals: vec![],
                name_to_local: HashMap::new(),
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}

#[test]
fn test_codegen_multiply() {
    let tokens = tokenize("def start(): 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![
                    Instruction::I64Const,
                    Instruction::I64Const,
                    Instruction::I64Mul
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![]
                ],
                operands: vec![vec![0], vec![1], vec![]],
                locals: vec![],
                name_to_local: HashMap::new(),
                symbols: strings(["start"]),
                ints: strings(["5", "10"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}

#[test]
fn test_codegen_divide() {
    let tokens = tokenize("def start(): 10 / 5");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![
                    Instruction::I64Const,
                    Instruction::I64Const,
                    Instruction::I64DivS
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![]
                ],
                operands: vec![vec![0], vec![1], vec![]],
                locals: vec![],
                name_to_local: HashMap::new(),
                symbols: strings(["start"]),
                ints: strings(["10", "5"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}

#[test]
fn test_codegen_add_then_multiply() {
    let tokens = tokenize("def start(): 3 + 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![
                    Instruction::I64Const,
                    Instruction::I64Const,
                    Instruction::I64Const,
                    Instruction::I64Mul,
                    Instruction::I64Add
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![],
                    vec![]
                ],
                operands: vec![vec![0], vec![1], vec![2], vec![], vec![]],
                locals: vec![],
                name_to_local: HashMap::new(),
                symbols: strings(["start"]),
                ints: strings(["3", "5", "10"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}

#[test]
fn test_codegen_multiply_then_add() {
    let tokens = tokenize("def start(): 3 * 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![
                    Instruction::I64Const,
                    Instruction::I64Const,
                    Instruction::I64Mul,
                    Instruction::I64Const,
                    Instruction::I64Add
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![],
                    vec![OperandKind::IntLiteral],
                    vec![]
                ],
                operands: vec![vec![0], vec![1], vec![], vec![2], vec![]],
                locals: vec![],
                name_to_local: HashMap::new(),
                symbols: strings(["start"]),
                ints: strings(["3", "5", "10"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}

#[test]
fn test_codegen_local_variables() {
    let source = r#"
def start():
    x = 5
    y = 20
    x + y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![
                    Instruction::I64Const,
                    Instruction::SetLocal,
                    Instruction::I64Const,
                    Instruction::SetLocal,
                    Instruction::GetLocal,
                    Instruction::GetLocal,
                    Instruction::I64Add
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::Local],
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::Local],
                    vec![OperandKind::Local],
                    vec![OperandKind::Local],
                    vec![]
                ],
                operands: vec![vec![0], vec![0], vec![1], vec![1], vec![0], vec![1], vec![]],
                locals: strings(["$x", "$y"]),
                name_to_local: HashMap::from_iter([(String::from("x"), 0), (String::from("y"), 1)]),
                symbols: strings(["start", "x", "y", "x", "y"]),
                ints: strings(["5", "20"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}

#[test]
fn test_codegen_multiple_functions() {
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
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![
                Function {
                    name: 0,
                    instructions: vec![
                        Instruction::I64Const,
                        Instruction::I64Const,
                        Instruction::Call,
                    ],
                    operand_kinds: vec![
                        vec![OperandKind::IntLiteral],
                        vec![OperandKind::IntLiteral],
                        vec![OperandKind::Symbol],
                    ],
                    operands: vec![vec![0], vec![1], vec![1]],
                    locals: strings([]),
                    name_to_local: HashMap::new(),
                    symbols: strings(["start", "sum_of_squares"]),
                    ints: strings(["5", "3"]),
                    arguments: 0,
                },
                Function {
                    name: 0,
                    instructions: vec![
                        Instruction::GetLocal,
                        Instruction::Call,
                        Instruction::SetLocal,
                        Instruction::GetLocal,
                        Instruction::Call,
                        Instruction::SetLocal,
                        Instruction::GetLocal,
                        Instruction::GetLocal,
                        Instruction::I64Add,
                    ],
                    operand_kinds: vec![
                        vec![OperandKind::Local],
                        vec![OperandKind::Symbol],
                        vec![OperandKind::Local],
                        vec![OperandKind::Local],
                        vec![OperandKind::Symbol],
                        vec![OperandKind::Local],
                        vec![OperandKind::Local],
                        vec![OperandKind::Local],
                        vec![],
                    ],
                    operands: vec![
                        vec![0],
                        vec![4],
                        vec![2],
                        vec![1],
                        vec![7],
                        vec![3],
                        vec![2],
                        vec![3],
                        vec![]
                    ],
                    locals: strings(["$x", "$y", "$x2", "$y2"]),
                    name_to_local: HashMap::from_iter([
                        (String::from("x"), 0),
                        (String::from("y"), 1),
                        (String::from("x2"), 2),
                        (String::from("y2"), 3),
                    ]),
                    symbols: strings([
                        "sum_of_squares",
                        "x",
                        "y",
                        "x2",
                        "square",
                        "x",
                        "y2",
                        "square",
                        "y",
                        "x2",
                        "y2"
                    ]),
                    ints: vec![],
                    arguments: 2,
                },
                Function {
                    name: 0,
                    instructions: vec![
                        Instruction::GetLocal,
                        Instruction::GetLocal,
                        Instruction::I64Mul,
                    ],
                    operand_kinds: vec![vec![OperandKind::Local], vec![OperandKind::Local], vec![],],
                    operands: vec![vec![0], vec![0], vec![]],
                    locals: strings(["$x"]),
                    name_to_local: HashMap::from_iter([(String::from("x"), 0)]),
                    symbols: strings(["square", "x", "x", "x"]),
                    ints: vec![],
                    arguments: 1,
                },
            ],
            name_to_function: HashMap::from_iter([
                (String::from("start"), 0),
                (String::from("sum_of_squares"), 1),
                (String::from("square"), 2),
            ])
        }
    );
}

#[test]
fn test_codegen_single_line_if() {
    let source = r#"
def start():
  x = 5
  y = 10
  if x < y: x else: y"#;
    let tokens = tokenize(source);
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            functions: vec![Function {
                name: 0,
                instructions: vec![
                    Instruction::I64Const,
                    Instruction::SetLocal,
                    Instruction::I64Const,
                    Instruction::SetLocal,
                    Instruction::GetLocal,
                    Instruction::GetLocal,
                    Instruction::I64LtS,
                    Instruction::If,
                    Instruction::GetLocal,
                    Instruction::Else,
                    Instruction::GetLocal,
                    Instruction::End,
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::Local],
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::Local],
                    vec![OperandKind::Local],
                    vec![OperandKind::Local],
                    vec![],
                    vec![],
                    vec![OperandKind::Local],
                    vec![],
                    vec![OperandKind::Local],
                    vec![],
                ],
                operands: vec![
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![1],
                    vec![0],
                    vec![1],
                    vec![],
                    vec![],
                    vec![0],
                    vec![],
                    vec![1],
                    vec![]
                ],
                locals: strings(["$x", "$y"]),
                name_to_local: HashMap::from_iter([(String::from("x"), 0), (String::from("y"), 1)]),
                symbols: strings(["start", "x", "y", "x", "y", "x", "y"]),
                ints: strings(["5", "10"]),
                arguments: 0,
            }],
            name_to_function: HashMap::from_iter([(String::from("start"), 0)])
        }
    );
}
