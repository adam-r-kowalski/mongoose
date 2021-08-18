use std::io::{Result, Write};

use crate::codegen::{Function, Instruction, OperandKind, Wasm};

pub fn write_i64_const<W: Write>(mut buffer: W, func: &Function, i: usize) -> Result<W> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::IntLiteral]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    let literal = &func.ints[operands[0]];
    write!(buffer, "\n    (i64.const {})", literal)?;
    Ok(buffer)
}

pub fn write_set_local<W: Write>(mut buffer: W, func: &Function, i: usize) -> Result<W> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Local]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    let local = &func.locals[operands[0]];
    write!(buffer, "\n    (set_local {})", local)?;
    Ok(buffer)
}

pub fn write_get_local<W: Write>(mut buffer: W, func: &Function, i: usize) -> Result<W> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Local]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    let local = &func.locals[operands[0]];
    write!(buffer, "\n    (get_local {})", local)?;
    Ok(buffer)
}

pub fn write_call<W: Write>(mut buffer: W, func: &Function, i: usize) -> Result<W> {
    assert_eq!(func.operand_kinds[i], vec![OperandKind::Symbol]);
    let operands = &func.operands[i];
    assert_eq!(operands.len(), 1);
    let symbol = &func.symbols[operands[0]];
    write!(buffer, "\n    (call ${})", symbol)?;
    Ok(buffer)
}

pub fn write_str<W: Write>(mut buffer: W, text: &str) -> Result<W> {
    write!(buffer, "\n    {}", text)?;
    Ok(buffer)
}

fn write_arguments<W: Write>(buffer: W, func: &Function) -> Result<W> {
    func.locals[..func.arguments]
        .iter()
        .try_fold(buffer, |mut buffer, local| {
            write!(buffer, " (param {} i64)", local)?;
            Ok(buffer)
        })
}

fn write_locals<W: Write>(buffer: W, func: &Function) -> Result<W> {
    func.locals[func.arguments..]
        .iter()
        .try_fold(buffer, |mut buffer, local| {
            write!(buffer, "\n    (local {} i64)", local)?;
            Ok(buffer)
        })
}

fn write_function<W: Write>(mut buffer: W, func: &Function) -> Result<W> {
    write!(buffer, "\n\n  (func ${}", func.symbols[func.name])?;
    let mut buffer = write_arguments(buffer, &func)?;
    write!(buffer, " (result i64)")?;
    let buffer = write_locals(buffer, &func)?;
    let mut buffer =
        func.instructions
            .iter()
            .enumerate()
            .try_fold(buffer, |buffer, (i, instruction)| match instruction {
                Instruction::I64Const => write_i64_const(buffer, &func, i),
                Instruction::I64Add => write_str(buffer, "i64.add"),
                Instruction::I64Sub => write_str(buffer, "i64.sub"),
                Instruction::I64Mul => write_str(buffer, "i64.mul"),
                Instruction::I64DivS => write_str(buffer, "i64.div_s"),
                Instruction::I64RemS => write_str(buffer, "i64.rem_s"),
                Instruction::I64Eq => write_str(buffer, "i64.eq"),
                Instruction::I64LtS => write_str(buffer, "i64.lt_s"),
                Instruction::SetLocal => write_set_local(buffer, &func, i),
                Instruction::GetLocal => write_get_local(buffer, &func, i),
                Instruction::Call => write_call(buffer, &func, i),
                Instruction::If => write_str(buffer, "if (result i64)"),
                Instruction::Else => write_str(buffer, "else"),
                Instruction::End => write_str(buffer, "end"),
            })?;
    write!(buffer, ")")?;
    Ok(buffer)
}

pub fn write<W: Write>(mut buffer: W, wasm: Wasm) -> Result<W> {
    write!(buffer, "(module")?;
    let mut buffer = wasm.functions.iter().fold(buffer, |buffer, function| {
        write_function(buffer, function).unwrap()
    });
    write!(buffer, "\n\n  (export \"_start\" (func $start)))")?;
    Ok(buffer)
}
