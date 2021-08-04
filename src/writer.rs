use std::io::{Result, Write};

use crate::codegen::{Instruction, OperandKind, Wasm};

pub fn write_i64_const<W: Write>(mut buffer: W, wasm: &Wasm, i: usize) -> Result<W> {
    assert_eq!(
        wasm.function.operand_kinds[i],
        vec![OperandKind::IntLiteral]
    );
    let operands = &wasm.function.operands[i];
    assert_eq!(operands.len(), 1);
    let literal = &wasm.ints[operands[0]];
    write!(buffer, "\n    (i64.const {})", literal)?;
    Ok(buffer)
}

pub fn write_set_local<W: Write>(mut buffer: W, wasm: &Wasm, i: usize) -> Result<W> {
    assert_eq!(wasm.function.operand_kinds[i], vec![OperandKind::Local]);
    let operands = &wasm.function.operands[i];
    assert_eq!(operands.len(), 1);
    let local = &wasm.ints[operands[0]];
    write!(buffer, "\n    (set_local {})", local)?;
    Ok(buffer)
}

pub fn write_get_local<W: Write>(mut buffer: W, wasm: &Wasm, i: usize) -> Result<W> {
    assert_eq!(wasm.function.operand_kinds[i], vec![OperandKind::Local]);
    let operands = &wasm.function.operands[i];
    assert_eq!(operands.len(), 1);
    let local = &wasm.ints[operands[0]];
    write!(buffer, "\n    (get_local {})", local)?;
    Ok(buffer)
}

pub fn write_str<W: Write>(mut buffer: W, text: &str) -> Result<W> {
    write!(buffer, "\n    {}", text)?;
    Ok(buffer)
}

pub fn write<W: Write>(mut buffer: W, wasm: Wasm) -> Result<W> {
    write!(
        buffer,
        r#"(module
  (func $start (result i64)"#
    )?;
    wasm.function
        .instructions
        .iter()
        .enumerate()
        .try_fold(buffer, |buffer, (i, instruction)| match instruction {
            Instruction::I64Const => write_i64_const(buffer, &wasm, i),
            Instruction::I64Add => write_str(buffer, "i64.add"),
            Instruction::I64Sub => write_str(buffer, "i64.sub"),
            Instruction::I64Mul => write_str(buffer, "i64.mul"),
            Instruction::I64DivS => write_str(buffer, "i64.div_s"),
            Instruction::LocalSet => write_set_local(buffer, &wasm, i),
            Instruction::LocalGet => write_get_local(buffer, &wasm, i),
        })
        .and_then(|mut buffer| {
            write!(
                buffer,
                r#")

  (export "_start" (func $start)))"#
            )?;
            Ok(buffer)
        })
}
