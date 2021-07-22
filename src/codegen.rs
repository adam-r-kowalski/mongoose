use std::collections::HashMap;

use crate::types::{
    ir::{self, Ir},
    x86::{self, Instruction, Kind, Register, TopLevel, X86},
};

impl<'a> x86::Block<'a> {
    fn new() -> x86::Block<'a> {
        x86::Block {
            instructions: vec![],
            operand_kinds: vec![],
            operands: vec![],
            literals: vec![],
        }
    }
}

fn instruction(mut block: x86::Block, instruction: Instruction) -> x86::Block {
    block.instructions.push(instruction);
    block.operand_kinds.push(vec![]);
    block.operands.push(vec![]);
    block
}

fn instruction_register(
    mut block: x86::Block,
    instruction: Instruction,
    register: Register,
) -> x86::Block {
    block.instructions.push(instruction);
    block.operand_kinds.push(vec![Kind::Register]);
    block.operands.push(vec![register as usize]);
    block
}

fn instruction_register_register(
    mut block: x86::Block,
    instruction: Instruction,
    to: Register,
    from: Register,
) -> x86::Block {
    block.instructions.push(instruction);
    block
        .operand_kinds
        .push(vec![Kind::Register, Kind::Register]);
    block.operands.push(vec![to as usize, from as usize]);
    block
}

fn instruction_register_literal<'a>(
    mut block: x86::Block<'a>,
    instruction: Instruction,
    register: Register,
    literal: &'a str,
) -> x86::Block<'a> {
    block.instructions.push(instruction);
    block
        .operand_kinds
        .push(vec![Kind::Register, Kind::Literal]);
    block
        .operands
        .push(vec![register as usize, block.literals.len()]);
    block.literals.push(literal);
    block
}

fn instruction_register_int(
    mut block: x86::Block,
    instruction: Instruction,
    register: Register,
    value: usize,
) -> x86::Block {
    block.instructions.push(instruction);
    block.operand_kinds.push(vec![Kind::Register, Kind::Int]);
    block.operands.push(vec![register as usize, value]);
    block
}

pub fn codegen<'a>(ir: &'a Ir) -> X86<'a> {
    let mut name_to_top_level = HashMap::new();
    let &ir::Entity(start) = ir.entities.name_to_entity.get("start").unwrap();
    let ir_top_level = &ir.top_level[start];
    name_to_top_level
        .try_insert(ir_top_level.name, start)
        .unwrap();
    let ir_block = &ir_top_level.environment.blocks[ir_top_level.value_block];
    let x86_block = x86::Block::new();
    let x86_block = instruction_register(x86_block, Instruction::Push, Register::Rbp);
    let x86_block =
        instruction_register_register(x86_block, Instruction::Mov, Register::Rbp, Register::Rsp);
    let x86_block =
        ir_block
            .kinds
            .iter()
            .enumerate()
            .fold(x86_block, |x86_block, (index, kind)| match kind {
                ir::Kind::Return => {
                    let return_entity = &ir_block.returns[ir_block.indices[index]];
                    let literal = ir_top_level
                        .environment
                        .entities
                        .literals
                        .get(return_entity)
                        .unwrap();
                    instruction_register_literal(
                        x86_block,
                        Instruction::Mov,
                        Register::Edi,
                        literal,
                    )
                }
                kind => panic!("kind {:?} is not yet supported in codegen", kind),
            });
    let x86_block = instruction_register_int(x86_block, Instruction::Mov, Register::Rax, 201);
    let x86_block = instruction(x86_block, Instruction::Syscall);
    X86 {
        top_level: vec![TopLevel {
            blocks: vec![x86_block],
        }],
        name_to_top_level,
    }
}
