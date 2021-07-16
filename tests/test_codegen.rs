use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::iter::FromIterator;

use ra::{
    codegen::codegen,
    lower::lower,
    parser::parse,
    types::{Instruction, OperandKind, Register, X86Block, X86},
};

#[test]
#[ignore]
fn codegen_literal() {
    let source = "(let start (Fn [] I32) (fn [] 0))";
    let ast = parse(source);
    let ir = lower(&ast);
    let x86 = codegen(&ir);
    assert_eq!(
        x86,
        X86 {
            blocks: vec![X86Block {
                instructions: vec![
                    Instruction::Push,
                    Instruction::Mov,
                    Instruction::Mov,
                    Instruction::Mov,
                    Instruction::Syscall
                ],
                operand_kinds: vec![
                    vec![OperandKind::Register],
                    vec![OperandKind::Register, OperandKind::Register],
                    vec![OperandKind::Register, OperandKind::Literal],
                    vec![OperandKind::Register, OperandKind::Int],
                    vec![],
                ],
                operands: vec![
                    vec![Register::Rbp as usize],
                    vec![Register::Rbp as usize, Register::Rsp as usize],
                    vec![Register::Edi as usize, 0],
                    vec![Register::Rax as usize, 201],
                ],
                literals: vec!["0"]
            }]
        }
    );
}
