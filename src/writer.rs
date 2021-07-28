use std::io::{Result, Write};

use crate::codegen::{Instruction, OperandKind, Wasm};

pub fn write_i32_const<W: Write>(mut buffer: W, wasm: &Wasm, i: usize) -> Result<W> {
    assert_eq!(
        wasm.function.operand_kinds[i],
        vec![OperandKind::IntLiteral]
    );
    let operands = &wasm.function.operands[i];
    assert_eq!(operands.len(), 1);
    let literal = &wasm.ints[operands[0]];
    write!(buffer, "\n    (i32.const {})", literal)?;
    Ok(buffer)
}

pub fn write_i32_add<W: Write>(mut buffer: W) -> Result<W> {
    write!(buffer, "\n    i32.add")?;
    Ok(buffer)
}

pub fn write<W: Write>(mut buffer: W, wasm: Wasm) -> Result<W> {
    write!(
        buffer,
        r#"(module
  (func $start (result i32)"#
    )?;
    wasm.function
        .instructions
        .iter()
        .enumerate()
        .try_fold(buffer, |buffer, (i, instruction)| match instruction {
            Instruction::I32Const => write_i32_const(buffer, &wasm, i),
            Instruction::I32Add => write_i32_add(buffer),
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
