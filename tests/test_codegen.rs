use pretty_assertions::assert_eq;

use ra::{
    codegen::{codegen, Function, Instruction, OperandKind, Wasm},
    parser::parse,
    tokenizer::tokenize,
};

use test_utilities::strings;

#[test]
fn test_codegen_int() {
    let tokens = tokenize("start() -> i64: 0");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            function: Function {
                instructions: vec![Instruction::I32Const],
                operand_kinds: vec![vec![OperandKind::IntLiteral]],
                operands: vec![vec![0]],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["0"]),
        }
    );
}

#[test]
fn test_codegen_add() {
    let tokens = tokenize("start() -> i64: 5 + 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            function: Function {
                instructions: vec![
                    Instruction::I32Const,
                    Instruction::I32Const,
                    Instruction::I32Add
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![]
                ],
                operands: vec![vec![0], vec![1], vec![]],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["5", "10"]),
        }
    );
}

#[test]
fn test_codegen_multiply() {
    let tokens = tokenize("start() -> i64: 5 * 10");
    let ast = parse(tokens);
    let wasm = codegen(ast);
    assert_eq!(
        wasm,
        Wasm {
            function: Function {
                instructions: vec![
                    Instruction::I32Const,
                    Instruction::I32Const,
                    Instruction::I32Mul
                ],
                operand_kinds: vec![
                    vec![OperandKind::IntLiteral],
                    vec![OperandKind::IntLiteral],
                    vec![]
                ],
                operands: vec![vec![0], vec![1], vec![]],
            },
            symbols: strings(["start", "i64"]),
            ints: strings(["5", "10"]),
        }
    );
}
