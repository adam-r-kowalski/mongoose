use pretty_assertions::assert_eq;

use ra::{
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
            function: Function {
                instructions: vec![Instruction::I64Const],
                operand_kinds: vec![vec![OperandKind::IntLiteral]],
                operands: vec![vec![0]],
            },
            symbols: strings(["start"]),
            ints: strings(["0"]),
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
            function: Function {
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
            },
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
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
            function: Function {
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
            },
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
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
            function: Function {
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
            },
            symbols: strings(["start"]),
            ints: strings(["5", "10"]),
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
            function: Function {
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
            },
            symbols: strings(["start"]),
            ints: strings(["10", "5"]),
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
            function: Function {
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
            },
            symbols: strings(["start"]),
            ints: strings(["3", "5", "10"]),
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
            function: Function {
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
            },
            symbols: strings(["start"]),
            ints: strings(["3", "5", "10"]),
        }
    );
}
