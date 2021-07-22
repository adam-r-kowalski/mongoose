use std::{
    io::{Error, Write},
    mem::transmute,
};

use crate::types::x86::{Instruction, Kind, Register, X86};

fn write_instruction<W: Write>(mut write: W, instruction: Instruction) -> Result<W, Error> {
    match instruction {
        Instruction::Mov => write!(write, "mov")?,
        Instruction::Push => write!(write, "push")?,
        Instruction::Syscall => write!(write, "syscall")?,
    }
    Ok(write)
}

fn write_register<W: Write>(write: W, register: Register) -> Result<W, Error> {
    Ok(write)
}

fn write_arguments<W: Write>(
    write: W,
    literals: &[&str],
    operand_kinds: &[Kind],
    operands: &[usize],
) -> Result<W, Error> {
    operand_kinds.iter().zip(operands.iter()).try_fold(
        write,
        |mut write, (operand_kind, &operand)| match operand_kind {
            Kind::Int => {
                write!(write, "{}", operand)?;
                Ok(write)
            }
            Kind::Literal => {
                write!(write, "{}", literals[operand])?;
                Ok(write)
            }
            Kind::Register => write_register(write, unsafe { transmute(operand) }),
        },
    )
}

pub fn write_assembly<W: Write>(mut write: W, x86: &X86) -> Result<W, Error> {
    let header = "
    default rel
    global _main

    section .text

_main:
";
    write!(write, "{}", header)?;
    let &start_index = x86.name_to_top_level.get("start").unwrap();
    let start = &x86.top_level[start_index];
    assert_eq!(start.blocks.len(), 1);
    let block = &start.blocks[0];
    let write =
        block
            .instructions
            .iter()
            .enumerate()
            .try_fold(write, |write, (i, &instruction)| {
                let write = write_instruction(write, instruction)?;
                let write = write_arguments(
                    write,
                    &block.literals,
                    &block.operand_kinds[i],
                    &block.operands[i],
                )?;
                Ok(write)
            });
    write
}