use pretty_assertions::assert_eq;

use ra::{
    codegen::codegen,
    lower::lower,
    parser::parse,
    types::x86::{Block, Instruction, Kind, Register, TopLevel, X86},
};

#[test]
fn codegen_literal() {
    let source = "(let start (Fn [] I32) (fn [] 0))";
    let ast = parse(source);
    let ir = lower(&ast);
    let x86 = codegen(&ir);
    assert_eq!(
        x86,
        X86 {
            top_level: vec![TopLevel {
                blocks: vec![Block {
                    instructions: vec![
                        Instruction::Push,
                        Instruction::Mov,
                        Instruction::Mov,
                        Instruction::Mov,
                        Instruction::Syscall
                    ],
                    operand_kinds: vec![
                        vec![Kind::Register],
                        vec![Kind::Register, Kind::Register],
                        vec![Kind::Register, Kind::Literal],
                        vec![Kind::Register, Kind::Int],
                        vec![],
                    ],
                    operands: vec![
                        vec![Register::Rbp as usize],
                        vec![Register::Rbp as usize, Register::Rsp as usize],
                        vec![Register::Edi as usize, 0],
                        vec![Register::Rax as usize, 201],
                    ],
                }],
                literals: vec!["0"]
            }]
        }
    );
}
